/// "Intermediate" Representation. This should be functionally
/// equivalent to the PlanOut IR references.
use serde::{Deserialize, Serialize};

pub type Value = serde_json::Value;
pub type Number = serde_json::Number;
pub type Object = serde_json::Map<String, Value>;

pub(crate) fn bool_true() -> Node {
    Node::Json(Value::Bool(true))
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Node {
    Op(Op),
    Json(Value),
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
    Product { values: Vec<Node> },
    Sum { values: Vec<Node> },
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

impl TryFrom<Node> for Op {
    type Error = anyhow::Error;
    fn try_from(node: Node) -> anyhow::Result<Op, Self::Error> {
        match node {
            Node::Op(op) => Ok(op),
            _ => Err(anyhow::anyhow!("found json not op")),
        }
    }
}
