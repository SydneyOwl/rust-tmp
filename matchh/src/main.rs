fn main() {
    println!("Hello, world!");
    let six = plus(Some(5));

}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(String)
}
fn value_cents(coin:Coin) ->usize{
    match &coin {
        Coin::Penny=>1,
        Coin::Dime=>2,
        Coin::Quarter(owl)=>{
            println!("{}",owl);
            3
        }
        _ => 1
    }
    // 用 iflet
    let mut k = 1;
    if let Coin::Quarter(k)=coin{
        println!("{}",k);
        k.trim().parse().expect("")
    }else{
        1
    }
}
fn plus(x: Option<i32>)->Option<i32>{
    match x {
        Some(i)=>Some(i+1),
        None=>None
    }
}