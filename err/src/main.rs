use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    // let greeting_file_result = File::open("hello.txt").unwrap();
    // let greeting_file_result = File::open("hello.txt").expect("");


}

fn readfile()->Result<String,io::Error>{
    let file_result = File::open("./d");
    let mut content_ptr = match file_result {
        Ok(file)=>file,
        Err(e)=>return Err(e)
    };
    let mut content = String::new();
    match content_ptr.read_to_string(&mut content) {
        Ok(_)=>Ok(content),
        Err(e)=>Err(e)
    }
}

fn readfile_spread()->Result<String,io::Error>{
    //OR: fs::read_to_string("hello.txt")
    let mut userName = String::new();
    File::open("b")?.read_to_string(&mut userName)?;
    Ok(userName)
}