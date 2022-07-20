use crate::crud::add::Add;
use crate::crud::delete::Delete;
use crate::crud::list::List;
use crate::crud::update::Update;
use std::ptr;
pub mod crud;
use clap::{Parser, Subcommand};
use windows_sys::Win32::NetworkManagement::NetManagement::*;

/// Simple cli to run on windows to manage local users and groups.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ///target Users
    Users {
        #[clap(subcommand)]
        crud: Crud,
    },
    /// target Groups
    Groups {
        #[clap(subcommand)]
        crud: Crud,
    },
}

#[derive(Subcommand, Debug)]
enum Crud {
    Add(Add),
    Update(Update),
    Delete(Delete),
    List(List),
}

fn main() {
    let cli = Cli::parse();
    dbg!(&cli);
    match &cli.command {
        Commands::Users { crud } => match crud {
            Crud::Add(name) => {
                println!("'myapp add' was used, name is: {:?}", name);
            }
            Crud::List(_) => {
                let mut user_buffer_ptr = ptr::null_mut();
                let server = ptr::null();
                let mut entries_read = 0;
                let mut total_entries = 0;
                let resume_handle = ptr::null_mut();
                println!("'myapp list' was used");
                unsafe {
                    println!(
                        "get list {:?}",
                        NetUserEnum(
                            server,
                            0,
                            0,
                            &mut user_buffer_ptr,
                            MAX_PREFERRED_LENGTH,
                            &mut entries_read,
                            &mut total_entries,
                            resume_handle
                        )
                    )
                }
            }
            Crud::Delete(name) => println!("'myapp add' was used, name is: {:?}", name),
            Crud::Update(name) => println!("'myapp add' was used, name is: {:?}", name),
        },
        Commands::Groups { crud } => match crud {
            Crud::Add(name) => println!("'myapp add' was used, name is: {:?}", name),
            Crud::List(name) => println!("'myapp add' was used, name is: {:?}", name),
            Crud::Delete(name) => println!("'myapp add' was used, name is: {:?}", name),
            Crud::Update(name) => println!("'myapp add' was used, name is: {:?}", name),
        },
    }
}
