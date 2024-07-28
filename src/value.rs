use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Value{
    pub number: i32,
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        Value {
            number: self.number * other.number,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        Value {
            number: self.number / other.number,
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        Value {
            number: self.number + other.number,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        Value {
            number: self.number - other.number,
        }
    }
}
