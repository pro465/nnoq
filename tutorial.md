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
    3. the inverses of the first two examples are valid. However, the third's converse is [not valid](https://github.com/catseye/Eqthy/issues/4). this is an example of why `:=` is not just a restricted `=`. It allows for more finer grained proofs than `=`, while also not giving up the expressivity of `=`.   

# Expressions
`:=`'s expressions on each side can have variables (like `x`, `a`, etc.), contants (like `True`, `False`, `EmptySet`, etc.), or functions of other subexpresions (like `And(x, y)`, `Or(x, Not(x))`, `Implies(p, Not(Not(p)))`, etc.).  
note that  variables begin with lower case, while functions and constants begin with upper case.
constants are handled the same as 0-ary functions, so you can say `True()` instead of `True` and it won't complain.

# Types
every valid expression has a type, whether that is inferred or defined.
expression's types are defined by using the `type` keyword.  
for example: 
```
type And :: (Bool, Bool) :: Bool
```
defines `And` to take two arguments of type `Bool` and return type `Bool`.
you can also omit the parameter part to define a constant, like so:
```
type True :: Bool
```
# Axioms
axioms are things that would be "taken for granted". for example, if you were dealing with logic, you would declare things like modus ponens as axioms, and declare things derived from them as theorems.
In nnoq, you use the `axiom` keyword to define an axiom, like so:
```
axiom and_commute :: [And(x, y) := And(y, x)]
```   
Note that only variables from the "LHS" of the `:=` are considered defined for the "RHS". those not in the "LHS" need to be explicitly stated in a parameter list, like this:
```
axiom or_id :: (x, y) :: [True := Or(And(x, y), Not(And(x, y)))]
```
In the above example, since `x` and `y` did not appear in the left expression (`True`), we need to explicitly define them.  
This won't work:
```
axiom or_id :: [True := Or(And(x, y), Not(And(x, y)))]
```
# Theorems
TODO
