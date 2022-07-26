use clap::{Args};

/// Delete the corresponding target
#[derive(Args,Debug,PartialEq)]
pub struct Delete {
    #[clap(value_parser)]
    name: Option<String>,
}