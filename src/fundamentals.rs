pub mod explanations {

    pub struct PERatio {}
    impl PERatio {
        pub fn info(&self) {
            println!("=============");
            println!("The P/E Ratio compares the price per share of the stock with the company's ");
            println!("EPS(earnings per share), which is calculated like this:\n");
            println!("- Stock price / EPS.\n");
            println!("The lower, the better.");
            println!("The P/E ratio of any company that's failry priced will equal its growth");
            println!("rate. If a given company's P/E is 15, you'd expect the company to be growing at");
            println!("=============");
        }
    }

    pub struct Equity {}
    impl Equity {
        pub fn info(&self) {
            println!("=============");
            println!("Equity, typically referred to as shareholder's equity, represents the amount");
            println!("of money that would be returned to a company's shareholders if all of the");
            println!("assets were liquidated and all of the company's debt was paid off in the");
            println!("case of liquidation.\n");
            println!("Equity is calculated like this:\n");
            println!("- Total Assets - Total Liabilities");
            println!("The closer the equity is to the market price, the safer the investment.");
            println!("=============");
        }
    }

    pub struct MarketValue {}
    impl MarketValue {
        pub fn info(&self) {
            println!("=============");
            println!("Commonly referred to as 'Market Cap'.");
            println!("Market value can be calculated like this:\n");
            println!("- Current Share Price / Total Number of Outstanding Shares ");
            println!("=============");
        }
    }

    pub struct PBRatio {}
    impl PBRatio {
        pub fn info(&self) {
            println!("=============");
            println!("For every $'P/B' paid for this, the company has $1 in Book Value.");
            println!("P/B ratios under 1 are typically considered solid investments.");
            println!("P/B ratio is calculated like this:\n");
            println!("- Stock price / Book Value\n");
            println!("For instance if you buy a stock for $20 and its P/B ratio is 1");
            println!("and you sell it, you get 100% for it.");
            println!("Beware though, if the P/B ratio is very low, the earnings are often also very low.");
            println!("");
            println!("=============");
        }
    }

    pub struct PEGRatio {}
    impl PEGRatio {
        pub fn info(&self) {
            println!("=============");
            println!("PEG ratio stands for Price-to-Earnings-to-Growth ratio.");
            println!("It is calculated like this:\n");
            println!("- P/E ratio / company's earnings growth rate");
            println!("To interpret the ratio, a result of 1 or lower says that the stock");
            println!("is either at par or undervalued, based on its growth rate.\n");
            println!("To take a step further, there's the 'Dividend-Adjusted PEG ratio'.");
            println!("This is particulary important when investing in blue-chip stocks");
            println!("as well as in certain specialty enterprises such as the major oil company stocks.");
            println!("Reinvested dividends, especially during stock market crashes, can");
            println!("create what one may refer to as a 'return accelerator' drastically");
            println!("shortening the time it takes to recover losses.\n");
            println!("If you buy a stock at 19 times earnings that's growing at only 6%,");
            println!("it may look expensive. However, if it's distributing a sustainable");
            println!("8& dividend, that's clearly a much better deal.");
            println!("This is calculated like this:\n");
            println!("- P/E ratio / (earnings growth + dividend yield)");
            println!("=============");
        }
    }

    pub struct DebtEquityRatio {}
    impl DebtEquityRatio {
        pub fn info(&self) {
            println!("=============");
            println!("Total Debt divided by Total Equity equals the Debt to Equity ratio.");
            println!("Anything is considered debt that shows up as 'debt' in the balance sheet.");
            println!("For a more conservative approach, take 'Total Liabilities' as Total Debt.\n");
            println!("The lower, the better.");
            println!("=============");
        }
    }

    pub struct CurrentRatio {}
    impl CurrentRatio {
        pub fn info(&self) {
            println!("=============");
            println!("Gives an idea how the company will handle debt in the next 12 months.");
            println!("It compares the current assets to the current liabilities.");
            println!("The higher, the better. > 1.50 is good.");
            println!("=============");
        }
    }

    pub struct Assets {}
    impl Assets {
        pub fn info(&self) {
            println!("=============");
            println!("Total Current Assets vs. Total Assets");
            println!("When a company lists something under 'Total Current Assets', they'll");
            println!("likely convert anything under this to cash during the next 12 months.\n");
            println!("'Total Current Assets' should be higher than 'Total Current Liabilities'.");
            println!("=============");
        }
    }

    pub struct Liabilities {}
    impl Liabilities {
        pub fn info(&self) {
            println!("=============");
            println!("Total Current Liabilities vs. Total Liabilities ");
            println!("Anything under 'Total Current Liabilities' will likely be paid off during the next 12 months.\n");
            println!("'Total Current Assets' should be higher than 'Total Current Liabilities'.");
            println!("=============");
        }
    }

    pub struct CashFlowStatement {}
    impl CashFlowStatement {
        pub fn info(&self) {
            println!("=============");
            println!("The cash flow statement shows where the money is being spent, generated and employed.\n");
            println!("The components are:");
            println!("- Operating Activities (all the money that was being generated. This is the most important activity.)");
            println!("- Investing Activities (investing type acttivites - buying buildings, more supplies purchase stocks of other companies etc.)");
            println!("- Financing Activities (issuing more shares, bonds)\n");
            println!("For instance, the balance might show a very strong net income, but");
            println!("the money was being generated through selling stocks or issuing bonds.");
            println!("Also, if the company purchased a public traded company recently and ");
            println!("the purchased company pays dividends to the owner. This is not listed");
            println!("in the income statement, but in the cash flow statement.");
            println!("=============");
        }
    }

    pub struct IncomeInvesting {}
    impl IncomeInvesting {
        pub fn info(&self) {
            println!("=============");
            println!("Income Investing is a strategy which aims to get a continuous flow");
            println!("through dividends, and not selling your stocks.");
            println!("Ideally, the payed dividend should be higher than inflation rate,");
            println!("so you don't lose purchasing power.\n");
            println!("A reasonable option is to find a company that pays 1/3 of their");
            println!("earnings through dividend and invest the other 2/3 into book");
            println!("value growth of the business (you don't pay taxes on this growth).");
            println!("=============");
        }
    }

    pub struct OperatingActivities {}
    impl OperatingActivities {
        pub fn info(&self) {
            println!("=============");
            println!("This number should be positive and show a steady growth.");
            println!("=============");
        }
    }

    pub struct InvestingActivities {}
    impl InvestingActivities {
        pub fn info(&self) {
            println!("=============");
            println!("Numbers like capital expenditures and buying/selling assets will show up in this one.");
            println!("Beware if a company sells too much of their assets during a short period of time.");
            println!("Should be negative because the company invests.");
            println!("=============");
        }
    }

    pub struct FinancingActivities {}
    impl FinancingActivities {
        pub fn info(&self) {
            println!("=============");
            println!("This number should be negative. If a company, for instance, buys stock back,");
            println!("pays dividends or pays off its debt, it will show as negative. If the amount of cash from financing");
            println!("activities is positive, this means that the company either didn't pay any dividends, sold stock or took some debt.");
            println!("");
            println!("=============");
        }
    }

    pub struct IssuanceOfStock {}
    impl IssuanceOfStock {
        pub fn info(&self) {
            println!("=============");
            println!("When this number is negative, it means that the company bought stock back.");
            println!("=============");
        }
    }

    pub struct ReturnOnEquity {}
    impl ReturnOnEquity {
        pub fn info(&self) {
            println!("=============");
            println!("Return on Equity (ROE) is considered a benchmark of a corporation's profitability");
            println!("and how efficient it is in generating profits. The higher the ROE, the more");
            println!("efficient a company's management is at generating income and growth from its equity financing.\n");
            println!("It's a measure of company's net income divided by its shareholder's equity.\n");
            println!("A ROE that's too high can also be bad. For instance, if a company has been");
            println!("borrowing aggressively because equity is equal to assets minus debts.");
            println!("The more debt a company has, the lower equity can fall.");
            println!("=============");
        }
    }

    pub struct ReturnOnAssets {}
    impl ReturnOnAssets {
        pub fn info(&self) {
            println!("=============");
            println!("Return on Assets indicates how profitable a company is in relation to its total assets.");
            println!("You can use the ROW to determine how efficiently a company uses its assets to generate profit.\n");
            println!("The metric is commonly expressed as a percentage by using a company's net income divided");
            println!("by its total assets. A higher ROA means a company is more efficient and productive");
            println!("at managing its balance sheet to generate profits.\n");
            println!("Both ROA and ROE measure how well a company utilizes its resources. But one of the key");
            println!("differences between the two is how they each treat a company's debt. ROA factors in how");
            println!("leveraged a company is or how much debt it carries. After all, its total assets include");
            println!("any capital it borrows to run its operations.\n");
            println!("On the other hand, ROE only measures the return on a company's equity, which leaves out");
            println!("its liabilities. Thus, ROA accounts for a company's debt and ROE doesn't. The more leverage");
            println!("and debt a company takes on, the higher ROE will be relative to ROA. Thus, as a company");
            println!("takes on more debt, its ROE would be higher than its ROA.\n");
            println!("One of the biggest issues is that ROA can't be used across industries. That's because");
            println!("companies in one industry have different asset bases than those in another.");
            println!("=============");
        }
    }

    pub struct BVPS {}
    impl BVPS {
        pub fn info(&self) {
            println!("=============");
            println!("BVPS stands for Book Value per Share. It's the ratio of equity available to common");
            println!("shareholders divided by the number of outstanding shares. This figure represents the");
            println!("minimum value of a company's equity and measures the book value of a firm on a per-share basis.\n");
            println!("BVPS can be used to gauge whether a stock price is undervalued by comparing it to the firm's");
            println!("market value per share. If the BVPS is higher than its market value per share - its");
            println!("current stock price - then the stock is considered undervalued. If the firm's BVPS");
            println!("increases, the stock should be perceived as more valuable, and the stock price should increase.\n");
            println!("In theory, BVPS is the sum that shareholders would receive in the even that the firm");
            println!("was liquidated, all of the tangible assets were sold and all of the liabilities were paid.");
            println!("However, as the assets would be sold at market prices, and book value used the historical");
            println!("costs of assets, market value is considered a better floor price than book value for a company.");
            println!("=============");
        }
    }
}