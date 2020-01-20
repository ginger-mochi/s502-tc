Description
===========

The 6502 Middle Level Language (Mill) is designed to make writing 6502 software
easier than otherwise with assembly by (mostly) allowing full control over the
program produced and offering better readability, some abstractions, and optimizations.

This language assumes you are familiar with 6502 assembly and this specification will
refer to specific address modes and other features of the CPU.

This reference implementation targets assembly meant for s502-as, but all assembly
generation and output is kept to the ``backend`` module so to target another assembler
one could swap out the implementation of ``backend`` while keeping the same interface
(refer to the docs) (docs will be generated from documenting the rust code).

Table of Contents
=================

1. Variables
2. Functions