use std::net::{TcpListener, TcpStream, Shutdown, IpAddr};
use std::io::{self, Write, Read};
use std::sync::{Arc, atomic::{AtomicU8, Ordering}};
use std::process::Command;
use std::thread;
use clap::{Parser, ValueEnum};
use colored::*;
use crate::templates::{get_template, Template};
mod banner;
mod templates;

#[derive(Debug, Clone, ValueEnum)]

enum Mode {
    Listen,
    Generate,
}
#[derive(Debug, Clone, ValueEnum)]

pub enum Format {
    Python3,
    Netcat,
    NetcatExe,
    Socat,
    Powershell,
    Mkfifo,
    PhpPentestmonkey,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {

    //mode to run
    #[arg(short='m', long, help="The mode to start RustShell in\n")]
    mode: Option<Mode>,
    //format for payload generator
    #[arg(short='f', long, help="The syntax format to use (only applies to generate)\n")]
    format: Option<Format>,
    //shell for payload generator
    #[arg(short='s', long, help="The name of the shell to use (only applies to generate)")]
    shell: Option<String>,
    //IP to Listen On
    #[arg(short='l', long, help="The IP address to listen on/callback to")]
    lhost: Option<String>,
    //Port to Listen On
    #[arg(short='p', long, help="The port to listen on/callback to")]
    lport: Option<String>,
}

fn print_disclaimer() {
    error("DISCLAIMER:");
    info("This program is for educational and ethical purposes only.");
    info("Do not use this tool on systems you do not own or have permission to test.");
    info("By using this software, you agree that the author is not responsible");
    info("for any actions or consequences resulting from its use.\n");
    println!("---------------------------------------------------------------------------------------");
}

fn parse_ip(input: &str) -> Result<IpAddr, String> {
    input.parse::<IpAddr>()
        .map_err(|_| format!("[-] Invalid IP Address: {}", input))
}

fn info(msg: &str) {
    println!("{} {}", "[!]".cyan().bold(), msg);
}

fn success(msg: &str) {
    println!("{} {}", "[+]".green().bold(), msg);
}

fn error(msg: &str) {
    println!("{} {}", "[-]".red().bold(), msg);
}

fn send_command(stream: &mut TcpStream, command: &mut String) -> std::io::Result<()> {
    let bytes: &[u8] = command.as_bytes();
    stream.write(bytes)?;
    Ok(())
}

fn print_help_menu() {
    println!("-help: Show this menu");
    println!("-exit: Exit Rustshell menu");
    println!("-interactive: Enter a local interactive shell( (in case you need more functionality)");
}

fn exec_local_command(cmd: &str) {
    let printinfo = format!("Executing Command: {}", cmd);
    info(&printinfo);
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output();

    match output {
        Ok(out) => {
            print!("{}", String::from_utf8_lossy(&out.stdout));
            print!("{}", String::from_utf8_lossy(&out.stderr));
        }
        Err(e) => {
            error(&format!("Failed to execute command: {}", e));
        }
    }
}

fn generate_shell(args: &Args) -> std::io::Result<()> {
    //Declare IP address 
//    let mut ip_addr = IpAddr::V4(Ipv4Addr::new(0,0,0,0));
    //Determine if user used the --lhost flag
    let address = match &args.lhost {
        Some(ip) => ip.clone(),
        None => {
            let mut input = String::new();
            info("Enter the IP address to connect back to:");
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    //Determine if user used the --lport flag
    let port = match &args.lport {
        Some(p) => p.clone(),
        None => {
            let mut input = String::new();
            info("Enter the Port to connect back to:");
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    //Parse address variable as an IpAddr object.
    let ip_addr : IpAddr = match parse_ip(&address) {
        Ok(ip) => ip,
        Err(_) => {
            let errormessage = format!("Invalid IP Address: {}", address);
            error(&errormessage);
            return Ok(())
            }
    };
    //Determine if user supplied a --format value
    let format : Format = match &args.format {
        Some(p) => p.clone(),
        None => {
            //Take in input supplied by the user for a format
            let mut input = String::new();
            info("Enter the Shell Format (Currently supported: Python3, Python, Nc, Socat):");
            std::io::stdin().read_line(&mut input)?;
            let input_trimmed = input.trim();
            //Parse input_trimmed into a Format type
            match Format::from_str(input_trimmed, true) {
                Ok(f) => f,
                Err(_) => {
                    error("Invalid Format Entered. Please Try Again!");
                    return Ok(())
                }
            }
        }
    };
    //Determine if user supplied a --shell value
    let shell = match &args.shell {
        Some(p) => p.clone(),
        None => {
            let mut input = String::new();
            info("Enter the full path of the shell to use:");
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    // Get template
    let template: Template = get_template(&format);
    // Replace placeholders
    let output = template
        .template
        .replace("<CONNECT_BACK_HOST>", &ip_addr.to_string())
        .replace("<CONNECT_BACK_PORT>", &port)
        .replace("<SHELL>", &shell);

    let title = format!("Format: {} - {}", template.name, template.description);
    success(&title);
    println!("Generated syntax:\n\n{}", output);
    println!("---------------------------------------------------------------------------------------");
    Ok(())
}

fn local_menu() {
    let mut exit_flag = false;
    info("Welcome to the local menu function! Type \"-help\" for a list of options");
    loop {
        if exit_flag {
            info("Returning to shell...");
            break;
        } else if !exit_flag {
            //print "rs> " line right next to users command
            print!("{}", "rs> ".cyan().bold());
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            //decision tree to decide whether to execute command or execute menu option
            match input.trim() {
                c if c.starts_with("-") => {
                    match c {
                        "-exit" => {
                            exit_flag = true;
                        }
                        "-help" => {
                            info("Type a shell command to execute a command on the local machine. Alternatively, here is a list of menu commands:");
                            print_help_menu();
                        }
                        "-interactive" => {
                            info("Interactive shell mode coming soon!");
                        }
                        _ => {
                            error("That is not a valid command. Try again!");
                            continue;
                        }
                    }
                }
                c if !c.starts_with("-") => {
                    let local_cmd = c;
                    exec_local_command(local_cmd);
                }
                _ => {
                    error("Invalid command detected, Try again!");
                    continue;
                }
            }
        } else {
            error("Something went wrong...Exiting");
            break;
        }
    }
}

fn start_listener(args: &Args) -> std::io::Result<()> {
    //Declare IP address 
//   let mut ip_addr = IpAddr::V4(Ipv4Addr::new(0,0,0,0));
    //Determine if user used the --lhost flag
    let address = match &args.lhost {
        Some(ip) => ip.clone(),
        None => {
            let mut input = String::new();
            info("Enter the IP to listen on:");
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    //Determine if user used the --lport flag
    let port = match &args.lport {
        Some(p) => p.clone(),
        None => {
            let mut input = String::new();
            info("Enter the Port to listen on:");
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    //Parse address variable as an IpAddr object.
    let ip_addr = match parse_ip(&address) {
        Ok(ip) => ip,
        Err(_) => {
            let errormessage = format!("Invalid IP Address: {}", address);
            error(&errormessage);
            return Ok(())
            }
    };
    //Combine ip_addr and port into one variable (format: ip_addr:port)
    let fullport = format!("{}:{}", ip_addr, port);
    //Attempt to start TcpListener on supplied fullport variable
    let listener = match TcpListener::bind(&fullport) {
    Ok(l) => {
            let listenmessage = format!("Listening on {} - Press Ctrl+C to Exit", fullport);
            info(&listenmessage); l
    },
    Err(e) => {
        error(&format!(
            "Failed to bind to {} ({})",
            fullport, e
        ));
        return Ok(()); // return to menu instead of crashing
    }
};
    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut exit_flag = false;
        handle_client(stream?);
        info("Would you like to keep listening? (yes/no)");
        loop {
            let mut keep_listening = String::new();
            std::io::stdin().read_line(&mut keep_listening)?;
            if keep_listening.trim() == "no" {
                error("Terminating Program...Goodbye!");
                exit_flag = true;
                break;
            } else if keep_listening.trim() == "yes" {
                let message = format!("Continue Listening on {}", listener.local_addr().unwrap());
                info(&message);
                break;
            } else  {
                error("Invalid option Entered. Try again");
            }
        }
        if exit_flag {
            break;
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let client = stream.peer_addr().unwrap();
    let mut read_stream = stream.try_clone().unwrap();
    let state = Arc::new(AtomicU8::new(STATE_CONNECTED));
    let reader_state = state.clone();
    //Declare states
    const STATE_CONNECTED: u8 = 1;
    const STATE_LOCAL_SHUTDOWN: u8 = 2;
    const STATE_REMOTE_SHUTDOWN: u8 = 3;

    //Upon successful connection, print message
    let connectmessage = format!("CONNECTION RECEIVED!\nFROM: {}\n", client);
    success(&connectmessage);
    info("Hint: type \"rs:\" to enter Rustshell menu");
        
    //Spawn a thread to continuously read from data stream
    thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    if reader_state.load(Ordering::SeqCst) == STATE_CONNECTED {
                        reader_state.store(STATE_REMOTE_SHUTDOWN, Ordering::SeqCst);
                        error("Conection Severed. Are you by chance using Netcat?!?");
                    }
                    break; // connection closed
                }
                Ok(n) => {
                    let output = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}", output);
                    io::stdout().flush().unwrap();
                }
                Err(e) => {
                    let errormessage = format!("Couldn't read From data stream: {} ", e);
                    error(&errormessage);
                    if reader_state.load(Ordering::SeqCst) == STATE_CONNECTED {
                        reader_state.store(STATE_REMOTE_SHUTDOWN, Ordering::SeqCst);
                    }
                    break;
                }
            }
        }
    });
    
    loop {
        match state.load(Ordering::SeqCst) {
            STATE_CONNECTED => {
                let mut command = String::new();
                std::io::stdin().read_line(&mut command).unwrap();
                if command.trim() == "rs:" {
                    info("Entering local menu...");
                    local_menu();
                    let _ = stream.write(b"\n");
                } else if command.trim() == "exit" {
                    state.store(STATE_LOCAL_SHUTDOWN, Ordering::SeqCst);   
                    let _ = send_command(&mut stream, &mut command);               
                } else {
                    let _ = send_command(&mut stream, &mut command);
                }
            }
            _ => break
        }
    }

    match state.load(Ordering::SeqCst) {
        STATE_REMOTE_SHUTDOWN => {
            info ("Shutting down TCP stream...");
            let _ = stream.shutdown(Shutdown::Both);
        }
        STATE_LOCAL_SHUTDOWN => {
            error("Exit command received, Terminating shell...");
            info("Sending exit command...");
            let _ = stream.write(b"exit\n");
            info("Shutting down TCP stream...");
            let _ = stream.shutdown(Shutdown::Both);
        }
        _ => error("Does....Not...COMPUTE!!!!")
    }
}

fn run_menu(args: &Args) -> std::io::Result<()> {
    let mut exit_flag = false;
    while !exit_flag {
        //create selected_option and set to 0
        let mut selected_option = String::new();
        //print out menu options
        println!("Please choose carefully from the menu as the following options may have changed:\n");
        println!("1. Start a reverse shell listener");
        println!("2. Generate reverse shell syntax");
        println!("3. Exit doing nothing");
        println!("---------------------------------------------------------------------------------------");
        //wait for user input
        std::io::stdin().read_line(&mut selected_option).unwrap();

        match selected_option.trim() {
            "1" => start_listener(&args)?,
            "2" => generate_shell(&args)?,
            "3" => {
                error("Exit command received...Terminating program...Goodbye!");
                exit_flag = true;
            },
            _ => {
                error("You have entered an invalid option. Try Again!");
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    //Print Random Banner
    banner::print_random_banner();
    println!("A simple program to catch your reverse shells!");
    println!("---------------------------------------------------------------------------------------\n");
    print_disclaimer();
    //Parse Arguments
    let args = Args::parse();
    //check the value of -m/--mode and make a decision based on it
      match args.mode {
        Some(Mode::Listen) => {
            let _ = start_listener(&args);
        }
        Some(Mode::Generate) => {
            let _ = generate_shell(&args);
        }
        None => {
            let _ = run_menu(&args);
        }
    }
    Ok(())
}

    
