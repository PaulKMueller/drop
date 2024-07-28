mod value;


use value::Value;

fn main() {
   let a = Value{number: 10};
   let b = Value{number: 20};

   println!("{:?}", a - b);
}
