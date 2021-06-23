use hostfile;
use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct MultiPassInventory {
    list: Vec<VirtualMachine>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VirtualMachine {
    ipv4: Option<Vec<String>>,
    name: String,
    release: String,
    state: String,
}

// Idea
// [
//      Enum (
//          Host {
//              "key":
//              {
//                  "source": String,
//                  "account": String,
//                  "key": String,
//                  Option (
//                      Comment,
//                      None,
//                  ),
//              }
//          },
//          Comment {
//              "text": String
//          },
//          Blank {
//              Bool
//          },
//
//      ),
// ]

fn main() {
    let our_names = vec!["jenkins", "gitlab"]; // Move to config file
    let output = Command::new("multipass")
        .arg("list")
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to run mutlipass list");

    let multipass_virtual_machines: MultiPassInventory = serde_json::from_str(
        std::str::from_utf8(&output.stdout)
            .expect("Could not convert from standard output to string"),
    )
    .expect("Could not deserialize to struct");

    let this_hostfile = hostfile::parse_hostfile().expect("Failed to parse hostfile.");

    for entry in this_hostfile {
        // Is it in our list of boxes?
        if our_names.iter().any(|thingy| &entry.names[0] == thingy) {
            for vm in &multipass_virtual_machines.list {
                // Is it named in multipass and a non zero address list?
                if vm.name == entry.names[0] && vm.ipv4.is_some() {
                    println!(
                        "{}\t{}",
                        match &vm.ipv4 {
                            Some(val) => &val[0],
                            None => panic!("Missing IP address for VM, is it shutoff?"),
                        },
                        entry.names[0]
                    );
                }
            }
        } else if entry.names.len() > 1 {
            println!("{}\t{}", entry.ip, entry.names.join("\t"));
        } else {
            println!("{}\t{}", entry.ip, entry.names[0]);
        }
    }
}
