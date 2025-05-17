// Mods
mod parser;
mod utils;
mod modules;

fn main() {
    println!("{}", utils::banner());
    let cmd = parser::initialize();
    modules::check(&cmd.req, &cmd.code);
}