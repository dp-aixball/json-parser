use serde_json::Value;
use std::fs;

fn print_json_key_values(value: &Value, prefix: &str) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                
                match val {
                    Value::Object(_) | Value::Array(_) => {
                        println!("{}: (nested object/array)", new_prefix);
                        print_json_key_values(val, &new_prefix);
                    }
                    _ => {
                        println!("{}: {}", new_prefix, val);
                    }
                }
            }
        }
        Value::Array(arr) => {
            for (index, val) in arr.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, index);
                match val {
                    Value::Object(_) | Value::Array(_) => {
                        println!("{}: (nested object/array)", new_prefix);
                        print_json_key_values(val, &new_prefix);
                    }
                    _ => {
                        println!("{}: {}", new_prefix, val);
                    }
                }
            }
        }
        _ => {
            println!("{}: {}", prefix, value);
        }
    }
}

fn main() {
    // 读取JSON文件
    let file_path = "data.json";
    
    match fs::read_to_string(file_path) {
        Ok(content) => {
            // 解析JSON
            match serde_json::from_str::<Value>(&content) {
                Ok(json_value) => {
                    println!("Successfully parsed JSON from '{}'", file_path);
                    println!("==========================================");
                    println!("All key-value pairs:");
                    println!("==========================================");
                    
                    // 打印所有键值对
                    print_json_key_values(&json_value, "");
                    
                    println!("==========================================");
                    println!("Total keys printed.");
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", file_path, e);
        }
    }
}