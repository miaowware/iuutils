use nix::unistd::{ResGid, ResUid, getresgid, getresuid, setresgid, setresuid}; 
use std::process::{self, Command};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::env::args;
use anyhow;

#[allow(unreachable_code)]

fn main() {
    let mut args = args();

    // Discarding the first arg
    println!("a_0: {:?}", args.next());

    let subcommand = match args.next() {
        Some(s) => SubCommand::from(s),
        None => SubCommand::Invalid(String::from("wth!!!")),
    };

    println!("cmd: {:?}", subcommand);
    println!("args: {:?}", args);



    process::exit(0);


    let id_path = Path::new("/usr/bin/id");

    let (resuid, resgid) = match get() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    };


    println!("{:?}", resuid);
    println!("{:?}", resgid);

    match drop(resuid, resgid) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    }

    Command::new(id_path).exec();
}

fn get() -> anyhow::Result<(ResUid, ResGid)> {
    Ok((getresuid()?, getresgid()?))
}

fn drop(ru: ResUid, rg: ResGid) -> anyhow::Result<()> {
    let eu = ru.effective;
    let eg = rg.effective;

    // Set all IDs to desired values
    setresgid(eg, eg, eg)?;  // Dropping group first to ensure dropping the user works
    setresuid(eu, eu, eu)?;

    Ok(())
}

#[derive(Debug)]
enum SubCommand {
    List,
    Copy,
    Move,
    Remove,
    Link,
    Makedir,
    Touch,
    Help,
    Version,
    Debug,
    None,
    Invalid(String),
}

impl From<String> for SubCommand {
    fn from(s: String) -> Self {
        match &s as &str {
            "ls" | "list" => Self::List,
            "cp" | "copy" => Self::Copy,
            "mv" | "move" => Self::Move,
            "rm" | "remove" => Self::Remove,
            "ln" | "link" => Self::Link,
            "mkdir" => Self::Makedir,
            "touch" => Self::Touch,
            "help" | "--help" | "-h" => Self::Help,
            "version" | "--version" | "-V" => Self::Version,
            "debug" => Self::Debug,
            "" => Self::None,
            _ => Self::Invalid(s),
        }
    }
}
