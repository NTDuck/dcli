use std::io::Write;

#[allow(dead_code)]
pub struct IoGateway;

impl IoGateway {
    #[allow(dead_code)]
    pub fn read_line(&self) -> String {
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input
            .trim_end_matches(&['\n', '\r'][..])
            .to_string();

        return input;
    }

    #[allow(dead_code)]
    pub fn write(&self, message: &str) {
        std::io::stdout()
            .write(message.as_bytes())
            .unwrap();
    }
    
    #[allow(dead_code)]
    pub fn write_line(&self, message: &str) {
        self.write(message);
        self.write("\n");
    }
}
