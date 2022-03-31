use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

use crate::ast::{Conditional, Node, Op, *};

use anyhow::{anyhow, bail, ensure, Result};

#[derive(Parser)]
#[grammar = "../planout.pest"]
pub struct PlanoutParser;

pub fn parse(src: &str) {
    let res = PlanoutParser::parse(Rule::program, src);
    println!("{:?}", res);
}

fn compile_set(mut inner: Pairs<Rule>) -> anyhow::Result<Node> {
    let id = inner.next().unwrap();
    assert_eq!(id.as_rule(), Rule::ident);

    skip_front(&mut inner, Rule::op_assign)?;
    skip_back(&mut inner, Rule::semi)?;

    let value = compile_block(inner)?;

    Ok(Node::Op(Op::Set {
        var: id.as_span().as_str().to_string(),
        value: Box::new(value),
    }))
}

fn compile_conditional(mut inner: Pairs<Rule>) -> Result<Conditional> {
    let when = inner
        .next()
        .ok_or(anyhow!("missing if condition"))
        .and_then(compile_op)?;

    if let Node::Op(then) = compile_block(inner)? {
        Ok(Conditional { when, then })
    } else {
        panic!("")
    }
}

fn compile_expr<'a>(pair: Pair<Rule>) -> Result<Node> {
    let inner = pair.into_inner();

    let mut vals = inner
        .into_iter()
        .map(compile_op)
        .collect::<Result<Vec<_>>>()?;

    vals.pop().ok_or(anyhow!("expected inner expression"))
}

fn compile_block<'a, T: IntoIterator<Item = Pair<'a, Rule>>>(mut inner: T) -> Result<Node> {
    let mut ops = vec![];

    for p in inner.into_iter() {
        match p.as_rule() {
            Rule::block_start | Rule::block_end => continue,
            _ => {
                let o = compile_op(p).unwrap();
                match o {
                    js @ Node::Json(..) => return Ok(js),
                    Node::Op(op) => ops.push(op),
                }
            }
        };
    }

    if ops.len() == 1 {
        Ok(Node::Op(ops.pop().unwrap()))
    } else {
        Ok(Node::Op(Op::Seq { seq: ops }))
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

fn compile_array(pair: Pair<Rule>) -> Result<Node> {
    let mut inner = pair.into_inner();
    skip_front(&mut inner, Rule::array_start)?;
    skip_back(&mut inner, Rule::array_end)?;

    let values = inner.map(compile_expr).collect::<Result<Vec<_>>>()?;

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

fn compile_op(pair: Pair<Rule>) -> Result<Node> {
    let rulety = pair.as_rule();
    eprintln!("compiling ty: {:?}", rulety);
    eprintln!("{:?}", pair);
    match rulety {
        Rule::ret => {
            let mut inner = pair.into_inner();
            let expr = inner.next().unwrap();
            skip_back(&mut inner, Rule::semi)?;
            compile_expr(expr)
        }
        Rule::number => compile_number(pair),
        Rule::expr => compile_expr(pair),
        Rule::statement => compile_block(pair.into_inner()),
        Rule::ident => Ok(Node::Op(Op::Get(Get {
            var: pair.as_str().to_string(),
        }))),
        Rule::assignment => compile_set(pair.into_inner()),
        Rule::conditional => {
            let cond = compile_conditional(pair.into_inner())?;
            Ok(Node::Op(Op::Cond { cond: vec![cond] }))
        }
        Rule::array => compile_array(pair),
        _ => todo!(),
    }
}

pub fn compile(src: &str) -> anyhow::Result<Op> {
    let pairs = PlanoutParser::parse(Rule::program, src)?;

    let mut ops = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::EOI => (),
            _ => match compile_op(pair)? {
                Node::Op(op) => ops.push(op),
                Node::Json(..) => bail!("constants do nothing as a top level statement"),
            },
        }
    }

    Ok(Op::Seq { seq: ops })
}

#[cfg(test)]
mod tests {}
