use crate::ast::*;
use crate::Variables;
use serde_json::*;

pub fn uniform_choice(op: &Op) -> serde_json::Value {
    match op {
        Op::Array { values } => match values {
            Value::Array(values) => values.first().unwrap().clone(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

pub fn evaluate_node<'p>(vars: &'p mut Variables, op: &'p Node) -> serde_json::Value {
    match op {
        Node::Json(value) => {
            println!("Evaluating JSON {:?}: ", value);
            value.clone()
        }
        Node::Op(op) => evaluate(vars, op),
    }
}

pub fn evaluate<'p>(vars: &'p mut Variables, op: &'p Op) -> serde_json::Value {
    match op {
        Op::Seq { seq } => {
            for op in seq {
                evaluate(vars, op);
            }

            serde_json::to_value(vars).expect("Vars serializable")
        }
        Op::Set { var, value } => {
            let eval = evaluate_node(vars, value.as_ref());
            vars.insert(var.clone(), eval);
            serde_json::to_value(vars).unwrap()
        }

        Op::Get(Get { var }) => vars
            .get(var.as_str())
            .cloned()
            .expect(&format!("Environmental variable {} should exist", var))
            .clone(),
        Op::UniformChoice { choices, unit: _ } => uniform_choice(choices.as_ref()),
        //Op::BernoulliTrial { p: _, unit: _ } => 0.into(),
        //Op::Product { values } => {
        //    let p = values.into_iter().fold(1.0, |acc, op| {
        //        let value = evaluate(vars, op);
        //        match value {
        //            Value::Number(n) => n.as_f64().unwrap() * acc,
        //            _ => unimplemented!(),
        //        }
        //    });

        //    p.into()
        //}
        Op::Array { values } => values.clone(),
        Op::Cond { cond } => {
            let result = cond.iter().find(|conditional| {
                evaluate_node(vars, &conditional.when).eq(&serde_json::Value::Bool(true))
            });

            println!("Found matching arm {:?}: ", result);

            result
                .map(|cond| evaluate(vars, &cond.then))
                .unwrap_or(serde_json::Value::Null)
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_serde_with_reference() {}
}
