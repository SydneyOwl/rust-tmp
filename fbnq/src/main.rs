use std::io;

fn main() {
    loop {
        println!("输入一个华氏温度：");
        let mut ftemp = String::new();
        io::stdin().read_line(&mut ftemp).expect("读取失败！");
        if ftemp.trim() == "exit"{
            return;
        }
        let ftemp:f32 = match ftemp.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("输入错误!");
                continue;
            }
        };
        let ctemp:f32 = (ftemp - 32.0)*1.8;
        println!("{}",ctemp)
    }

}
