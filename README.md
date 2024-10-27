# dc

A general-purpose RPN calculator that can be adapted to arbitrary objects.

This project is in active development, so documentation is somewhat patchy. 

Briefly, the actual calculator is in `calculator.rs`. Definitions of traits and stuff are in `number.rs`, and it's left to the user to define the objects the calculator uses, define how the operations are done on them and parse them from some input.

hell, you could make an RPN matrix operation calculator if you really wanted!

An example calculator using this library will be published shortly (pieces of it are included here, actually), and at some point I will use it to build an emulator of the arithmetic parts of TI-BASIC.