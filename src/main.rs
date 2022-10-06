mod hello;
mod readonly;

use clap::{Args, Parser, Subcommand};
use crate::readonly::MountOptions;

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
        #[clap(short, long, default_value = "readonly")]
        fs: String,
        #[clap(short, long, default_value = "false")]
        autoUnmount: String,
        #[clap(short, long, default_value = "false")]
        root: String
    },
    /// unmount a filesystem
    Unmount {},
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli.command {
        Commands::Mount { path, fs, autoUnmount, root } => {
            println!("mount was used, path is: {0} and type is: {1}", path, fs);
            let mount_options = readonly::MountOptions { auto_unmount: autoUnmount.trim().parse().unwrap(), root: root.trim().parse().unwrap()};

            readonly::main(path, mount_options);
        }
        Commands::Unmount {} => {
            println!("umount was called",)
        }
    }
}
