# cesk

## Questions
- Control can either be a reference to the ast or have ownership of some value or expression even?
- Configuration now has ownership of control?
- Swapped Addresses to int, how should an address be represented? (int, enum, struct)?
- Figure out the reference set up for configuration's members.
- Do we enforce curly braces for if blocks? all blocks?
- How does shadowing work?
- Responsibilities of successor function?
- Does break/continue utilize the successor function for its purpose?

## New Questions/Topics

- Are we doing semantic analysis?
  - Type checking arguments.
  - Type checking in general.
  - Number of arguments vs parameters.
- Do we need Expression Statements?
- Is there a reason for name (in ast)?
  - It's just a string reference.
- We'd like to discuss changing languages?
