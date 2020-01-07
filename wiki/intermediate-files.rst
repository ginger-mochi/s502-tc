Intermediate Files
==================

s502-as may output object files and symbol tables. s502-ld may also
output symbol tables. Those files have the following raw binary format,
defined in the form of C structs assuming no padding between members:

Object files (``*.65o``)
------------------------

The header of the object is simple:

.. code-block:: c

    struct object_header
    {
        uint32_t num_sections;
    };

``num_sections``
    The total number of sections in the file.

Then ``num_sections`` sections follow, each with the following header:

.. code-block:: c

    struct section
    {
        char     name[32];
        uint32_t payload_size
        uint32_t num_labels;
        uint32_t num_references;
    };


``name``
    Null-terminated name of the section.

``payload_size``
    Size of the section's payload.

``num_labels``
    Number of labels in the label block.

``num_references``
    Number of references in the reference block.

Each section is followed by the label block with ``num_labels`` labels.
The first label is guaranteed to be a root label, and some root labels
may be followed by a number of child labels;

.. code-block:: c

    struct root_label
    {
        char     name[32];
        uint32_t num_children;
        uint32_t address;
        uint32_t visibility;
    };

``name``
    Null-terminated name.

``num_children``
    The number of children of the label following this one.

``next_sibling``
    The index of the the label's next sibling.

``address``
    Offset into the section's payload that the label corresponds to.

``visibility``
    +-------+-------------------+
    | 0     | hidden            |
    +-------+-------------------+
    | 1     | visible to object |
    +-------+-------------------+
    | 2     | globally visible  |
    +-------+-------------------+

    This visibility directly corresponds to that described in s502-as.
    "Under its root label" in this context means "between the most recent
    root label address and the next root label address (both exclusive)".

.. code-block:: c

    struct child_label
    {
        char     name[32];
        uint32_t address;
        uint32_t visibility;
    };

``name``
    Null-terminated name.

``address``
    Offset into the section's payload that the label corresponds to.

``visibility``
    Exactly as described for root_label

Following the label block is the reference block:

.. code-block:: c

    struct reference
    {
        char     name[64];
        uint32_t address;
        uint32_t which_byte;
    };

``name``
    The null-terminated path of the label being referenced. It will always refer to
    a root label, and optionally be followed by a ``.`` and child label.

``address``
    Where to put the address of the label being referenced in the section's payload.

``which_byte``
    Which byte of the address being referenced to put into ``address``.
    
    +---+------+
    | 0 | both |
    +---+------+
    | 1 | high |
    +---+------+
    | 2 | low  |
    +---+------+

    The programmer may want to insert only one byte of the address into memory.
    If this field is non-zero, it specifies which and only one byte may be
    filled at ``address``.

    **NOTE:**

    If the instruction at ``address - 1`` is a branch instruction, then the 1 byte
    signed offset from ``address`` to the target label's address must be inserted,
    and is calculated as ``target_address - (reference.address + 1)``.

After the reference block is the section's payload. Its size is padded to 4 byte
alignment, which may be calculated with
``payload_size + ((4 - (payload_size & 3)) & 3)``,  though only the
first ``payload_size`` bytes are significant.

Symbol Table (``*.65s``)
------------------------

The symbol table is a subset of a section's label block
which only contains globally visible symbols and may be created from an
object file or by 6502-asm.

.. code-block:: c

    struct symbol_header
    {
        uint32_t num_symbols;
    };

``num_symbols``
    The number of symbols in the table

Followed by

.. code-block:: c

    struct symbol
    {
        char     name[128];
        uint32_t address;
    };

``name``
    Full path of symbol as if it were a reference. This is convenient for
    resolving references by simply comparing the two ``name`` fields.

``address``
    Address in memory of the symbol. This is not the same as the offset into a
    section's payload and may be determined by choosing a starting address
    for a section when creating the symbols, or by 6502-asm if it directly outputs binary.