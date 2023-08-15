# Installation
Firstly, you would need Rust and Cargo installed.
Then, just type:
```shell
cargo install --git https://github.com/pro465/nnoq
```
to install the latest version of nnoq.
then, run `nnoq` in your terminal. you should see the usage message. if you pass it a file name, and it is a syntactically valid and well typed nnoq file, and all the theorems follow from the axioms, then it should print `verified`.

# Preliminaries
nnoq is based on expression rewriting, much like eqthy and noq. however, unlike them, it is typed, and is based on a single operator, `:=` which is just `=` (equality) without commutavity.
What that means is, if you have `A = B`, it also means `B = A`, which is not true for `:=`: `A := B` does not mean `B := A`. Basically, `A := B` means "A can be rewritten to B".

for example: 
```
1 + 1 := 2
A and B := B and A
P and P -> Q := Q
```

Notes:   
    1. the examples are not valid nnoq syntax. However, they are just an example to give you an idea of what `:=` does.  
    2. in the first example, we cannot derive the inverse (`2 := 1 + 1`). However we can do so in the second one, since it is it's own inverse ([Involution](https://en.wikipedia.org/wiki/Involution_(mathematics))).   
    3. the inverses of the first two examples are valid. However, the third's converse is [not valid](https://github.com/catseye/Eqthy/issues/4). this is an example of why `:=` is not just a restriced `=`. It allows for more finer grained proofs than `=`, while also not giving up the expressivity of `=`.   
    
# Types
TODO
# Axioms
TODO
# Theorems
TODO
