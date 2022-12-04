pub mod Generics; 

fn main() {
    let value = Generics::Struct::f('b', 1, 2);
    println!("{}", value);
}
