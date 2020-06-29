import pytest

from .deps import *

class TestValue(Test):
    def test_to_string(self, internals):
        main_hir = internals.main
        main_hir.assign_global('a', 10)
        mod = internals.mod.build()

        internals.vm.load(mod)
        internals.vm.run()

        a = internals.vm.globals('a')
        self.assertEqual('10', str(a))

    def test_to_int(self, internals):
        main_hir = internals.main
        main_hir.assign_global('a', 10)
        mod = internals.mod.build()

        internals.vm.load(mod)
        internals.vm.run()

        a = internals.vm.globals('a')
        self.assertEqual(10, int(a))
