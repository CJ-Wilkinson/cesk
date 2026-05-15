# cesk

## TODO:
- remove forD
- write actual semantic rules
 

DOCUMENTATION:
- write explicit desugaring rules for later implementation
  - not a whole semantic pass, but maybe we just do it at runtime.



- Desugaring Rules
  - for -> while
  - While -> add continue at the end
  - explicit return inserted into every block.
  - declare -> a-normal form = (decl + assign) 
  - insert return 0 in main if there is no main return
  - `unit` return insert only for all functions

- Meta functions def - Kind of has dependency on the parser probably

## Assignments

### Santiago
- <= >= == etc do not work.

### Todd
- Implement unary rules

### Chance
- CallName -> CallRef
- Type checking

### Ava
- change all of the Names to Ids
- make sure Lvalue is in the right places
- visualization of the Rules

### Drew
- fix parser
- parser rules
- named (explicit) fields on every node in the AST
- unary op