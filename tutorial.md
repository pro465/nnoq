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
In this example, we are saying we take `And(x, y) := And(y, x)` as granted.
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
Theorems are derived from axioms and other theorems. for example, say we have an axiom or theorem  that says we can derive `2` from `1+1`, and another that says we can drive `1*2` from `2`. we can then say we can also derive `1*2` from `1+1`.  
Theorems in nnoq have basically the same syntax as axioms, except they are declared with the `theorem` keyword, and they have a block containing the proof. Example:
```
type Man :: (Thing) :: Bool
type Mortal :: (Thing) :: Bool
type Aristotle :: Thing

axiom men_are_mortal :: [Man(x) :=  Mortal(x)]

axiom aristotle_is_man :: [x :=  Man(Aristotle)]

theorem aristotle_is_mortal :: [x := Mortal(Aristotle)] {
    aristotle_is_man
    men_are_mortal
}
```
in this example, the proof demonstrates how to transform `x` into `Mortal(Aristotle)`, thus establising the fact that `x := Mortal(Aristotle)`.   
you can also assert that the rewritten expression after each transformation matches what you expect, as a form of "checked comment".
```
theorem aristotle_is_mortal :: [x := Mortal(Aristotle)] {
    aristotle_is_man        :: [:= Man(Aristotle)]
    men_are_mortal          :: [:= Mortal(Aristootle)]
}
```

you would also pass expressons as arguments to the calls if they need it. for example, to use the `or_id` axiom defined earlier in proofs:
```
... :: [:= True]
or_id(Not(j), k) :: [:= Or(And(Not(j), k), Not(And(Not(j), k)))]
```
By the way, although these examples only use axioms in their proofs, you can also use other theorems defined earlier in the file.
Note that you can only use those variables in the expressions as arguments which were defined in the "LHS" and the arguments to the current theorms. so you can not use, for example the variable `t` as argument in a proof of this:
```
theorem idk :: (a, b, c) :: [F(d) := G(d, a, b, c)]
```
(yes you can have parameters for theorems too.)  
Also note that you can only use axioms/theorems defined earlier to prove later theorems. This exists primarily to avoid recursion, but also keeps the proof linear and simpler for humans to manually check.

## subexpression rewriting
say you know (or have a proof) that `1 + 1 = 2` and `1 + 1 + 1 = 3`. You can then substitute the latter part of `1 + 1 + 1` to obtain a proof of `1 + 2 = 3`. This is known as congruence. basically, if `A=B` then `f(A)=f(B)`.   
nnoq also has this, in the form of subexpression rewriting. Basically, you specify a list of indices along with the axiom/theorem you are callng, and it goes to the respective subexpression of the current expression, and applies the axiom/theorem to _that_ subexpression.   
the following pseudo-python-code might help:
```py
def get_exp(axiom_or_theorem, expr, list_of_indices):
    if len(list_of_indices) == 0:
        return axiom_or_theorem(expr)
    return get_exp(expr.args[list_of_indices[0]], list_of_indices[1:])
```

for example, consider the following axiom:
```
axiom id :: [F(x) := G(x, x)]
```
and the following expression: 
```
G(
   F(F(F(x))),
   G(
       F(F(x)),
       F(F(F(F(F(F(x))))))
   )
)
```
`id[0]` would apply `id` to the first argument (it's zero-indexed), resulting in:
```
G(
   G(F(F(x)), F(F(x))),
   G(
       F(F(x)),
       F(F(F(F(F(F(x))))))
   )
)
```
`id[0, 0]` would apply `id` to the first argument of the first argument, resulting in:
```
G(
   F(G(F(x), F(x))),
   G(
       F(F(x)),
       F(F(F(F(F(F(x))))))
   )
)
```
if we then apply `id[0, 0, 1]` to the last expression, it would become:
```
G(
   F(G(F(x), G(x, x))),
   G(
       F(F(x)),
       F(F(F(F(F(F(x))))))
   )
)
```
similarly, applying `id[1, 0]` to the original would result in:
```
G(
   F(F(F(x))),
   G(
       G(F(x), F(x)),
       F(F(F(F(F(F(x))))))
   )
)
```
finally, applying `id[1, 1, 0]` would result in:
```
G(
   F(F(F(x))),
   G(
       F(F(x)),
       F(G(F(F(F(F(x)))), F(F(F(F(x))))))
   )
)
```
