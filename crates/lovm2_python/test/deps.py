from os.path import abspath, dirname, exists, join
import pytest
import subprocess
import sys

build_folder = abspath(join(dirname(__file__), "../target/debug"))

if not exists(build_folder):
    print("./target/debug not found. build using project `cargo build` first")
    exit()

so, so_importable = join(build_folder, "libpylovm2.so"), join(build_folder, "pylovm2.so")

subprocess.call(["rm", so_importable])
subprocess.call(["ln", "-s", so, so_importable])
sys.path.insert(0, build_folder)

import pylovm2
assert so_importable == pylovm2.__file__, "shared object in ./target/debug was not imported"


class Test:
    def assertIsInstance(self, obj, cls):
        self.assertTrue(isinstance(obj, cls))

    def assertEqual(self, expected, got):
        self.assertTrue(expected == got)

    def assertFalse(self, expr):
        self.assertTrue(not expr)

    def assertTrue(self, expr):
        assert expr

    def run_module_test(self, module, fn):
        self.assertIsInstance(module, pylovm2.LV2Module)

        out = {"called": False}

        def callback(ctx):
            out["called"] = True
            fn(ctx)

        vm = pylovm2.LV2Vm()
        vm.add_interrupt(10, callback)
        vm.add_main_module(module)
        vm.run()

        assert out["called"], "no tests were executed"


class Internals:
    def __init__(self):
        self.vm = pylovm2.LV2Vm()
        self.mod = pylovm2.LV2ModuleBuilder("main")
        self.main = self.mod.add(pylovm2.LV2_ENTRY_POINT)


@pytest.fixture
def internals():
    return Internals()
