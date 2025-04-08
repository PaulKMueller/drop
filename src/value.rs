use std::ops::{Add, Div, Mul, Sub};
use std::fmt;
use std::cmp;
use std::clone;

#[derive(Debug)]
pub struct Value{
    pub number: i32,
}

impl Value {
    pub fn new(number: i32) -> Self {
        Value { number }
    }

    pub fn get(&self) -> i32 {
        self.number
    }
}

impl clone::Clone for Value {
    fn clone(&self) -> Self {
        Self { number: self.number }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.number)
    }
}

impl cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
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


#[cfg(test)]

#[test]
fn test_operators() {
    let c = Value::new(5) + Value::new(4);
    assert_eq!(c, Value::new(9));

    let c = Value::new(5) - Value::new(4);
    assert_eq!(c, Value::new(1));

    let c = Value::new(8) / Value::new(4);
    assert_eq!(c, Value::new(2));

    let c = Value::new(2) * Value::new(4);
    assert_eq!(c, Value::new(8));
}
