pub mod get_financial_data {

    use scraper::Html;
    use regex::Regex;
    use chrono::prelude::*;
    use serde::Deserialize;
    use tokio::task;

    use crate::errors::error_handler::error_handler::YahooError as YahooError;

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

        pub async fn historical_price(
                &self, day: i32, month: i32, year: i32)
            -> Result<f64, reqwest::Error> {
            let start = Utc.with_ymd_and_hms(
                year, month as u32, day as u32, 0, 0, 0)
            .unwrap().timestamp();
            
            let end = Utc.with_ymd_and_hms(
                year, month as u32, day as u32, 23, 59, 59)
            .unwrap().timestamp();

            let url = format!(
                "https://query1.finance.yahoo.com/v8/finance/chart/{}?symbol={}&period1={}&period2={}&interval=1d",
                self.symbol, self.symbol, start, end
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
}
