# Quill
A Middle English inspired quantum programming language!

## How to install and use
TBD
## Why
This project has been a very fun and interesting way to teach myself more about Rust and compilers. (At the time of writing) Last summer, I worked at Quantinuum on a DSL based on the Python parser. This was very rewarding in that it allowed me to learn a lot about compiler optimization and code generation, but left the idea of constructing a parser or an AST structure abstracted away. With Quill, my aim was to take a fun spin on quantum programming and see if I could learn more about parsing and syntax trees, while still following through on creating a fully functional language. 

## Middle English
Some of the uncommon middle english terms I've used are listed below, and most are borrowed from this California State University document on the [100 Most Frequent Middle English Words](https://www.csustan.edu/sites/default/files/ENGLISH/Perrello/Chaucer_glossary4-30.pdf).

The notable words I've used are:
1. Canstow: Equivalent to "can thou", or in modern English, "can you"
2. Maistow: Equivalent to "may thou", or in modern English, "may you". More respectful than canstow, naturally
3. oo: Equivalent to "one"
4. Rede: Equivalent to "read", which we are using as measure
5. Quyken: Equivalent to "give life to", we are using this as "send value to", wherein we are taking the measurement from the qubit and "giving life" to the classical bit through the measurement

Some other interesting terms that I wrote down and were considered were:
1. Echo: Equivalent to "each one" -> I considered using this for the for loop syntax, but for loops were scrapped in the (initial) final product
2. Hastow: Equivalent to "have thou" -> I considered this for conditional branching
3. Trewe and Fals: Self-explanatory
4. Nys and Ne: Equivalent to "not" -> I was going to use nys as "not" (like the ! operator) and ne like "invert" (like the ~ operator)
5. Clepe: Equivalent to "call" -> I considered this in my gate application syntax, to be used as "calling a function" like "Clepe x unto qubit1"
6. Certes: Equivalent to "certainly" -> I was going to use this for constant variable declarations, but I realized this feature was not as necessary as I initially thought

## Features and Feature Roadmap
### Current Features
The current features are:
- Variable Assignment (to a set number of simple, relevant types)
- Gate Application to Qubits or Quantum Registers (QRegs)
- Measurement of Qubits and applying these values to classical bits
- Returning the output of running the circuit, as well as the generated code to an optional output type of the user's choice (QIR, QASM, or Qiskit)
- Comments (because everyone needs to document their code!)

All of these will be demonstrated in the "Examples" section.

There are some interesting interactions with the `PI` construct, which is native to Quill largely because `PI` (or multiples of it) are used frequently as inputs to parameterized gates, such as the `rx` gate. Because of its ubiquity, I decided to make it more useful, so `PI` can now take the forms:
1. `PI`: Use this just to get the value of `PI`
2. `PI[i]`: Use this to get the value of `PI * i`
3. `PI[i, j]`: Use this to get the value of `PI * (i / j)` 

Also note that the keywords "Canstow" and "Maistow" are important to consider, because if you don't use enough "Maistow"s for respect, you will lose out on optimizations! Make sure to keep the ratio of "Maistow" to total variable declarations above 50%! The ratio requirements may be subject to change :D

### Feature Roadmap:
On the docket for (potential) future additions to Quill are:
- If/elif/else statements
- For and While loops
- More potentially painful and perilous syntax (beware)

## Examples

