import pytest

from .deps import *

from pylovm2 import Expr

class TestBuilding(Test):
    def test_assign(self, internals):
        main_hir = internals.main
        main_hir.assign('n', 5)
        main_hir.assign('f', 5.0)
        main_hir.assign('b', True)
        main_hir.assign('s', 'string')

        result = internals.mod.build()

        self.assertIsInstance(result, pylovm2.Module)

    def test_assign_global(self, internals):
        main_hir = internals.main
        main_hir.assign_global('n', 5)
        main_hir.interrupt(10)

        result = internals.mod.build()

        self.assertIsInstance(result, pylovm2.Module)

        def testfn(ctx):
            assert 5 == int(ctx.globals('n'))

        self.run_module_test(result, testfn)

    def test_expressions(self, internals):
        main_hir = internals.main
        main_hir.assign('a', Expr.add(1, 1))
        main_hir.assign('b', Expr.sub(1, 1))
        main_hir.assign('c', Expr.mul(2, 3))
        main_hir.assign('d', Expr.div(2, 3))
        main_hir.assign('e', Expr.rem(2, 3))
        main_hir.assign('f', Expr.land(True, False))
        main_hir.assign('g', Expr.lor(True, False))
        main_hir.assign('h', Expr.lnot(True))

        result = internals.mod.build()

        self.assertIsInstance(result, pylovm2.Module)

    def test_expression_deep(self, internals):
        main_hir = internals.main
        main_hir.assign('a', Expr.eq(Expr.rem(15, 5), 0))

        result = internals.mod.build()

        self.assertIsInstance(result, pylovm2.Module)

    def test_repeat(self, internals):
        main_hir = internals.main
        main_hir.assign('i', 0)
        main_hir.repeat_until(Expr.eq(Expr.var('i'), 10)).assign('i', Expr.add(Expr.var('i'), 1))
        main_hir.interrupt(10)

        result = internals.mod.build()
        self.run_module_test(result, lambda ctx: None)

    def test_repeat_endless(self, internals):
        main_hir = internals.main
        main_hir.assign('i', 0)
        repeat = main_hir.repeat()
        branch = repeat.branch().add_condition(Expr.eq(Expr.var('i'), 10))
        branch.repeat_break()
        repeat.assign('i', Expr.add(Expr.var('i'), 1))
        main_hir.interrupt(10)

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            self.assertEqual(10, int(frame.local('i')))

        result = internals.mod.build()
        self.run_module_test(result, validate)

    def test_repeat_endless_continue(self, internals):
        main_hir = internals.main
        main_hir.assign('i', 0)
        repeat = main_hir.repeat()
        repeat.assign('i', Expr.add(Expr.var('i'), 1))
        branch = repeat.branch().add_condition(Expr.ne(Expr.var('i'), 10))
        branch.repeat_continue()
        repeat.repeat_break()
        main_hir.interrupt(10)

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            self.assertEqual(10, int(frame.local('i')))

        result = internals.mod.build()
        self.run_module_test(result, validate)

    def test_branching(self, internals):
        main_hir = internals.main
        main_hir.assign('a', 5)
        branch = main_hir.branch()
        branch.add_condition(Expr.eq(Expr.var('a'), 3)).assign('result', 'fizz')
        branch.add_condition(Expr.eq(Expr.var('a'), 5)).assign('result', 'buzz')
        branch.default_condition().assign('result', 'none')
        main_hir.interrupt(10)

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            self.assertEqual('buzz', str(frame.local('result')))

        result = internals.mod.build()
        self.run_module_test(result, validate)

    def test_with_arguments(self, internals):
        main_hir = internals.main
        main_hir.assign('doubled', Expr.call('double', 5))
        main_hir.interrupt(10)

        double_hir = internals.mod.add('double', ['n'])
        double_hir.ret(Expr.mul(Expr.var('n'), 2))

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            self.assertEqual(10, int(frame.local('doubled')))

        result = internals.mod.build()
        self.run_module_test(result, validate)

    def test_instantiation(self, internals):
        main_hir = internals.main
        main_hir.assign('d', {1: 1, '2': 2, '3': True})
        main_hir.assign('l', [1, 2, 3])
        main_hir.interrupt(10)

        def validate(ctx):
            frame = ctx.frame()
            self.assertTrue(frame)
            print(frame.local('d'))
            self.assertEqual(1, frame.local('d')[1])
            self.assertEqual(2, frame.local('d')['2'])
            self.assertEqual(True, frame.local('d')['3'])

            self.assertEqual(1, frame.local('l')[0])
            self.assertEqual(2, frame.local('l')[1])
            self.assertEqual(3, frame.local('l')[2])

        result = internals.mod.build()
        self.run_module_test(result, validate)

    def test_pynative_function(self, internals):
        internals.mod.add_pyfn('powr', lambda x: int(x) ** 2)
        result = internals.mod.build()
        internals.vm.add_module(result)
        self.assertEqual(4, int(internals.vm.call('main.powr', 2)))

    def test_repeating_iterator(self, internals):
        var = Expr.var('sum')

        sum_hir = internals.mod.add('sum')
        sum_hir.assign(var, 0)
        sum_hir.repeat_iterating([1, 2, 3, 4], 'i').assign('sum', Expr.add(var, Expr.var('i')))
        sum_hir.ret(var)

        iter_sum_hir = internals.mod.add('iter_sum', ['collection'])
        iter_sum_hir.assign(var, 0)
        iter_sum_hir.assign('it', Expr.iter(Expr.var('collection')))
        iter_sum_hir.repeat_iterating(Expr.var('it'), 'i').assign('sum', Expr.add(var, Expr.var('i')))
        iter_sum_hir.ret(var)

        result = internals.mod.build()
        internals.vm.add_module(result)

        self.assertEqual(10, int(internals.vm.call('main.sum')))
        self.assertEqual(15, int(internals.vm.call('main.iter_sum', [1, 2, 3, 4, 5])))

    def test_iterators(self, internals):
        main_hir = internals.main
        main_hir.assign_global('r1', Expr.range(5))
        main_hir.assign_global('r2', Expr.range(-5, 0))
        main_hir.assign_global('r3', Expr.range(5).reverse())

        result = internals.mod.build()
        vm = internals.vm
        vm.add_main_module(result)
        vm.run()

        self.assertEqual([0, 1, 2, 3, 4], vm.ctx().globals('r1'))
        self.assertEqual([-5, -4, -3, -2, -1], vm.ctx().globals('r2'))
        self.assertEqual([4, 3, 2, 1, 0], vm.ctx().globals('r3'))
