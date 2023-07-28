# nnoq
Not [noq](https://github.com/tsoding/Noq). Also not [eqthy](https://github.com/catseye/Eqthy).

This aims to be a very simple theorem prover (_nay, verifier_) based on functional expression rewriting to educate myself and others about the basics of theorem provers.

It also has types to ensure the expressions are not nonsensical.

Examples are on the examples directory.

# Foundations
The heart of nnoq is a single operator, `:=`, which is governed by the following axioms:  
    1. ${\displaystyle {{} \over \Gamma \vdash A := A}}$   
    2. ${\displaystyle {{} \over \Gamma, A := B, \Gamma ' \vdash A := B}}$   
    3. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Gamma \vdash B := C \over \Gamma \vdash A := C}}$  
    4. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash f(\overline{a}, A, \overline{a} ') := f(\overline{a}, B, \overline{a} ')}}$, where $\overline{a}$ and $\overline{a} '$ are the rest of the arguments of $f$.  
(that is, `:=` is reflexive, transitive, and congruent. in fact it is basically typed equational logic without the symmetry and substitution axioms.)  
the following two axioms are for the type analysis (inference and checking):  
    5. ${\displaystyle {{} \over \Pi, A : T, \Pi ' \vdash A : T}}$  
    6. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Pi \vdash A: T \over \Pi \vdash B: T}}$  

Okay, so I guess I should explain what the terms actually mean:
- $A: T$ means that $A$ is of type $T$.
- $\Gamma$ denotes a set $x_0 := y_0, x_1 := y_1, \ldots, x_n := y_n$ of axioms (defined using the `axiom` keyword).
- $\Pi$ denotes a set $x_1:A_1, x_2:A_2, \ldots, x_n: A_n$  of type assignments (defined using the `type` keyword).
