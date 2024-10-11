use chrono::NaiveDate;
use core::fmt;
use regex::Regex;

pub struct Pokemon {
    id: i32,
    generation: i32,
    name: String,
    description: String,
    type1: String,
    type2: String,
    abilities: Vec<String>,
    weight_kg: f32,
    height_m: f32,
    capture_rate: i32,
    is_legendary: bool,
    capture_date: NaiveDate,
}

impl Pokemon {
    pub fn new(
        id: i32,
        generation: i32,
        name: String,
        description: String,
        type1: String,
        type2: String,
        abilities: Vec<String>,
        weight_kg: f32,
        height_m: f32,
        capture_rate: i32,
        is_legendary: bool,
        capture_date: NaiveDate,
    ) -> Pokemon {
        Pokemon {
            id,
            generation,
            name,
            description,
            type1,
            type2,
            abilities,
            weight_kg,
            height_m,
            capture_rate,
            is_legendary,
            capture_date,
        }
    }
    pub fn from_str(linha: String) -> Pokemon {
        let regex = Regex::new(r#"\['(.*?)'\]"#).unwrap();
        //vai que nao tem habilidade preciso tratar
        let mut habilidades = regex.captures(&linha).unwrap()[0].to_string();
        // tirando as habilidades da linha
        let linha = regex.replace(&linha, "");

        // separando os atributos e colocando em um vetor
        let atributos: Vec<&str> = linha.split(",").collect();
        //ajeitando a string das habilidades
        habilidades = habilidades
            .replace("[", "")
            .replace("]", "")
            .replace("'", "");

        //separando as habilidades em um v
        let habilidades: Vec<String> = habilidades.split(",").map(|str| str.to_string()).collect();

        let mut legend = false;
        if atributos[10] == "1" {
            legend = true;
        }

        let pokemon = Pokemon::new(
            atributos[0].parse().unwrap(),
            atributos[1].parse().unwrap(),
            atributos[2].to_string(),
            atributos[3].to_string(),
            atributos[4].to_string(),
            atributos[5].to_string(),
            habilidades,
            atributos[7].parse().unwrap_or(0.0),
            atributos[8].parse().unwrap_or(0.0),
            atributos[9].trim().parse().unwrap(),
            legend,
            NaiveDate::parse_from_str(&atributos[11], "%d/%m/%Y").unwrap(),
        );
        return pokemon;
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pokemon: {}\nID: {}\nGeneration: {}\nDescription: {}\nType1: {}\nType2: {}\nAbilities: {:?}\nWeight: {}\nHeight: {}\nCapture Rate: {}\nIs Legendary: {}\nCapture Date: {}\n",
            self.name, self.id, self.generation, self.description, self.type1, self.type2, self.abilities, self.weight_kg, self.height_m, self.capture_rate, self.is_legendary, self.capture_date
        )
    }
}
