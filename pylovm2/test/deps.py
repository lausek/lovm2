from os.path import abspath, dirname, exists, join
import pytest
import sys

build_folder = abspath(join(dirname(__file__), '../target/debug'))

if not exists(build_folder):
    print('./target/debug not found. build using project `cargo build` first')
    exit()

if not exists(join(build_folder, 'libpylovm2.so')):
    print('no shared object found. make sure to create a link named `libpylovm2.so` inside ./target/debug')
    exit()

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

class Internals:
    def __init__(self):
        self.vm = pylovm2.Vm()
        self.mod = pylovm2.ModuleBuilder()
        self.main = self.mod.add("main").code()

@pytest.fixture
def internals():
    return Internals()
