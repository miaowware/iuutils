/*
    Infrastructure User Utils
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: BSD-3-Clause
*/


use nix::unistd::{ResGid, ResUid, getresgid, getresuid, setresgid, setresuid}; 
use std::process::{self, Command};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::env::{Args, args};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const PATH_LS: &str = "/usr/bin/ls";
const PATH_CP: &str = "/usr/bin/cp";
const PATH_MV: &str = "/usr/bin/mv";
const PATH_RM: &str = "/usr/bin/rm";
const PATH_LN: &str = "/usr/bin/ln";
const PATH_MKDIR: &str = "/usr/bin/mkdir";
const PATH_TOUCH: &str = "/usr/bin/touch";
const PATH_ID: &str = "/usr/bin/id";


fn main() {
    let mut args = args();

    // Discarding the first arg
    let arg0 = args.next();

    // We only accept the second argument as the subcommand
    let subcommand = SubCommand::from(args.next());
    
    println!("Binary name (arg 0): {:?}", arg0);
    println!("Subcommand: {:?}", subcommand);
    println!("Arguments: {:?}", args);

    match subcommand {
        SubCommand::List => run_command(Path::new(PATH_LS), args),
        SubCommand::Copy => run_command(Path::new(PATH_CP), args),
        SubCommand::Move => run_command(Path::new(PATH_MV), args),
        SubCommand::Remove => run_command(Path::new(PATH_RM), args),
        SubCommand::Link => run_command(Path::new(PATH_LN), args),
        SubCommand::Makedir => run_command(Path::new(PATH_MKDIR), args),
        SubCommand::Touch => run_command(Path::new(PATH_TOUCH), args),
        SubCommand::Version => version(),
        SubCommand::Help => full_help(),
        SubCommand::Id => run_command(Path::new(PATH_ID), args),
        SubCommand::None => short_help(None),
        SubCommand::Invalid(i) => short_help(Some(i)),
    }

    process::exit(0);
}

fn get() -> nix::Result<(ResUid, ResGid)> {
    Ok((getresuid()?, getresgid()?))
}

fn drop(ru: ResUid, rg: ResGid) -> nix::Result<()> {
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
    Id,
    None,
    Invalid(String),
}

impl From<Option<String>> for SubCommand {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Self::from(s),
            None => Self::None,
        }
    }
}

impl From<String> for SubCommand {
    fn from(s: String) -> Self {
        match &s as &str {
            "ls" => Self::List,
            "cp" => Self::Copy,
            "mv" => Self::Move,
            "rm" => Self::Remove,
            "ln" => Self::Link,
            "mkdir" => Self::Makedir,
            "touch" => Self::Touch,
            "id" => Self::Id,
            "--help" | "-h" => Self::Help,
            "--version" | "-V" => Self::Version,
            "" => Self::None,
            _ => Self::Invalid(s),
        }
    }
}

fn short_help(invalid: Option<String>) -> ! {
    match invalid {
        None => eprintln!("error: no subcommand specified"),  //? should this be stderr?
        Some(s) => eprintln!("error: invalid subcommand \"{}\"", s),
    }
    eprintln!("usage: iu <command> [ARGS...]\n");
    eprintln!("Use 'iu --help' to see the list of subcommands.");
    process::exit(64);
}

fn full_help() {
    println!("iuutils {}", VERSION);
    println!("{}\n", env!("CARGO_PKG_DESCRIPTION"));
    println!("usage:");
    println!("\tiu <command> [ARGS...]\n");
    println!("commands:");
    println!("\tls\tList directory contents ({})", PATH_LS);
    println!("\tcp\tCopy ({})", PATH_CP);
    println!("\tmv\tMove ({})", PATH_MV);
    println!("\trm\tRemove ({})", PATH_RM);
    println!("\tln\tLink ({})", PATH_LN);
    println!("\tmkdir\tCreate directory ({})", PATH_MKDIR);
    println!("\ttouch\tCreate file ({})", PATH_TOUCH);
    println!("\nothers:");
    println!("\tid\tShows the user and group IDs ({})", PATH_ID);
    println!("\t-h, --help\tShows this message");
    println!("\t-V, --version\tShows the iuutils version");
}

fn version() {
    println!("iuutils {}", VERSION);
}

fn run_command(command: &Path, args: Args) -> ! {
    let (resuid, resgid) = match get() {
        Ok(resid) => resid,
        Err(e) => {
            eprintln!("iuutils: error: failed to get user/group IDs: {}", e);
            process::exit(71);
        },
    };

    println!("{:?}", resuid); //TODO: make it env-based and better
    println!("{:?}", resgid);

    match drop(resuid, resgid) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("iuutils: error: failed to set user/group IDs: {}", e);
            process::exit(71);
        },
    }

    // Successful calls to exec() will never return, but failure returns an error
    let exec_error = Command::new(command)
                              .args(args)
                              .exec();
    eprintln!("iuutils: error: failed to exec '{}': {}", command.display(), exec_error);
    process::exit(71);
}
