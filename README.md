# iuutils - Infrastructure User Utils

Tool to safely allow running common file utils as a specific user/group using SUID and SGID

## Why?

Iuutils is intended to be used for situations where a non-login shared user and group owns "infrastructure" files and directory on a server. It facilitates usage of common file tools while ensuring the permissions are kept and properly set for new files and directories. The sudo equivalent would be `sudo -u <user> -g <group> <command>`, but it is lengthy, requires entering a password, and requires granting more privileges (sudo access) than actually needed to create files with the right permissions (which is the main goal, after all).

## How?

Iuutils is intended to be used as a setuid and setgid binary placed in path. When owned by the "infrastructure" user and its group, any member of that group can execute iuutils and manage files with it. "Others" must not have the execute permission on the binary.

## What tools?

Iuutils is hardcoded to only allow running the following tools: `ls`, `cp`, `mv`, `rm`, `ln`, `mkdir`, and `touch`.
Additionally, `id` can be run to help debug permission issues.

## Installation

First, you need to build the binary using cargo in release mode:

```shell
$ cargo build --release
```

The binary should now be present in `target/release/iu`. Copy it to any location present in `PATH`.

Change the owner and group of the binary to the user/group that owns the infrastructure.

**⚠️ To ensure that only the intended users be able to use it, the iuutils binary MUST NOT be executable by others (`chmod o-x iuutils`).**  
We recommend setting the permissions on the binary as `ug=rxs,o=` or `6550`. Both mean the same;
- **u**ser and **g**roup *can* **r**ead and **e**xecute
- **s**et id *on* **u**ser and **g**roup
- **o**thers *can* nothing

### Custom paths

The default hardcoded path to all tools is `/usr/bin/<tool>`. If you wish to change it, edit `paths.rs`, either manually or programmatically. The file shouldn't ever change much at all.

**⚠️ Do NOT replace the path to a tool by a different tool.**  
The *"absolutely safe"* binary that *"can't give a shell"* you are now chucking in a privilege excalation system *will* grant a shell to your favourite attacker.

You know it's going to happen  
Don't do it

*... or else the evil code pixies will do bad thing to your systems*


## Copyright

Copyright 2021 0x5c  
Released under the BSD 3-Clause License.  
See [`LICENCE`](LICENCE) for the full license text.
