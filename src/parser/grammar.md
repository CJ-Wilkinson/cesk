$$
\begin{aligned}
\langle Id \rangle
  &::= \texttt{[a-zA-Z][a-zA-Z0-9\_]*}
\\[0.5em]
\langle BinaryOp \rangle
  &::= \mathtt{+}
   \mid \mathtt{-}
   \mid \mathtt{*}
   \mid \mathtt{/}
   \mid \mathtt{\%}
   \mid \mathtt{==}
   \mid \mathtt{!=}
   \mid \mathtt{<}
   \mid \mathtt{>}
   \mid \mathtt{<=}
   \mid \mathtt{>=}
   \mid \mathtt{||}
   \mid \mathtt{\&\&}
\\[0.5em]
\langle UnaryOp \rangle
  &::= \mathtt{ !}
   \mid \mathtt{ -}
\\[0.5em]
\langle UnitType \rangle
  &::= \mathtt{()}
\\[0.5em]
\langle Type \rangle
  &::= \langle BaseType \rangle \ \langle ArraySuffix \rangle^{*}
\\
\langle BaseType \rangle
  &::= \mathtt{int}
   \mid \mathtt{bool}
   \mid \langle UnitType \rangle
\\
\langle ArraySuffix \rangle
  &::= \mathtt{[]}
\\[1em]
\langle Program \rangle
  &::= \langle Fun \rangle^{+}
\\[0.5em]
\langle Fun \rangle
  &::= \langle Type \rangle \ \langle Id \rangle
      \ \langle Parameters \rangle
      \ \langle Block \rangle
\\[0.5em]
\langle Parameters \rangle
  &::= \mathtt{(} \ \langle ParameterList \rangle? \ \mathtt{)}
\\
\langle ParameterList \rangle
  &::= \langle Parameter \rangle
   \mid \langle Parameter \rangle \ \mathtt{,} \ \langle ParameterList \rangle
\\
\langle Parameter \rangle
  &::= \langle Type \rangle \ \langle Id \rangle
\\[1em]
\langle Statement \rangle
  &::= \langle DeclD \rangle
   \mid \langle If \rangle
   \mid \langle While \rangle
   \mid \langle ExprStmt \rangle
   \mid \langle Block \rangle
   \mid \mathtt{continue;}
   \mid \mathtt{break;}
\\[0.5em]
\langle DeclD \rangle
  &::= \langle Type \rangle \ \langle Id \rangle
      \ \left(\mathtt{=} \ \langle Expr \rangle\right)?
      \ \mathtt{;}
\\[0.5em]
\langle If \rangle
  &::= \mathtt{for} \ \mathtt{(}
      \langle Expr \rangle \ \mathtt{;}
      \langle Expr \rangle \ \mathtt{;}
      \langle Expr \rangle
      \ \mathtt{)}
      \ \langle Block \rangle
\\
  &\mid \mathtt{if} \ \mathtt{(} \langle Expr \rangle \mathtt{)}
      \ \langle Block \rangle
      \ \left(\mathtt{else} \ \langle Block \rangle\right)?
\\[0.5em]
\langle While \rangle
  &::= \mathtt{while} \ \mathtt{(} \langle Expr \rangle \mathtt{)}
      \ \langle WhileBlock \rangle
\\[0.5em]
\langle WhileBlock \rangle
  &::= \mathtt{\{} \ \langle Statement \rangle^{*}
      \ \mathtt{continue;}
      \ \mathtt{\}}
\\[0.5em]
\langle Block \rangle
  &::= \mathtt{\{} \ \langle Statement \rangle^{*} \ \mathtt{\}}
\\[0.5em]
\langle ExprStmt \rangle
  &::= \langle Expr \rangle \ \mathtt{;}
\\[1em]
\langle Arguments \rangle
  &::= \mathtt{(} \ \langle ArgumentList \rangle? \ \mathtt{)}
\\
\langle ArgumentList \rangle
  &::= \langle Expr \rangle
   \mid \langle Expr \rangle \ \mathtt{,} \ \langle ArgumentList \rangle
\\[1em]
\langle Expr \rangle
  &::= \langle ValueExpr \rangle
   \mid \langle BinaryExpr \rangle
   \mid \langle UnaryExpr \rangle
   \mid \langle IdExpr \rangle
   \mid \langle FunCallExpr \rangle
   \mid \mathtt{(} \langle Expr \rangle \mathtt{)}
\\[0.5em]
\langle ValueExpr \rangle
  &::= \langle Val \rangle
\\[0.5em]
\langle Val \rangle
  &::= \langle IntLiteral \rangle
   \mid \mathtt{true}
   \mid \mathtt{false}
   \mid \mathtt{()}
\\[0.5em]
\langle UnaryExpr \rangle
  &::= \langle UnaryOp \rangle \ \langle Expr \rangle
\\[0.5em]
\langle BinaryExpr \rangle
  &::= \langle Expr \rangle \ \langle BinaryOp \rangle \ \langle Expr \rangle
\\[0.5em]
\langle IdExpr \rangle
  &::= \langle Id \rangle
\\[0.5em]
\langle FunCallExpr \rangle
  &::= \langle Id \rangle \ \langle Arguments \rangle
\end{aligned}
$$