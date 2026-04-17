use serde_json::Value;
use std::env;
use std::fs;

fn print_all_key_values(value: &Value, path: &str) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };
                
                match val {
                    Value::Object(_) | Value::Array(_) => {
                        print_all_key_values(val, &new_path);
                    }
                    _ => {
                        println!("{} = {}", new_path, val);
                    }
                }
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, i);
                match val {
                    Value::Object(_) | Value::Array(_) => {
                        print_all_key_values(val, &new_path);
                    }
                    _ => {
                        println!("{} = {}", new_path, val);
                    }
                }
            }
        }
        _ => {
            println!("{} = {}", path, value);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "data.json"
    };
    
    println!("Parsing JSON file: {}", file_path);
    
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match serde_json::from_str::<Value>(&content) {
                Ok(json_value) => {
                    println!("\n=== All Key-Value Pairs ===");
                    print_all_key_values(&json_value, "");
                    println!("=== End ===");
                }
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
        }
    }
}