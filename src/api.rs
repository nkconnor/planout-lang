use crate::ast::*;
use crate::*;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

macro_rules! plan {
    ( $x:expr  ) => {
        plan($x)
    };
}

fn plan(plan: &str) -> Plan {
    let path = Path::new("/tmp/planout-rs-plan.planout");
    {
        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(&file);
        writer.write(plan.as_bytes()).unwrap();
    }

    let node = "/usr/local/bin/node";

    let cmd = Command::new(node)
        .arg("./planout.js")
        .arg(path)
        .output()
        .expect("planout compiler failed to start");

    let raw = String::from_utf8_lossy(&cmd.stdout);
    println!("Evaluated to {}", raw);
    let ast = serde_json::from_str(&raw).unwrap();
    Plan::new(ast)
}

/// # Examples
///
/// ```
/// let plan = plan!(r#"
///  if (country == 'US') {
///    p = 0.2;
///  } else if (country == 'UK') {
///    p = 0.4;
///  } else {
///    p = 0.1;
///  }"#);
/// ```
pub struct Plan {
    id: String,
    ast: Node,
    overrides: Option<Variables>,
}

impl Plan {
    fn new(ast: Node) -> Plan {
        Plan {
            id: String::from("no-name"),
            ast: ast,
            overrides: None,
        }
    }

    pub fn name(self, name: &str) -> Plan {
        Plan {
            id: String::from(name),
            ast: self.ast,
            overrides: self.overrides,
        }
    }

    pub fn overrides(self, overrides: Variables) -> Plan {
        Plan {
            id: self.id,
            ast: self.ast,
            overrides: Some(overrides),
        }
    }
}

///
pub struct Assignment {
    vars: Variables,
    experiment_id: Option<String>,
}

impl Assignment {
    pub fn get<T: From<serde_json::Value>>(&self, parameter: &str) -> Result<T, serde_json::Error> {
        let v = self
            .vars
            .get(parameter)
            .map(|t| Ok(t.clone().into()))
            .unwrap();
        v
    }
}

pub struct Segment {
    variable: Option<String>,
    size: Option<usize>,
}

impl Segment {
    pub fn variable(variable: &str) -> Segment {
        Segment {
            variable: Some(String::from(variable)),
            size: None,
        }
    }

    pub fn size(self, size: usize) -> Segment {
        Segment {
            variable: self.variable,
            size: Some(size),
        }
    }
}

///
/// # Examples
pub trait Experiment {
    fn ast(&self) -> &Node;

    fn evaluate(&self, input: &mut Variables) -> Assignment {
        match evaluate_node(input, &self.ast()) {
            Value::Object(vars) => Assignment {
                vars,
                experiment_id: None,
            },
            _ => unimplemented!(),
        }
    }
}

impl Experiment for Plan {
    fn ast(&self) -> &Node {
        &self.ast
    }
}

pub struct Meta {
    segment: Segment,
    plans: Vec<Plan>,
}

/// Equivalent to a Namespace
///
/// # Examples
///
/// ```
/// use planout::*;
///
/// let segment = Segment::variable("user_id").size(1000);
///
/// Meta::segment(segment).plan(
///     plan!(first).named("asap_bid_pricing")
/// ).plan(
///    plan!(second).named("even_bid_pricing")
/// );
/// ```
impl Meta {
    fn segment(segment: Segment) -> Meta {
        Meta {
            segment,
            plans: Vec::new(),
        }
    }

    fn plan(self, plan: Plan) -> Meta {
        let mut plans = self.plans;
        plans.push(plan);

        Meta {
            segment: self.segment,
            plans: plans,
        }
    }
}

impl Experiment for Meta {
    fn ast(&self) -> &Node {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        let plan_txt = r#"
        if (country == 'US') {
  p = 0.2;
} else if (country == 'UK') {
  p = 0.4;
} else {
  p = 0.1;
}"#;

        plan!(
            r#"
            if (country == 'US') {
                p = uniformeChoice(choices=[1,20], unit=userid);
            }
        "#
        );

        let plan_compiled = r#"
       {
  "op": "seq",
  "seq": [
    {
      "op": "cond",
      "cond": [
        {
          "if": {
            "op": "equals",
            "left": {
              "op": "get",
              "var": "country"
            },
            "right": "US"
          },
          "then": {
            "op": "seq",
            "seq": [
              {
                "op": "set",
                "var": "p",
                "value": 0.2
              }
            ]
          }
        },
        {
          "if": {
            "op": "equals",
            "left": {
              "op": "get",
              "var": "country"
            },
            "right": "UK"
          },
          "then": {
            "op": "seq",
            "seq": [
              {
                "op": "set",
                "var": "p",
                "value": 0.4
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
                "var": "p",
                "value": 0.1
              }
            ]
          }
        }
      ]
    }
  ]
}"#;

        let plan_compiled: Node = serde_json::from_str(plan_compiled).unwrap();
        let plan_on_compile: Node = plan!(plan_txt);
        assert_eq!(plan(plan_txt), plan_compiled);
        assert_eq!(plan_on_compile, plan_compiled);
    }
}

// fn use_case() {
//     // call compiler
//     // compile
//     // serialized AST in binary at compile time
//     let plan = plan!(
//         if (user_id == 100) {
//             country = uniform_choice(10, 500)
//         } else {
//             country = "US"
//         }
//     );
//
//     let experiment = experiment!("test-such-and-such", plan);
//     let meta = Meta::new(segments, experiments);
//
//     let meta = Meta::new(name, segmentation, plan);
//     let result = meta.assign(user_id=10);
//     let country = result.get("country").unwrap();
//
//
// }