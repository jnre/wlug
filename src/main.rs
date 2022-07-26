use crate::crud::add::Add;
use crate::crud::delete::Delete;
use crate::crud::list::List;
use crate::crud::update::Update;
use std::{io, ptr};
pub mod crud;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use core::mem::transmute_copy;
use widestring::WideCString;
use windows_sys::Win32::Foundation::ERROR_MORE_DATA;
use windows_sys::Win32::NetworkManagement::NetManagement::*;

/// Simple cli to run on windows to manage local users and groups.
#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None,name="wlug",trailing_var_arg=true)]
struct Cli {
    #[clap(long = "generate", arg_enum, value_parser)]
    generator: Option<Shell>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
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

#[derive(Subcommand, Debug, PartialEq)]
enum Crud {
    Add(Add),
    Update(Update),
    Delete(Delete),
    List(List),
}
fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let cli = Cli::parse();
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {:?}...", generator);
        print_completions(generator, &mut cmd);
    } else {
        println!("{:#?}", cli);
    }
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
                let mut result: Vec<String> = Vec::new();
                println!("'myapp list' was used");

                let n_status = unsafe {
                    NetUserEnum(
                        server,
                        1,
                        0,
                        &mut user_buffer_ptr,
                        MAX_PREFERRED_LENGTH,
                        &mut entries_read,
                        &mut total_entries,
                        resume_handle,
                    )
                };
                dbg!(&n_status);
                // ADD BETTER ERROR HANDLING
                if n_status != 0 && n_status != ERROR_MORE_DATA {
                    panic!("api failed"); // API Failed
                }
                let mut tmpbuffer = user_buffer_ptr;
                // let user_info_0: USER_INFO_0 = unsafe { transmute_copy(&user_buffer_ptr) };
                for _i in 0..entries_read as usize {
                    // Iterate over read entries
                    let slice: &[u8] = unsafe { std::slice::from_raw_parts(tmpbuffer, 1 as usize) };
                    let p_user: USER_INFO_0 = unsafe { transmute_copy(&slice[0]) };
                    let user_name_wc: WideCString =
                        unsafe { WideCString::from_ptr_str(p_user.usri0_name) };
                    let user_name: String = user_name_wc.to_string_lossy();
                    // result.push(unsafe { p_user.usri0_name.as_ref().unwrap().to_string() });
                    result.push(user_name);
                    tmpbuffer = unsafe { tmpbuffer.add(56) };
                }
                println!("{:?}", result)
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
