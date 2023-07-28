# nnoq
Not [noq](https://github.com/tsoding/Noq). Also not [eqthy](https://github.com/catseye/Eqthy).

This aims to be a very simple theorem prover (_nay, verifier_) based on functional expression rewriting to educate myself and others about the basics of theorem provers.

It also has types to ensure the expressions are not nonsensical.

Examples are on the examples directory.

# Foundations
The heart of nnoq is a single operator, `:=`, which is governed by the following axioms:  
    1. ${\displaystyle {{} \over \Gamma \vdash A := A}}$ (reflexivity)   
    2. ${\displaystyle {{} \over \Gamma, A := B, \Gamma ' \vdash A := B}}$ (derivability from axioms)  
    3. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Gamma \vdash B := C \over \Gamma \vdash A := C}}$ (transitivity)  
    4. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash f(\overline{a}, A, \overline{a} ') := f(\overline{a}, B, \overline{a} ')}}$ (congruence), where $\overline{a}$ and $\overline{a} '$ are the rest of the arguments of $f$.    
    5. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash A[x/Y] := B[x/Y]}}$ (substitution)   
 
the following two axioms are for the type analysis (inference and checking):  
    6. ${\displaystyle {{} \over \Pi, A : T, \Pi ' \vdash A : T}}$ (derivability from type assignments/declarations)  
    7. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Pi \vdash A: T \over \Pi \vdash B: T}}$  

Okay, so I guess I should explain what the terms actually mean:
- $A: T$ means that $A$ is of type $T$.
- $\Gamma$ denotes a set $x_0 := y_0, x_1 := y_1, \ldots, x_n := y_n$ of axioms (defined using the `axiom` keyword).
- $\Pi$ denotes a set $x_1:A_1, x_2:A_2, \ldots, x_n: A_n$  of type assignments (defined using the `type` keyword).
- in $A[x/Y]$, $x/Y$ denotes a pattern/replacement pair, where $x$ is a variable, while $B$ is a term.    

- the replacement algorithm $A[x/Y]$ is defied as follows:
    1. ${\displaystyle {a[a/B] = B}}$.
    2. ${\displaystyle {b[a/B] = b}}$.
    3. ${\displaystyle {f(a_0, a_1, \ldots, a_n)[x/Y] = f(a_0[x/Y], a_1[x/Y], \ldots, a_n[x/Y])}}$.
