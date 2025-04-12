use std::cell::RefCell;
use std::clone;
use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

use ordered_float::OrderedFloat;

#[derive(Eq, PartialEq)]
pub struct Value {
    pub number: OrderedFloat<f32>,
    pub gradient: RefCell<OrderedFloat<f32>>,
    pub operation: Option<char>,
    pub children: RefCell<Vec<Rc<Value>>>,
}

impl Value {
    pub fn new(number: f32) -> Self {
        let number = OrderedFloat::from(number);
        Value {
            number,
            children: RefCell::from(vec![]),
            operation: None,
            gradient: RefCell::from(OrderedFloat::from(0.0)),
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
        let children = RefCell::from(children);
        let gradient = RefCell::from(gradient);
        Value {
            number,
            children,
            operation,
            gradient,
        }
    }

    pub fn backpropagate(self: &Rc<Self>) {
        *self.gradient.borrow_mut() = OrderedFloat(1.0);

        let mut visited = HashSet::new();
        self._backward_recursive(&mut visited);
    }

    fn _backward_recursive(self: &Rc<Self>, visited: &mut HashSet<*const Value>) {
        let ptr = Rc::as_ptr(self);
        if visited.contains(&ptr) {
            return;
        }

        visited.insert(ptr);

        self.backward();
        let children = self.children.borrow();
        for child in children.iter() {
            child._backward_recursive(visited);
        }
    }

    pub fn backward(&self) {
        println!(
            "→ BACKWARD on ptr {:p} val={} grad={:?}",
            self,
            self.number,
            self.gradient.borrow()
        );
        match self.operation {
            Some('+') => {
                let grad = *self.gradient.borrow();
                let children = self.children.borrow();
                for child in children.iter() {
                    *child.gradient.borrow_mut() += grad;
                }
            }
            Some('-') => {
                let grad = *self.gradient.borrow();
                let children = self.children.borrow();
                assert_eq!(children.len(), 2, "Subtraction expects two children");
    
                let left = &children[0];
                let right = &children[1];
    
                *left.gradient.borrow_mut() += grad;
                *right.gradient.borrow_mut() += -grad;
            }
            Some('*') => {
                let grad = *self.gradient.borrow();
                let children = self.children.borrow();
                assert_eq!(children.len(), 2, "Multiplication expects two children");
    
                let left = &children[0];
                let right = &children[1];
    
                *left.gradient.borrow_mut() += grad * right.number;
                *right.gradient.borrow_mut() += grad * left.number;
            }
            Some('/') => {
                let grad = *self.gradient.borrow();
                let children = self.children.borrow();
                assert_eq!(children.len(), 2, "Division expects two children");
    
                let left = &children[0];
                let right = &children[1];
    
                *left.gradient.borrow_mut() += grad / right.number;
                *right.gradient.borrow_mut() += -grad * left.number / (right.number * right.number);
            }
            None => {
                println!("Reached input node: val = {}", self.number);
            }
            Some(op) => {
                println!("Operator '{}' not supported.", op);
            }
        }
    }

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
            gradient: self.gradient.clone(),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value({}, {:?})", self.number, self.operation)
    }
}

#[derive(Clone)]
pub struct V(pub Rc<Value>);

impl Add<&V> for &V {
    type Output = V;

    fn add(self, other: &V) -> V {
        let result = Value {
            number: self.0.number + other.0.number,
            gradient: RefCell::new(OrderedFloat(0.0)),
            operation: Some('+'),
            children: RefCell::new(vec![self.0.clone(), other.0.clone()]),
        };
        V(Rc::new(result))
    }
}

impl Sub<&V> for &V {
    type Output = V;

    fn sub(self, other: &V) -> V {
        let result = Value {
            number: self.0.number - other.0.number,
            gradient: RefCell::new(OrderedFloat(0.0)),
            operation: Some('-'),
            children: RefCell::new(vec![self.0.clone(), other.0.clone()]),
        };
        V(Rc::new(result))
    }
}

impl Div<&V> for &V {
    type Output = V;

    fn div(self, other: &V) -> V {
        let result = Value {
            number: self.0.number / other.0.number,
            gradient: RefCell::new(OrderedFloat(0.0)),
            operation: Some('/'),
            children: RefCell::new(vec![self.0.clone(), other.0.clone()]),
        };
        V(Rc::new(result))
    }
}

impl Mul<&V> for &V {
    type Output = V;

    fn mul(self, other: &V) -> V {
        let result = Value {
            number: self.0.number * other.0.number,
            gradient: RefCell::new(OrderedFloat(0.0)),
            operation: Some('*'),
            children: RefCell::new(vec![self.0.clone(), other.0.clone()]),
        };
        V(Rc::new(result))
    }
}

impl Add for V {
    type Output = V;
    fn add(self, other: V) -> V {
        &self + &other
    }
}

impl Sub for V {
    type Output = V;
    fn sub(self, other: V) -> V {
        &self + &other
    }
}

impl Mul for V {
    type Output = V;
    fn mul(self, other: V) -> V {
        &self * &other
    }
}

impl Div for V {
    type Output = V;
    fn div(self, other: V) -> V {
        &self / &other
    }
}

impl V {
    pub fn new(n: f32) -> V {
        V(Rc::new(Value::new(n)))
    }

    pub fn backpropagate(&self) {
        self.0.backpropagate();
    }

    pub fn grad(&self) -> f32 {
        self.0.gradient.borrow().0
    }
}
