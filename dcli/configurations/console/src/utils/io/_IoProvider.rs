pub trait IoProvider {
    fn write(&self, s: &str);
    fn read_line(&self) -> String;

    fn write_line(&self, s: &str) {
        self.write(s);
        println!();
    }
}
