use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Deserializer};
pub mod error;
use error::TezaursApiError;


use std::str::FromStr;
use serde_json::Value;

const API: &str = "http://api.tezaurs.lv:8182";

fn deserialize_from_string<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    usize::from_str(&s).map_err(serde::de::Error::custom)
}


fn deserialize_from_string_option<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let result = usize::from_str(&s).ok();
    Ok(result)
}


pub struct TezaursApi {
    client: Client,
}

impl TezaursApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn analyze(&self, word: String) -> Result<Vec<AnalyzedWord>, TezaursApiError> {
        let url = format!("{}/analyze/{}", API, word);
        let response = self.client.get(url).send().await?;
        let response_text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&response_text)?;
        let data: Vec<AnalyzedWord> = serde_json::from_value(json.clone())?;

        Ok(data)
    }


    pub async fn tokenize(&self, sentence: String) -> Result<Vec<Token>, TezaursApiError> {
        let url = format!("{}/tokenize/{}", API, sentence);
        let response = self.client.get(url).send().await?;
        let response_text = response.text().await?;
        let json: Value = serde_json::from_str(&response_text)?;
        let data: Vec<Token> = serde_json::from_value(json.clone())?;
        Ok(data)
    }

    pub async fn normalize_phrase(&self, sentence: String) -> Result<String, TezaursApiError> {
        let url = format!("{}/normalize_phrase/{}", API, sentence);
        let response = self.client.get(url).send().await?;
        let response_text = response.text().await?;
        Ok(response_text)
    }

    pub async fn suitable_paradigm(&self, word: String) -> Result<Vec<Paradigm>, TezaursApiError> {
        let url = format!("{}/suitable_paradigm/{}", API, word);
        let response = self.client.get(url).send().await?;
        let response_text = response.text().await?;
        let json: Value = serde_json::from_str(&response_text)?;
        let data: Vec<Paradigm> = serde_json::from_value(json.clone())?;
        Ok(data)
    }

    pub async fn inflect_phrase(&self, sentence: String) -> Result<Vec<Inflection>, TezaursApiError> {
        let url = format!("{}/inflect_phrase/{}", API, sentence);
        let response = self.client.get(url).send().await?;
        let response_text = response.text().await?;
        let json: Value = serde_json::from_str(&response_text)?;
        let map: HashMap<String, String> = serde_json::from_value(json).unwrap();
        let inflections: Vec<Inflection> = map.into_iter().map(|(case, sentence)| {
            let case = match case.as_str() {
                "Akuzatīvs" => Case::Accusative,
                "Ģenitīvs" => Case::Genitive,
                "Datīvs" => Case::Datīvs,
                "Lokatīvs" => Case::Locative,
                "Nominatīvs" => Case::Nominative,
                "Instrumentālis" => Case::Instrumental,
                "Vokatīvs" => Case::Vocative,
                _ => panic!("Unexpected case"), // Handle unexpected cases appropriately
            };
            Inflection { case, sentence }
        }).collect();

        Ok(inflections)
    }
}

#[derive(Deserialize, Debug)]
pub struct Paradigm {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct TokenizeResponse {
    pub tokens: Vec<Token>
}

#[derive(Deserialize, Debug)]
pub struct Token {
    #[serde(rename = "Vārds")]
    pub word: String,
    #[serde(rename = "Marķējums")]
    pub tag: String,
    #[serde(rename = "Pamatforma")]
    pub base_form: String,
}

#[derive(Deserialize, Debug)]
pub struct AnalyzedWord {
    #[serde(rename = "Skaitlis")]
    pub number: Number,
    #[serde(rename = "Šķirkļa ID", default, deserialize_with = "deserialize_from_string_option")]
    pub id: Option<usize>,
    #[serde(rename = "Vārds")]
    pub word: String,
    #[serde(rename = "Šķirkļa cilvēklasāmais ID")]
    pub word_id: Option<String>,
    #[serde(rename = "Leksēmas nr", deserialize_with = "deserialize_from_string")]
    pub lexem: usize,
    #[serde(rename = "FreeText")]
    pub free_text: Option<String>,
    #[serde(rename = "Galotnes nr", deserialize_with = "deserialize_from_string")]
    pub end: usize,
    #[serde(rename = "Avots")]
    pub source: Option<String>,
    #[serde(rename = "Vārdšķira")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Mija", deserialize_with = "deserialize_from_string")]
    pub swap: usize,
    #[serde(rename = "Minēšana")]
    pub mention: String,
    #[serde(rename = "Pamatforma")]
    pub basic_form: String,
    #[serde(rename = "Locījums")]
    pub case: Case,
    #[serde(rename = "Dzimte")]
    pub gender: Gender,
    #[serde(rename = "Vārdgrupas nr", deserialize_with = "deserialize_from_string")]
    pub group: usize,
    #[serde(rename = "Deklinācija", deserialize_with = "deserialize_from_string")]
    pub declination: usize,

}

#[derive(Deserialize, Debug)]

pub enum Number {
    #[serde(rename = "Vienskaitlis")]
    Singular,
    #[serde(rename = "Daudzskaitlis")]
    Plural,
}

#[derive(Deserialize, Debug)]

pub enum Gender {
    #[serde(rename = "Sieviešu")]
    Female,
    #[serde(rename = "Vīriešu")]
    Male,
}

#[derive(Deserialize, Debug)]

pub enum PartOfSpeech {
    #[serde(rename = "Lietvārds")]
    Noun,
    #[serde(rename = "Īpašības vārds")]
    Adjective,
    #[serde(rename = "Skaitļa vārds")]
    Numeral,
    #[serde(rename = "Darbības vārds")]
    Verb,
}


#[derive(Deserialize, Debug)]
pub struct Inflection {
    pub case: Case,
    pub sentence: String,
}

#[derive(Deserialize, Debug)]

pub enum Case {
    #[serde(rename = "Nominatīvs")]
    Nominative,
    #[serde(rename = "Ģenitīvs")]
    Genitive,
    #[serde(rename = "Datīvs")]
    Datīvs,
    #[serde(rename = "Akuzatīvs")]
    Accusative,
    #[serde(rename = "Instrumentālis")]
    Instrumental ,
    #[serde(rename = "Lokatīvs")]
    Locative,
    #[serde(rename = "Vokatīvs")]
    Vocative,
}



#[cfg(test)]
mod tests {

    use crate::{TezaursApi,};

    #[tokio::test]
    async fn my_test() -> Result<(), Box<dyn std::error::Error>> {

        let api = TezaursApi::new();


        let words = api.analyze("jūra".to_string()).await?;
        println!("{:?}", words);
        // [AnalyzedWord { number: Singular, id: None, word: "jūra", word_id: None, lexem: 1033983, free_text: None, end: 28, source: Some("VVC paplašinātais vārdadienu saraksts 2014-10-31"), part_of_speech: Noun, swap: 1, mention: "Nav", basic_form: "Jūris", case: Genitive, gender: Male, group: 3, declination: 2 }, AnalyzedWord { number: Singular, id: Some(134187), word: "jūra", word_id: Some("jūra:1"), lexem: 138064, free_text: None, end: 75, source: None, part_of_speech: Noun, swap: 0, mention: "Nav", basic_form: "jūra", case: Nominative, gender: Female, group: 7, declination: 4 }]

        let tokens = api.tokenize(String::from("es domāju")).await?; //
        println!("{:?}", tokens);
        // [Token { word: "es", tag: "pp10snn", base_form: "es" }, Token { word: "domāju", tag: "vmnip_21san", base_form: "domāt" }]

        let text = api.normalize_phrase(String::from("Latvijas Universitātes Matemātikas un Informātikas Institūtam")).await?;

        println!("{:?}", text);
        // Latvijas Universitātes Matemātikas un Informātikas Institūts


        let inflections = api.inflect_phrase(String::from("Latvijas Universitātes Matemātikas un Informātikas Institūtam")).await?;
        println!("{:?}", inflections);
        //[Inflection { case: Accusative, sentence: "Latvijas Universitātes Matemātikas un Informātikas Institūtu" }, Inflection { case: Datīvs, sentence: "Latvijas Universitātes Matemātikas un Informātikas Institūtam" }, Inflection { case:Locative, sentence: "Latvijas Universitātes Matemātikas un Informātikas Institūtā" }, Inflection { case: Nominative, sentence: "Latvijas Universitātes Matemātikas un Informātikas Institūts" }, Inflection { case: Genitive, sentence: "Latvijas Universitātes Matemātikas un Informātikas Institūta" }]

        let paradigms = api.suitable_paradigm(String::from("pokemonizators")).await?;
        println!("{:?}", paradigms);
        // [Paradigm { id: 1, description: "noun-1a" }, Paradigm { id: 13, description: "adj-1" }, Paradigm { id: 39, description: "foreign" }]

        assert!(true);

        Ok(())
    }
}

