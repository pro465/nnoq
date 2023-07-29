# nnoq
Not [noq](https://github.com/tsoding/Noq). Also not [eqthy](https://github.com/catseye/Eqthy).

This aims to be a very simple theorem prover (_nay, verifier_) based on functional expression rewriting to educate myself and others about the basics of theorem provers.

It also has types to ensure the expressions are not nonsensical.

Examples are on the examples directory.

# Foundations
I guess I should define the terms first:
- small letters denote variable name, capital letters denote terms (exceptions below).
- $T$ denotes a type name.
- $V$ denotes the set of all variable names.
- $F$ denotes the set of all function/constant names.
- term definition:
    1. if $x \in V$ then $x$ is a term.
    2. if $f \in F$ then:
         - $f$ is a term.
         - if $A$ and $B$ are terms then so is $f(A, B)$.

- $A: T$ means that $A$ is of type $T$.
- $\Gamma$ denotes a set $X_0 := Y_0, X_1 := Y_1, \ldots, X_n := Y_n$ of axioms (defined using the `axiom` keyword).
- $\Pi$ denotes a set $X_1: T_1, X_2: T_2, \ldots, X_n: T_n$  of type assignments (defined using the `type` keyword).
- in $A[x/Y]$, $x/Y$ denotes a pattern/replacement pair, where $x$ is a variable, while $Y$ is a term.      
- the substitution algorithm $A[x/Y]$ is defined as follows:
    1. ${\displaystyle {x[x/Y] = Y}}$.
    2. ${\displaystyle {a[x/Y] = a}}, a \in V - \set{x}$.
    3. ${\displaystyle {f[x/Y] = f}}$.
    4. ${\displaystyle {f(A, B)[x/Y] = f(A[x/Y], B[x/Y])}}$.
 

The heart of nnoq is a single operator, `:=`, which is governed by the following axioms:  
    1. ${\displaystyle {{} \over \Gamma \vdash A := A}}$ (reflexivity)   
    2. ${\displaystyle {{} \over \Gamma, A := B, \Gamma ' \vdash A := B}}$ (derivability from axioms)  
    3. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Gamma \vdash B := C \over \Gamma \vdash A := C}}$ (transitivity)  
    4. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash E[x/A] := E[x/B]}}$ (congruence)    
    5. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash A[x/Y] := B[x/Y]}}$ (substitution)   
 
the following two axioms are for the type analysis (inference and checking):  
    6. ${\displaystyle {{} \over \Pi, A : T, \Pi ' \vdash A : T}}$ (derivability from type assignments/declarations)  
    7. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Pi \vdash A: T \over \Pi \vdash B: T}}$  

