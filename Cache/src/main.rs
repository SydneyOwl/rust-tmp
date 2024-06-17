use std::io;

fn main() {
    let array = [1,2,3,4,5];
    let mut userInput = String::new();
    io::stdin().read_line(&mut userInput).expect("Error!");
    let userInput:usize = userInput.trim().parse().expect("Error!");
    let ele = array[userInput];
    for eleb in array{
        println!("{}", eleb);
    }
    println!("Input is: {ele}");
    println!("{}", aTest(1));
}

fn weirdOwlll(){
    let mut counter = 0;
    let res = loop{
        counter+=1;
        if counter==10{
            break counter;
        }
    };
}

fn aTest(x: i32) -> usize{
    println!("Value Is: {x}");
    let a = {
        let k = 1;
        k+3
    };
    if x == 1{
        5
    }else{
        6
    }
}

fn liftoff(){
    for num in (1..2).rev(){
        println!("{}", num)
    }
}