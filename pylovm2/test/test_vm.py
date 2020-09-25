import pytest

from .deps import *

class TestVm(Test):
    def test_no_entry_point(self, capfd, internals):
        vm = internals.vm
        with pytest.raises(RuntimeError):
            vm.run()

    def test_interrupt(self, internals):
        vm = internals.vm
        main_hir = internals.main
        main_hir.interrupt(10)

        out = {
            'called': False
        }
        def callback(_ctx):
            out['called'] = True

        vm.add_interrupt(10, callback)
        vm.load(internals.mod.build())
        vm.run()

        self.assertTrue(out['called'])

    def test_raise_exception(self, internals):
        internals.mod.add('ret').pyfn(lambda: 1/0)
        internals.vm.load(internals.mod.build())
        with pytest.raises(ZeroDivisionError):
            internals.vm.call('ret')
