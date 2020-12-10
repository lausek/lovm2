# Interrupt

`vm.set_interrupt(n, callback)`

Interrupts are more like a runtime extension of the bytecode. You can use this to implement optional extensions and frequently used functions without the overhead of a name lookup.

The test environment uses `Interrupt(10)` to analyse the programs state at a certain point of execution.
