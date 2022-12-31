pub mod cmd {
    use tokio_postgres::{Client, Error};

    use crate::database::database::database::{read_database_url, set_database_url};
    use crate::fundamentals::explanations::print_expl;
    use crate::{Opt, Command, init_mode};
    use crate::database::queries::queries as database_query;
    use crate::scraper::financial_data::get_financial_data::{self as stock_scraper, split_date, print_history_price};

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
    
                print_expl(expl)
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
            Command::History { stock_name, date } => {
                let stock = database_query::search(&mut client, stock_name.clone(), false);
    
                let splitted_date = split_date(date);

                let current_price = stock.await.unwrap().current_price;

                print_history_price(stock_name, splitted_date, current_price).await
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