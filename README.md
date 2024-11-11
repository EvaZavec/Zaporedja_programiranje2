# Zaporedja - Projektna naloga pri premetu Programiranje 2

TODO
## Namestitev in uporaba
TODO
## Razpoložljiva zaporedja
Spodaj so našteta in opisana implementirana zaporedja s številom prametrov ter zaporedij potrebnih za njihovo tvorbo:

### Arithmetic sequence

📚 **Description**: `arithmetic`, sequence with two parameters: the starting element and the step.<br>⚙️ **Parameters**:   2 <br>🚀 **Sequences**:   0  


### Chaos

📚 **Description**: `chaos` sequence takes two parameters, the starting element and the parameter of chaos, and returns the next element according to the formula: $ c_{k} = r \cdot c_{k-1} \cdot (1 - c_{k-1}) $. <br>⚙️ **Parameters**:   2  <br>🚀 **Sequences**:   0


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
 
 📚 **Description**: `cross_product` sequence takes two sequences $a_n$ and $b_n$ and returns the next element as the cross product:  $a_n \cdot b_{n-1} + a_{n-1} \cdot b_n$ . <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2


### Drop
 
 📚 **Description**: `drop` sequence is the given sequence shifted by the parameter delay. <br>⚙️ **Parameters**:   1  <br>🚀 **Sequences**:   1


### Fibonacci sequence
 
 📚 **Description**: `fibonacci` sequence takes two sequences and as the next element returns the sum of previous two elements, each multiplied by a coresponding element from one of the sequences. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2


### Linear combination
 
 📚 **Description**: `lin_comb` calculated by the formula: $a \cdot a_k + b \cdot b_k + c$ . <br>⚙️ **Parameters**:   3  <br>🚀 **Sequences**:   2


### Maximum

📚 **Description**: `maximum` sequence takes two sequences and returns their maximum. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2

### Quadratic

📚 **Description**: `quadratic` sequence is calculated according to the quadratic function: $a \cdot x^{2} + bx + c $, where a, b, c are parameters and x is the k-th element of the given sequence. <br>⚙️ **Parameters**:   3  <br>🚀 **Sequences**:   1



### Random

📚 **Description**: `random` sequence returns a random number between k-th elements of both sequences. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   2



### Switch

📚 **Description**: `switch` sequence takes three parameters, the lower and upper limit and the switch. If the random number between limits is lower than the switch, it returns the k-th element of the first sequence and if it is higher the k-th element of the second sequence.. <br>⚙️ **Parameters**:   3  <br>🚀 **Sequences**:   2


### Weighted average

📚 **Description**: `weighted_average` is calculated by formula: $w_{k} a_{k} + (1-w_k) b_{k}$, where $w_k$ is the weighted sequence and $a_k, b_k$ are two other sequences. <br>⚙️ **Parameters**:   0  <br>🚀 **Sequences**:   3



## License
[MIT](https://choosealicense.com/licenses/mit/)

