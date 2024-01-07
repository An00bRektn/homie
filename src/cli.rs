use clap::{Parser, Subcommand};

use std::path::PathBuf;

// command line parsing
#[derive(Parser)]
#[command(author="An00bRektn", version="0.1.0", about="a homie that's really into organization", long_about = None)]
pub struct Args {
    /// Specifies path to config file (default searches parent directories for .homie.yml)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Adds a new host to the config file
    Add {
        /// (String) New IP address to add
        #[arg(short, long)]
        ip: String,
        /// (optional, String) hostname of host
        #[arg(short='n', long)]
        hostname: Option<String>,
        /// (optional, String) operating system, should be enum
        #[arg(short, long)]
        os: Option<String>,
        /// (optional, bool) if machine can be accessed
        #[arg(short, long)]
        access: Option<bool>,
        /// (optional, String) domain to which machine is joined
        #[arg(short, long)]
        domain: Option<String>
    },
    /// Deletes a host from the config file
    Delete {
        #[arg(short, long)]
        ip: String,
    },
    /// Gets information on the hosts stored in the config file (can specify a host by IP)
    Info {
        /// (String) access info on machine by IP
        #[arg(short, long)]
        ip: Option<String>
    },
    /// Initializes a new workspace
    Init {
        /// (String) specify location for new homie workspace or path to config to rebuild workspace from
        #[arg(short, long, value_name = "FILE")]
        directory: Option<PathBuf>
    },
    /// Updates an existing entry in .homie.yml
    Update {
        #[arg(short, long)]
        ip: String,
        /// (optional, String) hostname of host
        #[arg(short='n', long)]
        hostname: Option<String>,
        /// (optional, String) operating system, should be enum
        #[arg(short, long)]
        os: Option<String>,
        /// (optional, bool) if machine can be accessed
        #[arg(short, long)]
        access: Option<bool>,
        /// (optional, String) domain to which machine is joined
        #[arg(short, long)]
        domain: Option<String>
    },    
}
