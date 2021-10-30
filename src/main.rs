use the_mac::log;

fn main() {
    #[log]
    fn simple(arg: &str) -> String {
        arg.to_string()
    }

    simple("unused");

    #[log]
    fn logs_own_stuff() {
        println!("Here is stuff from the function");
    }

    logs_own_stuff();
}
