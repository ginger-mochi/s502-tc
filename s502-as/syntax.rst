s502-as Syntax in EBNF
======================

.. code-block::

    program         = { macro " directive | line };

    macro           = ident "equ" number;

    directive       = "org", number     (* case insensitive *)
                    | "dfb", byte
                    | "dfw", word
                    | "inl", string
                    | "sct", string
                    | "txt", string
                    | "lng", string

    line            = [ label ], { mnemonic };

    label           = ident, [ ".", ident ], ":";

    ident           = id_char, 30 * { id_char };

    number          = byte | word;

    byte            = "%", bin digit, 7 * { digit }
                    | "@", oct digit, 2 * { digit }
                    | "$", hex digit, { digit }
                    | dec digit, 2 * { digit };

    word            = "%", bin digit, 15 * { digit }
                    | "@", oct digit, 5 * { digit }
                    | "$", hex digit, 3 * { digit }
                    | dec digit, 4 * { digit };

    id_char         = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h"
                    | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p"
                    | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x"
                    | "y" | "z"
                    | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H"
                    | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P"
                    | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X"
                    | "Y" | "Z" | "_";
    
    mnemonic        = "adc" | "and" | "asl" | "and" | "bcc" | "bcs" (* case-insensitive *)
                    | "beq" | "bit" | "bmi" | "bne" | "bpl" | "brk"
                    | "bvc" | "bvs" | "clc" | "cld" | "cli" | "clv"
                    | "cmp" | "cpx" | "cpy" | "dec" | "dex" | "dey"
                    | "eor" | "inc" | "inx" | "iny" | "jmp" | "jsr"
                    | "lda" | "ldx" | "ldy" | "lsr" | "nop" | "ora"
                    | "pha" | "php" | "pls" | "plp" | "rol" | "ror"
                    | "rti" | "rts" | "sbc" | "sec" | "sed" | "sei"
                    | "sta" | "stx" | "sty" | "tax" | "tay" | "tsx"
                    | "txa" | "txs" | "tya" | "hlt"
                    
    dec digit       = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7"
                    | "8" | "9";
    
    bin digit       = "0" | "1";

    oct digit       = dec digit - ( "8" | "9" );

    hex digit       = dec digit | "a" | "b" | "c" | "d" | "e" | "f"
                    | "A" | "B" | "C" | "D" | "E" | "F";