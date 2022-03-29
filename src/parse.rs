use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

use crate::ast::{Conditional, Node, Op, *};

pub enum Error {
    ParseError,
}

#[derive(Parser)]
#[grammar = "../planout.pest"]
pub struct PlanoutParser;

pub fn parse(src: &str) {
    let res = PlanoutParser::parse(Rule::program, src);
    println!("{:?}", res);
}

fn compile_set(mut inner: Pairs<Rule>) -> Node {
    let id = inner.next().unwrap();
    assert_eq!(id.as_rule(), Rule::ident);

    assert_eq!(inner.next().unwrap().as_rule(), Rule::op_assign);

    assert_eq!(inner.next_back().unwrap().as_rule(), Rule::semi);

    let value = compile_block(inner);

    Node::Op(Op::Set {
        var: id.to_string(),
        value: Box::new(value),
    })
}

fn compile_conditional(mut inner: Pairs<Rule>) -> Conditional {
    let when = compile_op(inner.next().expect("cond when")).expect("cond op");
    if let Node::Op(then) = compile_block(inner) {
        Conditional { when, then }
    } else {
        panic!("")
    }
}

fn compile_expr<'a, T: IntoIterator<Item = Pair<'a, Rule>>>(mut inner: T) -> Node {
    match inner.into_iter().map(compile_op).next() {
        Some(Some(js @ Node::Json(..))) => js,
        Some(Some(op @ Node::Op(..))) => op,
        _ => panic!("simple exprs only"),
    }
}

fn compile_block<'a, T: IntoIterator<Item = Pair<'a, Rule>>>(mut inner: T) -> Node {
    let mut ops = vec![];

    for p in inner.into_iter() {
        match p.as_rule() {
            Rule::block_start | Rule::block_end => continue,
            _ => {
                let o = compile_op(p).unwrap();
                match o {
                    js @ Node::Json(..) => return js,
                    Node::Op(op) => ops.push(op),
                }
            }
        };
    }

    if ops.len() == 1 {
        Node::Op(ops.pop().unwrap())
    } else {
        Node::Op(Op::Seq { seq: ops })
    }
}

fn compile_op(pair: Pair<Rule>) -> Option<Node> {
    let rulety = pair.as_rule();
    println!("compiling ty: {:?}", rulety);
    eprintln!("{:?}", pair);
    match rulety {
        Rule::ret => {
            let mut inner = pair.into_inner();
            let expr = inner.next().unwrap();
            assert_eq!(expr.as_rule(), Rule::expr);
            assert_eq!(inner.next_back().unwrap().as_rule(), Rule::semi);
            Some(compile_expr(expr.into_inner()))
        }
        Rule::number => Some(Node::Json(pair.as_str().parse::<f64>().unwrap().into())),
        Rule::block_start | Rule::block_end => None,
        Rule::expr => Some(compile_expr(pair.into_inner())),
        Rule::statement => Some(compile_block(pair.into_inner())),
        Rule::ident => Some(Node::Op(Op::Get(Get {
            var: pair.as_str().to_string(),
        }))),
        Rule::assignment => Some(compile_set(pair.into_inner())),
        Rule::conditional => {
            let cond = compile_conditional(pair.into_inner());
            Some(Node::Op(Op::Cond { cond: vec![cond] }))
        }
        Rule::EOI => None,
        _ => todo!(),
    }
}

pub fn compile(src: &str) -> Result<Node, Error> {
    let pairs = PlanoutParser::parse(Rule::program, src).unwrap();

    let mut ops = Vec::new();

    for pair in pairs {
        match compile_op(pair) {
            Some(Node::Op(op)) => ops.push(op),
            Some(Node::Json(..)) => panic!("constants do nothing as a top level statement"),
            None => continue,
        }
    }

    Ok(Node::Op(Op::Seq { seq: ops }))
}
