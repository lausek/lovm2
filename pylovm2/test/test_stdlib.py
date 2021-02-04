import pytest
import subprocess

from .deps import *

"""
PYLOVM2_DIR=$(dirname `realpath $0`)
cd $PYLOVM2_DIR

docker build -t pylovm2-test -f test.Dockerfile .

docker run -ti \
    -v $PYLOVM2_DIR/target/wheels:/deps/pylovm2 \
    -v $PYLOVM2_DIR/stdlib/target/wheels:/deps/pylovm2_stdlib \
    pylovm2-test
"""

class TestStdlib(Test):
    def cmd(self, *args):
        return subprocess.run(args, capture_output=True)

    def test_installation(self):
        from pathlib import Path

        pylovm2_dir = Path(__file__).parent.parent
        pylovm2_mount = '{}/target/wheels:/deps/pylovm2'.format(pylovm2_dir)
        pylovm2_stdlib_mount = '{}/stdlib/target/wheels:/deps/pylovm2_stdlib'.format(pylovm2_dir)

        img_name = 'pylovm2-test'
        img_file = pylovm2_dir / 'test' / 'test.Dockerfile'
        img_volume = 'pylovm2-test-vol'

        py_script = 'import pylovm2; pylovm2.Vm.with_std().call("print", "hej")'
        py_version = '3.7'
        py_cache_mount = '{}:/usr/local/lib/python{}/site-packages/'.format(img_volume, py_version)

        def run_script():
            return self.cmd(
                'docker', 'run', '-t',
                '-v', pylovm2_mount, '-v', pylovm2_stdlib_mount,
                '-v', py_cache_mount,
                img_name, 'python -c \'{}\''.format(py_script)
            )


        def install_script_for(modname):
            version = py_version.replace('.', '')
            return 'find "/deps/{}" -name "*{}*" | xargs pip3 install'.format(modname, version)
        

        # build image
        complete = self.cmd('docker', 'build', '-t', img_name, '-f', img_file, '.')
        self.assertEqual(0, complete.returncode)

        # prepare volume
        print(self.cmd('docker', 'system', 'prune', '--force'))
        print(self.cmd('docker', 'volume', 'remove', img_volume))
        complete = self.cmd('docker', 'volume', 'create', img_volume)
        self.assertEqual(0, complete.returncode)

        # pylovm2 is not installed; this fails
        complete = run_script()
        self.assertEqual(1, complete.returncode)
        error1 = complete.stdout

        # install pylovm2
        complete = self.cmd(
            'docker', 'run', '-t',
            '-v', pylovm2_mount, '-v', pylovm2_stdlib_mount,
            '-v', py_cache_mount,
            img_name, install_script_for('pylovm2')
        )
        self.assertEqual(0, complete.returncode)

        complete = run_script()
        self.assertEqual(0, complete.returncode)
        print(complete.stdout.decode('utf-8'))
        self.assertEqual('hej', complete.stdout.decode('utf-8'))

        print(self.cmd('docker', 'system', 'prune', '--force'))
        print(self.cmd('docker', 'volume', 'remove', img_volume))
