pub mod get_financial_data {
    use scraper::Html;
    use regex::Regex;
    use chrono::prelude::*;
    use serde::Deserialize;
    use tokio::task;

    use crate::{
        errors::error_handler::error_handler::YahooError as YahooError, 
        file::cmd::cmd::historical_price, 
        parse_date
    };

    #[derive(Deserialize, Debug)]
    pub struct Response {
        pub chart: RespResult,
    }

    #[derive(Deserialize, Debug)]
    pub struct RespResult {
        pub result: Vec<QuoteBlock>,
    }

    #[derive(Deserialize, Debug)]
    pub struct QuoteBlock {
        pub indicators: Quote,
    }

    #[derive(Deserialize, Debug)]
    pub struct Quote {
        pub quote: Vec<QuoteList>,
    }

    #[derive(Deserialize, Debug)]
    pub struct QuoteList {
        pub close: Vec<Option<f64>>,
    }

    pub struct Url {
        pub symbol: String,
    }

    impl Url {
        pub async fn scrapped_home(&self) -> Html {
            let url = format!(
                "https://finance.yahoo.com/quote/{}?p={}&.tsrc=fin-srch", self.symbol, self.symbol
            );

            let response = task::spawn_blocking(move || {
                reqwest::blocking::get(url).unwrap().text().unwrap()
            }).await.unwrap();
            
            scraper::Html::parse_document(&response)
        }

        pub async fn scrapped_statistics(&self) -> Html {
            let url = format!(
                "https://finance.yahoo.com/quote/{}/key-statistics?p={}", 
                self.symbol, self.symbol
            );

            let response = task::spawn_blocking(move || {
                reqwest::blocking::get(url).unwrap().text().unwrap()
            }).await.unwrap();
    
            scraper::Html::parse_document(&response)
        }
    }

    #[derive(Clone)]
    pub struct StockData {
        pub symbol: String,
        pub url: Html,
    }

    impl StockData {
        pub fn trailing_eps(&self) -> f64 {
            key_value_from_summary(
                &self.url, 
                "td[data-test='EPS_RATIO-value']"
            ).parse::<f64>().unwrap()
        }

        pub fn pe_ratio(&self) -> f64 {
            key_value_from_summary(
                &self.url, 
                "td[data-test='PE_RATIO-value']"
            ).parse::<f64>().unwrap()
        }

        pub fn current_price(&self) -> f64 {
            key_value_from_summary(
                &self.url, 
                "fin-streamer[data-test='qsp-price']"
            ).parse::<f64>().unwrap()
        }
        
        pub fn change_since(&self) -> String {
            let change_in_percentage = key_value_from_summary(
                &self.url, 
                "div[id='quote-header-info'] fin-streamer[data-field='regularMarketChangePercent'] span"
            );

            let string_date = &Local::now().format("%d.%m.%Y").to_string();

            change_in_percentage + " " + string_date
        }

        pub fn market_cap(&self) -> String {
            key_value_from_summary(&self.url, "td[data-test='MARKET_CAP-value']")
        }

        pub fn debt_equity_ratio(&self) -> f64 {
            key_value_from_statistics(
                &self.url, 
                "Total Debt/Equity", 
                "Total Debt/Equity"
            ).unwrap().parse::<f64>().unwrap()
        }

        pub fn price_to_book(&self) -> f64 {
            key_value_from_statistics(
                &self.url, 
                "Price/Book", 
                "Price/Book"
            ).unwrap().parse::<f64>().unwrap()
        }

        pub fn peg_ratio(&self) -> f64 {
            key_value_from_statistics(
                &self.url, 
                "PEG Ratio (5 yr expected)", 
                "PEG ratio"
            ).unwrap().parse::<f64>().unwrap()
        }

        pub fn revenue(&self) -> String {
            key_value_from_statistics(
                &self.url, 
                "Revenue</span> <!-- -->(ttm)", 
                "Revenue (ttm)"
            ).unwrap()
        }

        pub fn gross_profit(&self) -> String {
            key_value_from_statistics(
                &self.url, 
                "Gross Profit</span> <!-- -->(ttm)", 
                "Guess Profit (ttm)"
            ).unwrap()
        }

        pub fn total_cash(&self) -> String {
            key_value_from_statistics(
                &self.url, 
                "Total Cash</span> <!-- -->(mrq)", 
                "Total Cash (mrq)"
            ).unwrap()
        }

        pub fn total_debt(&self) -> String {
            key_value_from_statistics(
                &self.url, 
                "Total Debt</span> <!-- -->(mrq)", 
                "Total Debt (mrq)"
            ).unwrap()
        }

        pub fn return_on_equity(&self) -> String {
            let roe = key_value_from_statistics(
                &self.url, 
                "Return on Equity", 
                "Return on Equity (ttm)"
            ).unwrap();

            roe + "%"
        }

        pub fn return_on_assets(&self) -> String {
            let roe = key_value_from_statistics(
                &self.url, 
                "Return on Assets", 
                "Return on Assets (ttm)"
            ).unwrap();
            
            roe + "%"
        }

        pub fn bvps(&self) -> f64 {
            key_value_from_statistics(
                &self.url, "Book Value Per Share", "Book Value Per Share (mrq)"
            ).unwrap().parse::<f64>().unwrap()
        }
    }

    fn key_value_from_summary(url: &Html, stock_key: &str) -> String {
        let selector = scraper::Selector::parse(stock_key).unwrap();
        url.select(&selector).next().unwrap().inner_html()
    }

    fn key_value_from_statistics(
           url: &Html, name_to_scrape: &str, error_message: &str
       ) -> Result<String, YahooError> {
        let selector = match scraper::Selector::parse(r#"tr"#) {
            Ok(selector) => selector,
            Err(_) => return Err(YahooError::ParseError { value: name_to_scrape.to_string() })
        };

        let html: String = url.select(&selector)
                            .map(|x| x.inner_html())
                            .filter(|data| data.contains(name_to_scrape))
                            .collect();

        let val = match Regex::new(r"\d+\.\d?.[A-Z]?") {
            Ok(val) => val.captures(&html),
            Err(_) => return Err(YahooError::RegexError { value: name_to_scrape.to_string() })
        };

        let value = match val {
            Some(n) => n[0].to_string(),
            None => {
                println!("Error happened when trying to get '{}', so this will be displayed as 0.0.", error_message);
                "0.0".to_string()
            },
        };

        Ok(value)
    }

    pub async fn calc_historical_price(symbol: &str, day: i32, month: i32, year: i32) -> f64 {
        let start = Utc.with_ymd_and_hms(
            year, month as u32, day as u32, 0, 0, 0)
        .unwrap().timestamp();
        
        let end = Utc.with_ymd_and_hms(
            year, month as u32, day as u32, 23, 59, 59)
        .unwrap().timestamp();

        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?symbol={}&period1={}&period2={}&interval=1d",
            symbol, symbol, start, end
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

        price
    }

    pub fn split_date(mut date: String) -> Vec<i32> {
        if date.contains("day") || 
            date.contains("week") ||
            date.contains("month") ||
            date.contains("year") {
            date = parse_date(date)
        }
            
        let splitted_date: Vec<i32> = date.split(".")
            .into_iter()
            .map(|d| d.parse::<i32>().unwrap())
            .collect();
    
        vec![splitted_date[0], splitted_date[1], splitted_date[2]]
    }
    
    pub fn format_date(mut splitted_date: Vec<i32>) -> Result<chrono::NaiveDate, &'static str> {
        let mut parsed_date = NaiveDate::from_ymd_opt(
            splitted_date[2], 
            splitted_date[1].try_into().unwrap(), 
            splitted_date[0].try_into().unwrap()
        ).unwrap();
    
        if parsed_date > Local::now().date_naive() {
            return Err("The entered date lies in the future. Please provide a date from the past.");
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
    
        Ok(parsed_date)
    }
    
    pub async fn print_history_price(symbol: String, splitted_date: Vec<i32>, current_price: f64) {
        let splitted_date_cloned = splitted_date.clone();
    
        match format_date(splitted_date_cloned) {
            Ok(parsed_date) => {
                let stock_name_cloned = symbol.clone();
        
                let price_history = historical_price(
                    symbol.as_str(), splitted_date[0], splitted_date[1], splitted_date[2]
                ).await;
    
                match price_history {
                    Ok(price) => {
                        if price == 0.0 {
                            return println!("Please take another day.")
                        }
    
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
            },
            Err(e) => println!("{}", e)
        }
    }
}
