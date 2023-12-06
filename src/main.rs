mod cli;
mod core;
mod license_renderers;
mod util;

fn main() {
    if let Err(error) = core::run() {
        println!("{}", error);
        std::process::exit(1);
    }
}
