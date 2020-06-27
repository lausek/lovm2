import pytest

from .deps import *

class TestBuilding(Test):
    def test_assign(self, internals):
        main_hir = internals.main
        main_hir.assign('n', 5)
        main_hir.assign('f', 5.0)
        main_hir.assign('b', True)
        main_hir.assign('s', 'string')

        result = internals.mod.build()

        self.assertIsInstance(result, pylovm2.hir.Module)

    def test_expressions(self, internals):
        Expr = pylovm2.hir.Expr

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

        self.assertIsInstance(result, pylovm2.hir.Module)

    def test_expression_deep(self, internals):
        Expr = pylovm2.hir.Expr

        main_hir = internals.main
        main_hir.assign('a', Expr.eq(Expr.rem(15, 5), 0))

        result = internals.mod.build()

        self.assertIsInstance(result, pylovm2.hir.Module)
