<h1 style="text-align: center;">TezaursApi</h1>

[![Crates.io](https://img.shields.io/crates/v/vkapi2)](https://crates.io/crates/tezaursapi)
[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/donate/?hosted_button_id=HPUSR7EB559TU)
![Crates.io](https://img.shields.io/crates/d/tezaursapi)

# Async Rust wrapper of [tezaurs](https://tezaurs.lv/)
### [documentation](http://api.tezaurs.lv:8182/)

## Please fell free to contribute

### create api, call functions:

```rust
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

```

### impelented:
- [x] analyze
- [x] tokenize
- [x] normalize_phrase