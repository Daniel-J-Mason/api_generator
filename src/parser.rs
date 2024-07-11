use std::error::Error;
use std::fs;
use crate::domain::{JavaDomain, JavaObject};

pub struct Config {
    pub file_path: String,
    pub function: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let file_path = args[1].clone();
        let optional_function = if args.len() == 3 {
            Some(args[2].clone())
        } else {
            None
        };

        Ok(Config {
            file_path,
            function: optional_function,
        })
    }
}

pub fn run(config: Config) -> Result<JavaDomain, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let domain = parse_content(contents)?;

    Ok(domain)
}

fn parse_content(content: String) -> Result<JavaDomain, Box<dyn Error>> {

    let mut name = String::new();
    let mut objects: Vec<JavaObject> = Vec::new();

    let mut parentheses = false;

    for line in content.lines() {
        if line.contains("record") {
            name.push_str(&parse_name(line).ok_or("Could not parse name")?)
        }

        if line.contains(")") {
            break;
        }

        if parentheses {
            let object = parse_object(line)?;
            objects.push(object);
        }


        if line.contains("(") {
            parentheses = true;
        }

    }

    let domain = JavaDomain {
        name,
        objects,
    };

    Ok(domain)
}

fn parse_name(line: &str) -> Option<String> {
    let mut words = line.split_whitespace();

    while let Some(word) = words.next(){
        if word == "record" {
            if let Some(next_word) = words.next() {
                return Some(String::from(
                    next_word.trim_matches('(')
                ));
            }
        }
    }

    None
}

fn parse_object(line: &str) -> Result<JavaObject, &str> {
    let mut words = line.split_whitespace();

    let class = words.next().ok_or("Expected class name")?;
    let variable_name = words.next().ok_or("Expected variable name")?;
    let variable_name = variable_name.trim_matches(',');

    Ok(JavaObject{
        class: String::from(class),
        variable_name: String::from(variable_name),
    })
}