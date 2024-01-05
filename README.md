# Quill
A Middle English inspired quantum programming language

## How to install and use
TBD
## Why
This project has been a very fun and interesting way to teach myself more about Rust and compilers. (At the time of writing) Last summer, I worked at Quantinuum on a DSL based on the Python parser. This was very rewarding in that it allowed me to learn a lot about compiler optimization and code generation, but left the idea of constructing a parser or an AST structure abstracted away. With Quill, my aim was to take a fun spin on quantum programming and see if I could learn more about parsing and syntax trees, while still following through on creating a fully functional language. 

## Middle English

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

## Examples

