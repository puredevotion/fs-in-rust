# fs-in-rust
Learning Rust by writing File Systems

## FS 0: a read-oly file system
We're going to recreate the hello-world example from fuser[link to fuser github], 
to get a basic understanding of the FUSE rust library and FUSE system as a whole.

We need 4 crates to get started:
* fuser: the crate that implement the FUSE library in Rust. there are a few implementations,but afaict this is the best maitained and most stable
* clap: a library that allows us to built terminal-apps, which we'll use to admi this filesystem, and those to come. We need `derive` and `cargo` features.
* libc: so we an send correct errors, see also https://www.gnu.org/software/libc/manual/html_node/Error-Codes.html
* envlogger: for logging errors to the terminal

### step 1: create the project and start of clap terminal interface
let's start with creating a new cargo project
> cargo new _path_
 
if you want a different name for your project than the file-path, you can do so via the `--name` flag

in path/src/main.rs we're going to create the basic terminal interface.

```rust
use clap::{Args, Parser, Subcommand}; // load clap library

// setting up command line arguments for the central FS program
// We're using Clap, the most used command line parser
// clap reads 3-slashes as comments for its interface
/// A program to create, mount and analyse filesystems  
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create and mount a file system
    Mount {
        path: String,
        #[clap(short, long, default_value = "fat")]
        fs: String,
    },
    /// unmount a filesystem
    Unmount,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli.command {
        Commands::Mount { path, fs } => {
            println!("mount was used, path is: {0} and type is: {1}", path, fs)
        }
        Commands::Unmount {  } => {
            println!("umount was called with path {0}", path)
        }
        Commands::hello {} => {
            hello::main();
        }
    }
}

```

In the main() we're first initatig the command line interface (cli). 
The Cli is a struct, that tells which commands are available, in the Commads enum.
For now. we need only 2 commands  create a filesystem (mount in jargon) and delete  a filesystem (unmount)
Within curly braces we can provide parameters, for mount I decided on:
* Path: on which location would we create the new file system?
* Fs: the type of file system. For now we have only one filesystem, but this allows for extension further down the line. We're giving it a default value of "readonly" so we don't need to set it explicitedly. Note that `short` and `long` arguments default to first character (f) and the full argument name (fs) respectively

For Unmount, we provide no arguments. It just needs to unmount the readonly filesystem.

If you run this, you'll find a nice overview of your cli-app, and if you run `mount --help` you'll get info on the mount command.

### step 2: creating the readonly file system.
Let's createa file `readonly.rs` and create our readonly FS in there.

```rust
pub mod filesystem {
    use fuser::{
        FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry,
        Request,
    };

    pub fn main(mountpoint: &str) {
        env_logger::init();

        let mut options = vec![MountOption::RO, MountOption::FSName("readonly-fs".to_string()), MountOption::AutoUnmount];

        fuser::mount2(ReadonlyFS, mountpoint, &options).unwrap();
    }
}
```

we're importing fuser and all the stuff we need to create the FS.
in main, we're expecting a mountpoit (the path where we create the new FS), iitialise the logger
and create a vector of options:
* mountOption::RO set as read-only
* FSName create a name for the FS
* autoUnmount: automatically delete the FS on termination

The the Fuser::mount2 will actually create the FS, referencing ReadonlyFS which we haven't created yet, so let's get to it.

