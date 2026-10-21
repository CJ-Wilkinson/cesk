# Outline

0. abstract
    1. what is an abstract machine
    2. what is CESK
    3. semantic analysis using CESK???
    4. memory safety
    5. full description of our CESK

1. meat and potatoes
    - Transition rule
        - what is it
    - $\lambda$-calculus definition
        - no discussion, just say that it exists
    - Standard CESK
    - Description of our language
    - How ours is different
2. Type checking
3. Appendix
    - Full definition of CESK machine
    - Example derivation from a language construct into a transition rule
    - Full set of transition rules
    - Peano arithmetic
    - Language grammar
    - Repository links

---------------------------------------------------------

## 0. Abstract

This paper presents an abstract interpreter using a memory model based on the CESK machine by Might et al. We have developed a C-like language to facilitate the exploration of an imperative CESK machine through call-by-value λ-calculus encodings. We also discuss our CESK machine's implications of memory safety in array indexing.  We demonstrate that our techinques for deriving a lanugage can be scaled to larger language constructs, such as classes or anonymous functions. Included is a compilation of relevant work and tools necessary to reproduce our work.

---------------------------------------------------------
Imperative implementation of Might et al.'s CESK machine.

C-like imperative language.

Memory saftey in array indexing

Recursively enumerable to context sensitive

Next steps, plan for after the paper is written.

Appendix at the end including everything relevant to reproduce our work

---------------------------------------------------------
