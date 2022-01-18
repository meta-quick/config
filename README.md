## Install

```rust
[dependencies]
metaconfig = "0.10.0"
```

## Usage

Let's say you application relies on the following environment variables:

* `DB_HOST`
* `DB_PORT`

And you want to initialize `Config` structure like this one:

```rust,ignore
struct Config {
    host: String,
    port: u16,
}
```

You can achieve this with the following code without boilerplate:

```rust
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,
}

fn main() {
    // Assuming the following environment variables are set
    std::env::set_var("DB_HOST", "127.0.0.1");

    // Initialize config from environment variables or terminate the process.
    let config = Config::init_from_env().unwrap();

    assert_eq!(config.db_host, "127.0.0.1");
    assert_eq!(config.db_port, 5432);
}
```

## Testing

When writing tests you should avoid using environment variables. Cargo runs Rust tests in parallel by default which means
you can end up with race conditions in your tests if two or more are fighting over an environment variable. 

To solve this you can initialise your `struct` from a `HashMap<String, String>` in your tests. The `HashMap` should 
match what you expect the real environment variables to be; for example `DB_HOST` environment variable becomes a 
`DB_HOST` key in your `HashMap`.

```rust
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,
}

#[test]
fn test_config_can_be_loaded_from_hashmap() {
    // Create a HashMap that looks like your environment 
    let mut hashmap = HashMap::new();
    hashmap.insert("DB_HOST".to_string(), "127.0.0.1".to_string());

    // Initialize config from a HashMap to avoid test race conditions
    let config = Config::init_from_hashmap(&hashmap).unwrap();

    assert_eq!(config.db_host, "127.0.0.1");
    assert_eq!(config.db_port, 5432);
}
```

## Contributing

### Running tests

Tests do some manipulation with environment variables, so to
prevent flaky tests they have to be executed in a single thread:

```
cargo test -- --test-threads=1
```

## License

[MIT](https://github.com/greyblake/envconfig-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)