use crate::cli::{Args, Commands};
use crate::hosts::{HostsFile, Host};
use std::{fs, io};
use std::io::Write;
use std::path::{PathBuf, Path};
use std::str::FromStr;
use clap::Parser;
use serde_yaml::Mapping;

mod cli;
mod hosts;

fn host_to_mapping(new_host: &Host) -> Mapping {
    let mut new_mapping = Mapping::new();

    // TODO: Find a way to clean this up
    new_mapping.insert("hostname".into(), new_host.hostname.clone().into());
    new_mapping.insert("os".into(), new_host.os.clone().into());
    new_mapping.insert("access".into(), new_host.access.into());
    new_mapping.insert("domain".into(), new_host.domain.clone().unwrap_or(String::from("")).into());
    new_mapping
}

fn check_config(config_path: Option<PathBuf>) -> String {
    match config_path.as_deref() {
        Some(conf) => {
            // check if the path exists
            let path_check = Path::new(conf.as_os_str().to_str().unwrap());
            match path_check.try_exists() {
                Ok(true) => {
                    String::from(path_check.as_os_str().to_str().unwrap())
                },
                Ok(false) => {
                    panic!("[!] Error, path does not exist: {:?}", path_check)
                },
                Err(_) => {
                    panic!("[!] Error, can't find path: {:?}", path_check)
                }
            }
        },
        None => {
            // search for config
            match find_config_file() {
                Some(path) => {
                    String::from(path.as_os_str().to_str().unwrap())
                },
                None => {
                    todo!("Ask to create a config file and do that")
                },
            }
        }
    }  
}

// https://codereview.stackexchange.com/questions/236743/find-a-file-in-current-or-parent-directories
fn find_config_file() -> Option<PathBuf>{
    let mut path: PathBuf = std::env::current_dir().unwrap();
    let file = Path::new(".homie.yaml");

    loop {
        path.push(file);

        if path.is_file() {
            break Some(path);
        }

        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}

fn create_new_wkspace(ip_address: &String, path_str: &String) {
    let mut path = PathBuf::from_str(path_str).unwrap();
    path.pop();
    
    let new_dir = Path::new(ip_address);
    path.push(new_dir);

    if !path.is_dir() {
        println!("[*] Did you want to create a new directory for this host? (y/n)");
        let mut user_choice = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("[!] Failed to read user input");

        if user_choice.chars().nth(0).unwrap() == 'y' {
            println!("[*] Making workspace...");
            // TODO error handling
            // TODO figure out how to make this a loop because I don't want to fight the borrow checker right now
            //let subdirs = vec!["www", "loot", "scans"];
            fs::create_dir(&path).unwrap();
            path.push("www");
            fs::create_dir(&path).unwrap();
            path.pop(); path.push("loot");
            fs::create_dir(&path).unwrap();
            path.pop(); path.push("scans");
            fs::create_dir(&path).unwrap();
            path.pop();
            println!("[+] Created! Located at {:?}", path.as_os_str().to_str().unwrap());
        } else {
            println!("[+] Skipping...")
        }

    }
    
}

fn value_to_host(host_mapping: serde_yaml::Value) -> Result<Host, serde_yaml::Error> {
    serde_yaml::from_value(host_mapping)
}

fn main() {
    let args = Args::parse();

    let true_config_path = check_config(args.config); // validate .homie.yaml location
    let config_file_str = fs::read_to_string(true_config_path.clone()).expect("Unable to open file");
    let mut hosts_file: HostsFile = serde_yaml::from_str::<HostsFile>(&config_file_str).unwrap(); // do the serializing stuff
    
    // TODO: Update and Delete commands
    // TODO: Credential commands

    // Actually executing stuff
    match &args.command {
        // COMMAND: ADD
        // add a new host to yaml file
        Some(Commands::Add { ip, hostname, os, access, domain }) => {
            // TODO: Check if IP address is a duplicate, if so, throw an error
            // TODO: Validate if IP address is real?
            println!("[+] Adding IP Address: {}", ip); 
            // create new host struct and add to mapping
            let new_host: Host = Host::new(hostname.clone(), os.clone(), access.clone(), domain.clone());
            let new_mapping: Mapping = host_to_mapping(&new_host);
            hosts_file.hosts.insert(ip.clone().into(), new_mapping.into());
            let back_to_yaml = serde_yaml::to_string(&hosts_file).unwrap(); // convert new mapping to yaml
            // write to file
            let mut fd = std::fs::OpenOptions::new().write(true).truncate(true).open(true_config_path.clone()).unwrap();
            let _ = fd.write_all(back_to_yaml.as_bytes());
            let _ = fd.flush();

            // ask if we want to create a new directory and do that
            create_new_wkspace(ip, &true_config_path)
        },
        // COMMAND: INFO
        // print info about a specific ip, if no ip is supplied, print information about all hosts
        Some(Commands::Info { ip }) => {
            // was an ip supplied?
            match ip.clone() {
                // if so, let's only use that
                Some(ip_key) => {
                    // and check if it actually corresponds to a recorded host
                    let host_value = hosts_file.hosts.get(ip_key.clone());
                    match host_value {
                        Some(selected_host) => {
                            match value_to_host(selected_host.clone()) {
                                Ok(_) => println!("{:#?}", selected_host), // TODO: pretty print info with colors, tabled?,
                                Err(_) => println!("[!] Error: Couldn't find a host with ip {}", ip_key), // I actually don't know how we reach here
                            }
                        },
                        // otherwise we say that this host hasn't been recorded
                        None => println!("[!] Error: Couldn't find a host with ip {}", ip_key),
                    }
                    //let selected_host: Host = value_to_host().unwrap();
                    //println!("{:#?}", selected_host); // TODO: pretty print info with colors, tabled?
                },
                // no ip, let's print everything
                None => {
                    println!("{:#?}", hosts_file);
                },
            }
        },
        None => println!("[!] Error: Did not specify a subcommand"),
    }
}