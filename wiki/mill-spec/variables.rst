Variables
=========

Variables can have two types, ``char`` and ``int``. They have the properties:

+---+-------------------------+
| l | little-endian (default) |
+---+-------------------------+
| b | big-endian              |
+---+-------------------------+
| i | signed (default)        |
+---+-------------------------+
| u | unsigned                |
+---+-------------------------+
| z | zeropage                |
+---+-------------------------+

.. code-block::

    char (u) var0; // chars have no endianness
    int (bi) var1;
    int (u) var2 = $5050; // give initial value
    char var3; // default value of 0

Pointers are a compile-time abstraction that give a name to an address:

.. code-block::

    // numbers use the same base notation as in s502-as
    char (u) $10 var4; // pointer to char at address $10
    int $0200 var5; // pointer to int at address $0200
    int $30 var6;

To use a pointer anonymously in an expression:

.. code-block::

    char (u) $10 = A; // put register A into address $10
    // equivalent to
    var4 = A;

Zeropage variables may also be declared, and they are simply shortcuts for
pointers to zeropage where the compiler chooses the exact address.

.. code-block::

    #zpg $80 // zeropage variables may begin at address $80

    char (z) var7;

Some ``int`` s may be treated as pointers. 

.. code-block::

    call (var2); // push return address, then jmp indirect

    A = (var2); // error, no suitable address mode
    A = (var6); // will use either x, ind or ind, y

In addition to these variables, the reserved keywords ``A, X, Y, S (stack pointer),
P (processor staus)`` allow you to access those registers directly. They all have the default
type ``char (i)`` but can be be made unsigned: ``(u) A`` .

Placement
---------

Variables are placed in the ``vars`` section and pointers are compiled into macros:

.. code-block::

    ; pointers
    var4 equ $10
    var5 equ $0200

    ; functions
    sct text

    ; variables
    sct vars
    var0: dfb $0
    var1: dfw $0
    var2: dfw $5050
    var3: dfb $0