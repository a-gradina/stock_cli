use chronoutil::{shift_months, shift_years};
use tokio_postgres::{Error, NoTls};
use clap::Parser;
use chrono::{prelude::*, Duration};
use serde_yaml;
use std::{
    fs::{OpenOptions, File}, 
    io::{BufWriter, Write}, 
    path::Path
};

mod database;
mod file;
mod errors;
mod scraper;
mod fundamentals;

use database::{cmd::cmd as database_cmd, database::database::read_database_url};
use file::cmd::cmd as file_cmd;
use errors::error_handler::error_handler as error;

use crate::database::database::database::setup_database;

#[derive(Parser, Debug)]
#[clap(about = "Stock CLI")]
pub struct Opt {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    Add { stock_name: String },
    List {},
    Search { stock_name: String },
    Drop { stock_name: String },
    Update { stock_name: String },
    UpdateAll {},
    History { stock_name: String, date: String },
    Info {
        #[clap(default_value = "")]
        explanation: String,
    },
    Init {},
    ShowDB {},
    SetDB { url: String },
    Mode {},
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::parse();

    match check_mode() {
        Ok(mode) => {
            if mode == "database" {
                match read_database_url() {
                    Ok(url) => {
                        match tokio_postgres::connect(url.as_str(), NoTls).await {
                            Ok((client, connection)) => {
                                tokio::spawn(async move {
                                    if let Err(e) = connection.await {
                                        eprintln!("connection error: {}", e);
                                    }
                                });
                    
                                if let Err(e) = database_cmd::run(opt, client).await {
                                    println!("Error occurred: {}", e);
                                }
                            },
                            Err(_) => {
                                if let Err(e) = database_cmd::fail_safe(opt).await {
                                    println!("Error occurred: {}", e);
                                }
                            }
                        }
                    },
                    Err(_) => {
                        if let Err(e) = database_cmd::fail_safe(opt).await {
                            println!("Error occurred: {}", e);
                        }
                    }
                }
            } else if mode == "file" {
                if let Err(e) = file_cmd::run(opt).await {
                    println!("Error occurred: {}", e);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
            init_mode().await;       
        }
    }

    Ok(())
}

fn parse_date(date: String) -> String {
    let splitted_date: Vec<&str> = date.split(".").collect();

    let subtr_date = match splitted_date[1] {
        "day" | "days" => Local::now() - Duration::days(splitted_date[0].parse::<i64>().unwrap()),
        "week" | "weeks" => Local::now() - Duration::weeks(splitted_date[0].parse::<i64>().unwrap()),
        "month" | "months" => shift_months(Local::now(), -splitted_date[0].parse::<i32>().unwrap()),
        "year" | "years" => shift_years(Local::now(), -splitted_date[0].parse::<i32>().unwrap()),
        _ => Local::now() + Duration::days(1)
    };

    if subtr_date >= Local::now() {
        println!("Date could not be read.");
        println!("Date needs to in DMY(01.01.2020) format or NUMBER.days/weeks/months/years.ago");
        std::process::exit(0);
    }

    subtr_date.format("%d.%m.%Y").to_string()
}

pub fn set_mode() -> Result<(), error::SetFileError> {
    println!("There's two modes for you to choose. Type the indicating number and press enter.");
    println!("1. Save stocks to your local postgres database.");
    println!("2. Save stocks into config/stocks.txt.");

    let mut mode = rpassword::prompt_password("").unwrap();

    if mode == "1" {
        mode = "mode: database".to_string();
    } else if mode == "2" {
        mode = "mode: file".to_string();
    } else {
        return Err(error::SetFileError::InvalidInput)
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("config/mode.yml")
        .expect("Couldn't create file.");

    let mut writer = BufWriter::new(file);
    write!(writer, "{}", mode)?;

    Ok(())
}

pub fn check_mode() -> Result<String, error::CheckFileError> {
    let file = File::open("config/mode.yml")?;
    let mode: serde_yaml::Mapping = serde_yaml::from_reader(file)?;

    Ok(mode["mode"].as_str().unwrap().to_string())
}

pub async fn init_mode() {
    match set_mode() {
        Ok(()) => {
            match check_mode() {
                Ok(mode) => {
                    if mode == "file" {
                        println!("Mode is set to file.");

                        if !Path::new("config/stocks.txt").exists() {
                            File::create("config/stocks.txt").unwrap();
                        }
                    } else if mode == "database" {
                        println!("Mode is set to database.");

                        match read_database_url() {
                            Ok(_) => println!("Database URL is already set. If you want to change it, run 'set-db' and pass the new URL."),
                            Err(_) => setup_database().await.unwrap()
                        }
                    }
                },
                Err(e) => println!("{}", e)
            }
        },
        Err(e) => {
            println!("{}", e);
            println!("Mode could not be set.")
        }
    }
}

