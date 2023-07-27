# nnoq
Not [noq](https://github.com/tsoding/Noq). Also not [eqthy](https://github.com/catseye/Eqthy).

This aims to be a very simple theorem prover (_nay, verifier_) based on functional expression rewriting to educate myself and others about the basics of theorem provers.

It also has types to ensure the expressions are not nonsensical.

Examples are on the examples directory.

# Foundations
The heart of nnoq is a single operator, `:=`, which is governed by the following axioms ($A: T$ means that $A$ is of type $T$):
1. ${\displaystyle {\over \Gamma \vdash A := A}}$
2. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad \Gamma \vdash B := C \over \Gamma \vdash A := C}}$
3. ${\displaystyle {\Gamma \vdash A := B \qquad \qquad A: T \over B: T}}$
4. ${\displaystyle {\Gamma \vdash A := B \over \Gamma \vdash f(A, \overline{a}) := f(B, \overline{a})}}$, where $\overline{a}$ is the rest of the arguments of $f$.

that is, `:=` is reflexive, transitive, and congruent. in fact it is basically typed equational logic without the symmetry and substitution axioms.
