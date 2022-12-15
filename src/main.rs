use chronoutil::{shift_months, shift_years};
use ::scraper::Html;
use tokio::fs;
use tokio_postgres::{Client, NoTls, Error};
use clap::Parser;
use chrono::{prelude::*, Duration};
use serde_yaml;
use std::{
    fs::{OpenOptions, File}, 
    io::{BufWriter, Write, BufRead, BufReader}
};

mod database;
mod errors;
mod scraper;
mod fundamentals;

use database::queries::queries as database_query;
use errors::error_handler::error_handler as error;
use crate::scraper::financial_data::get_financial_data as stock_scraper;

#[derive(Parser, Debug)]
#[clap(about = "Stock CLI")]
struct Opt {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Parser)]
enum Command {
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
}

async fn run(opt: Opt, mut client: Client) -> Result<(), Error> {
    match opt.cmd {
        Command::Add { stock_name } => {
            let stock_already_exists = database_query::stock_exists(
                &mut client, 
                stock_name.to_lowercase()
            ).await;

            if stock_already_exists {
                println!("Stock already exists")
            } else {
                let url = stock_scraper::Url { symbol: stock_name.clone() };

                let selector = ::scraper::Selector::parse("section[id='lookup-page']");

                match selector {
                    Ok(selector_parse) => {
                        let uri = url.scrapped_home().await;
                        let html = uri.select(&selector_parse).next();
        
                        if html == None {
                            match database_query::insert(&mut client, stock_name).await {
                                Ok(_) => println!("Stock was added"),
                                Err(e) => println!("Error: {}", e)
                            };
                        } else {
                            println!("Stock symbol is not valid. Make sure that it exists.");
                        }
                    },
                    Err(e) => println!("Error occurred when trying to parse page: {:?}", e)
                }
            }
        },
        Command::List {} => {
            match database_query::list(&mut client).await {
                Err(e) => println!("failed: {}", e),
                _ => (),
            }
        },
        Command::Info { explanation } => {
            let expl = explanation.to_lowercase();

            match expl.as_str() {
                "pe_ratio" => fundamentals::explanations::PERatio {}.info(),
                "equity" => fundamentals::explanations::Equity {}.info(),
                "market_value" => fundamentals::explanations::MarketValue {}.info(),
                "pb_ratio" => fundamentals::explanations::PBRatio {}.info(),
                "bvps" => fundamentals::explanations::BVPS {}.info(),
                "peg_ratio" => fundamentals::explanations::PEGRatio {}.info(),
                "debt_equity_ratio" => fundamentals::explanations::DebtEquityRatio {}.info(),
                "return_on_equity" => fundamentals::explanations::ReturnOnEquity {}.info(),
                "return_on_assets" => fundamentals::explanations::ReturnOnAssets {}.info(),
                "current_ratio" => fundamentals::explanations::CurrentRatio {}.info(),
                "assets" => fundamentals::explanations::Assets {}.info(),
                "liabilities" => fundamentals::explanations::Liabilities {}.info(),
                "cash_flow_statement" => fundamentals::explanations::CashFlowStatement {}.info(),
                "income_investing" => fundamentals::explanations::IncomeInvesting {}.info(),
                "issuance_of_stock" => fundamentals::explanations::IssuanceOfStock {}.info(),
                "cash_from_operating_activities" => fundamentals::explanations::OperatingActivities {}.info(),
                "cash_from_financing_activities" => fundamentals::explanations::FinancingActivities {}.info(),
                "cash_from_investing_activities" => fundamentals::explanations::InvestingActivities {}.info(),
                _ => {
                    let options = vec!["equity", "market_value", "pb_ratio", "bvps", "peg_ratio", 
                        "debt_equity_ratio", "return_on_equity", "return_on_assets", "current_ratio", "assets", 
                        "liabilities", "cash_flow_statement", "income_investing", "issuance_of_stock", 
                        "cash_from_operating_activities", "cash_from_financing_activities", "cash_from_investing_activities"
                    ];

                    println!("No term was found. The following are supported:");
                    for term in options {
                        println!("  - {}", term)
                    }
                }
            }
        },
        Command::Search { stock_name } => {
            match database_query::search(&mut client, stock_name.to_lowercase(), true).await {
                Err(e) => println!("Error occurred when searching for stock: {}", e),
                _ => (),
            }
        },
        Command::Drop { stock_name } => {
            match database_query::drop(&mut client, stock_name).await {
                Ok(result) => {
                    if result == 1 {
                        println!("Stock was deleted")
                    } else if result == 0 {
                        println!("Stock could not be found")
                    }
                },
                Err(e) => println!("Error: {}", e)
            }
        },
        Command::Update { stock_name } => {
            match database_query::update(&mut client, stock_name).await {
                Err(e) => println!("Error: {}", e),
                _ => println!("Stock was updated")
            }
        },
        Command::UpdateAll {} => {
            println!("This may take a while...");

            database_query::update_all(&mut client).await;
        },
        Command::History { stock_name, mut date } => {
            let stock = database_query::search(&mut client, stock_name.clone(), false);
            let stock_data = stock_scraper::StockData { symbol: stock_name.clone(), url: Html::parse_document("") };

            if date.contains("day") || 
                date.contains("week") ||
                date.contains("month") ||
                date.contains("year") {
                date = parse_date(date)
            }
                
            let mut splitted_date: Vec<i32> = date.split(".")
                .into_iter()
                .map(|d| d.parse::<i32>().unwrap())
                .collect();

            let mut parsed_date = NaiveDate::from_ymd_opt(
                splitted_date[2], 
                splitted_date[1].try_into().unwrap(), 
                splitted_date[0].try_into().unwrap()
            ).unwrap();

            if parsed_date > Local::now().date_naive() {
                println!("The entered date lies in the future. Please provide a date from the past.");
                return Ok(());
            };
            
            if parsed_date.format("%A").to_string() == "Saturday" || 
                parsed_date.format("%A").to_string() == "Sunday" {
                if parsed_date.format("%A").to_string() == "Saturday" {
                    splitted_date[0] = splitted_date[0] + 2;
                    println!("It's a Saturday so we'll take Monday.");
                } else if parsed_date.format("%A").to_string() == "Sunday" {
                    splitted_date[0] = splitted_date[0] + 1;
                    println!("It's a Sunday so we'll take Monday.");
                }
                parsed_date = NaiveDate::from_ymd_opt(
                    splitted_date[2], 
                    splitted_date[1].try_into().unwrap(), 
                    splitted_date[0].try_into().unwrap()
                ).unwrap();
            }

            let price_history = stock_data.historical_price(
                splitted_date[0], splitted_date[1], splitted_date[2]
            );

            match price_history.await {
                Ok(price) => {
                    if price == 0.0 {
                        println!("Please take another day.");
                        return Ok(());
                    }

                    let unwraped_stock = stock.await.unwrap();

                    let date = splitted_date[0].to_string() 
                                + "." + 
                                splitted_date[1].to_string().as_str() 
                                + "." + 
                                splitted_date[2].to_string().as_str();
        
                    println!("Stock: {}", unwraped_stock.name.to_uppercase());
                    println!("Price since last update: {}", unwraped_stock.current_price);
                    
                    let percentage = (unwraped_stock.current_price / price) * 100.0;
                    println!("Date {}, {}", date, parsed_date.format("%A"));
                    println!("Price: {:.2}", price);
                    if percentage > 100.0 {
                        println!("Increase until today: {:.2}%", percentage - 100.0);
                    } else {
                        println!("Decrease until today: {:.2}%", percentage - 100.0);
                    }
                },
                Err(e) => println!("Error happened: {}", e)
            }
        },
        Command::Init {} => {
            println!("You already ran through the initialization process.");
            println!("If you want to change the URL of your database, run 'setdb'");
        },
        Command::ShowDB {} => {
            println!("{}", read_database_url().unwrap());
        }
        Command::SetDB { url } => {
            match set_database_url(
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                url
            ).await {
                Ok(_) => println!("Database URL was set!"),
                Err(e) => {
                    println!("Error occurred when trying to set Database URL");
                    println!("Error: {}", e);
                }
            }
        }
    }

    Ok(())
}

async fn fail_safe(opt: Opt) -> Result<(), Error> {
    match opt.cmd {
        Command::Init {} => {
            println!("Please note that you need to have Postgres installed!");

            let mut host = rpassword::prompt_password("Enter your host. If it's localhost, leave blank and press Enter").unwrap();
            let user_name = rpassword::prompt_password("Enter your postgres user name").unwrap();
            let pw = rpassword::prompt_password("Enter your postgres password").unwrap();
            let mut port = rpassword::prompt_password("Enter port. Default is 5433. Leave blank if default and press Enter").unwrap();

            host = if host == "" { "localhost".to_string() } else { host };
            port = if port == "" { "5433".to_string() } else { port };

            match set_database_url(
                user_name.clone(), 
                pw.clone(), 
                host.clone(), 
                port.clone(), 
                "".to_string()
            ).await {
                Ok(_) => {
                    let config = format!("host={} user={} password={} port={}", host, user_name, pw, port);
                    let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;
                    
                    tokio::spawn(async {
                        if let Err(e) = connection.await {
                            eprintln!("connection error: {}", e);
                        }
                    });

                    println!("Creating database...");
                    
                    let file = File::open("config/schema.sql").unwrap();
                    let reader = BufReader::new(file);

                    for line in reader.lines().enumerate() {
                        match client.batch_execute(line.1.unwrap().as_str()).await {
                            Err(e) => println!("error: {}", e),
                            _ => (),
                        }
                    }
                },
                Err(e) => println!("{}", e)
            }
        },
        Command::ShowDB {} => {
            println!("{:?}", read_database_url().unwrap());
        }
        Command::SetDB { url } => {
            match set_database_url(
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                "".to_string(), 
                url
            ).await {
                Ok(_) => println!("Database URL was set!"),
                Err(e) => {
                    println!("Error occurred when trying to set Database URL");
                    println!("Error: {}", e);
                }
            }
        },
        _ => {
            println!("This command can only be run if there's a connection with your database.");
            println!("If you haven't gone through the initialization process, run 'init'");
            println!("Make sure that the correct URL of your database is set.");
            println!("You can check it by running 'show-db'");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    match read_database_url() {
        Ok(url) => {
            match tokio_postgres::connect(url.as_str(), NoTls).await {
                Ok((client, connection)) => {
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            eprintln!("connection error: {}", e);
                        }
                    });
        
                    let opt = Opt::parse();
        
                    if let Err(e) = run(opt, client).await {
                        println!("Error occurred: {}", e);
                    }
                },
                Err(_) => {
                    let opt = Opt::parse();
        
                    if let Err(e) = fail_safe(opt).await {
                        println!("An error occurred: {}", e);
                    }
                }
            }
        },
        Err(_) => {
            let opt = Opt::parse();
        
            if let Err(e) = fail_safe(opt).await {
                println!("An error occurred: {}", e);
            }
        }
    }

    Ok(())
}

fn read_database_url() -> Result<String, error::FileError> {
    let file = std::fs::File::open("config/database.yml")?;
    let url: serde_yaml::Mapping = serde_yaml::from_reader(file)?;

    Ok(url["database_url"].as_str().unwrap().to_string())
}

async fn set_database_url(
    user_name: String, 
    pw: String, 
    host: String,
    port: String,
    mut url: String
) -> Result<(), error::FileError> {
    fs::remove_file("config/database.yml").await.ok();

    let f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("config/database.yml")
        .expect("Couldn't open file");

    if url == "" {
        url = format!("database_url: postgresql://{}:{}@{}:{}/", user_name, pw, host, port);
    } else {
        url = format!("database_url: {}", url);
    }

    let mut f = BufWriter::new(f);
    write!(f, "{}", url)?;

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
