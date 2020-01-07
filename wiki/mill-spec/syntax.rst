Mill syntax in BNF
==================

.. code-block:: bnf

    <program>       ::= { <declaration> | <function> }

    <declaration>   ::= { char | int } ( <variable> | <pointer> semi

    <variable>      ::= [ lparen [ l | b ] [ i | u ] rparen ]
                        ident [ equal <number> ]

    <number>        ::= [ percent | at | dollar ] ( byte | word )

    <pointer>       ::= [ lparen [ l | b ] [ i | u ] rparen ]
                        <number> ident
                    |   [ lparen [ l | b ] [ i | u ] z rparen ]
                        ident

    <function>      ::= [ pub | file ] fn ident lbrace { <statement> } rbrace

    <statement>     ::=