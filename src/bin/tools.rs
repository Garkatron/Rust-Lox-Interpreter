use std::env;
use std::fs::{File, create_dir_all};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = &args[1];

    // Asegúrate de que el directorio de salida existe
    create_dir_all(output_dir)?;

    define_ast(output_dir, "Stmt", vec![
        "Expression : Expr expression",
        "Print      : Expr expression"
    ])?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    let path = Path::new(output_dir).join(format!("{}.rs", base_name.to_lowercase()));
    let mut file = File::create(&path)?; // Verifica si hay error al crear el archivo

    writeln!(file, "pub mod {} {{", base_name.to_lowercase())?;
    writeln!(file, "    use super::Token;")?;
    writeln!(file, "    use std::any::Any;")?; // Importar Any para su uso
    writeln!(file, "    pub enum {} {{", base_name)?;

    for type_def in &types {
        let parts: Vec<&str> = type_def.split(':').collect();
        let class_name = parts[0].trim();
        let fields = parts[1].trim();
        define_type(&mut file, class_name, fields)?;
    }

    writeln!(file, "    }}")?; // Cierra el enum

    // Definición del Visitor
    define_visitor(&mut file, base_name, types)?;

    writeln!(file, "}}")?; // Cierra el módulo

    Ok(())
}

// base_name: &str
fn define_type(file: &mut File, class_name: &str, field_list: &str) -> io::Result<()> {
    writeln!(file, "        {} {{", class_name)?;
    
    let fields: Vec<&str> = field_list.split(',').collect();
    for field in &fields {
        let field_parts: Vec<&str> = field.trim().split_whitespace().collect();
        let field_type = field_parts[0];
        let field_name = field_parts[1];
        writeln!(file, "            {}: {},", field_name, map_type(field_type))?;        
    }

    writeln!(file, "        }},")?; // Cierra la variante de la estructura
    Ok(())
}

// Genera la definición del Visitor
fn define_visitor(file: &mut File, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    writeln!(file, "    pub trait Visitor<R> {{")?;

    for type_def in &types {
        let class_name = type_def.split(':').next().unwrap().trim();
        writeln!(file, "        fn visit_{}(&self, expr: &{}) -> R;", class_name.to_lowercase(), class_name)?;
    }

    writeln!(file, "    }}")?; // Cierra el trait

    // Método accept para el enum
    writeln!(file, "    impl {} {{", base_name)?;
    writeln!(file, "        pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {{")?;
    writeln!(file, "            match self {{")?;

    for type_def in &types {
        let class_name = type_def.split(':').next().unwrap().trim();
        writeln!(file, "                {}::{} {{..}} => visitor.visit_{}(self),", base_name, class_name, class_name.to_lowercase())?;
    }

    writeln!(file, "            }}")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?; // Cierra la implementación

    Ok(())
}

// Mapea tipos de Java a tipos de Rust
fn map_type(java_type: &str) -> &str {
    match java_type {
        "Expr" => "Box<Expr>",    // Tipo recursivo, normalmente Boxeado en Rust
        "Token" => "Token",
        "Any" => "Box<dyn Any>",  // Cambia Object por Box<dyn Any>
        _ => java_type,           // Usa el mismo nombre de tipo si no se necesita mapeo especial
    }
}
