use std::io::Write;

pub struct IoGateway;

impl IoGateway {
    pub fn readLine(&self) -> String {
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input
            .trim_end_matches(&['\n', '\r'][..])
            .to_string();

        return input;
    }

    pub fn write(&self, message: &str) {
        std::io::stdout()
            .write(message.as_bytes())
            .unwrap();
    }
    
    pub fn writeLine(&self, message: &str) {
        self.write(message);
        self.write("\n");
    }
}
