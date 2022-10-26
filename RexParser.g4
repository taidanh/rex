parser grammar RexParser;

options { tokenVocab=RexLexer; }


root
  : rexStatements EOF
  ;

rexStatements
  : rexStatement*
  ;

rexStatement
  : createStatement SEMI
  ;

createStatement
  : CREATE rexExpr OR rexExpr
  ;

rexExpr
  : QUOTE STRING QUOTE
  ;
