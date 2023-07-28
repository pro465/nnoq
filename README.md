# nnoq
Not [noq](https://github.com/tsoding/Noq). Also not [eqthy](https://github.com/catseye/Eqthy).

This aims to be a very simple theorem prover (_nay, verifier_) based on functional expression rewriting to educate myself and others about the basics of theorem provers.

It also has types to ensure the expressions are not nonsensical.

Examples are on the examples directory.

# Foundations
I guess I should explain what the terms actually mean first:
- $A: T$ means that $A$ is of type $T$.
- $\Gamma$ denotes a set $x_0 := y_0, x_1 := y_1, \ldots, x_n := y_n$ of axioms (defined using the `axiom` keyword).
- $\Pi$ denotes a set $x_1:A_1, x_2:A_2, \ldots, x_n: A_n$  of type assignments (defined using the `type` keyword).
- in $A[x/Y]$, $x/Y$ denotes a pattern/replacement pair, where $x$ is a variable, while $B$ is a term.
- $\overline{P}$ denotes a vector of terms $P_0, P_1, \ldots, P_n$.    
- the substitution algorithm $A[x/Y]$ is defined as follows (taking $V$ to be the set of all variable names):
    1. ${\displaystyle {a[a/B] = B}}$.
    2. ${\displaystyle {b[a/B] = b}}, b \in V - \set{a}$.
    3. ${\displaystyle {f(A_0, A_1, \ldots, A_n)[x/Y] = f(A_0[x/Y], A_1[x/Y], \ldots, A_n[x/Y])}}$.
 

The heart of nnoq is a single operator, `:=`, which is governed by the following axioms:  
    1. ${\displaystyle {{} \over \Gamma \vdash A := A}}$ (reflexivity)   
    2. ${\displaystyle {{} \over \Gamma, A := B, \Gamma ' \vdash A := B}}$ (derivability from axioms)  
    3. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Gamma \vdash B := C \over \Gamma \vdash A := C}}$ (transitivity)  
    4. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash f(\overline{P}, A, \overline{P} ') := f(\overline{P}, B, \overline{P} ')}}$ (congruence)    
    5. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash A[x/Y] := B[x/Y]}}$ (substitution)   
 
the following two axioms are for the type analysis (inference and checking):  
    6. ${\displaystyle {{} \over \Pi, A : T, \Pi ' \vdash A : T}}$ (derivability from type assignments/declarations)  
    7. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Pi \vdash A: T \over \Pi \vdash B: T}}$  

