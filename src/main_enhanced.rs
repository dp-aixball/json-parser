use clap::{Parser, ValueEnum};
use colored::*;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Enhanced JSON Parser with filtering and formatting options
#[derive(Parser)]
#[command(name = "json-parser")]
#[command(version = "0.2.0")]
#[command(about = "Enhanced JSON parser with filtering and formatting options")]
#[command(long_about = "A powerful JSON parser that can filter keys, format output, and extract specific values from JSON files.")]
struct Cli {
    /// JSON file to parse (default: data.json)
    #[arg(default_value = "data.json")]
    file: PathBuf,
    
    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Pretty)]
    format: OutputFormat,
    
    /// Filter keys by prefix (e.g., "address.")
    #[arg(long)]
    filter: Option<String>,
    
    /// Show only leaf values (no nested objects/arrays)
    #[arg(short = 'l', long)]
    leaves_only: bool,
    
    /// Show only keys (no values)
    #[arg(short = 'k', long)]
    keys_only: bool,
    
    /// Show only values (no keys)
    #[arg(short = 'v', long)]
    values_only: bool,
    
    /// Maximum depth for nested structures (0 = unlimited)
    #[arg(short = 'd', long, default_value_t = 0)]
    max_depth: usize,
    
    /// Extract specific key path (e.g., "address.city")
    #[arg(short = 'e', long)]
    extract: Option<String>,
    
    /// Show statistics about the JSON structure
    #[arg(short = 's', long)]
    stats: bool,
    
    /// Output in JSON format (for extracted values)
    #[arg(short = 'j', long)]
    json_output: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Pretty colored output
    Pretty,
    /// Simple plain text
    Simple,
    /// CSV format
    Csv,
    /// Markdown table format
    Markdown,
}

fn print_json_key_values(
    value: &Value, 
    prefix: &str, 
    depth: usize,
    config: &PrintConfig,
    stats: &mut JsonStats
) {
    if config.max_depth > 0 && depth > config.max_depth {
        return;
    }
    
    match value {
        Value::Object(map) => {
            stats.objects += 1;
            for (key, val) in map {
                stats.total_keys += 1;  // Count this key
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                
                // Apply filter if specified
                if let Some(filter) = &config.filter {
                    if !new_prefix.starts_with(filter) {
                        continue;
                    }
                }
                
                match val {
                    Value::Object(_) | Value::Array(_) => {
                        if !config.leaves_only {
                            print_key_value(&new_prefix, &val, "(nested)", config);
                        }
                        print_json_key_values(&val, &new_prefix, depth + 1, config, stats);
                    }
                    _ => {
                        stats.leaves += 1;
                        print_key_value(&new_prefix, &val, "", config);
                    }
                }
            }
        }
        Value::Array(arr) => {
            stats.arrays += 1;
            for (index, val) in arr.iter().enumerate() {
                stats.total_keys += 1;  // Count array elements as keys
                let new_prefix = format!("{}[{}]", prefix, index);
                // Apply filter if specified
                if let Some(filter) = &config.filter {
                    if !new_prefix.starts_with(filter) {
                        continue;
                    }
                }
                
                match val {
                    Value::Object(_) | Value::Array(_) => {
                        if !config.leaves_only {
                            print_key_value(&new_prefix, &val, "(nested)", config);
                        }
                        print_json_key_values(&val, &new_prefix, depth + 1, config, stats);
                    }
                    _ => {
                        stats.leaves += 1;
                        print_key_value(&new_prefix, &val, "", config);
                    }
                }
            }
        }
        _ => {
            stats.leaves += 1;
            if !config.leaves_only {
                print_key_value(prefix, value, "", config);
            }
        }
    }
}

struct PrintConfig {
    format: OutputFormat,
    filter: Option<String>,
    leaves_only: bool,
    keys_only: bool,
    values_only: bool,
    max_depth: usize,
}

struct JsonStats {
    objects: usize,
    arrays: usize,
    leaves: usize,
    total_keys: usize,
}

impl JsonStats {
    fn new() -> Self {
        Self {
            objects: 0,
            arrays: 0,
            leaves: 0,
            total_keys: 0,
        }
    }
    
    fn print(&self) {
        println!("\n{}", "=== JSON Statistics ===".bold().cyan());
        println!("Total objects: {}", self.objects.to_string().yellow());
        println!("Total arrays: {}", self.arrays.to_string().yellow());
        println!("Total leaf values: {}", self.leaves.to_string().yellow());
        println!("Total keys (including nested): {}", self.total_keys.to_string().yellow());
    }
}

fn print_key_value(key: &str, value: &Value, suffix: &str, config: &PrintConfig) {
    match config.format {
        OutputFormat::Pretty => {
            if config.keys_only {
                println!("{}", key.cyan());
            } else if config.values_only {
                println!("{}", format_value(value).green());
            } else {
                let value_str = if suffix.is_empty() {
                    format_value(value)
                } else {
                    suffix.to_string()
                };
                println!("{}: {}", key.cyan(), value_str.green());
            }
        }
        OutputFormat::Simple => {
            if config.keys_only {
                println!("{}", key);
            } else if config.values_only {
                println!("{}", value);
            } else {
                let value_str = if suffix.is_empty() {
                    format!("{}", value)
                } else {
                    suffix.to_string()
                };
                println!("{}: {}", key, value_str);
            }
        }
        OutputFormat::Csv => {
            if !config.keys_only && !config.values_only {
                println!("\"{}\",\"{}\"", key, value);
            }
        }
        OutputFormat::Markdown => {
            if !config.keys_only && !config.values_only {
                println!("| {} | {} |", key, value);
            }
        }
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        _ => value.to_string(),
    }
}

fn extract_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    let parts: Vec<&str> = path.split('.').collect();
    
    for part in parts {
        if part.ends_with(']') {
            // Handle array access like "hobbies[0]"
            let bracket_pos = part.find('[')?;
            let key = &part[..bracket_pos];
            let index_str = &part[bracket_pos + 1..part.len() - 1];
            let index: usize = index_str.parse().ok()?;
            
            current = current.get(key)?;
            current = current.get(index)?;
        } else {
            current = current.get(part)?;
        }
    }
    
    Some(current)
}

fn main() {
    let cli = Cli::parse();
    
    println!("{}", "=== Enhanced JSON Parser ===".bold().blue());
    println!("Parsing file: {}", cli.file.display().to_string().yellow());
    
    match fs::read_to_string(&cli.file) {
        Ok(content) => {
            match serde_json::from_str::<Value>(&content) {
                Ok(json_value) => {
                    let config = PrintConfig {
                        format: cli.format,
                        filter: cli.filter.clone(),
                        leaves_only: cli.leaves_only,
                        keys_only: cli.keys_only,
                        values_only: cli.values_only,
                        max_depth: cli.max_depth,
                    };
                    
                    let mut stats = JsonStats::new();
                    
                    // Handle extraction if requested
                    if let Some(path) = &cli.extract {
                        match extract_value(&json_value, path) {
                            Some(value) => {
                                if cli.json_output {
                                    println!("{}", serde_json::to_string_pretty(value).unwrap());
                                } else {
                                    println!("{}: {}", path.cyan(), format_value(value).green());
                                }
                                return;
                            }
                            None => {
                                eprintln!("{}: Key path '{}' not found", "Error".red(), path);
                                std::process::exit(1);
                            }
                        }
                    }
                    
                    // Print header based on format
                    match config.format {
                        OutputFormat::Csv => println!("Key,Value"),
                        OutputFormat::Markdown => {
                            println!("| Key | Value |");
                            println!("|-----|-------|");
                        }
                        _ => {
                            println!("\n{}", "=== Key-Value Pairs ===".bold().cyan());
                        }
                    }
                    
                    // Print all key-values
                    print_json_key_values(&json_value, "", 1, &config, &mut stats);
                    
                    // Print statistics if requested
                    if cli.stats {
                        stats.print();
                    }
                    
                    println!("\n{}", "=== End ===".bold().cyan());
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error parsing JSON".red(), e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error reading file".red(), e);
            std::process::exit(1);
        }
    }
}