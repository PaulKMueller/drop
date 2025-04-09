use std::clone;
use std::cmp;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

// TODO: Add gradient variable
// TODO: Build Graphviz visualization including gradient and Value labels

pub struct Value {
    pub number: i32,
    pub children: Option<Vec<Value>>,
    pub operation: Option<char>,
}

impl Value {
    pub fn new(number: i32, children: Option<Vec<Value>>, operation: Option<char>) -> Self {
        Value { number, children, operation}
    }

    pub fn get(&self) -> i32 {
        self.number
    }
}

impl clone::Clone for Value {
    fn clone(&self) -> Self {
        Self {
            number: self.number,
            children: self.children.clone(),
            operation: self.operation,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value({}, {:?})", self.number, self.children)
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
            children: Some(vec![self, other]),
            operation: Some('*'),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        Value {
            number: self.number / other.number,
            children: Some(vec![self, other]),
            operation: Some('/'),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        Value {
            number: self.number + other.number,
            children: Some(vec![self, other]),
            operation: Some('+'),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        Value {
            number: self.number - other.number,
            children: Some(vec![self, other]),
            operation: Some('-')
        }
    }
}

#[cfg(test)]
#[test]
fn test_operators() {
    let c = Value::new(5, None, None) + Value::new(4, None, None);
    assert_eq!(c, Value::new(9, None, Some('+')));

    let c = Value::new(5, None, None) - Value::new(4, None, None);
    assert_eq!(c, Value::new(1, None, Some('-')));

    let c = Value::new(8, None, None) / Value::new(4, None, None);
    assert_eq!(c, Value::new(2, None, Some('*')));

    let c = Value::new(2, None, None) * Value::new(4, None, None);
    assert_eq!(c, Value::new(8, None, Some('/')));

    let c = Value::new(2, None, None) + Value::new(3, None, None) * Value::new(4, None, None);
    println!("{:?}", c);
    assert_eq!(c, Value::new(14, None, Some('+')));
}
