use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Number {
    I64(i64),
    U64(u64),
    F64(f64),
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        use Number::*;

        match (self, rhs) {
            (I64(l), I64(r)) => I64(l + r),
            (I64(l), U64(r)) => I64(l + i64::try_from(r).unwrap()),
            (I64(l), F64(r)) => F64(l as f64 + r),
            (U64(l), U64(r)) => U64(l + r),
            (F64(l), F64(r)) => F64(l + r),
            (F64(l), U64(r)) => F64(l + r as f64),
            (l, r) => r + l,
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        use Number::*;

        match (self, rhs) {
            (I64(l), I64(r)) => I64(l * r),
            (I64(l), U64(r)) => I64(l * i64::try_from(r).unwrap()),
            (I64(l), F64(r)) => F64(l as f64 * r),
            (U64(l), U64(r)) => U64(l * r),
            (F64(l), F64(r)) => F64(l * r),
            (F64(l), U64(r)) => F64(l * r as f64),
            (l, r) => r * l,
        }
    }
}

impl From<serde_json::Number> for Number {
    fn from(js: serde_json::Number) -> Self {
        if js.is_i64() {
            return Self::I64(js.as_i64().unwrap());
        }
        if js.is_u64() {
            return Self::U64(js.as_u64().unwrap());
        }
        if js.is_f64() {
            return Self::F64(js.as_f64().unwrap());
        }

        unreachable!()
    }
}
