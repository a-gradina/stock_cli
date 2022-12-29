# Stock CLI
A CLI tool written in Rust to save information of stocks to the local postgres database or a simple text file. There's also the option to display definitions of certain financial terms such as Price-to-Earnings Ratio, Equity, Price-to-Book Ratio and many more.
It is also possible to display how much a stock was worth in the past.

Stock infos are scraped from Yahoo Finance and history prices are called from a Yahoo API.

# Requirements
You need to have rust installed. Furthermore if you want to save stocks data into your database, you'll need postgres as well.

# Usage
As mentioned earlier, there's two ways to save data. Either to your local postgres database or a text file (`config/stocks.txt`).

Run `cargo init` and follow the instructions.

If you chose `file` during `init`, `stocks.txt` is created in `config`. If you chose `database`, `database.yml` is created in `config`.

You can change `mode` whenever you want. Just run `init` again.

Inside `database.yml` file, your database URL is stored. You can always check on it with `cargo run show-db` or change it with `cargo run set-db YOUR-NEW-URL`. Though beware that `show-db` and `set-db` can only be run when `mode` is set to `file`.

After a `init`, you can `add`, `search`, `delete`, `update` stocks. You can also `list` all of your stocks and `update-all` all of them.

Let's presume you want to add the Apple stock to your database. Run `cargo run add aapl` to add it. To show its data, run `cargo run search aapl`.
It is important that you provide the ticker symbol of the stock, not the name of the company itself.

If you want to display the stock price from a date in the past, run `cargo run history STOCK-SYMBOL-YOUT-WANT-TO-DISPLAY D.M.YYYY`. If you don't fancy typing a date, you can instead type `NUMBER.DAYS/WEEKS/MONTHS/YEARS.ago`.

For definitions of various financial terms, for instance Equity, run `cargo run info equity`. If you want a list of all the terms available, run `cargo run info`.

# License
MIT 