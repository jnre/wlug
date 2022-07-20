use clap::Args;

// Update the corresponding target
#[derive(Args, Debug)]
pub struct Update {
    #[clap(value_parser)]
    name: Option<String>,
}