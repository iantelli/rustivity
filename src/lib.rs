use std::error::Error;

pub struct Config {
    pub origin: String,
    pub target: String,
    pub sensitivity: String,
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
    match game.to_uppercase().as_str() {
        "OW" | "OVERWATCH" => yaw_values(Yaw::Overwatch),
        "VAL" | "VALORANT" => yaw_values(Yaw::Valorant),
        "APEX" | "APEX LEGENDS" => yaw_values(Yaw::Apex),
        "CS" | "CSGO" => yaw_values(Yaw::CSGO),
        _ => panic!(
            "Unknown game \n Supported games: [\nOverwatch | OW, \nValorant | VAL, \nApex Legends | APEX, \nCSGO | CS \n]"
        ),
    }
}

pub fn convert_sens<'a>(origin: &str, target: &str, sensitivity: &str) -> String {
    // Forumla: converted = sensitivity * (origin game yaw / target game yaw)
    let converted = (sensitivity.parse::<f64>().unwrap()
        * (game_to_yaw(origin) / game_to_yaw(target)))
    .to_string();
    converted
}

pub fn normalize_game_name(game: String) -> String {
    match game.to_uppercase().as_str() {
        "OW" | "OVERWATCH" => "Overwatch".to_string(),
        "VAL" | "VALORANT" => "Valorant".to_string(),
        "APEX" | "APEX LEGENDS" => "Apex Legends".to_string(),
        "CS" | "CSGO" => "CS:GO".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let converted = convert_sens(&config.origin, &config.target, &config.sensitivity);
    println!(
        "{} \n Origin game: {} \n Target game: {} \n Original sensitivity: {} \n Converted sensitivity: {} \n{}",
        "-".repeat(50),
        normalize_game_name(config.origin),
        normalize_game_name(config.target),
        config.sensitivity,
        converted,
        "-".repeat(50)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let origin = "OW";
        let target = "VAL";
        let sensitivity = "6";

        assert_eq!(
            "0.5657142857142856",
            convert_sens(origin, target, sensitivity)
        )
    }

    #[test]
    fn case_insensitive() {
        let origin = "ow";
        let target = "val";
        let sensitivity = "6";

        assert_eq!(
            "0.5657142857142856",
            convert_sens(origin, target, sensitivity)
        )
    }

    #[test]
    fn aliases() {
        let origin = "ow";
        let target = "Valorant";
        let sensitivity = "6";

        assert_eq!(
            "0.5657142857142856",
            convert_sens(origin, target, sensitivity)
        )
    }
}
