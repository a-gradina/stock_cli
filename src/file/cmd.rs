pub mod cmd {
    use std::fs::{OpenOptions, File};

    use chrono::{NaiveDate, Local, Utc, TimeZone};
    use serde::Deserialize;
    use tokio::task;
    use tokio_postgres::Error;

    use crate::{Opt, Command, fundamentals, parse_date, init_mode};
    use crate::file::queries::queries as file_query;
    use crate::scraper::financial_data::get_financial_data as stock_scraper;

    #[derive(Deserialize, Debug)]
    struct Response {
        chart: RespResult,
    }

    #[derive(Deserialize, Debug)]
    struct RespResult {
        result: Vec<QuoteBlock>,
    }

    #[derive(Deserialize, Debug)]
    struct QuoteBlock {
        indicators: Quote,
    }

    #[derive(Deserialize, Debug)]
    struct Quote {
        quote: Vec<QuoteList>,
    }

    #[derive(Deserialize, Debug)]
    struct QuoteList {
        close: Vec<Option<f64>>,
    }

    pub async fn historical_price(
        stock_name: String, day: i32, month: i32, year: i32
    ) -> Result<f64, reqwest::Error> {
        let start = Utc.with_ymd_and_hms(
            year, month as u32, day as u32, 0, 0, 0)
        .unwrap().timestamp();
        
        let end = Utc.with_ymd_and_hms(
            year, month as u32, day as u32, 23, 59, 59)
        .unwrap().timestamp();

        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?symbol={}&period1={}&period2={}&interval=1d",
            stock_name, stock_name, start, end
        );

        let data = task::spawn_blocking(|| {
            let resp = reqwest::blocking::get(url).unwrap();
            resp.json::<Response>()
        }).await.unwrap();
        
        let price = match data {
                Ok(price) => price.chart.result
                    .get(0)
                    .unwrap()
                    .indicators
                    .quote
                    .get(0)
                    .unwrap()
                    .close
                    .get(0)
                    .unwrap()
                    .unwrap(),
                Err(_) => {
                    println!("Date is a holiday or a day in which the stock exchange was closed.");
                    0.0
            }
        };

        Ok(price)
    }

    pub async fn run(opt: Opt) -> Result<(), Error> {
        match opt.cmd {
            Command::Add { stock_name } => {
                let file = OpenOptions::new()
                                .append(true)
                                .open("config/stocks.txt")
                                .unwrap();

                let url = stock_scraper::Url { symbol: stock_name.clone() };

                let selector = ::scraper::Selector::parse("section[id='lookup-page']");

                match selector {
                    Ok(selector_parse) => {
                        let uri = url.scrapped_home().await;
                        let html = uri.select(&selector_parse).next();
        
                        if html == None {
                            file_query::insert(&file, stock_name, false).await;
                        } else {
                            println!("Stock symbol is not valid. Make sure that it exists.");
                        }
                    },
                    Err(e) => println!("Error occurred when trying to parse page: {:?}", e)
                }
            }
            Command::List {} => {
                let file = File::open("config/stocks.txt").unwrap();

                file_query::list(file);
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
                let file = File::open("config/stocks.txt").unwrap();
                file_query::search(file, stock_name.to_lowercase());
            }
            Command::Drop { stock_name } => {
                let file = File::open("config/stocks.txt").unwrap();
                
                file_query::drop(file, stock_name);
            }
            Command::Update { stock_name } => {
                let file = File::open("config/stocks.txt").unwrap();

                file_query::update(file, stock_name).await;
            }
            Command::UpdateAll {} => {
                let file = File::open("config/stocks.txt").unwrap();

                file_query::update_all(file).await;
            }
            Command::History { stock_name, mut date } => {
                let contents = file_query::file_to_string(&File::open("config/stocks.txt").unwrap());

                let mut symbol_exists = false;
                let mut stock_row = "";

                for row in contents.split(";") {
                    let fields = row.split(",").collect::<Vec<&str>>();
                    let symbol = fields[0].replace("\n", "");
        
                    if symbol == stock_name {
                        symbol_exists = true;
                        stock_row = row;
                    } 
                }

                if !symbol_exists {
                    println!("Stock {} was not found.", stock_name);
                    return Ok(());
                }
    
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
                
                let stock_name_cloned = stock_name.clone();
                
                let price_history = historical_price(
                    stock_name, splitted_date[0], splitted_date[1], splitted_date[2]
                );
    
                match price_history.await {
                    Ok(price) => {
                        if price == 0.0 {
                            println!("Please take another day.");
                            return Ok(());
                        }
    
                        let current_price = stock_row.split(" ").collect::<Vec<&str>>()[2].parse::<f64>().unwrap();
    
                        let date = splitted_date[0].to_string() 
                                            + "." + 
                                            splitted_date[1].to_string().as_str() 
                                            + "." + 
                                            splitted_date[2].to_string().as_str();
            
                        println!("Stock: {}", stock_name_cloned.to_uppercase());
                        println!("Price since last update: {}", current_price);
                        
                        let percentage = (current_price / price) * 100.0;
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
            Command::ShowDB { .. } => {
                println!("This command is only available if mode is set to database.");
            }
            Command::SetDB { .. } => {
                println!("This command is only available if mode is set to database.");
            }
            Command::Mode {} => {
                println!("File")
            }
        }
    
        Ok(())
    }
}