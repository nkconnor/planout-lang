use crate::ir::*;
use crate::Plan;
use crate::Variables;

pub(crate) fn evaluate_op<'p>(
    vars: &'p mut Variables,
    op: &'p Op,
) -> anyhow::Result<serde_json::Value> {
    let res = match op {
        Op::Seq { seq } => {
            for op in seq {
                evaluate_op(vars, op)?;
            }

            serde_json::to_value(vars).expect("Vars serializable")
        }
        Op::Array { values } => {
            let vs: Vec<serde_json::Value> = values
                .iter()
                .map(|v| evaluate_node(vars, v))
                .collect::<anyhow::Result<_>>()?;

            vs.into()
        }
        Op::Set { var, value } => {
            let eval = evaluate_node(vars, value.as_ref())?;
            vars.insert(var.clone(), eval);
            serde_json::to_value(vars).unwrap()
        }

        Op::Get(Get { var }) => vars
            .get(var.as_str())
            .cloned()
            .expect(&format!("Environmental variable {} should exist", var))
            .clone(),

        // TODO planout-py supports stuff like "3" * 5 = "33333"
        // and "3" * true = "3", etc.
        Op::Product { values } => {
            let p = values.into_iter().fold(anyhow::Result::Ok(1.0), |acc, op| {
                let acc = acc?;
                let value = evaluate_node(vars, op)?;
                match value {
                    serde_json::Value::Number(n) => Ok(n.as_f64().unwrap() * acc),
                    _ => anyhow::bail!("multiplication is only defined for numbers"),
                }
            });

            p?.into()
        }
        //Op::Array { values } => values.clone(),
        Op::Cond { cond } => {
            for conditional in cond {
                if evaluate_node(vars, &conditional.when)?.eq(&serde_json::Value::Bool(true)) {
                    return evaluate_op(vars, &conditional.then);
                }
            }

            serde_json::Value::Null
        }
        _ => todo!(),
    };

    Ok(res)
}

pub(crate) fn evaluate_node<'p>(
    vars: &'p mut Variables,
    op: &'p Node,
) -> anyhow::Result<serde_json::Value> {
    match op {
        Node::Json(value) => Ok(value.clone()),
        Node::Op(op) => evaluate_op(vars, op),
    }
}

pub(crate) fn evaluate(
    inputs: &mut Variables,
    overrides: Option<&Variables>,
    plan: &Plan,
) -> anyhow::Result<serde_json::Value> {
    for op in plan.ops.iter() {
        evaluate_op(inputs, op)?;
    }

    let mut map: serde_json::Map<String, serde_json::Value> = plan
        .params
        .iter()
        .map(|param| {
            (
                param.to_owned(),
                inputs
                    .remove(param)
                    .expect(format!("param {} is set", param).as_str()),
            )
        })
        .collect();

    match overrides {
        Some(overrides) => {
            for (key, value) in overrides {
                let e = map.get_mut(key).ok_or(anyhow::anyhow!(
                    "expected to override {} but it wasn't a parameter",
                    key
                ))?;
                *e = value.clone();
            }
        }

        _ => (),
    };

    Ok(map.into())
}

#[cfg(test)]
mod tests {}
