use raur::AurResponse;
use raur::Config;
use std::env;
use std::error::Error;
use std::process;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let resp = AurResponse();
    println!("{:#?}", resp);
    Ok(())
}
