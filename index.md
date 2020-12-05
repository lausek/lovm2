---
layout: default
title: lovm2
---

[![crates.io badge](https://img.shields.io/crates/v/lovm2.svg)](https://crates.io/crates/lovm2)
[![docs.rs badge](https://docs.rs/lovm2/badge.svg?version=latest)](https://docs.rs/lovm2/)

```
lovm2 = "0.4.5"
```

## Features

- [X] dynamic typing
- [X] generate bytecode using highlevel intermediate representation
- [X] call into shared objects: [lovm2_extend](lovm2_extend/README.md)
- [X] python bindings: [pylovm2](pylovm2/README.md)
- [X] define own callbacks for interrupts

## Examples

### Projects

- [lol - a lisp language](https://github.com/lausek/lol)
- [quasicode - the best language around](https://github.com/witling/quasicode)

### Source Code References

- [Bytecode](https://github.com/lausek/lovm2/blob/master/src/bytecode.rs)
- [Context](https://github.com/lausek/lovm2/blob/master/src/context.rs)
- [Vm](https://github.com/lausek/lovm2/blob/master/src/vm.rs)
