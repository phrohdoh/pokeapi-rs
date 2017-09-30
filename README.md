# `pokeapi`

## What is this project?

The `pokeapi` library ("crate" in the Rust ecosystem) allows applications to retrieve data about [Pokémon](https://en.wikipedia.org/wiki/Pok%C3%A9mon) from the [pokeapi](https://pokeapi.co) service.

## Getting the code

From your shell:

```sh
git clone git@github.com:Phrohdoh/pokeapi-rs.git
```

## Building the crate

From your shell:

```sh
# from the root of the project (the directory containing the Cargo.toml file)
cargo build
```

## Running the included command-line application

From your shell:

```sh
# from the root of the project (the directory containing the Cargo.toml file)
cargo run --features=clap -- --name=pikachu
```

When you run the above command you should see this output:

```
Pokemon { id: 25, name: "pikachu", base_experience: 112, height: 4, is_default: true, order: 32, weight: 60 }
```