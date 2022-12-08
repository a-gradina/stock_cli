pub mod queries {
    use tokio_postgres::{Client, Error, types::Type};

    use crate::scraper::financial_data::get_financial_data as stock_scraper;

    #[derive(Debug, Clone)]
    pub struct Stock {
        pub name: String,
        pub current_price: f64,
        eps_ttm: f64,
        pe_ratio: f64,
        total_debt_equity: f64,
        change_since: String,
        market_cap: String,
        peg_ratio: f64,
        price_to_book: f64,
        revenue: String,
        gross_profit: String,
        total_cash: String,
        total_debt: String,
        return_on_equity: String,
        return_on_assets: String,
        bvps: f64,
    }

    impl Default for Stock {
        fn default() -> Stock {
            Stock {
                name: "".to_string(), current_price: 0.0, eps_ttm: 0.0, pe_ratio: 0.0,
                total_debt_equity: 0.0, change_since: "".to_string(), market_cap: "".to_string(),
                peg_ratio: 0.0, price_to_book: 0.0, revenue: "".to_string(), gross_profit: "".to_string(),
                total_cash: "".to_string(), total_debt: "".to_string(), return_on_equity: "".to_string(),
                return_on_assets: "".to_string(), bvps: 0.0,
            }
        }
    }

    pub async fn list(client: &mut Client) -> Result<(), Error> {
        for row in client.query("SELECT * FROM stocks", &[]).await? {
            let stock_name: String = row.get(1);
            println!("{:?}", stock_name);
        }

        Ok(())
    }

    pub async fn insert(client: &mut Client, name: String) -> Result<u64, Error> {
        let statement = client.prepare_typed(
            "INSERT INTO stocks 
            (NAME, CURRENT_PRICE, EPS_TTM, PE_RATIO, 
            TOTAL_DEBT_EQUITY, CHANGE_SINCE, MARKET_CAP,
            PEG_RATIO, PRICE_TO_BOOK, REVENUE, GROSS_PROFIT,
            TOTAL_CASH, TOTAL_DEBT, RETURN_ON_EQUITY,
            RETURN_ON_ASSETS, BVPS) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 
            $9, $10, $11, $12, $13, $14, $15, $16)",
            &[Type::VARCHAR, Type::FLOAT8, Type::FLOAT8, 
                Type::FLOAT8, Type::FLOAT8, Type::VARCHAR,
                Type::VARCHAR, Type::FLOAT8, Type::FLOAT8,
                Type::VARCHAR, Type::VARCHAR, Type::VARCHAR,
                Type::VARCHAR, Type::VARCHAR, Type::VARCHAR,
                Type::FLOAT8],
        ).await?;

        let url = stock_scraper::Url { symbol: name.clone() };
        
        let uri = url.scrapped_home().await;
        let uri_statistic = url.scrapped_statistics().await;

        let stock_data = stock_scraper::StockData { symbol: name.clone(), url: uri };
        let stock_data_statistic = stock_scraper::StockData { symbol: name.clone(), url: uri_statistic };

        client.execute(&statement, 
            &[
                &name, 
                &stock_data.current_price(), 
                &stock_data.trailing_eps(), 
                &stock_data.pe_ratio(),
                &stock_data_statistic.debt_equity_ratio(), 
                &stock_data.change_since(),
                &stock_data.market_cap(),
                &stock_data_statistic.peg_ratio(),
                &stock_data_statistic.price_to_book(),
                &stock_data_statistic.revenue(),
                &stock_data_statistic.gross_profit(),
                &stock_data_statistic.total_cash(),
                &stock_data_statistic.total_debt(),
                &stock_data_statistic.return_on_equity(),
                &stock_data_statistic.return_on_assets(),
                &stock_data_statistic.bvps(),
            ]
        ).await
    }

    pub async fn drop(client: &mut Client, name: String) -> Result<u64, Error> {
        let statement = client.prepare_typed(
            "DELETE FROM stocks WHERE name = $1",
            &[Type::VARCHAR],
        ).await?;

        client.execute(&statement, &[&name]).await
    }

    pub async fn update(client: &mut Client, name: String) -> Result<u64, Error> {
        let statement = client.prepare_typed(
            "UPDATE stocks SET 
            current_price = $2, 
            eps_ttm = $3, 
            pe_ratio = $4, 
            total_debt_equity = $5,
            change_since = $6,
            market_cap = $7,
            peg_ratio = $8,
            price_to_book = $9,
            revenue = $10,
            gross_profit = $11,
            total_cash = $12,
            total_debt = $13,
            return_on_equity = $14,
            return_on_assets = $15,
            bvps = $16 WHERE name = $1",
            &[Type::VARCHAR, Type::FLOAT8, Type::FLOAT8, 
                Type::FLOAT8, Type::FLOAT8, Type::VARCHAR,
                Type::VARCHAR, Type::FLOAT8, Type::FLOAT8,
                Type::VARCHAR, Type::VARCHAR, Type::VARCHAR,
                Type::VARCHAR, Type::VARCHAR, Type::VARCHAR,
                Type::FLOAT8],
        ).await?;
        
        let url = stock_scraper::Url { symbol: name.clone() };
        
        let uri = url.scrapped_home().await;
        let uri_statistic = url.scrapped_statistics().await;
        
        let stock_data = stock_scraper::StockData { symbol: name.clone(), url: uri };
        let stock_data_statistic = stock_scraper::StockData { symbol: name.clone(), url: uri_statistic };
        
        client.execute(&statement, 
            &[
                &name, 
                &stock_data.current_price(), 
                &stock_data.trailing_eps(), 
                &stock_data.pe_ratio(), 
                &stock_data_statistic.debt_equity_ratio(), 
                &stock_data.change_since(),
                &stock_data.market_cap(),
                &stock_data_statistic.peg_ratio(),
                &stock_data_statistic.price_to_book(),
                &stock_data_statistic.revenue(),
                &stock_data_statistic.gross_profit(),
                &stock_data_statistic.total_cash(),
                &stock_data_statistic.total_debt(),
                &stock_data_statistic.return_on_equity(),
                &stock_data_statistic.return_on_assets(),
                &stock_data_statistic.bvps(),
            ]
        ).await
    }

    pub async fn update_all(client: &mut Client) {
        for row in client.query("SELECT name FROM stocks", &[]).await.unwrap() {
            let stock = Stock {
                name: row.get(0),
                current_price: 0.0,
                eps_ttm: 0.0,
                pe_ratio: 0.0,
                total_debt_equity: 0.0,
                change_since: "".to_string(),
                market_cap: "".to_string(),
                peg_ratio: 0.0,
                price_to_book: 0.0,
                revenue: "".to_string(),
                gross_profit: "".to_string(),
                total_cash: "".to_string(),
                total_debt: "".to_string(),
                return_on_equity: "".to_string(),
                return_on_assets: "".to_string(),
                bvps: 0.0,
            };

            match update(client, stock.clone().name).await {
                Ok(result) => {
                    if result == 1 {
                        println!("Stock updated: {}", stock.name)
                    } else if result == 0 {
                        println!("Stock could not be updated: {}", stock.name)
                    }
                },
                Err(e) => println!("Error: {}.", e),
            }
        };
        println!("Updating finished!")
    }

    pub async fn search(client: &mut Client, name: String, print: bool) -> Result<Stock, Error> {
        let mut stock = Stock::default();
        for row in client.query("SELECT * FROM stocks WHERE name = $1", &[&name]).await? {
            stock.name = row.get(1);
            stock.current_price = row.get(2);
            stock.eps_ttm = row.get(3);
            stock.pe_ratio = row.get(4);
            stock.total_debt_equity = row.get(5);
            stock.change_since = row.get(6);
            stock.market_cap = row.get(7);
            stock.peg_ratio = row.get(8);
            stock.price_to_book = row.get(9);
            stock.revenue = row.get(10);
            stock.gross_profit = row.get(11);
            stock.total_cash = row.get(12);
            stock.total_debt = row.get(13);
            stock.return_on_equity = row.get(14);
            stock.return_on_assets = row.get(15);
            stock.bvps = row.get(16);

            if print {
                println!("Stock: {}", stock.name.to_uppercase());
                println!("  - Current Price: {} {}", stock.current_price, stock.change_since);
                println!("  - Market Cap: {}", stock.market_cap);
                println!("  - EPS (ttm): {}", stock.eps_ttm);
                println!("  - P/E: {}", stock.pe_ratio);
                println!("  - PEG ratio: {}", stock.peg_ratio);
                println!("  - Price/Book (mrq): {}", stock.price_to_book);
                println!("  - Book Value per Share (mrq): {}", stock.bvps);
                println!("  - Revenue (ttm): {}", stock.revenue);
                println!("  - Gross Profit (ttm): {}", stock.gross_profit);
                println!("  - Total Cash (mrq): {}", stock.total_cash);
                println!("  - Total Debt (mrq): {}", stock.total_debt);
                println!("  - Total Debt/Equity: {}", stock.total_debt_equity);
                println!("  - Return on Equity (ttm): {}", stock.return_on_equity);
                println!("  - Return on Assets (ttm): {}", stock.return_on_assets);
            }
        }

        if client.query("SELECT * FROM stocks WHERE name = $1", &[&name]).await.unwrap().is_empty() {
            println!("Stock was not found")
        }

        Ok(stock)
    }

    pub async fn stock_exists(client: &mut Client, name: String) -> bool {
        let stock_found = client.query(
            "SELECT * FROM stocks WHERE name = $1", &[&name]
        ).await.unwrap().len();

        stock_found > 0
    }
}
