fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4("127.0.0.1".to_string());
    let work = IpAddrKind::V6("::1".to_string());
    Message::Write(String::new()).call();
    Message::test();

    let a = Some(1);
    let b:Option<i32> = None;
    let c :i8 = 6;
    // 不行
    // let z = a+c;
}
enum IpAddrKind{
    V4(String),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
    fn test(){

    }
}