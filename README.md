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

Also note that the keywords "Canstow" and "Maistow" are important to consider, because if you don't use enough "Maistow"s for respect, you will lose out on optimizations! Make sure to keep the ratio of "Maistow" to total variable declarations above 50%! The ratio requirements may be subject to change :D

### Feature Roadmap:
On the docket for (potential) future additions to Quill are:
- If/elif/else statements
- For and While loops
- More potentially painful and perilous syntax (beware)

## Examples
Below is a simple example of creating a Bell State using Quill:
```
// Comment Test: Example Bell State program in Quill
Maistow create oo creg c1 with value 0
Maistow create oo creg c2 with value 0
Canstow create oo qubit q1 with value +
Canstow create oo qubit q2 with value 0
Thy cnot shalt target q2 and control on q1
Rede q1 and quyken c1
Rede q2 and quyken c2
Return 1024, qir
```

## Full Documentation
_Note_: Variable names cannot start with a number, but otherwise can contain alphanumeric entries.

### Types and their Values:
In Quill, we refer to the types in all lowercase, as such: qubit, qreg, cbit, creg, int, float. Below are more details on what values they hold and how to instantiate them.

1. Qubit: Can take on the values `0`, `1`, `+`, and `-`, representing the Z and X computational bases.
2. QReg: Can take on the values `qubit[N]`, where qubit is one of the Qubit values, and N is an integer. An example is `+[3]`. In addition, you can "add" QRegs together during instantiation, which acts as a tensor product. An example of this is `0[4] + +[3] + 1[2]`. In this way, you can instantiate different ranges of a QReg with different values.
3. CBit: Can take on the values `0` and `1`, acts as a boolean but is designed to be the recipient of the measurement of qubits.
4. CReg: Very similar to the QReg, with the main difference being instead of `qubit[N]`, it is `cbit[N]`, where cbit is one of the CBit values. The "addition" / "tensor product" rules remain the same as the QReg.
5. Int: Quite self-explanatory (I hope), any positive or negative integer can be input, such as `-42` or `123456789` or even `0`! To be clear, factorials are not supported (but could be if there was, for some reason, enough demand).
6. PI: Technically its own thing, I've added `PI` natively to the language for ease of use. See below.

There are some interesting interactions with the `PI` construct, which is native to Quill largely because `PI` (or multiples of it) are used frequently as inputs to parameterized gates, such as the `rx` gate. Because of its ubiquity, I decided to make it more useful, so `PI` can now take the forms:
1. `PI`: Use this just to get the value of `PI`
2. `PI[i]`: Use this to get the value of `PI * i`
3. `PI[i, j]`: Use this to get the value of `PI * (i / j)` 

### Variable Assignment:
(Canstow / Maistow) create oo `type` `variable_name` with value `value_of_var`

_Ex_: `Canstow create oo qubit q1 with value 0`

### Gate Application:
There are many different types of gates, and some slight nuances in how we use some types of gates; below are the details of all of these differences. Before we begin, most parameters to gate application statements are either the gate name or a variable, which can be of type Qubit or of type QRegSlice. What this means is that we can apply a gate to something like `qubit1` or something like `qreg1[i]`. In the case of single qubit gates, we can also apply them to `qreg[i..j]`, for example, which will apply a given gate to all of the selected qubits at once. QRegs are 0-indexed.

*Single Qubit Gate*: Thy `gate_name` shalt target `variable`
_Ex1_: `Thy x shalt target q1` (Applied to qubit `q1`)
_Ex2_: `Thy y shalt target qreg1[2]` (Applied to qubit 2 in quantum register `qreg1`)
_Ex3_: `Thy z shalt target qreg1[0..3]` (Applied to qubits 0 through 3, inclusive, in quantum register `qreg1`)

*Single Qubit Parameterized Gate*: Thy `gate_name` shalt target `variable` with [`var1`, `var2`, ...]
_Note_: `var1`, `var2`, etc., are only valid when numerical values such as integers, floats, or the PI construct.

_Ex_: `Thy u3 shalt target q1 with [1.2, 1, PI[2]]`

*Double Qubit Gate*: Thy `gate_name` shalt target `variable1` and control on `variable2`
_Ex_: `Thy `
