use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{Number, Value};


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Node {
    Op(Op),
    Json(serde_json::Value)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "op")]
#[serde(rename_all = "camelCase")]
pub enum Op {
    Set {
        var: String,
        value: Box<Node>,
    },
    Get {
        var: String
    },
    Seq {
        seq: Vec<Op>
    },
    UniformChoice {
        choices: Box<Op>,
        unit: Box<Op>,
    },
    BernoulliTrial {
        p: f64,
        unit: Box<Op>,
    },
    Product {
        values: Vec<Op>
    },
    Array {
        values: Value // JsValue..
    },
    Cond {
        cond: Vec<Conditional>
    },
}


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Conditional {
    #[serde(rename = "if")]
    pub when: Node,
    pub then: Op,
}