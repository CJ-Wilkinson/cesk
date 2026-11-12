# Outline

0. abstract
    1. what is an abstract machine
    2. what is CESK
    3. semantic analysis using CESK???
    4. memory safety
    5. full description of our CESK

1. introduction

2. background
    - Transition rule
        - what is it
    - $\lambda$-calculus definition
        - no discussion, just say that it exists
    - Standard CESK
    - Description of our language
    - How ours is different
    - LBA vs Turing machines

3. contributions
    -

4. methodology

5. results

6. conclusion (pretty short)
    - claims about what you demonstrated
    - mention what you expect future work to look like

7. Appendix
    - Full definition of CESK machine
    - Example derivation from a language construct into a transition rule
    - Full set of transition rules
    - Peano arithmetic
    - Language grammar
    - Repository links

---------------------------------------------------------

## 0. Abstract

This paper presents an interpreter using a memory model based on the CESK machine. We have developed a C-like language to facilitate the exploration of an imperative CESK machine through call-by-value λ-calculus encodings. We also discuss our CESK machine's implications of memory safety with array indexing and limited memory. We demonstrate that our techniques for deriving a language can be scaled to larger language constructs, such as classes or anonymous functions.


This one vvvv
The CESK machine is an abstract machine that defines a program’s meaning as a sequence of small, explicit steps over its components. We present an adaptation of the CESK machine to a C-like imperative language with mutable variables, functions, conditionals, loops, and arrays. Each language construct is given as a set of transition rules, implemented as a concrete executable interpreter that can display every intermediate machine state. All memory is accessed through the machine’s explicit store, so array values carry their bounds and out-of-range accesses halt the machine rather than reading arbitrary memory. We discuss how this rule-derivation approach could extend to larger language features and release our implementation and full rule set.


---------------------------------------------------------
Imperative implementation of Might et al.'s CESK machine.

C-like imperative language.

Memory saftey in array indexing

Recursively enumerable to context sensitive

Next steps, plan for after the paper is written.

Appendix at the end including everything relevant to reproduce our work

---------------------------------------------------------
