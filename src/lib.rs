#![allow(dead_code)]
extern crate serde;

extern crate nom;

pub mod api;
pub mod ast;
pub mod compiler;
pub mod eval;
pub mod parse;

use eval::*;

pub type Variable = serde_json::Value;
type Variables = serde_json::Map<String, Variable>;

#[cfg(test)]
mod tests {
    use crate::ast::Op;
    use crate::{evaluate, Variable, Variables};
    use std::str::FromStr;

    fn check(compiled_test: &str, expected: &str) {
        let expected: Variable = serde_json::Value::from_str(expected).unwrap();

        let op: Op = serde_json::from_str(compiled_test).unwrap();
        let mut vars = Variables::new();
        let result = evaluate(&mut vars, &op);

        assert_eq!(result, expected)
    }

    #[test]
    fn planout_demo() {
        let expected = r#"{
 "group_size": 1,
 "specific_goal": 0,
 "test": true
}"#;

        let compiled = r#"{
  "op": "seq",
  "seq": [
    {
      "op": "set",
      "var": "group_size",
      "value": {
        "choices": {
          "op": "array",
          "values": [
            1,
            10
          ]
        },
        "unit": {
          "op": "get",
          "var": "userid"
        },
        "op": "uniformChoice"
      }
    },
    {
      "op": "set",
      "var": "specific_goal",
      "value": {
        "p": 0.8,
        "unit": {
          "op": "get",
          "var": "userid"
        },
        "op": "bernoulliTrial"
      }
    },
    {
      "op": "cond",
      "cond": [
        {
          "if": {
            "op": "get",
            "var": "specific_goal"
          },
          "then": {
            "op": "seq",
            "seq": [
              {
                "op": "set",
                "var": "ratings_per_user_goal",
                "value": {
                  "choices": {
                    "op": "array",
                    "values": [
                      8,
                      16,
                      32,
                      64
                    ]
                  },
                  "unit": {
                    "op": "get",
                    "var": "userid"
                  },
                  "op": "uniformChoice"
                }
              },
              {
                "op": "set",
                "var": "ratings_goal",
                "value": {
                  "op": "product",
                  "values": [
                    {
                      "op": "get",
                      "var": "group_size"
                    },
                    {
                      "op": "get",
                      "var": "ratings_per_user_goal"
                    }
                  ]
                }
              }
            ]
          }
        },
        {
          "if": true,
          "then": {
            "op": "seq",
            "seq": [
              {
                "op": "set",
                "var": "test",
                "value": true
              }
            ]
          }
        }
      ]
    }
  ]
}"#;

        check(compiled, expected)
    }

    #[test]
    fn set_variables() {
        let compiled = r#"{
  "op": "seq",
  "seq": [
    {
      "op": "set",
      "var": "test",
      "value": "ok"
    },
    {
      "op": "set",
      "var": "ok",
      "value": 5
    }
  ]
}"#;
        let expected_result = r#"{
 "ok": 5,
 "test": "ok"
}"#;

        let expected_result: serde_json::Value =
            serde_json::Value::from_str(expected_result).unwrap();

        let op: Op = serde_json::from_str(compiled).unwrap();
        let mut vars = Variables::new();
        let result = evaluate(&mut vars, &op);

        assert_eq!(expected_result, result)
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
