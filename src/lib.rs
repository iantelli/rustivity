use std::error::Error;

pub struct Config {
    pub origin: String,
    pub target: String,
    pub sensitivity: String,
}

pub enum Yaw {
    Overwatch,
    Valorant,
    Apex,
    CSGO,
}

fn yaw_values(yaw: Yaw) -> f64 {
    match yaw {
        Yaw::Overwatch => 0.0066,
        Yaw::Valorant => 0.07,
        Yaw::Apex => 0.022,
        Yaw::CSGO => 0.022,
    }
}

fn game_to_yaw(game: &str) -> f64 {
    match game {
        "OW" => yaw_values(Yaw::Overwatch),
        "VAL" => yaw_values(Yaw::Valorant),
        "APEX" => yaw_values(Yaw::Apex),
        "CSGO" => yaw_values(Yaw::CSGO),
        _ => 0.0,
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            panic!("not enough arguments");
        }
        let origin = args[1].clone();
        let target = args[2].clone();
        let sensitivity = args[3].clone();

        Ok(Config {
            origin,
            target,
            sensitivity,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let converted = convert_sens(&config.origin, &config.target, &config.sensitivity);
    println!(
        "From: {} to {}. Converted Sens: {}",
        config.origin, config.target, converted
    );
    Ok(())
}

pub fn convert_sens<'a>(origin: &str, target: &str, sensitivity: &str) -> String {
    let converted = (sensitivity.parse::<f64>().unwrap()
        * (game_to_yaw(origin) / game_to_yaw(target)))
    .to_string();
    converted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let origin = "OW";
        let target = "VAL";
        let sensitivity = "6";

        assert_eq!(
            "0.5657142857142856",
            convert_sens(origin, target, sensitivity)
        )
    }
}
