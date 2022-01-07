use crate::domain::fetch_all_pokemon;
use crate::repositories::pokemon::Repository;
use std::sync::Arc;

#[derive(Debug)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn run(repo: Arc<dyn Repository>) {
    match fetch_all_pokemon::execute(repo) {
        Ok(res) if res.is_empty() => {
            println!("Pokedex does not contain any pokemon")
        }
        Ok(res) => res.into_iter().for_each(|p| {
            println!(
                "{:?}",
                Response {
                    number: p.number,
                    name: p.name,
                    types: p.types,
                }
            );
        }),
        Err(fetch_all_pokemon::Error::Unknown) => println!("An unknown error occurred"),
    };
}
