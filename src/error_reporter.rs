
pub struct ErrorReporter {
    had_error: bool
}
impl ErrorReporter {

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line," ".to_string(), message)
    }

    fn report(&mut self, line: usize, where_is: String, message: String){
        println!("Error {} at line {}\nMessage: {} ", where_is, line, message);
        self.had_error = true;
    }

    pub fn reset(&mut self) {
        self.had_error = false
    }
}
