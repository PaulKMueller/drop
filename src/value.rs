use std::ops::Add;

#[derive(Debug)]
pub struct Value{
    pub number: i32,
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        Value {
            number: self.number + other.number,
        }
    }
}
