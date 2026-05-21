```
Id ::= [a-zA-Z][a-zA-Z0-9_]*

Program ::= Fun* MainFun Fun*

MainFun ::= unit main ( ) Block

Fun ::= Type Id Parameters Block


Type ::= BaseType ArraySuffix*

BaseType ::= int
           | bool
           | unit

ArraySuffix ::= [ ]


Parameters ::= ( ParameterList? )

ParameterList ::= Parameter (, Parameter)*

Parameter ::= Type Id


Statement ::= ForD
            | If
            | While
            | Assign
            | ExprStmt
            | DeclD
            | Return
            | Block
            | continue ;
            | break ;


Block ::= { Statement* }


DeclD ::= Type Id (= Expr)? ;


Assign ::= AssignTarget = Expr ;

AssignTarget ::= Id
               | Id [ Expr ]


ExprStmt ::= Expr ;


Return ::= return Expr ;


If ::= if ( Expr ) Block ElseClause?

ElseClause ::= else Block


While ::= while ( Expr ) Block


ForD ::= for ( Expr? ; Expr ; Expr? ) Block


Expr ::= Expr BinaryOp Expr
       | UnaryOp Expr
       | Val
       | Id
       | Id Arguments
       | Id [ Expr ]
       | [ ArgumentList? ]
       | ( Expr )


Arguments ::= ( ArgumentList? )

ArgumentList ::= Expr (, Expr)*


Val ::= IntLiteral
      | true
      | false
      | ()


BinaryOp ::= +
           | -
           | *
           | /
           | %
           | ==
           | !=
           | <
           | >
           | <=
           | >=


UnaryOp ::= !
          | -
```