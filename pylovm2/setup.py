#!/usr/bin/python3

from setuptools import find_packages, setup

setup(
    name='pylovm2',
    version='0.0.1',
    author='lausek',
    description='bindings for lovm2',
    url='https://github.com/lausek/lovm2',
    packages=find_packages(),
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: GNU General Public License v3 (GPLv3)',
        'Operating System :: OS Independent',
    ],
    install_requires=[ ],
    python_requires='>=3.5',
)
