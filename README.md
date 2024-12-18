# Sequences - Project Assignment for the Course Programming 2

This project implements a sequence generator for floating-point numbers `(f64)` indexed by natural numbers `(u64)`. The generator operates as an `HTTP` server, capable of producing sequences, processing requests for sequence compositions, and interacting with other generators registered in the central registry.

## Installation and Usage

Use the command below to start the generator:

```bash
cargo run -- <REGISTRY_IP> <GENERATOR_IP> <PORT> 
```
When the generator starts, it automatically registers itself with the central registry using the specified IP and port. When they are not given, the default values `0.0.0.0` and `9000` are used. 

The generator can answer the following `GET` and `POST` requests:
- `GET /ping/`: It returns information about the generator, that were given in registration.
- `GET /sequence/`: It provides information about sequences available on this generator.
- `POST /sequence/<seq-name>/`: It returns the elements of `<seq-name>`with given arguments and within given range. If sequence uses other sequences, they can be requested from other available generators.

## Available Sequences
Below are listed and described the implemented sequences, including the number of parameters and sequences required for their formation:

### Arithmetic sequence

📚 **Description**: `arithmetic`, sequence with two parameters: the starting element and the step.<br>⚙️ **Parameters**:   2 <br>🚀 **Sequences**:   0  


### Chaos

📚 **Description**: `chaos` sequence takes two parameters, the starting element and the parameter of chaos, and returns the next element according to the formula: cₖ = r ⋅ cₖ₋₁ ⋅ (1 - cₖ₋₁). <br>⚙️ **Parameters**:   2  <br>🚀 **Sequences**:   0


 ### Constant sequence

📚 **Description**: `constant` sequence of a given parameter: $c$. <br>⚙️ **Parameters**:   1  <br>🚀 **Sequences**:   0   


 
### Geometric sequence

📚 **Description**: `geometric` sequence with two parameters: the starting element and the ratio. <br>⚙️ **Parameters**:   2  <br>🚀 **Sequences**:   0 



### Sum

📚 **Description**: `sum` sequence takes two sequences and returns their sum. <br>⚙️ **Parameters**:   0 <br>🚀 **Sequences**:   2   
 


 ### Product

📚 **Description**: `product` sequence takes two sequences and returns their product. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2 

 

 ### Partial product
 
📚 **Description**: `partial_product` sequence returns products of all previous element of the given sequence. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   1
 

### Cross product
 
 📚 **Description**: `cross_product` sequence takes two sequences $a_n$ and $b_n$ and returns the next element as the cross product:  $a_n  b_{n-1} + a_{n-1}  b_n$ . <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2


### Drop
 
 📚 **Description**: `drop` sequence is the given sequence shifted by the parameter delay. <br>⚙️ **Parameters**:   1  <br>🚀 **Sequences**:   1


### Fibonacci sequence
 
 📚 **Description**: `fibonacci` sequence takes two sequences and as the next element returns the sum of previous two elements, each multiplied by a coresponding element from one of the sequences. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2


### Linear combination
 
 📚 **Description**: `lin_comb` calculated by the formula: $a \cdot a_k + b \cdot b_k + c$ . <br>⚙️ **Parameters**:   3  <br>🚀 **Sequences**:   2


### Maximum

📚 **Description**: `maximum` sequence takes two sequences and returns their maximum. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2

### Quadratic

📚 **Description**: `quadratic` sequence is calculated according to the quadratic function: $a x^{2} + bx + c $, where a, b, c are parameters and x is the k-th element of the given sequence. <br>⚙️ **Parameters**:   3  <br>🚀 **Sequences**:   1



### Random

📚 **Description**: `random` sequence returns a random number between k-th elements of both sequences. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2



### Switch

📚 **Description**: `switch` sequence takes three parameters, the lower and upper limit and the switch. If the random number between limits is lower than the switch, it returns the k-th element of the first sequence and if it is higher the k-th element of the second sequence.. <br>⚙️ **Parameters**:   3  <br>🚀 **Sequences**:   2


### Weighted average

📚 **Description**: `weighted_average` is calculated by formula: $w_{k} a_{k} + (1-w_k) b_{k}$, where $w_k$ is the weighted sequence and $a_k, b_k$ are two other sequences. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   3


## Authors
Leila Mokrovič in Eva Zavec
## License
[MIT](https://choosealicense.com/licenses/mit/)

