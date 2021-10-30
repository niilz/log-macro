use the_mac::log;

fn main() {
    #[log]
    fn boob(arg: &str) -> String {
        arg.to_string()
    }
}
