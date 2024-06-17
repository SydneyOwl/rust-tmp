use std::collections::HashMap;
use std::fmt::format;

fn main() {
    let arr = vec![1,2,3];
    let mut arr = Vec::new();
    arr.push(0);
    //panic!
    // let ele = &arr[10];
    let m = arr.get(0).expect("Error!");
    // OR..
    let m = match arr.get(10) {
        Some(i)=>i,
        None=>&0
    };
    if let Some(i) = arr.get(10){
        //
    }else{
        //
    }
    for i in &mut vec![1,2,3]{
        println!("{}",i);
        *i+=1
    }

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    let a = format!("{s1}++{s2}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Owl"),10);
    scores.insert(String::from("Owl1"),20);
    // 不存在则插入
    scores.entry("Yellow".to_string()).or_insert(23);

    let score = scores.get("Owl").copied().unwrap_or(0);
    for (x,y) in &mut scores {
        print!("{}-=-{}",y,x);
        *y+=1;
    }

}
