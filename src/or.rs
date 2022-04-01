/// "Optimized" Representation
use crate::number::Number;

//struct Function {}

macro_rules! types {
    ($($ty:ident),+) => {

        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
        pub(crate) enum Ty {
            $($ty),+
        }

        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub(crate) enum Val {
            $($ty($ty)),+
        }

        impl Val {
            pub(crate) fn ty(&self) -> Ty {
               match self {
                    $(Self::$ty(..) => Ty::$ty),+
               }
            }
        }

        $(
            impl From<$ty> for Val {
                fn from(inner: $ty) -> Self {
                    Self::$ty(inner)
                }
            }
        )+
    };
}

types!(
    Number,
    Bool,
    String,
    Assign,
    Pointer,
    Param,
    Array,
    Stack,
    Mul,
    Sum,
    Conditional
);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Array(Vec<Val>);

pub type Bool = bool;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Assign {
    pub(crate) var: String,
    pub(crate) value: Box<Val>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Param {
    pub(crate) name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Pointer {
    pub(crate) var: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Stack {
    pub(crate) inner: Vec<Val>,
}

// Multiplication
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Mul {
    pub(crate) base: Option<Number>,
    pub(crate) rest: Vec<Val>,
}

// Addition and Subtraction
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Sum {
    lhs: Box<Val>,
    rhs: Box<Val>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Branch {
    // If some, evaluate for truth, else always then
    when: Option<Val>,
    then: Stack,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Conditional {
    // Ordered by priority, i.e. inner[0] is evaluated first
    inner: Vec<Branch>,
}

//#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
//#[serde(tag = "op")]
//#[serde(rename_all = "camelCase")]
//pub enum Op {
//    Set { var: String, value: Box<Node> },
//    Get(Get),
//    Seq { seq: Vec<Op> },
//    UniformChoice { choices: Box<Op>, unit: Box<Op> },
//    BernoulliTrial { p: f64, unit: Box<Op> },
//    Product { values: Vec<Node> },
//    Sum { values: Vec<Node> },
//    Array { values: Vec<Node> },
//    Cond { cond: Vec<Conditional> },
//    Index { index: String, base: Get }:
//}
//
//#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
//pub struct Get {
//    pub(crate) var: String,
//}
//
//#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
//pub struct Conditional {
//    #[serde(rename = "if")]
//    pub when: Node,
//    pub then: Op,
//}
//
//impl TryFrom<Node> for Op {
//    type Error = anyhow::Error;
//    fn try_from(node: Node) -> anyhow::Result<Op, Self::Error> {
//        match node {
//            Node::Op(op) => Ok(op),
//            _ => Err(anyhow::anyhow!("found json not op")),
//        }
//    }
//}
