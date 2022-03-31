use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Node {
    Op(Op),
    Json(serde_json::Value),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "op")]
#[serde(rename_all = "camelCase")]
pub enum Op {
    Set { var: String, value: Box<Node> },
    Get(Get),

    Seq { seq: Vec<Op> },
    UniformChoice { choices: Box<Op>, unit: Box<Op> },
    BernoulliTrial { p: f64, unit: Box<Op> },
    Product { values: Vec<Op> },
    Array { values: Vec<Node> },
    Cond { cond: Vec<Conditional> },
    Index { index: String, base: Get },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Get {
    pub(crate) var: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Conditional {
    #[serde(rename = "if")]
    pub when: Node,
    pub then: Op,
}
