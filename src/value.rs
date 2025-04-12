use std::clone;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

// TODO: Add gradient variable
// TODO: Build Graphviz visualization including gradient and Value labels

use ordered_float::OrderedFloat;

#[derive(Eq, Hash, PartialEq)]
pub struct Value {
    pub number: OrderedFloat<f32>,
    pub gradient: OrderedFloat<f32>,
    pub operation: Option<char>,
    pub children: Vec<Rc<Value>>,
}

impl Value {
    pub fn new(number: f32) -> Self {
        let number = OrderedFloat::from(number);
        Value {
            number,
            children: vec![],
            operation: None,
            gradient: OrderedFloat::from(0.0),
        }
    }

    pub fn new_manual(
        number: f32,
        children: Vec<Rc<Value>>,
        operation: Option<char>,
        gradient: f32,
    ) -> Self {

        let number = OrderedFloat::from(number);
        let gradient = OrderedFloat::from(gradient);
        Value {
            number,
            children,
            operation,
            gradient,
        }
    }

    // pub fn backward(&mut self) {
    //     self.gradient = 1;
    //     for child in self.children.as_slice() {
    //         child.backward();
    //     }
    // }

    pub fn get(&self) -> OrderedFloat<f32> {
        self.number
    }
}

impl clone::Clone for Value {
    fn clone(&self) -> Self {
        Self {
            number: self.number,
            children: self.children.clone(),
            operation: self.operation,
            gradient: self.gradient,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value({}, {:?})", self.number, self.operation)
    }
}

// impl cmp::PartialEq for Value {
//     fn eq(&self, other: &Self) -> bool {
//         self.number == other.number
//     }
// }

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        Value {
            number: self.number * other.number,
            children: vec![self.into(), other.into()],
            operation: Some('*'),
            gradient: OrderedFloat::from(0.0),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        Value {
            number: self.number / other.number,
            children: vec![self.into(), other.into()],
            operation: Some('/'),
            gradient: OrderedFloat::from(0.0),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        Value {
            number: self.number + other.number,
            children: vec![self.into(), other.into()],
            operation: Some('+'),
            gradient: OrderedFloat::from(0.0),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        Value {
            number: self.number - other.number,
            children: vec![self.into(), other.into()],
            operation: Some('-'),
            gradient: OrderedFloat::from(0.0),
        }
    }
}

#[cfg(test)]
#[test]
fn test_operators() {
    // let c = Value::new(5.0) + Value::new(4.0);
    // assert_eq!(c, Value::new_manual(9.0, vec![], Some('+'), 0.0));

    // let c = Value::new(5.0) - Value::new(4.0);
    // assert_eq!(c, Value::new_manual(1.0, vec![], Some('-'), 0.0));

    // let c = Value::new(8.0) / Value::new(4.0);
    // assert_eq!(c, Value::new_manual(2.0, vec![], Some('*'), 0.0));

    // let c = Value::new(2.0) * Value::new(4.0);
    // assert_eq!(c, Value::new_manual(8.0, vec![], Some('/'), 0.0));

    // let c = Value::new(2.0) + Value::new(3.0) * Value::new(4.0);
    // println!("{:?}", c);
    // assert_eq!(c, Value::new_manual(14.0, vec![], Some('+'), 0.0));
}
