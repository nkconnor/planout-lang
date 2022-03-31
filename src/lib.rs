#![allow(dead_code)]
extern crate serde;

#[macro_use]
extern crate pest_derive;

pub(crate) mod compile;
pub(crate) mod eval;
pub(crate) mod ir;

type Variable = serde_json::Value;
type Variables = serde_json::Map<String, Variable>;

pub use compile::compile;

pub struct Plan {
    ops: Vec<ir::Op>,
    params: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::{compile::compile, eval::evaluate};
    use serde_json::{json, Value};

    fn run_test(
        plan: &'static str,
        input: Value,
        overrides: impl Into<Option<Value>>,
        output: Value,
    ) {
        if let Value::Object(mut input) = input {
            let ir = compile(plan).expect("compile ok");

            match overrides.into() {
                Some(Value::Object(ov)) => {
                    let res = evaluate(&mut input, Some(&ov), &ir).unwrap();
                    assert_eq!(res, output);
                }
                None => {
                    let res = evaluate(&mut input, None, &ir).unwrap();
                    assert_eq!(res, output);
                }
                _ => panic!("overrides not an object"),
            }
        } else {
            panic!("input is not an object")
        }
    }

    #[test]
    fn test_return_breaks() {
        run_test(
            r#"x = y * 2;
            return 3;
            z = 5.0;"#,
            json!({ "y": 3.0 }),
            None,
            json!({"x": 6.0}),
        )
    }

    #[test]
    fn test_product_mul() {
        run_test(
            r#"x = y * 2;"#,
            json!({ "y": 3.0 }),
            None,
            json!({"x": 6.0}),
        )
    }

    #[test]
    fn test_simple_overrides() {
        run_test(
            r#"
            x = y * 2;
            "#,
            json!({"y": 3.0}),
            json!({"x": 2.0}),
            json!({"x": 2.0}),
        )
    }

    #[test]
    fn test_simple_fp_and_int_passthrough() {
        run_test(
            r#"
            z = 3.0;
        "#,
            json!({}),
            None,
            json!({
                "z": 3.0
            }),
        );
        run_test(
            r#"
            z = 3;
        "#,
            json!({}),
            None,
            json!({
                "z": 3
            }),
        )
    }

    #[test]
    fn test_array_assignment() {
        run_test(
            r#"
            z = [y, x, "string"];
        "#,
            json!({
                "y": 3,
                "x": 5.0
            }),
            None,
            json!({
                "z": [3, 5.0, "string"]
            }),
        )
    }
}
