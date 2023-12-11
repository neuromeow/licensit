mod cli;
mod core;
mod util;

fn main() {
    if let Err(error) = core::run() {
        println!("{}", error);
        std::process::exit(1);
    }
}
