use clap::{App, Arg};
use repositories::pokemon::InMemoryRepository;
use std::sync::Arc;

mod api;
mod cli;
mod domain;
mod repositories;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = App::new("pokedex")
        .version("1")
        .author("Darren")
        .arg(Arg::new("cli").long("cli").help("Runs in CLI mode"))
        .get_matches();

    match matches.occurrences_of("cli") {
        0 => api::serve("localhost:8000", repo),
        _ => cli::run(repo),
    }
}
