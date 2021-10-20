// This file contains the paths to each tool iuutils can run.
// Paths here are intended to be modified by a sysadmin if needed.
// This file is almost guaranteed to never change,
// patches will probably never break.


// * ⚠️ Do NOT replace the path to a tool by a different tool. ⚠️
// The "absolutely safe" binary that "can't give a shell" you are now chucking in
// a privilege excalation system *will* grant a shell to your favourite attacker.
//
// You know it's going to happen
// Don't do it
//
// ... or else the evil code pixies will do bad thing to your systems


pub const PATH_LS: &str = "/usr/bin/ls";
pub const PATH_CP: &str = "/usr/bin/cp";
pub const PATH_MV: &str = "/usr/bin/mv";
pub const PATH_RM: &str = "/usr/bin/rm";
pub const PATH_LN: &str = "/usr/bin/ln";
pub const PATH_MKDIR: &str = "/usr/bin/mkdir";
pub const PATH_TOUCH: &str = "/usr/bin/touch";
pub const PATH_ID: &str = "/usr/bin/id";
