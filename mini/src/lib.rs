use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        case_insensitive(&config.query, &contents)
    } else {
        case_sensitive(&config.query, &contents)
    };

    println!("{:?}", results);
    Ok(())
}

pub struct Config{
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config{
    pub fn build(args: &[String])->Result<Config,&'static str>{
        if args.len()<3{
            return Err("Args insufficient");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config{
            query,
            file_path,
            ignore_case
        })
    }
}
pub fn case_sensitive<'a>(query: &str, contents: &'a str) ->Vec<&'a str>{
    let mut result = Vec::new();
    for x in contents.lines() {
        if x.contains(query){
            result.push(x);
        }
    }
    result
}

pub fn case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for x in contents.lines(){
        if x.to_lowercase().contains(&query.to_lowercase()[..]){
            result.push(x)
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query = "!";
        let contents = "\
Rust:
Very hard!
        ";
        assert_eq!(vec!["Very hard!"], case_sensitive(query, contents));
        assert_eq!(vec!["Very hard!"],case_insensitive("very", contents));
    }
}