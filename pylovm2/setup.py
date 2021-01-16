#!/usr/bin/python3

from setuptools import find_packages, setup
from setuptools_rust import Binding, RustExtension

setup(
    name='pylovm2',
    version='0.4.8',
    author='lausek',
    author_email='spam@lausek.eu',
    description='bindings for lovm2',
    long_description='bindings for lovm2',
    url='https://github.com/lausek/lovm2',
    rust_extensions=[RustExtension('pylovm2.pylovm2', binding=Binding.PyO3)],
    packages=['pylovm2'],
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: GNU General Public License v3 (GPLv3)',
        'Operating System :: OS Independent',
    ],
    install_requires=[],
    setup_requires = ['setuptools-rust>=0.10.1', 'wheel'],
    zip_safe=False,
    python_requires='>=3.5',
)
