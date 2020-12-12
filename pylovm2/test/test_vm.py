import pytest

from .deps import *

class TestVm(Test):
    def test_no_entry_point(self, capfd, internals):
        vm = internals.vm
        with pytest.raises(Exception):
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
        internals.mod.add_pyfn('ret', lambda: 1/0)
        internals.vm.load(internals.mod.build())
        with pytest.raises(ZeroDivisionError):
            internals.vm.call('ret')

    def test_load_hook_exception(self, internals):
        def load_hook(name):
            raise Exception()

        main_hir = internals.mod.entry()
        main_hir.load(pylovm2.Expr.val('std'))

        internals.vm.set_load_hook(load_hook)
        internals.vm.load(internals.mod.build())

        with pytest.raises(Exception):
            internals.vm.run()
            assert False

    def test_unknown_use(self, internals):
        main_hir = internals.mod.entry()
        main_hir.load(pylovm2.Expr.val('unkown_module'))

        internals.vm.load(internals.mod.build())

        with pytest.raises(Exception):
            internals.vm.run()
