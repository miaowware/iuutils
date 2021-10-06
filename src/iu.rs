use nix::unistd::{
    getresuid,
    getresgid,
    setresuid,
    setresgid,
}; 
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::path::Path;


fn main() {
    let id_path = Path::new("/usr/bin/id");

    let resuid = getresuid().unwrap();
    let resgid = getresgid().unwrap();

    let effuid = resuid.effective;
    let effgid = resgid.effective;

    println!("{:?}", resuid);
    println!("{:?}", resgid);

    // Set all IDs to desired values
    setresgid(effgid, effgid, effgid).unwrap();
    setresuid(effuid, effuid, effuid).unwrap();

    Command::new(id_path).exec();
}
