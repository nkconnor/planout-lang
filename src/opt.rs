use std::collections::HashMap;

use crate::ir::{self, Node, Op};
use crate::number::Number;
use crate::or::{self, *};
use anyhow::{anyhow, bail, ensure, Result};

// lifetimes questionable
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Var {
    Val(Val),
    Branch(Vec<Val>),
}

struct State {
    inner: HashMap<String, Var>,
}

impl Default for State {
    fn default() -> Self {
        State {
            inner: HashMap::default(),
        }
    }
}

impl State {
    fn insert(&mut self, var: &str, val: &Val) {
        self.inner.insert(var.to_string(), Var::Val(val.clone()));
    }

    fn get(&mut self, var: &str) -> Option<Var> {
        let v = self.inner.get(var).cloned();
        eprintln!("{} is {:?}", var, v);
        v
    }
}

fn optimize_mul_transitive(vals: Vec<Val>) -> Result<Mul> {
    fn split_recursive(vals: Vec<Val>) -> (Vec<Number>, Vec<Val>) {
        let mut nums = vec![];
        let mut others = vec![];

        for val in vals {
            match val {
                Val::Number(n) => nums.push(n),
                Val::Mul(Mul { base, rest }) => {
                    let (l, r) = split_recursive(rest);
                    others.extend(r);
                    nums.extend(l);
                    if let Some(n) = base {
                        nums.push(n)
                    }
                }
                other => others.push(other),
            }
        }

        (nums, others)
    }

    eprintln!("before: {:?}", vals.clone());
    let (nums, vals) = split_recursive(vals);
    eprintln!("after: {:?} {:?}", nums.clone(), vals.clone());

    let base = nums.into_iter().reduce(|acc, n| acc * n);

    Ok(Mul { base, rest: vals })
}

fn optimize_mul(vals: Vec<or::Val>) -> Result<Mul> {
    // Our first "optimization"
    optimize_mul_transitive(vals)
}

fn optimize_product(values: Vec<ir::Node>, state: &mut State) -> Result<Mul> {
    let vals = values
        .into_iter()
        .map(|n| optimize_node(n, state))
        .collect::<Result<Vec<_>>>()?;

    optimize_mul(vals)
}

// supposably could generate 1 or more Val, this could probably
// take a handle to the stack
fn optimize_node(node: Node, state: &mut State) -> Result<Val> {
    Ok(match node {
        Node::Json(ir::Value::Number(num)) => Val::Number(Number::from(num)),
        Node::Json(ir::Value::String(str)) => Val::String(str),
        Node::Json(ir::Value::Bool(b)) => Val::Bool(b),

        Node::Op(ir::Op::Get(ir::Get { var })) => {
            match state.get(var.as_str()) {
                None => Val::Param(Param { name: var }),
                // replace pointer with the value ??
                // not clear til we do conditionals
                Some(Var::Val(val)) => val,

                _ => unimplemented!(),
            }
        }

        Node::Op(ir::Op::Set { var, value }) => {
            let val = optimize_node(*value, state)?;
            state.insert(var.as_str(), &val);
            Val::Assign(Assign {
                var,
                value: Box::new(val),
            })
        }

        Node::Op(ir::Op::Product { values }) => Val::Mul(optimize_product(values, state)?),

        _ => todo!(),
    })
}

fn optimize(nodes: Vec<Op>) -> Result<Stack> {
    let mut state = State::default();
    let mut stack = Vec::with_capacity(nodes.len());

    let mut nodes = nodes.into_iter();
    while let Some(node) = nodes.next() {
        stack.push(optimize_node(Node::Op(node), &mut state)?);
    }

    // Stack will have a fixed size from now on
    stack.shrink_to_fit();
    Ok(Stack { inner: stack })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::compile;
    use pretty_assertions::assert_eq;

    fn assert_stack(script: &str, stack: Stack) {
        let ir = compile(script).unwrap().ops;

        eprintln!("{ir:#?}");
        let or = optimize(ir).unwrap();

        assert_eq!(or, stack)
    }

    #[test]
    fn test_mul_opt() {
        assert_stack(
            r#"
            y = x * 2.0 * 2.0;
        "#,
            Stack {
                inner: vec![Val::Assign(Assign {
                    var: "y".to_owned(),
                    value: Box::new(Val::Mul(Mul {
                        base: Some(Number::F64(4.0)),
                        rest: vec![Val::Param(Param {
                            name: "x".to_string(),
                        })],
                    })),
                })],
            },
        )
    }

    // We should be able to track assignments
    // at any step during compilation, and for example,
    // lookup a Pointer("y") -> Mul { 4.0, x }
    //
    // TODO
    // What about the case like this:
    //
    // if(x) {
    //    // local state is maintained that doesn't effect
    //    // the parent state
    //    y = 3.0;
    //    y = y * 2.0;
    // } else {
    //    y = 5.0;
    // }
    //
    // // y != deterministic, shouldn't be able to get the full state of it
    // // or, would need multiple branches
    // z = y * y
    #[test]
    fn test_mul_opt_deeper() {
        let rest_x = vec![Val::Param(Param {
            name: "x".to_string(),
        })];
        assert_stack(
            r#"
            y = x * 2.0 * 2.0;
            z = y * 2.0;
        "#,
            Stack {
                inner: vec![
                    Val::Assign(Assign {
                        var: "y".to_owned(),
                        value: Box::new(Val::Mul(Mul {
                            base: Some(Number::F64(4.0)),
                            rest: rest_x.clone(),
                        })),
                    }),
                    Val::Assign(Assign {
                        var: "z".to_owned(),
                        value: Box::new(Val::Mul(Mul {
                            base: Some(Number::F64(8.0)),
                            rest: rest_x,
                        })),
                    }),
                ],
            },
        )
    }
}
