from os.path import abspath, dirname, exists, join
import pytest
import sys

build_folder = abspath(join(dirname(__file__), '../target/debug'))

if not exists(build_folder):
    print('./target/debug not found. build using project `cargo build` first')
    exit()

if not exists(join(build_folder, 'pylovm2.so')):
    import subprocess
    subprocess.call(['ln', '-s', join(build_folder, 'libpylovm2.so'), join(build_folder, 'pylovm2.so')])

sys.path.insert(0, build_folder)

import pylovm2

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
            
        out = {
            'called': False
        }
        def callback(ctx):
            out['called'] = True
            fn(ctx)
    
        vm = pylovm2.LV2Vm()
        vm.add_interrupt(10, callback)
        vm.add_main_module(module)
        vm.run()
    
        self.assertTrue(out['called'])

class Internals:
    def __init__(self):
        self.vm = pylovm2.LV2Vm()
        self.mod = pylovm2.LV2ModuleBuilder("main")
        self.main = self.mod.add(pylovm2.LV2_ENTRY_POINT)

@pytest.fixture
def internals():
    return Internals()
