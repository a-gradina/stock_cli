pub mod cmd {
    use scraper::Html;
    use tokio_postgres::{Client, Error};
    use chrono::{NaiveDate, Local};

    use crate::{Opt, Command, fundamentals, parse_date, set_database_url, read_database_url, init_mode};
    use crate::database::queries::queries as database_query;
    use crate::scraper::financial_data::get_financial_data as stock_scraper;

    pub async fn run(opt: Opt, mut client: Client) -> Result<(), Error> {
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
            }
            Command::List {} => {
                match database_query::list(&mut client).await {
                    Err(e) => println!("failed: {}", e),
                    _ => (),
                }
            }
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
            }
            Command::Search { stock_name } => {
                match database_query::search(&mut client, stock_name.to_lowercase(), true).await {
                    Err(e) => println!("Error occurred when searching for stock: {}", e),
                    _ => (),
                }
            }
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
            }
            Command::Update { stock_name } => {
                match database_query::update(&mut client, stock_name).await {
                    Err(e) => println!("Error: {}", e),
                    _ => println!("Stock was updated")
                }
            }
            Command::UpdateAll {} => {
                println!("This may take a while...");
    
                database_query::update_all(&mut client).await;
            }
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
            }
            Command::Init {} => {
                init_mode().await
            }
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
            Command::Mode {} => {
                println!("Database")
            }
        }
    
        Ok(())
    }

    pub async fn fail_safe(opt: Opt) -> Result<(), Error> {
        match opt.cmd {
            Command::Init {} => {
                init_mode().await
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
                println!("There's neither a connection with your database nor a stocks.txt file in /config");
                println!("If you haven't gone through the initialization process, run 'init'");
                println!("If you already have gone through initialization and chose database, make sure that the correct URL of your database is set.");
                println!("You can check it by running 'show-db'");
            }
        }
    
        Ok(())
    }
}