use std::error::Error;

pub struct Config {
    pub origin: String,
    pub target: String,
    pub dpi: String,
    pub sensitivity: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            panic!("not enough arguments");
        }
        let origin = args[1].clone();
        let target = args[2].clone();
        let dpi = args[3].clone();
        let sensitivity = args[4].clone();

        Ok(Config {
            origin,
            target,
            dpi,
            sensitivity,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dpi = config.dpi;
    println!("DPI:\n{dpi}");
    Ok(())
}
