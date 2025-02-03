pragma circom 2.1.6;
// x1 == 0 || x2 == 0 || x3 == 0 circuit
template Example () {
    signal input x1;
    signal input x2;
    signal input x3;
    signal output out;
    
    // Boolean constraints (0/1 checks)
    x1 * (x1 - 1) === 0;
    x2 * (x2 - 1) === 0;
    x3 * (x3 - 1) === 0;

    // Quadratic reformulation of x1*x2*x3 === 0
    signal temp;
    temp <== x2 * x3;  // Quadratic constraint
    x1 * temp === 0;   // Quadratic constraint

    out <== temp * x1;

}

component main { public [x1,x2,x3] } = Example();


/* INPUT = {
    "x1": "1",
    "x2": "1",
    "x3": "0"
} */