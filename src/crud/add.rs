use clap::Args;

/// Add the corresponding target
#[derive(Args, Debug)]
pub struct Add {
    #[clap(value_parser)]
    name: Option<String>,
}
