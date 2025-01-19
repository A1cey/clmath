# The allowed grammar for this math interpreter is as follows:

Any amount of whitespaces between every Terminal or Non-Terminal are allowed and will be ignored.
    START ::= EXPRESSION | eps
    
    EXPRESSION ::= OPENING_BRACKET MATH_EXPRESSION CLOSING_BRACKET | MATH_EXPRESSION

    MATH_EXPRESSION ::=  NUMBER | VARIABLE | FUNCTION 

    NUMBER ::= [0-9]+(\.[0-9]+)?                                                // Numbers are handled as 64bit floating point numbers

    VARIABLE ::= [a-z][a-zA-Z]*

    FUNCTION ::= ELEMENTARY_FUNCTION | HIGHER_ORDER_FUNCTION

    ELEMENTARY_FUNCTION ::= EXPRESSION ELEMENTARY_FUNCTION_KEYWORD EXPRESSION
    ELEMENTARY_FUNCTION_KEYWORD ::= [A-Z][a-zA-Z]*                             // Function Keywords are predefined

    HIGHER_ORDER_FUNCTION ::= HIGHER_ORDER_FUNCTION_KEYWORD OPENING_BRACKET PARAMS CLOSING_BRACKET
    HIGHER_ORDER_FUNCTION_KEYWORD ::= [A-Z][a-zA-Z]*                            // Function Keywords are predefined

    PARAMS ::= EXPRESSION (COMMA EXPRESSION)*

    COMMA ::= ,
   
    OPENING_BRACKET ::= (
    CLOSING_BRACKET ::= )
    
    5 + 6 + 7
    
    5 + 
    
    lhs: 6