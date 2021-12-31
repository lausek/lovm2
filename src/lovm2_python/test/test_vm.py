import pytest

from .deps import *

from pylovm2 import LV2ModuleBuilder


class TestVm(Test):
    def test_no_entry_point(self, capfd, internals):
        vm = internals.vm
        with pytest.raises(Exception):
            vm.run()

    def test_interrupt(self, internals):
        vm = internals.vm
        main_hir = internals.main
        main_hir.trigger(10)

        out = {"called": False}

        def callback(_ctx):
            out["called"] = True

        vm.add_interrupt(10, callback)
        vm.add_main_module(internals.mod.build())
        vm.run()

        self.assertTrue(out["called"])

    def test_raise_exception(self, internals):
        internals.mod.add_pyfn("ret", lambda: 1 / 0)
        internals.vm.add_module(internals.mod.build())
        with pytest.raises(ZeroDivisionError):
            internals.vm.call("main.ret")

    def test_load_hook_exception(self, internals):
        def load_hook(_name, _relative_to):
            raise ImportError()

        main_hir = internals.mod.entry()
        main_hir.import_("std")

        internals.vm.set_load_hook(load_hook)
        internals.vm.add_main_module(internals.mod.build())

        with pytest.raises(ImportError):
            internals.vm.run()

    def test_unknown_use(self, internals):
        main_hir = internals.mod.entry()
        main_hir.import_("unkown_module")

        internals.vm.add_main_module(internals.mod.build())

        with pytest.raises(Exception):
            internals.vm.run()

    def test_import_from(self, internals):
        def load_hook(_name, _relative_to):
            m = LV2ModuleBuilder()

            m.add("what").ret(42)

            return m.build()

        internals.main.import_from("other")

        internals.vm.set_load_hook(load_hook)
        internals.vm.add_main_module(internals.mod.build())
        internals.vm.run()

        assert 42 == int(internals.vm.call("what"))
