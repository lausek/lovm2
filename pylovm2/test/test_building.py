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

        result = internals.mod.build()

        self.assertIsInstance(result, pylovm2.Module)

        internals.vm.load(result)
        internals.vm.run()

        val = internals.vm.globals('n')

        self.assertEqual(5, int(val))

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
