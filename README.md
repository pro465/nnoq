# nnoq
Not [noq](https://github.com/tsoding/Noq). Also not [eqthy](https://github.com/catseye/Eqthy).

This aims to be a very simple theorem prover (_nay, verifier_) based on functional expression rewriting to educate myself and others about the basics of theorem provers.

It also has types to ensure the expressions are not nonsensical.

Examples are on the examples directory.

# Foundations
I guess I should define the terms first:
- small letters denote variable name, capital letters denote terms (exceptions below).
- $T$ denotes a type name.
- $\mathbf{V}$ denotes the set of all variable names.
- $\mathbf{F}$ denotes the set of all function names.
- $\mathbf{K}$ denotes the set of all constants.
- term definition:
    1. if  $x \in \mathbf{V} \cup \mathbf{K}$ then $x$ is a term.
    2. if $f \in \mathbf{F}$, and $A$ and $B$ are terms, then so is $f(A, B)$.    
- the functions have type signature $(T_1, T_2) \mapsto T_3$.
- $\mathbf{T}$ denotes any type i.e., type names as well as function types.  
- $A: \mathbf{T}$ means that $A$ is of type $\mathbf{T}$.
- $\Gamma$ denotes a set $X_0 := Y_0, X_1 := Y_1, \ldots, X_n := Y_n$ of axioms (defined using the `axiom` keyword).
- $\Pi$ denotes a set $X_1: \mathbf{T}_1, X_2: \mathbf{T}_2, \ldots, X_n: \mathbf{T}_n$  of type assignments (defined using the `type` keyword).
- in $A[x/Y]$, $x/Y$ denotes a pattern/replacement pair, where $x$ is a variable, while $Y$ is a term.      
- the substitution algorithm $A[x/Y]$ is defined as follows:
    1. ${\displaystyle {x[x/Y] = Y}}$.
    2. ${\displaystyle {a[x/Y] = a, a \in \mathbf{V} - \set{x}}}$.
    3. ${\displaystyle {k[x/Y] = k, k \in \mathbf{K}}}$.
    4. ${\displaystyle {f(A, B)[x/Y] = f(A[x/Y], B[x/Y])}}$.
 

The heart of nnoq is a single operator, `:=`, which is governed by the following axioms:  
    1. ${\displaystyle {\Pi \vdash A: T \over \Gamma \vdash A := A}}$ (reflexivity)   
    2. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Gamma \vdash B := C \over \Gamma \vdash A := C}}$ (transitivity)  
    3. ${\displaystyle {(A:= B) \in \Gamma \qquad \qquad \Pi \vdash A: T \qquad \qquad \Pi \vdash B: T \over \Gamma \vdash A := B}}$ (derivability from axioms)  
    4. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Gamma \vdash C := D \qquad \qquad \Pi \vdash f(A, C): T \over \Gamma \vdash f(A, C) := f(B, D)}}$ (congruence)    
    5. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Pi \vdash A: T_A \qquad \qquad \Pi \vdash A[x/Y]: T_A \over \Gamma \vdash A[x/Y] := B[x/Y]}}$ (substitution)   
 
the following five axioms are for the type analysis (inference and checking):  
    6. ${\displaystyle {{} \over \Pi, A : \mathbf{T}, \Pi ' \vdash A : \mathbf{T}}}$ (derivability from type assignments/declarations)  
    7. ${\displaystyle {(A := b) \in \Gamma \qquad \qquad \Pi \vdash A: T \over \Pi \vdash b : T}}$ (type inference from `:=` #1)    
    8. ${\displaystyle {(a := B) \in \Gamma \qquad \qquad \Pi \vdash B: T \over \Pi \vdash a : T}}$ (type inference from `:=` #2)    
    9. ${\displaystyle {\Pi, x: T_x \vdash A: T_A \qquad \qquad \Pi, A[x/Y]: T_A \vdash Y: T_x \over \Pi \vdash A[x/Y] : T_A}}$ (typed variable elimination)    
    10. ${\displaystyle {\Pi \vdash f: (T_1, T_2) \mapsto T_3 \qquad \qquad x \neq y \qquad \qquad \forall T. x: T \notin \Pi \qquad \qquad \forall T. y: T \notin \Pi\over \Pi, x: T_1, y: T_2 \vdash f(x, y): T_3}}$ (typed variable introduction)    
nnoq builds on top of this foundation by generalizing the functions to arbitrary arity. however, this does not increase its power, as an n-arity function $f: (T_1, \ldots, T_n) \mapsto T_{ret}$ can be easily emulated in the core by n functions $f_1: (T_{n}, F_0) \mapsto F_1, f_2: (T_{n-1}, F_1) \mapsto F_2, \ldots, f_n: (T_1, F_{n-1}) \mapsto T_{ret}$ and a constant $f_0: F_0$.
