import os
import os.path
import pytest

from .deps import *

from pylovm2 import Expr

class TestPersistance(Test):
    def test_persistance(self, internals):
        main_hir = internals.main
        main_hir.assign_global('n', 2)
        main_hir.interrupt(10)

        result = internals.mod.build()

        self.assertIsInstance(result, pylovm2.Module)

        path = '/tmp/persistance.lovm2'
        if os.path.exists(path):
            os.remove(path)
        self.assertFalse(os.path.exists(path))
        result.save(path)
        self.assertTrue(os.path.exists(path))

        def testfn(ctx):
            assert 2 == int(ctx.globals('n'))

        module = pylovm2.Module.load(path)
        self.run_module_test(module, testfn)
