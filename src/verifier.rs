use crate::{
    error::{Error, ErrorTy, Loc},
    parser::{Def, Expr, ParamTy, Rule, Type},
};
use std::{
    collections::{HashMap, HashSet},
    vec::IntoIter,
};

pub struct Verifier {
    ty: HashMap<String, Type>,
    rules: HashMap<String, Rule>,
    rest: IntoIter<Def>,
}

impl Verifier {
    pub fn new(v: Vec<Def>) -> Self {
        Self {
            rest: v.into_iter(),
            ty: HashMap::new(),
            rules: HashMap::new(),
        }
    }

    pub fn verify_next(&mut self) -> Result<Option<()>, Error> {
        let curr = match self.rest.next() {
            Some(x) => x,
            None => return Ok(None),
        };

        match curr {
            Def::Type(t) => {
                if self.ty.contains_key(&t.name) {
                    return Err(Error {
                        loc: t.loc,
                        ty: ErrorTy::VerifError,
                        desc: "duplicate type signature definition for ".to_string() + &t.name,
                    });
                }
                self.ty.insert(t.name.clone(), t);
            }
            Def::Rule(a) => {
                if self.rules.contains_key(&a.name) {
                    return Err(Error {
                        loc: a.loc,
                        ty: ErrorTy::VerifError,
                        desc: "duplicate rule definition for ".to_string() + &a.name,
                    });
                }
                self.check_closed(&a)?;
                let (map, ret) = self.typecheck(&a)?;

                if !self.ty.contains_key(&a.name) {
                    self.ty.insert(
                        a.name.clone(),
                        Type {
                            name: a.name.clone(),
                            loc: a.loc,
                            param: a.params.iter().map(|p| map[p].clone()).collect(),
                            ret,
                        },
                    );
                }
                if a.proof.is_some() {
                    self.verify_proof(map, &a)?;
                }

                self.rules.insert(a.name.clone(), a);
            }
        }

        Ok(Some(()))
    }

    fn check_closed(&self, a: &Rule) -> Result<(), Error> {
        let mut param_names = HashSet::new();
        for p in a.params.iter() {
            if !param_names.insert(p.clone()) {
                return Err(Error {
                    loc: a.loc,
                    ty: ErrorTy::VerifError,
                    desc: format!("duplicate variable declaration for {}", p),
                });
            }
        }
        let mut pat_names = HashSet::new();
        insert_vars(&mut pat_names, &a.pat);

        if !pat_names.is_disjoint(&param_names) {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!(
                    "duplicate parameter name(s): {}",
                    vars_to_string(pat_names.intersection(&param_names).cloned().collect())
                ),
            });
        }

        let mut rep_names = HashSet::new();
        insert_vars(&mut rep_names, &a.rep);
        let union = &param_names | &pat_names;
        if !rep_names.is_subset(&union) {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!(
                    "undeclared variable(s): {}",
                    vars_to_string(rep_names.difference(&union).cloned().collect())
                ),
            });
        }

        if !param_names.is_subset(&rep_names) {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!(
                    "unused parameter(s): {}",
                    vars_to_string(param_names.difference(&rep_names).cloned().collect())
                ),
            });
        }

        if let Some(proof) = &a.proof {
            for call in proof {
                if !self.rules.contains_key(&call.f) {
                    return Err(Error {
                        loc: call.loc,
                        ty: ErrorTy::VerifError,
                        desc: format!("rule named `{}` not declared", call.f),
                    });
                }
                for arg in &call.args {
                    let mut free_vars = HashSet::new();
                    insert_vars(&mut free_vars, arg);
                    if !free_vars.is_subset(&union) {
                        return Err(Error {
                            loc: call.loc,
                            ty: ErrorTy::VerifError,
                            desc: format!(
                                "undeclared variable(s): {}",
                                vars_to_string(free_vars.difference(&union).cloned().collect())
                            ),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn typecheck(&self, a: &Rule) -> Result<(HashMap<String, ParamTy>, String), Error> {
        let typedef = self.ty.get(&a.name);

        let mut map = if let Some(typedef) = typedef {
            if typedef.param.len() != a.params.len() {
                return Err(Error {
                    loc: a.loc,
                    ty: ErrorTy::VerifError,
                    desc: format!("mismatched arity for rule named `{}`: type signature implies arity is {} while actual arity is {}", a.name, typedef.param.len(), a.params.len()),
                });
            }
            a.params
                .iter()
                .cloned()
                .zip(typedef.param.iter().cloned())
                .collect()
        } else {
            HashMap::new()
        };
        let ty = self.typecheck_expr(
            &mut map,
            &a.pat,
            a.loc,
            typedef.map(|typedef| ParamTy {
                name: typedef.ret.clone(),
                loc: typedef.loc,
            }),
        )?;

        let ty = self.typecheck_expr(&mut map, &a.rep, a.loc, ty)?;
        let ty = if let Some(ty) = ty {
            ty
        } else {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!("cannot determine type for {}", a.name),
            });
        };

        if let Some(proof) = &a.proof {
            for call in proof {
                let call_typedef = self.ty[&call.f].clone();
                if call_typedef.param.len() != call.args.len() {
                    return Err(Error {
                        loc: call.loc.clone(),
                        ty: ErrorTy::VerifError,
                        desc: format!(
                            "function requires {} arguments while {} were provided",
                            call_typedef.param.len(),
                            call.args.len()
                        ),
                    });
                }

                for (ty, arg) in call_typedef.param.iter().zip(call.args.iter()) {
                    self.typecheck_expr(&mut map, arg, a.loc, Some(ty.clone()))?;
                }
            }
        }

        Ok((map, ty.name))
    }

    fn typecheck_expr(
        &self,
        m: &mut HashMap<String, ParamTy>,
        a: &Expr,
        floc: Loc,
        ty: Option<ParamTy>,
    ) -> Result<Option<ParamTy>, Error> {
        match a {
            Expr::Var { name, loc } => {
                let ty = match ty {
                    Some(ty) => ty,
                    None => {
                        if let Some(ty) = m.get(name) {
                            return Ok(Some(ty.clone()));
                        } else {
                            return Ok(None);
                        }
                    }
                };
                let v = m.entry(name.clone()).or_insert(ty.clone());
                if v.name == ty.name {
                    Ok(Some(ty))
                } else {
                    Err(Error {
                        loc: loc.clone(),
                        ty: ErrorTy::VerifError,
                        desc: format!("mismatched types: expected {}, found {}", ty.name, v.name),
                    })
                }
            }
            Expr::Func { name, args, loc } => {
                if !self.ty.contains_key(name) {
                    return Err(Error {
                        loc: loc.clone(),
                        ty: ErrorTy::VerifError,
                        desc: format!("no type signatures declared for function: {}", name),
                    });
                }
                let v = self.ty[name].clone();

                if v.param.len() != args.len() {
                    return Err(Error {
                        loc: loc.clone(),
                        ty: ErrorTy::VerifError,
                        desc: format!(
                            "function requires {} arguments while {} were provided",
                            v.param.len(),
                            args.len()
                        ),
                    });
                }
                for (ty, arg) in v.param.iter().zip(args.iter()) {
                    self.typecheck_expr(
                        m,
                        arg,
                        floc,
                        Some(ParamTy {
                            name: ty.name.clone(),
                            loc: floc,
                        }),
                    )?;
                }

                let ty = match ty {
                    Some(t) => t,
                    None => {
                        return Ok(Some(ParamTy {
                            name: v.ret.clone(),
                            loc: floc,
                        }))
                    }
                };

                if v.ret == ty.name {
                    Ok(Some(ty))
                } else {
                    Err(Error {
                        loc: loc.clone(),
                        ty: ErrorTy::VerifError,
                        desc: format!("mismatched types: expected {}, found {}", ty.name, v.ret),
                    })
                }
            }
        }
    }
    fn verify_proof(&mut self, mut ty: HashMap<String, ParamTy>, a: &Rule) -> Result<(), Error> {
        let mut res = a.pat.clone();
        for call in a.proof.clone().unwrap() {
            let rule_ty = &self.ty[&call.f];
            let rule = &self.rules[&call.f];
            let res2 = index(&mut res, &call.idxs)?;
            self.typecheck_expr(
                &mut ty,
                res2,
                a.loc,
                Some(ParamTy {
                    name: rule_ty.ret.clone(),
                    loc: call.loc,
                }),
            )?;
            let mut vars = HashMap::new();
            match_(&mut vars, &rule.pat, res2)?;
            for (v, e) in rule.params.iter().zip(call.args.iter()) {
                vars.insert(v.clone(), e.clone());
            }
            *res2 = rule.rep.clone();
            replace(&vars, res2);
            if let Some(ret) = call.ret {
                if !is_eq(&res, &ret) {
                    return Err(Error {
                        loc: call.loc,
                        ty: ErrorTy::VerifError,
                        desc: format!("expected expression `{}`, found expression `{}`", ret, res),
                    });
                }
            }
        }

        if !is_eq(&res, &a.rep) {
            return Err(Error {
                loc: a.loc,
                ty: ErrorTy::VerifError,
                desc: format!(
                    "expected expression `{}`, found expression `{}`",
                    a.rep, res
                ),
            });
        }
        Ok(())
    }
}

fn insert_vars(m: &mut HashSet<String>, a: &Expr) {
    match a {
        Expr::Var { name, .. } => {
            m.insert(name.clone());
        }
        Expr::Func { args, .. } => args.iter().for_each(|v| insert_vars(m, v)),
    }
}

fn match_(m: &mut HashMap<String, Expr>, pat: &Expr, e: &Expr) -> Result<(), Error> {
    match (pat, e) {
        (Expr::Var { name, loc, .. }, _) => {
            if let Some(v) = m.get(name) {
                if !is_eq(v, e) {
                    return Err(Error {
                        loc: loc.clone(),
                        ty: ErrorTy::VerifError,
                        desc: format!("bindings not equal: {} != {}", v, e),
                    });
                }
            }
            m.insert(name.clone(), e.clone());
        }

        (
            Expr::Func {
                name: x, args: pb, ..
            },
            Expr::Func {
                name: y, args: eb, ..
            },
        ) if x == y && pb.len() == eb.len() => {
            for (p, e) in pb.iter().zip(eb.iter()) {
                match_(m, p, e)?;
            }
        }

        (Expr::Func { loc, .. }, ..) => {
            return Err(Error {
                loc: loc.clone(),
                ty: ErrorTy::VerifError,
                desc: format!("cannot match pattern {} with expression {}", pat, e),
            })
        }
    }

    Ok(())
}

fn is_eq(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Var { name, .. }, Expr::Var { name: n2, .. }) => name == n2,
        (
            Expr::Func { name, args, .. },
            Expr::Func {
                name: n2, args: a2, ..
            },
        ) => name == n2 && args.iter().zip(a2.iter()).all(|(a, b)| is_eq(a, b)),
        _ => false,
    }
}

fn replace(m: &HashMap<String, Expr>, succed: &mut Expr) {
    match succed {
        Expr::Var { name, .. } => {
            *succed = m[name].clone();
        }
        Expr::Func { args, .. } => {
            for c in args.iter_mut() {
                replace(m, c);
            }
        }
    }
}

fn index<'a>(r: &'a mut Expr, idxs: &[usize]) -> Result<&'a mut Expr, Error> {
    if idxs.is_empty() {
        Ok(r)
    } else {
        match r {
            Expr::Func { args, .. } if idxs[0] < args.len() => {
                if let Expr::Func { args, .. } = r {
                    index(&mut args[idxs[0]], &idxs[1..])
                } else {
                    unreachable!()
                }
            }

            Expr::Func { loc, .. } | Expr::Var { loc, .. } => Err(Error {
                loc: loc.clone(),
                ty: ErrorTy::VerifError,
                desc: format!("cannnot index into {} of {}", idxs[0], r),
            }),
        }
    }
}

fn vars_to_string(v: Vec<String>) -> String {
    "`".to_string() + &v.join("`,`") + "`"
}
