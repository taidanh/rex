lexer grammar RexLexer;

channels { COMMENT, ERROR }

// Skip

SPACE:                               [ \t\r\n]+     -> channel(HIDDEN);
SPEC_MYSQL_COMMENT:                  '/*!' .+? '*/' -> channel(COMMENT);
COMMENT_INPUT:                       '/*' .*? '*/'  -> channel(HIDDEN);
LINE_COMMENT:                        (
                                       ('--' [ \t] | '#') ~[\r\n]* ('\r'? '\n' | EOF)
                                       | '--' ('\r'? '\n' | EOF)
                                     ) -> channel(HIDDEN);

// Operators

OR:     'or' ;

// Keywords

CREATE: 'create' ;

// Misc

STRING: [a-z]+ ;

// Symbols

SEMI:   ';' ;
DQUOTE: '"' ;
SQUOTE: '\'' ;
QUOTE:  DQUOTE|SQUOTE ;
