import pytest

from .deps import *

class TestBuilding(Test):
    def test_assign(self):
        mod = pylovm2.hir.ModuleBuilder()

        main_hir = mod.add("main")
        main_hir.assign("n", 5)
        main_hir.assign("f", 5.0)
        main_hir.assign("b", True)
        main_hir.assign("s", "string")

        result = mod.build()

        self.assertIsInstance(result, pylovm2.hir.Module)
