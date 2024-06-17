fn main() {
    println!("Hello, world!");
}

// Function
fn largest<T: PartialOrd>(list: &[T])->&T{
    let mut largest = &list[0];
    for x in list {
        if x>largest{
            largest = x;
        }
    }
    largest
}

// Struct
struct Point<T>{
    x:T,
    y:T
}
impl <T> Point<T>{
    fn x(&self)->&T{
        &self.x
    }
}