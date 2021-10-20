/*
    Infrastructure User Utils
    Copyright (c) 2021 0x5c
    SPDX-License-Identifier: BSD-3-Clause
*/


use nix::unistd::{ResGid, ResUid, getresgid, getresuid, setresgid, setresuid}; 
use std::process::{self, Command};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::env;

// Importing the tool paths from a separate file to allow easy patching
// ayyyyyyyy compile-time configuration
mod paths;
use crate::paths::*;


const VERSION: &str = env!("CARGO_PKG_VERSION");


fn main() {
    let debug = matches!(env::var("IUUTILS_DEBUG"), Ok(v) if v == "true" || v == "1");

    let mut args = env::args();

    // Arg0 is (in some cases) the command path/name
    let arg0 = args.next();

    // We only accept the second argument as the subcommand
    let subcommand = SubCommand::from(args.next());
    
    if debug {
        eprintln!("iuutils: Binary name (arg 0): {:?}", arg0.unwrap_or_default());
        eprintln!("iuutils: Subcommand: {:?}", subcommand);
        eprintln!("iuutils: Arguments: {:?}", args);
    }

    match subcommand {
        SubCommand::List => run_command(Path::new(PATH_LS), args, debug),
        SubCommand::Copy => run_command(Path::new(PATH_CP), args, debug),
        SubCommand::Move => run_command(Path::new(PATH_MV), args, debug),
        SubCommand::Remove => run_command(Path::new(PATH_RM), args, debug),
        SubCommand::Link => run_command(Path::new(PATH_LN), args, debug),
        SubCommand::Makedir => run_command(Path::new(PATH_MKDIR), args, debug),
        SubCommand::Touch => run_command(Path::new(PATH_TOUCH), args, debug),
        SubCommand::Version => version(),
        SubCommand::Help => full_help(),
        SubCommand::Id => run_command(Path::new(PATH_ID), args, debug),
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
    // Dropping group first to ensure dropping the user works
    setresgid(eg, eg, eg)?;
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
        None => eprintln!("error: no subcommand specified"),
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
    println!("\n\tSetting the environment variable IUUTILS_DEBUG to 'true' or 1");
    println!("\twill print permissions/invocation debug info to stderr.");
    println!("\nCopyright 2021 0x5c <dev@0x5c.io>");
    println!("{}", env!("CARGO_PKG_REPOSITORY"));
}


fn version() {
    println!("iuutils {}", VERSION);
}


fn run_command(command: &Path, args: env::Args, debug: bool) -> ! {
    let (resuid, resgid) = match get() {
        Ok(resid) => resid,
        Err(e) => {
            eprintln!("iuutils: error: failed to get user/group IDs: {}", e);
            process::exit(71);
        },
    };

    if debug {
        eprintln!("iuutils: User real/effective/saved: {}/{}/{}",
                  resuid.real, resuid.effective, resuid.saved);
        eprintln!("iuutils: Group real/effective/saved: {}/{}/{}",
                  resgid.real, resgid.effective, resgid.saved);
    }

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
