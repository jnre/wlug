use clap::Args;

// Update the corresponding target
#[derive(Args, Debug,PartialEq)]
pub struct Update {
    #[clap(value_parser)]
    name: Option<String>,
}