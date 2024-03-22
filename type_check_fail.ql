// Comment Test: Example CNOT program in Quill
Maistow create oo cbit c1 with value b0
Maistow create oo cbit c2 with value b1
Canstow create oo qubit q1 with value +
Canstow create oo qubit q2 with value 0
Thy cnot shalt target q2 and control on q1
Rede q1 and quyken c1
Return 1024
// error on prev line
Rede q2 and quyken c2
Return 1024, qir
