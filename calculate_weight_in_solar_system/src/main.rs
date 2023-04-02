use std::{io, str::FromStr};
// use strum_macros::EnumString;

// #[derive(EnumString, Debug)]
enum SolarSystemEntities {
    Sun,
    Mercury,
    Venus,
    Earth,
    Moon,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
}

// We could have used strum_macros EnumString which automatically converts strings to enum
impl FromStr for SolarSystemEntities {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sun" => Ok(SolarSystemEntities::Sun),
            "mercury" => Ok(SolarSystemEntities::Mercury),
            "venus" => Ok(SolarSystemEntities::Venus),
            "earth" => Ok(SolarSystemEntities::Earth),
            "moon" => Ok(SolarSystemEntities::Moon),
            "mars" => Ok(SolarSystemEntities::Mars),
            "jupiter" => Ok(SolarSystemEntities::Jupiter),
            "saturn" => Ok(SolarSystemEntities::Saturn),
            "uranus" => Ok(SolarSystemEntities::Uranus),
            "neptune" => Ok(SolarSystemEntities::Neptune),
            "pluto" => Ok(SolarSystemEntities::Pluto),
            _ => Err(format!("Unknown entity type, {}", s)),
        }
    }
}

fn main() {
    let mut input_weight = String::new();
    let mut input_entity = String::new();

    println!("Please enter your weight in Kgs: ");
    io::stdin()
        .read_line(&mut input_weight)
        .expect("Error: Unable to process the input");

    println!("Please enter your planet name: ");
    io::stdin()
        .read_line(&mut input_entity)
        .expect("Error: Unable to process the input");

    let parsed_weight: f32 = input_weight
        .trim()
        .parse()
        .expect("Error: Unable to convert input into float");

    input_entity = input_entity.trim().to_lowercase();
    let parsed_entity: SolarSystemEntities = input_entity
        .parse()
        .expect("Error: Unable to identify the planet name");

    println!(
        "Weight on {} is {}kg",
        input_entity,
        calculate_weight_by_type(parsed_weight, parsed_entity)
    );
}

fn calculate_weight_by_type(weight: f32, entity: SolarSystemEntities) -> f32 {
    match entity {
        SolarSystemEntities::Sun => weight * 27.01,
        SolarSystemEntities::Mercury => weight * 0.38,
        SolarSystemEntities::Venus => weight * 0.91,
        SolarSystemEntities::Earth => weight * 1.0,
        SolarSystemEntities::Moon => weight * 0.166,
        SolarSystemEntities::Mars => weight * 0.38,
        SolarSystemEntities::Jupiter => weight * 2.34,
        SolarSystemEntities::Saturn => weight * 1.06,
        SolarSystemEntities::Uranus => weight * 0.92,
        SolarSystemEntities::Neptune => weight * 1.19,
        
        SolarSystemEntities::Pluto => weight * 0.06,
    }
}
