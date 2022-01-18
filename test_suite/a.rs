#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
extern crate config;
use config::MetaConfig;
pub struct Config {
    #[value(from = "PORT")]
    pub port: u16,
    #[value(from = "HOST")]
    pub host: String,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Config {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Config {
                port: ref __self_0_0,
                host: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Config");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "port", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "host", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl MetaConfig for Config {
    fn init_from_env() -> ::std::result::Result<Self, ::config::Error> {
        let config = Self {
            port: ::config::load_var("PORT", None)?,
            host: ::config::load_var("HOST", None)?,
        };
        Ok(config)
    }
    fn init_from_hashmap(
        hashmap: &std::collections::HashMap<String, String>,
    ) -> ::std::result::Result<Self, ::envconfig::Error> {
        let config = Self {
            port: ::config::load_var("PORT", Some(hashmap))?,
            host: ::config::load_var("HOST", Some(hashmap))?,
        };
        Ok(config)
    }
    fn init() -> ::std::result::Result<Self, ::config::Error> {
        Self::init_from_env()
    }
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() {
    std::env::set_var("PORT", "1234");
    std::env::set_var("HOST", "HO");
    let config = Config::init_from_env();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
            &["", "\n"],
            &match (&config,) {
                _args => [::core::fmt::ArgumentV1::new(
                    _args.0,
                    ::core::fmt::Debug::fmt,
                )],
            },
            &[::core::fmt::rt::v1::Argument {
                position: 0usize,
                format: ::core::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                    flags: 4u32,
                    precision: ::core::fmt::rt::v1::Count::Implied,
                    width: ::core::fmt::rt::v1::Count::Implied,
                },
            }],
            unsafe { ::core::fmt::UnsafeArg::new() },
        ));
    };
}
