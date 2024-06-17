fn main() {

    // i32实现了copy trait, 在堆上，所以不会丢失所有权
    let x  =5;
    let b = x;

    // a的所有权转移到b,a不可用！
    let a = String::from("333");
    let k = a;

    //使用clone克隆堆上的数据
    let mut l = k.clone();

    modify(&mut l);

    // 这样不行，引用只能有一个
    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s; // ri失效了
    // 但是可以有多个不可变引用
    //  let r1 = &s; // 没问题
    //  let r2 = &s; // 没问题
    //  println!("{} and {}", r1, r2);
    //
    // println!("{}, {}", r1, r2);
    let mut s = String::from("hello");

    // 反正引用ptr一瞬和
    &&&&&&&s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`

    // 不行！
    // let mut test = String::from("owl");
    // let p = &test[0..1];
    // test.clear();
    // println!("{}",p)
}

//这样不行
// fn modify(something: &String){
//     something.push("444");
// }

// fn dangerous() -> &String {
//     let a = String::from("dd");
//     这样不行，因为a被释放了，指针还在
//     &a
// }

// 引用的规则
// 让我们概括一下之前对引用的讨论：
//
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。
fn modify(something: &mut String){
    something.push_str("ss");
}