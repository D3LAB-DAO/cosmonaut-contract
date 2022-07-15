#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate term_painter;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::process::{Command, Stdio};
use term_painter::Attr::*;
use term_painter::Color::*;
use term_painter::ToStyle;
use toml::Value;

#[derive(Deserialize)]
struct Config {
    item: Vec<Item>,
}

#[derive(Deserialize)]
#[derive(Clone)]
struct Item {
    cmd: String,
    code: String,
    desc: String,
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

fn read_config() -> Result<String, String> {
    File::open("./run.toml")
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
}

fn parse_config(config: String) -> Result<Config, String> {
    toml::from_str(&config).map_err(|e: toml::de::Error| e.to_string())
}

fn print_menu(config: Config) -> Result<Config, String> {
    println!("{}", "==============================================");
    for item in &config.item {
        println!("{:6} {}", Bold.paint(&item.code), item.desc);
    }
    println!("{}", "==============================================");

    Ok(config)
}

fn ask_for_option(config: Config) -> Result<(Config, String), String> {
    println!("{}", Green.paint("Select an option"));

    io::stdout().flush();

    let mut opt = String::new();
    io::stdin().read_line(&mut opt);

    if let Some('\n') = opt.chars().next_back() {
        opt.pop();
    }

    if let Some('\r') = opt.chars().next_back() {
        opt.pop();
    }

    let value = (config, opt);


    Ok(value)
}

fn find_item(tuple: (Config, String)) -> Result<Item, String> {
    let (config, opt) = tuple;

    config
        .item
        .iter()
        .find(|&item| item.code == opt)
        .cloned()
        .ok_or("Option not found".to_string())
}

fn run_item(item: Item) -> Result<std::process::Output, String> {
    let parts: Vec<&str> = item.cmd.split(" ").collect();

    parts
        .split_first()
        .ok_or("Command not found".to_string())
        .and_then(|(cmd, rest)| {
            Command::new(cmd)
                .args(rest)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|err| err.to_string())
        })

}

fn main() {
    let result = read_config()
        .and_then(parse_config)
        .and_then(print_menu)
        .and_then(ask_for_option)
        .and_then(find_item)
        .and_then(run_item);

    match result {
        Ok(config) => println!("{}", ""),

        Err(err) => println!("{}", err),
    }

}
