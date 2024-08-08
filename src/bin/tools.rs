use std::env;
use std::fs::File;
use std::io::{BufWriter, Write, Result};
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Print the arguments for debugging
    println!("Arguments: {:?}", args);

    // Check if exactly one argument (output_dir) is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <output_dir>", args[0]);
        process::exit(64);
    }
    
    let output_dir = &args[1];

    if let Err(e) = define_ast(output_dir, "Expr".to_string(), vec![
        "Binary   : Expr left, Token operator, Expr right".to_string(),
        "Grouping : Expr expression".to_string(),
        "Literal  : Object value".to_string(),
        "Unary    : Token operator, Expr right".to_string(),
    ]) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn define_ast(output_dir: &str, base_name: String, types: Vec<String>) -> Result<()> {
    let path = PathBuf::from(output_dir).join(format!("{}.rs", base_name));
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    
    writeln!(writer, "// This file is generated automatically")?;
    writeln!(writer, "")?;
    writeln!(writer, "pub trait {} {{", base_name)?;
    writeln!(writer, "}}")?;
    writeln!(writer, "")?;

    // The AST classes.
    for type_str in types {
        let parts: Vec<&str> = type_str.split(':').map(str::trim).collect();
        if parts.len() != 2 {
            eprintln!("Invalid type format: {}", type_str);
            continue;
        }
        let class_name = parts[0];
        let fields = parts[1];
        if let Err(e) = define_type(&mut writer, &base_name, class_name, fields) {
            eprintln!("Error defining type {}: {}", class_name, e);
            continue;
        }
    }

    Ok(())
}

fn define_type(
    writer: &mut BufWriter<File>,
    _base_name: &str, // Renamed parameter to _base_name to silence the warning
    class_name: &str,
    field_list: &str
) -> Result<()> {
    // Write the struct definition
    writeln!(writer, "pub struct {} {{", class_name)?;

    // Write the fields
    let fields: Vec<&str> = field_list.split(',').map(str::trim).collect();
    for field in &fields {
        let parts: Vec<&str> = field.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Invalid field format: {}", field);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid field format"));
        }
        let field_type = parts[0];
        let field_name = parts[1];
        writeln!(writer, "    pub {}: {},", field_name, field_type)?;
    }

    writeln!(writer, "}}")?;

    // Write the implementation block for the struct
    writeln!(writer, "")?;
    writeln!(writer, "impl {} {{", class_name)?;

    // Write the constructor (equivalent) function
    writeln!(writer, "    pub fn new({}) -> Self {{", field_list)?;

    // Initialize the struct fields
    for field in &fields {
        let parts: Vec<&str> = field.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Invalid field format: {}", field);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid field format"));
        }
        let field_name = parts[1];
        writeln!(writer, "        {}: {},", field_name, field_name)?;
    }

    writeln!(writer, "    }}")?;
    writeln!(writer, "}}")?;

    Ok(())
}
