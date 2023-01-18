/**
 * HyperDB CLI
 *
 * CLI for HyperDB (https://github.com/AfaanBilal/hyperdb)
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use clap::Parser;
use std::io::{self, Write};

mod client;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Server Address
    #[arg(short, long, default_value_t = String::from("http://localhost:8765"))]
    address: String,
}

fn main() {
    let args = Args::parse();
    let mut hc = client::HyperClient::new(args.address);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Command read failed.");

        line = line.trim().to_string();

        let line_parts: Vec<_> = line.split(' ').collect();
        let command = &line_parts[0].to_ascii_uppercase()[..];

        let mut exit = false;
        let r = match command {
            "PING" => hc.ping().expect("PING failed."),

            "VER" | "VERSION" => hc.version().expect("VERSION failed."),

            "HAS" => {
                if line_parts.len() < 2 {
                    String::from("Usage: HAS [key]. Enter ? for help.")
                } else {
                    hc.has(line_parts[1]).expect("HAS failed.")
                }
            }

            "GET" => {
                if line_parts.len() < 2 {
                    String::from("Usage: GET [key]. Enter ? for help.")
                } else {
                    hc.get(line_parts[1]).expect("GET failed.")
                }
            }

            "SET" => {
                if line_parts.len() < 3 {
                    String::from("Usage: SET [key] [value]. Enter ? for help.")
                } else {
                    hc.set(line_parts[1], line_parts[2]).expect("SET failed.")
                }
            }

            "DEL" | "DELETE" => {
                if line_parts.len() < 2 {
                    String::from("Usage: DEL [key]. Enter ? for help.")
                } else {
                    hc.delete(line_parts[1]).expect("DEL failed.")
                }
            }

            "EMPTY" => hc.empty().expect("EMPTY failed."),

            "ALL" => hc.all().expect("ALL failed."),

            "CLEAR" => hc.clear().expect("CLEAR failed."),

            "SAVE" => hc.save().expect("SAVE failed."),

            "RELOAD" => hc.reload().expect("RELOAD failed."),

            "RESET" => hc.reset().expect("RESET failed."),

            "QUIT" | "EXIT" => {
                exit = true;
                String::from("Goodbye.")
            }

            "HELP" | "?" => String::from(
                "[HyperDB Client]
COMMANDS:
HELP | ?            Print this help message
PING                Ping server
VERSION | VER       Get server version
HAS [key]           Check if there is a value for [key]
GET [key]           Get the value for [key]
SET [key] [value]   Set the [value] for [key]
DEL [key]           Delete the value for [key]
EMPTY               Check if the store is empty
ALL                 Get all stored data
CLEAR               Delete all stored data from memory
SAVE                Save stored data to disk
RELOAD              Reload store from disk
RESET               Delete all stored data from memory and disk
QUIT | EXIT         Quit",
            ),
            "" => String::from(""),
            _ => String::from("Unknown command."),
        };

        println!("{}", r);

        if exit {
            break;
        }
    }
}
