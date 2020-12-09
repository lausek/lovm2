# Introduction

Dynamic languages such as Python are built on top of an interpreter that is able to understand a broad variety of bytecode instructions allowing them to replicate algorithms and process data. This design makes programs based on interpreter languages well-suited for platform independency and allows fast iterations in development.

`lovm2` - *love em too* - is a small language building framework that comes with a dead-simple stack-based virtual machine written in Rust doing exactly that. Furthermore, it comes with tools for generating said bytecode out of the box allowing you to rapidly prototype your own coding language without a hassle. There are no advanced concepts to care about. No polymorphism, closures, asynchronous runtime... just straightforward functions, variables and data structures.

The static `lovm2` library is to tiny that compiling it into your language yields almost no overhead and also makes it applicable for usage inside a Python environment via [pylovm2](https://github.com/lausek/lovm2/tree/master/pylovm2).

The project is in an early development stage and no API is stable yet. Feel free to [contribute](https://github.com/lausek/lovm2/issues).
