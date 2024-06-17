use std::fmt::{Debug, Display};
use traitt::{Summary, Tweet};

fn main() {
    let tw = Tweet {
        username:"owl".to_string(),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("{}", tw.summarize());
    notify(&tw);
    notify1(&tw);
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify1<T:Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify3<T:Summary>(item1:&T,item2:&T){

}
// 如果item同时实现了多个特征？
pub fn notify4<T:Summary+Display>(item:&T){

}
pub fn notify5(&item: &(impl Display+Summary)){

}
//简化：
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U)  {}
fn some_function1<T,U>(t:&T,u:&U)
where
T:Display+Clone,
U:Clone+Debug{
    
}