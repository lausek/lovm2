import pytest

from .deps import *

from pylovm2 import LV2Expr, LV2Module, LV2Variable


class TestBuilding(Test):
    def test_assign(self, internals):
        main_hir = internals.main
        main_hir.assign("n", 5)
        main_hir.assign("f", 5.0)
        main_hir.assign("b", True)
        main_hir.assign("s", "string")

        result = internals.mod.build()

        self.assertIsInstance(result, LV2Module)

    def test_assign_global(self, internals):
        main_hir = internals.main
        main_hir.assign_global("n", 5)
        main_hir.interrupt(10)

        result = internals.mod.build()

        self.assertIsInstance(result, LV2Module)

        def testfn(ctx):
            assert 5 == int(ctx.globals("n"))

        self.run_module_test(result, testfn)

    def test_expressions(self, internals):
        main_hir = internals.main
        main_hir.assign("a", LV2Expr(1).add(1))
        main_hir.assign("b", LV2Expr(1).sub(1))
        main_hir.assign("c", LV2Expr(2).mul(3))
        main_hir.assign("d", LV2Expr(2).div(3))
        main_hir.assign("e", LV2Expr(2).rem(3))
        main_hir.assign("f", LV2Expr(True).land(False))
        main_hir.assign("g", LV2Expr(True).lor(False))
        main_hir.assign("h", LV2Expr(True).lnot())

        result = internals.mod.build()

        self.assertIsInstance(result, LV2Module)

    def test_expression_deep(self, internals):
        main_hir = internals.main
        main_hir.assign("a", LV2Expr(15).rem(5).eq(0))

        result = internals.mod.build()

        self.assertIsInstance(result, LV2Module)

    def test_repeat_until(self, internals):
        i = LV2Variable("i")

        main_hir = internals.main
        main_hir.assign(i, 0)
        main_hir.repeat_until(LV2Expr(i).eq(10)).assign(i, LV2Expr(i).add(1))
        main_hir.interrupt(10)

        self.run_module_test(internals.mod.build(), lambda ctx: None)

    def test_repeat_endless(self, internals):
        i = LV2Variable("i")

        main_hir = internals.main
        main_hir.assign(i, 0)

        repeat = main_hir.repeat()
        repeat.branch().add_condition(LV2Expr(i).eq(10)).repeat_break()
        repeat.assign(i, LV2Expr(i).add(1))

        main_hir.interrupt(10)

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            self.assertEqual(10, int(frame.local(i)))
            self.assertEqual(10, int(frame.local(i)))

        self.run_module_test(internals.mod.build(), validate)

    def test_repeat_endless_continue(self, internals):
        i = LV2Variable("i")

        main_hir = internals.main
        main_hir.assign(i, 0)
        repeat = main_hir.repeat()
        repeat.assign(i, LV2Expr(i).add(1))
        repeat.branch().add_condition(LV2Expr(i).ne(10)).repeat_continue()
        repeat.repeat_break()
        main_hir.interrupt(10)

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            self.assertEqual(10, int(frame.local(i)))

        result = internals.mod.build()
        self.run_module_test(result, validate)

    def test_branching(self, internals):
        a, result = LV2Variable("a"), LV2Variable("result")

        main_hir = internals.main
        main_hir.assign(a, 5)

        branch = main_hir.branch()
        branch.add_condition(LV2Expr(a).eq(3)).assign(result, "fizz")
        branch.add_condition(LV2Expr(a).eq(5)).assign(result, "buzz")
        branch.default_condition().assign(result, "none")
        main_hir.interrupt(10)

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            self.assertEqual("buzz", str(frame.local(result)))

        self.run_module_test(internals.mod.build(), validate)

    def test_with_arguments(self, internals):
        n = LV2Variable("n")

        main_hir = internals.main
        main_hir.assign("doubled", LV2Expr.call("double", 5))
        main_hir.interrupt(10)

        double_hir = internals.mod.add("double", [n])
        double_hir.ret(LV2Expr(n).mul(2))

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            self.assertEqual(10, int(frame.local("doubled")))

        result = internals.mod.build()
        self.run_module_test(result, validate)

    def test_instantiation(self, internals):
        main_hir = internals.main
        main_hir.assign("d", {1: 1, "2": 2, "3": True})
        main_hir.assign("l", [1, 2, 3])
        main_hir.interrupt(10)

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            print(frame.local("d"))
            self.assertEqual(1, frame.local("d")[1])
            self.assertEqual(2, frame.local("d")["2"])
            self.assertEqual(True, frame.local("d")["3"])

            self.assertEqual(1, frame.local("l")[0])
            self.assertEqual(2, frame.local("l")[1])
            self.assertEqual(3, frame.local("l")[2])

        result = internals.mod.build()
        self.run_module_test(result, validate)

    def test_pynative_function(self, internals):
        internals.mod.add_pyfn("powr", lambda x: int(x) ** 2)
        result = internals.mod.build()
        internals.vm.add_module(result)
        self.assertEqual(4, int(internals.vm.call("main.powr", 2)))

    def test_repeating_iterator(self, internals):
        c, i, sum, it = (
            LV2Variable("c"),
            LV2Variable("i"),
            LV2Variable("sum"),
            LV2Variable("it"),
        )

        sum_hir = internals.mod.add("sum")
        sum_hir.assign(sum, 0)
        sum_hir.repeat_iterating([1, 2, 3, 4], i).assign(sum, LV2Expr(sum).add(i))
        sum_hir.ret(sum)

        iter_sum_hir = internals.mod.add("iter_sum", [c])
        iter_sum_hir.assign(sum, 0)
        iter_sum_hir.assign(it, LV2Expr(c).to_iter())
        iter_sum_hir.repeat_iterating(it, i).assign(sum, LV2Expr(sum).add(i))
        iter_sum_hir.ret(sum)

        result = internals.mod.build()
        internals.vm.add_module(result)

        assert 10 == int(internals.vm.call("main.sum"))
        assert 15 == int(internals.vm.call("main.iter_sum", [1, 2, 3, 4, 5]))

    def test_iterators(self, internals):
        main_hir = internals.main
        main_hir.assign_global("r1", LV2Expr.range(5))
        main_hir.assign_global("r2", LV2Expr.range(-5, 0))
        main_hir.assign_global("r3", LV2Expr.range(5).reverse())

        result = internals.mod.build()
        vm = internals.vm
        vm.add_main_module(result)
        vm.run()

        self.assertEqual([0, 1, 2, 3, 4], vm.ctx().globals("r1"))
        self.assertEqual([-5, -4, -3, -2, -1], vm.ctx().globals("r2"))
        self.assertEqual([4, 3, 2, 1, 0], vm.ctx().globals("r3"))

    def test_shifting_value(self, internals):
        main_hir = internals.main
        main_hir.assign_global("a", LV2Expr(2).shl(2))
        main_hir.assign_global("b", LV2Expr(16).shr(2))
        main_hir.interrupt(10)
        module = internals.mod.build()

        def testfn(ctx):
            assert 8 == int(ctx.globals("a"))
            assert 4 == int(ctx.globals("b"))

        self.run_module_test(module, testfn)

    def test_building_expr(self, internals):
        n, m = LV2Variable("n"), LV2Variable("m")

        main_hir = internals.main
        main_hir.assign_global(n, LV2Expr(1).add(2))
        main_hir.assign_global(m, LV2Expr(n).add(2))
        main_hir.interrupt(10)

        def testfn(ctx):
            assert 3 == int(ctx.globals(n))
            assert 5 == int(ctx.globals(m))

        self.run_module_test(internals.mod.build(), testfn)

    def test_putting_expr(self, internals):
        var, val = LV2Variable("var"), LV2Variable("val")

        internals.main.step(LV2Expr.call("wrt", 5))
        internals.main.interrupt(10)

        wrt_hir = internals.mod.add("wrt", [val])
        wrt_hir.assign_global(var, val)

        def testfn(ctx):
            assert 5 == int(ctx.globals(var))

        self.run_module_test(internals.mod.build(), testfn)

    def test_manual_iteration(self, internals):
        it, c, has = LV2Variable("it"), LV2Variable("c"), LV2Variable("has")

        main_hir = internals.main
        main_hir.assign_global(c, 0)
        main_hir.assign_global(it, LV2Expr([1, 2, 3, 4]).to_iter())
        repeat = main_hir.repeat_until(LV2Expr(it).has_next().lnot())
        repeat.assign_global(c, LV2Expr(it).next().add(c))
        main_hir.assign_global(has, LV2Expr(it).has_next())
        main_hir.interrupt(10)

        def testfn(ctx):
            assert 10 == int(ctx.globals(c))
            assert False == ctx.globals(has)

        self.run_module_test(internals.mod.build(), testfn)
