import pytest

from .deps import *


class TestValue(Test):
    def test_to_string(self, internals):
        main_hir = internals.main
        main_hir.assign_global("a", 10)
        main_hir.trigger(10)
        module = internals.mod.build()

        def testfn(ctx):
            assert "10" == str(ctx.globals("a"))

        self.run_module_test(module, testfn)

    def test_to_int(self, internals):
        main_hir = internals.main
        main_hir.assign_global("a", 10)
        main_hir.trigger(10)
        module = internals.mod.build()

        def testfn(ctx):
            assert 10 == int(ctx.globals("a"))

        self.run_module_test(module, testfn)

    def test_to_float(self, internals):
        main_hir = internals.main
        main_hir.assign_global("a", 22.0)
        main_hir.trigger(10)
        module = internals.mod.build()

        def testfn(ctx):
            assert 22.0 == int(ctx.globals("a"))

        self.run_module_test(module, testfn)
