pub mod queries {
    use std::{io::{Write, Read, BufReader}, fs::File};

    use crate::scraper::financial_data::get_financial_data as stock_scraper;

    pub fn list(file: std::fs::File) {
        let contents = file_to_string(&file);

        for row in contents.split(";") {
            let fields = row.split(",").collect::<Vec<&str>>();
            let symbol = fields[0].replace("\n", "");

            println!("{}", symbol.to_uppercase());
        }
    }

    pub async fn insert(mut file: &std::fs::File, name: String, update: bool) {
        let url = stock_scraper::Url { symbol: name.clone() };
        
        let uri = url.scrapped_home().await;
        let uri_statistic = url.scrapped_statistics().await;

        let stock_data = stock_scraper::StockData { symbol: name.clone(), url: uri };
        let stock_data_statistic = stock_scraper::StockData { symbol: name.clone(), url: uri_statistic };

        let name = format!("{},", &name);
        let current_price = format!("Current Price: {} {},", &stock_data.current_price(), &stock_data.change_since());
        let trailing_eps = format!("EPS: {},", &stock_data.trailing_eps());
        let pe = format!("P/E Ratio: {},", &stock_data.pe_ratio());
        let debt_equity = format!("Debt to Equity Ratio: {},", &stock_data_statistic.debt_equity_ratio());
        let market_cap = format!("Market Cap: {},", &stock_data.market_cap());
        let peg = format!("PEG Ratio: {},", &stock_data_statistic.peg_ratio());
        let price_to_book = format!("Price to Book: {},", &stock_data_statistic.price_to_book());
        let revenue = format!("Revenue: {},", &stock_data_statistic.revenue());
        let gross_profit = format!("Gross Profit: {},", &stock_data_statistic.gross_profit());
        let total_cash = format!("Total Cash: {},", &stock_data_statistic.total_cash());
        let total_debt = format!("Total Debt: {},", &stock_data_statistic.total_debt());
        let return_on_equity = format!("Return on Equity: {},", &stock_data_statistic.return_on_equity());
        let return_on_assets = format!("Return on Assets: {},", &stock_data_statistic.return_on_assets());
        let bvps = format!("Book Value per Share: {};", &stock_data_statistic.bvps());

        let cloned_name = name.clone();

        let mut row = name + current_price.as_str() + trailing_eps.as_str() + pe.as_str() + 
            debt_equity.as_str() + market_cap.as_str() + peg.as_str() +
            price_to_book.as_str() + revenue.as_str() + gross_profit.as_str() + 
            total_cash.as_str() + total_debt.as_str() + return_on_equity.as_str() + 
            return_on_assets.as_str() + bvps.as_str();

        if update {
            match file.write_all(row.as_bytes()) {
                Ok(_) => println!("Stock {} was updated!", cloned_name.split(",").collect::<Vec<&str>>()[0].to_uppercase()),
                Err(e) => {
                    println!("Error occurred when updating stock.");
                    println!("Error: {}", e);
                }
            }
        } else {
            row = row + "\n";
            match file.write_all(row.as_bytes()) {
                Ok(_) => println!("Stock was added!"),
                Err(e) => {
                    println!("Error occurred when adding stock.");
                    println!("Error: {}", e);
                }
            }
        }

    }

    pub fn drop(file: std::fs::File, name: String)  {
        let contents = file_to_string(&file);

        let mut file = File::create("config/stocks.txt").unwrap();

        file.write_all("".as_bytes()).unwrap();
        
        let mut symbol_found = false;

        for row in contents.split(";") {
            let fields = row.split(",").collect::<Vec<&str>>();
            let symbol = fields[0].replace("\n", "");
            
            let row = row.to_string() + ";";

            if symbol != name && symbol != "" {
                file.write_all(row.as_bytes()).unwrap();
            } else if symbol == name {
                symbol_found = true;
            }
        }

        file.write_all("\n".as_bytes()).unwrap();

        if symbol_found {
            println!("Stock was deleted!");
        } else {
            println!("Stock was not found.");
        }
    }

    pub async fn update(file: std::fs::File, name: String) {
        let contents = file_to_string(&file);

        let mut file = File::create("config/stocks.txt").unwrap();

        file.write_all("".as_bytes()).unwrap();

        let mut symbol_found = false;
        let mut stock_to_update_is_first = false;        

        for (i, row) in contents.split(";").enumerate() {
            let fields = row.split(",").collect::<Vec<&str>>();
            let symbol = fields[0].replace("\n", "");

            let row = row.to_string() + ";";

            if symbol != name && symbol != "" {
                if stock_to_update_is_first {
                    let (_, formatted_row) = row.split_at(1);
                    file.write_all(formatted_row.as_bytes()).unwrap();
                    stock_to_update_is_first = false
                } else {
                    file.write_all(row.as_bytes()).unwrap();
                }
            } else if symbol == name {
                if i == 0 {
                    stock_to_update_is_first = true
                }

                symbol_found = true;
            }
        }

        if symbol_found {
            file.write_all("\n".as_bytes()).unwrap();
            insert(&file, name, true).await;
        } else {
            println!("Stock was not found.");
        }

        file.write_all("\n".as_bytes()).unwrap();
    }

    pub async fn update_all(file: std::fs::File) {
        let contents = file_to_string(&file);

        let mut file = File::create("config/stocks.txt").unwrap();

        file.write_all("".as_bytes()).unwrap();

        for row in contents.split(";") {
            let fields = row.split(",").collect::<Vec<&str>>();
            let symbol = fields[0].replace("\n", "");

            if symbol != "" {
                insert(&file, symbol, true).await;
                file.write_all("\n".as_bytes()).unwrap();
            } 
        }

        println!("Updating of stocks done!");
    }

    pub fn search(file: std::fs::File, name: String) {
        let contents = file_to_string(&file);

        for row in contents.split(";") {
            let fields = row.split(",").collect::<Vec<&str>>();
            let symbol = fields[0].replace("\n", "");

            if symbol == name {
                println!("Stock: {}", name.to_uppercase());
                for field in fields.iter().skip(1) {

                    println!("  - {}", field);
                }
            }
        }
    }

    pub fn file_to_string(file: &std::fs::File) -> String {
        let mut contents = String::new();
        let mut buf_reaer = BufReader::new(file);
        buf_reaer.read_to_string(&mut contents).unwrap();

        contents
    }
}
