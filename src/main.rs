mod aur;
mod func;
mod settings;
use settings::Config;
use std::env;
use std::error::Error;
use std::process;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let resp = config.run();
    println!("{:#?}", resp);
    Ok(())
}
