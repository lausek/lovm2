import pytest
import subprocess

from .deps import *


class TestStdlib(Test):
    def cmd(self, *args):
        return subprocess.run(args, capture_output=True)
