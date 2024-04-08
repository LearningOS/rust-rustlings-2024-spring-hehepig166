// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.



use std::any::{type_name, TypeId};
 
fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() {
    let mut x : i64 = 10;
    print_type_of(&x);
    print_type_of(&10);
    print_type_of(&2147483648u32);
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
