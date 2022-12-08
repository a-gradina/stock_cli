# Stock CLI
A CLI tool written in Rust to save information of stocks to the local postgres database. There's also the option to display definitions of certain financial terms such as Price-to-Earnings Ratio, Equity, Price-to-Book Ratio and many more.
It is also possible to display how much a stock was worth in the past.

Stock infos are scraped from Yahoo Finance and history prices are called from a Yahoo API.

# Requirements
You need to have postgres and rust installed.

# Usage
At first you want to create a database.
Run `cargo run init` and follow the instructions. When it's done, `database.yml` gets created in the `config` folder. Inside the `yml` file, your database URL is stored. You can always check on it with `cargo run show-db` or change it with `cargo run set-db YOUR-NEW-URL`.

After a database connection has been established, you can `add`, `search`, `delete`, `update` stocks. You can also `list` all of your stocks and `update-all` all of them.

If you want to display the stock price from a date in the past, run `cargo run history STOCK-SYMBOL-YOUT-WANT-TO-DISPLAY D.M.YYYY`. If you don't fancy typing a date, you can instead type `NUMBER.DAYS/WEEKS/MONTHS/YEARS.ago`.

For definitions of various financial terms, for instance Equity, run `cargo run info equity`. If you want a list of all the terms available, run `cargo run info`.

# License
MIT 