pub mod cmd {
    use std::fs::{OpenOptions, File};

    use tokio_postgres::Error;


    use crate::fundamentals::explanations::print_expl;
    use crate::{Opt, Command, init_mode};
    use crate::file::queries::queries as file_query;
    use crate::scraper::financial_data::get_financial_data::{
        self as stock_scraper, calc_historical_price, split_date, print_history_price
    };

    pub async fn historical_price(
        symbol: &str, day: i32, month: i32, year: i32
    ) -> Result<f64, reqwest::Error> {
        let price = calc_historical_price(symbol, day, month, year).await;

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
    
                print_expl(expl)
            }
            Command::Search { stock_name } => {
                let file = File::open("config/stocks.txt").unwrap();

                file_query::search(file, stock_name.to_lowercase())
            }
            Command::Drop { stock_name } => {
                let file = File::open("config/stocks.txt").unwrap();
                
                file_query::drop(file, stock_name)
            }
            Command::Update { stock_name } => {
                let file = File::open("config/stocks.txt").unwrap();

                file_query::update(file, stock_name).await
            }
            Command::UpdateAll {} => {
                let file = File::open("config/stocks.txt").unwrap();

                file_query::update_all(file).await
            }
            Command::History { stock_name, date } => {
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
                    return Ok(())
                }
    
                let splitted_date = split_date(date);

                let current_price = stock_row.split(" ").collect::<Vec<&str>>()[2].parse::<f64>().unwrap();

                print_history_price(stock_name, splitted_date, current_price).await
            }
            Command::Init {} => init_mode().await,
            Command::ShowDB { .. } => println!("This command is only available if mode is set to database."),
            Command::SetDB { .. } => println!("This command is only available if mode is set to database."),
            Command::Mode {} => println!("File")
        }
    
        Ok(())
    }
}