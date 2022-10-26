grammar Expr;
prog:   (expr NEWLINE)* ;
expr:   expr ('*'|'/') expr
    |   expr ('+'|'-') expr
    |   INT
    |   '(' expr ')'
    ;
NEWLINE : [\r\n]+ ;
INT     : [0-9]+ ;

expr:   expr ADD term
    |   expr SUB term
    |   term
term:   term MUL factor
    |   term DIV factor
    |   factor
factor: '\(' expr '\)'
      : '{' expr '}'
      | NUM
      ;

NUM : ([1-9][0-9]*)|0 ;
ADD : \+ ;
MUL : \* ;
SUB : - ;
DIV : / ;
