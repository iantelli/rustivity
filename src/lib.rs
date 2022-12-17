use std::error::Error;

pub struct Config {
    pub origin: String,
    pub target: String,
    pub sensitivity: String,
    pub dpi: String,
    pub dpi2: String,
}

pub fn err_message(message: &str) -> &str {
    panic!(
        "\n {}\n {}\n {} \n",
        "-".repeat(50),
        message,
        "-".repeat(50)
    )
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            err_message(
                "Please provide the following arguments: \n <Origin> <Target> <Sensitivity> <DPI> \n",
            );
        }
        let origin = args[1].clone();
        let target = args[2].clone();
        let sensitivity = args[3].clone();
        let dpi = args[4].clone();

        Ok(Config {
            origin,
            target,
            sensitivity,
            dpi,
            dpi2: if args.len() > 5 {
                args[5].clone()
            } else {
                args[4].clone()
            },
        })
    }
}

enum Yaw {
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

fn normalize_game_name(game: String) -> String {
    match game.to_uppercase().as_str() {
        "OW" | "OVERWATCH" => "Overwatch".to_string(),
        "VAL" | "VALORANT" => "Valorant".to_string(),
        "APEX" | "APEX LEGENDS" => "Apex Legends".to_string(),
        "CS" | "CSGO" => "CS:GO".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn convert_sens<'a>(
    origin: &str,
    target: &str,
    sensitivity: &str,
    dpi: &str,
    dpi2: &str,
) -> String {
    // Forumla: converted = (sensitivity * (origin game yaw / target game yaw)) * (target dpi/origin dpi)
    let sensitivity = sensitivity.parse::<f64>().unwrap();
    let dpi = dpi.parse::<f64>().unwrap();
    let dpi2 = dpi2.parse::<f64>().unwrap();
    let converted =
        ((sensitivity * (game_to_yaw(origin) / game_to_yaw(target))) * (dpi2 / dpi)).to_string();
    converted
}

fn calculate_cm360<'a>(origin: &str, dpi: &str, sensitivity: &str) -> String {
    // Formula: (360 / (yaw * dpi * sensitivity)) * 2.54
    let cm360 = ((360.00
        / (game_to_yaw(origin)
            * dpi.parse::<f64>().unwrap()
            * sensitivity.parse::<f64>().unwrap()))
        * 2.54)
        .to_string();
    cm360
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let converted = convert_sens(
        &config.origin,
        &config.target,
        &config.sensitivity,
        &config.dpi,
        &config.dpi2,
    );
    let cm360 = calculate_cm360(
        config.origin.as_str(),
        config.dpi.as_str(),
        config.sensitivity.as_str(),
    );
    println!(
        "{} \n Origin game: {} \n Target game: {} \n Original sensitivity: {} \n Converted sensitivity: {:.5} \n CM/360: {:.5} \n{}",
        "-".repeat(50),
        normalize_game_name(config.origin),
        normalize_game_name(config.target),
        config.sensitivity,
        converted,
        cm360,
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
        let dpi = "800";
        let dpi2 = "800";

        assert_eq!(
            "0.5657142857142856",
            convert_sens(origin, target, sensitivity, dpi, dpi2)
        )
    }

    #[test]
    fn case_insensitive() {
        let origin = "ow";
        let target = "val";
        let sensitivity = "6";
        let dpi = "800";
        let dpi2 = "800";

        assert_eq!(
            "0.5657142857142856",
            convert_sens(origin, target, sensitivity, dpi, dpi2)
        )
    }

    #[test]
    fn aliases() {
        let origin = "ow";
        let target = "Valorant";
        let sensitivity = "6";
        let dpi = "800";
        let dpi2 = "800";

        assert_eq!(
            "0.5657142857142856",
            convert_sens(origin, target, sensitivity, dpi, dpi2)
        )
    }

    #[test]
    fn dpi_conv() {
        let origin = "ow";
        let target = "val";
        let sensitivity = "6";
        let dpi = "800";
        let dpi2 = "1600";

        assert_eq!(
            "1.1314285714285712", // converted * (dpi2 / dpi)
            convert_sens(origin, target, sensitivity, dpi, dpi2)
        )
    }

    #[test]
    fn cm360() {
        let origin = "Valorant";
        let sensitivity = "0.5657142857142856";
        let dpi = "800";

        assert_eq!(
            "28.863636363636363",
            calculate_cm360(origin, dpi, sensitivity)
        )
    }
}
