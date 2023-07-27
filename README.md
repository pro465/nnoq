# nnoq
Not [noq](https://github.com/tsoding/Noq). Also not [eqthy](https://github.com/catseye/Eqthy).

This aims to be a very simple theorem prover (_nay, verifier_) based on functional expression rewriting to educate myself and others about the basics of theorem provers.

It also has types to ensure the expressions are not nonsensical.

Examples are on the examples directory.

# Foundations
The heart of nnoq is a single operator, :=, which is governed by the following axioms:
1. $A := A$
2. $((A := B) \wedge (B := C)) \Rightarrow (A := C)$
3. $((A := B) \wedge (A: T)) \Rightarrow (B: T)$, where $A: T$ informally means "$A$ is of type $T$"
4. $((A := B) \wedge (A: T)) \Rightarrow (f(A, \overline{a}) := f(B, \overline{a}))$, where $\overline{a}$ is the rest of the arguments of $f$.

that is, := is reflexive, transitive, and congruent. in fact it is basically typed equational logic without the symmetry and substitution axioms.
