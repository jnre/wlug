use clap::{Args};

/// Delete the corresponding target
#[derive(Args,Debug)]
pub struct Delete {
    #[clap(value_parser)]
    name: Option<String>,
}