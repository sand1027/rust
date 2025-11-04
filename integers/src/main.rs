use std::rc::Rc;
use std::sync::{Arc, Mutex};
fn integers(){
    let a = 10;  //integer on stack
    let b = Box::new(20); //Boxed integer :- Boxed integer is nothiong but the integer stored in heap
    let c = Rc::new(Box::new(30)); //wraps the boxed integer with reference counter
    let d = Arc::new(Mutex::new(40));
    println!("a: {:?}", a);
}
fn main() {
    integers();

}
