use std::{fs::File, path::PathBuf};

use clap::{Args, Parser};
use semigroup::Semigroup;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Parser)]
#[command(version, about, long_about = None)]
struct App {
    file: Option<PathBuf>,

    #[command(flatten)]
    person: Person,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Args, Semigroup)]
#[semigroup(with = "semigroup::op::Coalesce")]
struct Person {
    /// person name
    #[arg(env, short, long)]
    name: Option<String>,

    /// person age
    #[arg(env, short, long)]
    age: Option<u64>,
}

/// Reading configs from multiple sources
///
/// # Usage
///  ```sh
/// cargo run --example clap_serde
/// cargo run --example clap_serde -- semigroup/examples/john_doe.json
/// cargo run --example clap_serde -- --name=alice
/// NAME=bob cargo run --example clap_serde -- --name=alice
/// NAME=bob AGE=42 cargo run --example clap_serde
/// NAME=bob AGE=42 cargo run --example clap_serde -- semigroup/examples/john_doe.json --name=alice
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::parse();

    let cli_person = App::parse_from(std::env::args()).person;
    let env_person = App::parse_from(["app"]).person;
    let file_person = app
        .file
        .as_ref()
        .map(|f| Ok::<_, std::io::Error>(serde_json::from_reader(File::open(f)?)?))
        .transpose()?
        .unwrap_or_default();

    let person = cli_person.semigroup(env_person).semigroup(file_person);
    println!("{person:?}");
    Ok(())
}
