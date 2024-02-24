use json::{parse, JsonValue};
// use std::error::Error as StdError;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Welcome to Json Reader");
    match read_json("data.json") {
        Ok(data) => {
            match &data["todos"] {
                JsonValue::Array(todos) => {
                    for todo in todos {
                        if let JsonValue::Object(todo_obj) = todo {
                            println!("{}", json::stringify_pretty(todo_obj.clone(), 2));
                        }
                    }
                }
                _ => eprintln!("'todos' is not an array"),
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_json(file: &str) -> Result<JsonValue, io::Error> {
    let mut file = File::open(file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let data = parse(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    // println!("{}", data.pretty(2)); // prints out the value as JSON string.
    Ok(data)
}
