# The allowed grammar for this math interpreter is as follows:

Any amount of whitespaces between every Terminal or Non-Terminal are allowed and will be ignored.

    EXPRESSION ::= ( MATH_EXPRESSION ) | MATH_EXPRESSION | eps

    MATH_EXPRESSION ::=  NUMBER | VARIABLE | FUNCTION 

    NUMBER ::= [0-9]+(\.[0-9]+)?                                                // Numbers are handled as 64bit floating point numbers

    VARIABLE ::= [a-z][a-zA-Z]*

    FUNCTION ::= ELEMENTARY_FUNCTION | HIGHER_ORDER_FUNCTION

    ELEMENTARY_FUNCTION ::= EXPRESSION ELEMENTARY_FUNCTION_KEYWORD EXPRESSION
    ELEMENTARY_FUNCTION_KEYWORD ::= [A-Z][a-zA-Z]*                             // Function Keywords are predefined

    HIGHER_ORDER_FUNCTION ::= HIGHER_ORDER_FUNCTION_KEYWORD ( EXPRESSION )
    HIGHER_ORDER_FUNCTION_KEYWORD ::= [A-Z][a-zA-Z]*                            // Function Keywords are predefined
