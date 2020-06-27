import pytest

from .deps import *

class TestVm(Test):
    def test_no_entry_point(self, capfd, internals):
        vm = internals.vm
        with pytest.raises(TypeError):
            vm.run()
