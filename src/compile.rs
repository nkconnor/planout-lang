use crate::{
    ir::{Conditional, Node, Op, *},
    Plan,
};
use anyhow::{anyhow, bail, ensure, Context, Result};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use std::{collections::HashSet, fmt::Debug};

// experiment assignments / output parameters
type Params = HashSet<String>;

#[derive(Parser)]
#[grammar = "../planout.pest"]
struct PlanoutParser;

// Compile Op::Set
// Compiler could collect a list of assignments here
fn compile_set(pair: Pair<Rule>, params: &mut Params) -> anyhow::Result<Node> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap();
    anyhow::ensure!(id.as_rule() == Rule::ident, "expected ident");
    let var = id.as_span().as_str().to_string();

    skip_front(&mut inner, Rule::op_assign)?;
    skip_back(&mut inner, Rule::semi)?;

    let value = compile_op(
        inner
            .next()
            .ok_or(anyhow!("expected assignment to expr or value"))?,
        params,
    )?;

    params.insert(var.clone());

    Ok(Node::Op(Op::Set {
        var,
        value: Box::new(value),
    }))
}

// Compile Op::Conditional
fn compile_conditional(pair: Pair<Rule>, params: &mut Params) -> Result<Node> {
    fn compile_arm<'a>(
        inner: &mut (impl DoubleEndedIterator<Item = Pair<'a, Rule>> + Debug),
        params: &mut Params,
        is_else: bool,
    ) -> Result<Conditional> {
        let when = if is_else {
            crate::ir::bool_true()
        } else {
            inner
                .next()
                .ok_or(anyhow!("missing if condition"))
                .and_then(|op| compile_op(op, params))?
        };

        let then = compile_block(inner, params)?;

        Ok(Conditional { when, then })
    }

    let mut inner = pair.into_inner();

    let mut conds = Vec::new();

    while let Some(pair) = inner.next() {
        match pair.as_rule() {
            Rule::op_if | Rule::op_else_if => {
                conds.push(compile_arm(&mut inner, params, false)?);
            }
            Rule::op_else => {
                conds.push(compile_arm(&mut inner, params, true)?);
            }
            _ => anyhow::bail!("off track compiling conditional. maybe simplify the parser?"),
        }
    }

    Ok(Node::Op(Op::Cond { cond: conds }))
}

fn compile_dyadic_expr(pair: Pair<Rule>, params: &mut Params) -> Result<Node> {
    let mut inner = pair.into_inner();
    let lhs = compile_op(inner.next().unwrap(), params)?;

    let verb = inner.next().unwrap();

    let rhs = compile_op(inner.next().unwrap(), params)?;

    let op = match verb.as_str() {
        "*" => Op::Product {
            values: vec![lhs, rhs],
        },
        x => anyhow::bail!("unimplemented verb {}", x),
    };

    Ok(Node::Op(op))
}

fn compile_expr<'a>(pair: Pair<Rule>, params: &mut Params) -> Result<Node> {
    let inner = pair.into_inner();

    let mut vals = inner
        .into_iter()
        .map(|op| compile_op(op, params))
        .collect::<Result<Vec<_>>>()?;

    vals.pop().ok_or(anyhow!("expected inner expression"))
}

fn compile_block<'a>(
    inner: &mut (impl DoubleEndedIterator<Item = Pair<'a, Rule>> + Debug),
    params: &mut Params,
) -> Result<Op> {
    skip_front(inner, Rule::block_start)?;

    let mut ops = inner
        // TODO, nested blocks are supported, right?
        .take_while(|i| i.as_rule() != Rule::block_end)
        .map(|i| compile_op(i, params))
        .map(|res| {
            res.and_then(|node| match node {
                Node::Json(..) => Err(anyhow::anyhow!("unexpected json in block")),
                Node::Op(op) => Ok(op),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    if ops.len() == 1 {
        Ok(ops.pop().unwrap())
    } else {
        Ok(Op::Seq { seq: ops })
    }
}

fn skip_front<'a>(iter: &mut impl Iterator<Item = Pair<'a, Rule>>, ty: Rule) -> anyhow::Result<()> {
    let found = iter.next().ok_or(anyhow!("expected {:?}", ty))?.as_rule();
    ensure!(found == ty, "expected {:?} found {:?}", ty, found);
    Ok(())
}

fn skip_back<'a>(
    iter: &mut impl DoubleEndedIterator<Item = Pair<'a, Rule>>,
    ty: Rule,
) -> anyhow::Result<()> {
    let found = iter
        .next_back()
        .ok_or(anyhow!("expected {:?}", ty))?
        .as_rule();
    ensure!(found == ty, "expected {:?} found {:?}", ty, found);
    Ok(())
}

fn compile_array(pair: Pair<Rule>, params: &mut Params) -> Result<Node> {
    let mut inner = pair.into_inner();
    skip_front(&mut inner, Rule::array_start)?;
    skip_back(&mut inner, Rule::array_end)?;

    let values = inner
        .map(|op| compile_expr(op, params))
        .collect::<Result<Vec<_>>>()?;

    Ok(Node::Op(Op::Array { values }))
}

fn compile_number(pair: Pair<Rule>) -> Result<Node> {
    let n = pair
        .into_inner()
        .next()
        .ok_or(anyhow!("expected int or decimal"))?;

    let n: serde_json::Number = serde_json::from_str(n.as_str())?;
    Ok(Node::Json(serde_json::Value::Number(n)))
}

fn compile_string(pair: Pair<Rule>) -> Result<Node> {
    let s = pair
        .into_inner()
        .next()
        .ok_or(anyhow!("expected string_inner"))?;

    Ok(Node::Json(s.as_span().as_str().into()))
}

fn compile_op(pair: Pair<Rule>, params: &mut Params) -> Result<Node> {
    let rule_ty = pair.as_rule();
    //eprintln!("compiling ty: {:?}", rule_ty);
    //eprintln!("{:?}", pair);
    match rule_ty {
        Rule::number => compile_number(pair),
        Rule::string => compile_string(pair),
        Rule::expr | Rule::terms => compile_expr(pair, params),
        Rule::dyadic_expr => compile_dyadic_expr(pair, params),
        //Rule::statement => compile_block(pair.into_inner(), params),
        Rule::ident => Ok(Node::Op(Op::Get(Get {
            var: pair.as_str().to_string(),
        }))),
        Rule::assignment => compile_set(pair, params),
        Rule::conditional => compile_conditional(pair, params),
        Rule::array => compile_array(pair, params),
        rule => anyhow::bail!("rule {:?} isn't implemented", rule),
    }
}

pub fn compile(src: &str) -> anyhow::Result<Plan> {
    let pairs = PlanoutParser::parse(Rule::program, src).context("parsing")?;

    let mut params = HashSet::new();
    let mut ops = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI | Rule::ret => break,
            _ => match compile_op(pair, &mut params)? {
                Node::Op(op) => ops.push(op),
                Node::Json(..) => bail!("constants do nothing as a top level statement"),
            },
        }
    }

    Ok(Plan {
        ops,
        params: params.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    //use crate::ir::Op;
    //use crate::{eval::evaluate, Variable, Variables};
    //use std::str::FromStr;
    //
    //fn check(compiled_test: &str, expected: &str) {
    //    let expected: Variable = serde_json::Value::from_str(expected).unwrap();

    //    let op: Op = serde_json::from_str(compiled_test).unwrap();
    //    let params = Vec::new();
    //    let mut vars = Variables::new();
    //    let result = evaluate(&mut vars, &op);

    //    assert_eq!(result, expected)
    //}
    //
    //    #[ignore]
    //    #[test]
    //    fn planout_demo() {
    //        let expected = r#"{
    // "group_size": 1,
    // "specific_goal": 0,
    // "test": true
    //}"#;
    //
    //        let compiled = r#"{
    //  "op": "seq",
    //  "seq": [
    //    {
    //      "op": "set",
    //      "var": "group_size",
    //      "value": {
    //        "choices": {
    //          "op": "array",
    //          "values": [
    //            1,
    //            10
    //          ]
    //        },
    //        "unit": {
    //          "op": "get",
    //          "var": "userid"
    //        },
    //        "op": "uniformChoice"
    //      }
    //    },
    //    {
    //      "op": "set",
    //      "var": "specific_goal",
    //      "value": {
    //        "p": 0.8,
    //        "unit": {
    //          "op": "get",
    //          "var": "userid"
    //        },
    //        "op": "bernoulliTrial"
    //      }
    //    },
    //    {
    //      "op": "cond",
    //      "cond": [
    //        {
    //          "if": {
    //            "op": "get",
    //            "var": "specific_goal"
    //          },
    //          "then": {
    //            "op": "seq",
    //            "seq": [
    //              {
    //                "op": "set",
    //                "var": "ratings_per_user_goal",
    //                "value": {
    //                  "choices": {
    //                    "op": "array",
    //                    "values": [
    //                      8,
    //                      16,
    //                      32,
    //                      64
    //                    ]
    //                  },
    //                  "unit": {
    //                    "op": "get",
    //                    "var": "userid"
    //                  },
    //                  "op": "uniformChoice"
    //                }
    //              },
    //              {
    //                "op": "set",
    //                "var": "ratings_goal",
    //                "value": {
    //                  "op": "product",
    //                  "values": [
    //                    {
    //                      "op": "get",
    //                      "var": "group_size"
    //                    },
    //                    {
    //                      "op": "get",
    //                      "var": "ratings_per_user_goal"
    //                    }
    //                  ]
    //                }
    //              }
    //            ]
    //          }
    //        },
    //        {
    //          "if": true,
    //          "then": {
    //            "op": "seq",
    //            "seq": [
    //              {
    //                "op": "set",
    //                "var": "test",
    //                "value": true
    //              }
    //            ]
    //          }
    //        }
    //      ]
    //    }
    //  ]
    //}"#;
    //
    //        check(compiled, expected)
    //    }
    //
    //    #[test]
    //    fn set_variables() {
    //        let compiled = r#"{
    //  "op": "seq",
    //  "seq": [
    //    {
    //      "op": "set",
    //      "var": "test",
    //      "value": "ok"
    //    },
    //    {
    //      "op": "set",
    //      "var": "ok",
    //      "value": 5
    //    }
    //  ]
    //}"#;
    //        let expected_result = r#"{
    // "ok": 5,
    // "test": "ok"
    //}"#;
    //
    //        let expected_result: serde_json::Value =
    //            serde_json::Value::from_str(expected_result).unwrap();
    //
    //        let op: Op = serde_json::from_str(compiled).unwrap();
    //        let mut vars = Variables::new();
    //        let result = evaluate(&mut vars, &op);
    //
    //        assert_eq!(expected_result, result)
    //    }
}
