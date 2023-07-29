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
- $F$ denotes the set of all function names.
- $K$ denotes the set of all constants.
- the functions have type signature $(T_1, T_2) \mapsto T_3$.
- $\textbf{T}$ denotes any type i.e., type names as well as function types. 
- term definition:
    1. if $x \in V \cup K$ then $x$ is a term.
    3. if $f \in F$, and $A$ and $B$ are terms then so is $f(A, B)$.

- $A: T$ means that $A$ is of type $T$.
- $\Gamma$ denotes a set $X_0 := Y_0, X_1 := Y_1, \ldots, X_n := Y_n$ of axioms (defined using the `axiom` keyword).
- $\Pi$ denotes a set $X_1: \textbf{T}_1, X_2: \textbf{T}_2, \ldots, X_n: \textbf{T}_n$  of type assignments (defined using the `type` keyword).
- in $A[x/Y]$, $x/Y$ denotes a pattern/replacement pair, where $x$ is a variable, while $Y$ is a term.      
- the substitution algorithm $A[x/Y]$ is defined as follows:
    1. ${\displaystyle {x[x/Y] = Y}}$.
    2. ${\displaystyle {a[x/Y] = a, a \in V - \set{x}}}$.
    3. ${\displaystyle {k[x/Y] = k, k \in K}}$.
    4. ${\displaystyle {f(A, B)[x/Y] = f(A[x/Y], B[x/Y])}}$.
 

The heart of nnoq is a single operator, `:=`, which is governed by the following axioms:  
    1. ${\displaystyle {{} \over \Gamma \vdash A := A}}$ (reflexivity)   
    2. ${\displaystyle {{} \over \Gamma, A := B, \Gamma ' \vdash A := B}}$ (derivability from axioms)  
    3. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Gamma \vdash B := C \over \Gamma \vdash A := C}}$ (transitivity)  
    4. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash A[x/Y] := B[x/Y]}}$ (substitution)   
    5. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Pi \vdash f(A, C): T \over \Gamma \vdash f(A, C) := f(B, C)}}$ (congruence1)    
    6. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Pi \vdash f(C, A): T \over \Gamma \vdash f(C, A) := f(C, B)}}$ (congruence2)    
 
the following two axioms are for the type analysis (inference and checking):  
    7. ${\displaystyle {{} \over \Pi, A : \textbf{T}, \Pi ' \vdash A : \textbf{T}}}$ (derivability from type assignments/declarations)  
    8. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Pi \vdash A: T \over \Pi \vdash B : T}}$    
    9. ${\displaystyle {\Pi \vdash f: (T_1, T_2) \mapsto T_3 \qquad \qquad \Pi \vdash A: T_1 \qquad \qquad \Pi \vdash B: T_2 \over \Pi \vdash f(A, B): T_3}}$  

