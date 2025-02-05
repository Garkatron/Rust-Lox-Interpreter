#[derive(Debug)]
pub enum Color {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn to_ansi_code(&self) -> &str {
        match self {
            Color::Reset => "\x1b[0m",     // Reset
            Color::Red => "\x1b[31m",      // Rojo
            Color::Green => "\x1b[32m",    // Verde
            Color::Yellow => "\x1b[33m",   // Amarillo
            Color::Blue => "\x1b[34m",     // Azul
            Color::Magenta => "\x1b[35m",  // Magenta
            Color::Cyan => "\x1b[36m",     // Cian
            Color::White => "\x1b[37m",    // Blanco
        }
    }
    
    pub fn cprintln(text: &str, color: Color) {
        println!("{}{}{}",color.to_ansi_code(), text, Color::Reset.to_ansi_code());
    }
    pub fn cprint(text: &str, color: Color) {
        print!("{}{}{}",color.to_ansi_code(), text, Color::Reset.to_ansi_code());
    }
    pub fn ecprintln(text: &str, color: Color) {
        eprintln!("{}{}{}",color.to_ansi_code(), text, Color::Reset.to_ansi_code());
    }
}
