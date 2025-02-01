use std::io::Write;

pub struct ConsoleView;

impl ConsoleView {
    pub fn read_line() -> String {
        let mut input = String::new();
        
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim_end_matches(['\n', '\r']).to_string();

        return input;
    }

    pub fn write(message: &str) {
        print!("{}", message);
    }
    
    pub fn write_line(message: &str) {
        println!("{}", message);
    }
}
