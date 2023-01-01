pub mod explanations {

    pub fn print_expl(expl: String) {
        match expl.as_str() {
            "pe_ratio" => PERatio {}.info(),
            "equity" => Equity {}.info(),
            "market_value" => MarketValue {}.info(),
            "pb_ratio" => PBRatio {}.info(),
            "bvps" => BVPS {}.info(),
            "peg_ratio" => PEGRatio {}.info(),
            "debt_equity_ratio" => DebtEquityRatio {}.info(),
            "return_on_equity" => ReturnOnEquity {}.info(),
            "return_on_assets" => ReturnOnAssets {}.info(),
            "current_ratio" => CurrentRatio {}.info(),
            "assets" => Assets {}.info(),
            "liabilities" => Liabilities {}.info(),
            "cash_flow_statement" => CashFlowStatement {}.info(),
            "income_investing" => IncomeInvesting {}.info(),
            "issuance_of_stock" => IssuanceOfStock {}.info(),
            "cash_from_operating_activities" => OperatingActivities {}.info(),
            "cash_from_financing_activities" => FinancingActivities {}.info(),
            "cash_from_investing_activities" => InvestingActivities {}.info(),
            "cost_of_capital" => CostOfCapital {}.info(),
            "discount_rate" => DiscountRate {}.info(),
            "discounted_cash_flow" => DiscountedCashFlow {}.info(),
            "net_present_value" => NetPresentValue {}.info(),
            "wacc" => WACC {}.info(),
            _ => {
                let options = vec!["equity", "market_value", "pb_ratio", "bvps", "peg_ratio", 
                    "debt_equity_ratio", "return_on_equity", "return_on_assets", "current_ratio", "assets", 
                    "liabilities", "cash_flow_statement", "income_investing", "issuance_of_stock", 
                    "cash_from_operating_activities", "cash_from_financing_activities", "cash_from_investing_activities",
                    "cost_of_capital", "discount_rate", "discounted_cash_flow", "net_present_value", "wacc",
                ];

                println!("No term was found. The following are supported:");
                for term in options {
                    println!("  - {}", term)
                }
            }
        }
    }

    struct PERatio {}
    impl PERatio {
        fn info(&self) {
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

    struct Equity {}
    impl Equity {
        fn info(&self) {
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

    struct MarketValue {}
    impl MarketValue {
        fn info(&self) {
            println!("=============");
            println!("Commonly referred to as 'Market Cap'.");
            println!("Market value can be calculated like this:\n");
            println!("- Current Share Price / Total Number of Outstanding Shares ");
            println!("=============");
        }
    }

    struct PBRatio {}
    impl PBRatio {
        fn info(&self) {
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

    struct PEGRatio {}
    impl PEGRatio {
        fn info(&self) {
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

    struct DebtEquityRatio {}
    impl DebtEquityRatio {
        fn info(&self) {
            println!("=============");
            println!("Total Debt divided by Total Equity equals the Debt to Equity ratio.");
            println!("Anything is considered debt that shows up as 'debt' in the balance sheet.");
            println!("For a more conservative approach, take 'Total Liabilities' as Total Debt.\n");
            println!("The lower, the better.");
            println!("=============");
        }
    }

    struct CurrentRatio {}
    impl CurrentRatio {
        fn info(&self) {
            println!("=============");
            println!("Gives an idea how the company will handle debt in the next 12 months.");
            println!("It compares the current assets to the current liabilities.");
            println!("The higher, the better. > 1.50 is good.");
            println!("=============");
        }
    }

    struct Assets {}
    impl Assets {
        fn info(&self) {
            println!("=============");
            println!("Total Current Assets vs. Total Assets");
            println!("When a company lists something under 'Total Current Assets', they'll");
            println!("likely convert anything under this to cash during the next 12 months.\n");
            println!("'Total Current Assets' should be higher than 'Total Current Liabilities'.");
            println!("=============");
        }
    }

    struct Liabilities {}
    impl Liabilities {
        fn info(&self) {
            println!("=============");
            println!("Total Current Liabilities vs. Total Liabilities ");
            println!("Anything under 'Total Current Liabilities' will likely be paid off during the next 12 months.\n");
            println!("'Total Current Assets' should be higher than 'Total Current Liabilities'.");
            println!("=============");
        }
    }

    struct CashFlowStatement {}
    impl CashFlowStatement {
        fn info(&self) {
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

    struct IncomeInvesting {}
    impl IncomeInvesting {
        fn info(&self) {
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

    struct OperatingActivities {}
    impl OperatingActivities {
        fn info(&self) {
            println!("=============");
            println!("This number should be positive and show a steady growth.");
            println!("=============");
        }
    }

    struct InvestingActivities {}
    impl InvestingActivities {
        fn info(&self) {
            println!("=============");
            println!("Numbers like capital expenditures and buying/selling assets will show up in this one.");
            println!("Beware if a company sells too much of their assets during a short period of time.");
            println!("Should be negative because the company invests.");
            println!("=============");
        }
    }

    struct FinancingActivities {}
    impl FinancingActivities {
        fn info(&self) {
            println!("=============");
            println!("This number should be negative. If a company, for instance, buys stock back,");
            println!("pays dividends or pays off its debt, it will show as negative. If the amount of cash from financing");
            println!("activities is positive, this means that the company either didn't pay any dividends, sold stock or took some debt.");
            println!("");
            println!("=============");
        }
    }

    struct IssuanceOfStock {}
    impl IssuanceOfStock {
        fn info(&self) {
            println!("=============");
            println!("When this number is negative, it means that the company bought stock back.");
            println!("=============");
        }
    }

    struct ReturnOnEquity {}
    impl ReturnOnEquity {
        fn info(&self) {
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

    struct ReturnOnAssets {}
    impl ReturnOnAssets {
        fn info(&self) {
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

    struct BVPS {}
    impl BVPS {
        fn info(&self) {
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

    struct CostOfCapital {}
    impl CostOfCapital {
        fn info(&self) {
            println!("=============");
            println!("Cost of capital refers t the required return necessary to make a project or investment worthwile.");
            println!("This is specifically attributed to the type of funding used to pay for the investment or project.");
            println!("If it is financed internally, it refers to the cost of equity. If externally, it's used to refer to the cost of debt.\n");
            println!("The cost of capital is the company's required return. The company's lenders and owners don't extend financing for free.");
            println!("They want to be paid for delaying their own consumption and assuming investment risk.");
            println!("The cost of capital helps establish a benchmark return that the company must achieve to satisfy its debt and equity investors.\n");
            println!("The most widely used method of calculating capital costs is the relative weight of all capital investment sources and then adjusting the required return accordingly.\n");
            println!("If a firm were financed by bonds or other loans, its cost of capital would be equal to its cost of debt.");
            println!("Conversely, if the firm were financed entirely through common or preferred stock issues, then the cost of capital would be equal to its cost of equity.");
            println!("Since most firms combine debt and equity financing, the WACC (Weighted Average Cost of Capital) helps turn the cost of debt and cost of equity into one meaningful figure.\n");
            self.info_bottom_line();
            println!("Source: https://www.investopedia.com/ask/answers/052715/what-difference-between-cost-capital-and-discount-rate.asp");
            println!("=============");
        }

        pub fn info_bottom_line(&self) {
            println!("The cost of capital and the discount rate work hand in hand to determine whether a prospective investment or project will be profitable.");
            println!("The cost of capital refers to the minimum rate of return needed from an investment to make it worthwhile, ");
            println!("whereas the discount rate is the rate used to discount the future cash flows from an investment to the present value to determine if an investment will be profitable.");
            println!("The discount rate usually takes into consideration a risk premium and therefore is usually higher than the cost of capital\n");
        }
    }

    struct DiscountRate {}
    impl DiscountRate {
        fn info(&self) {
            println!("=============");
            println!("The discount rate is the interest rate used to determine the present value of future cash flows in a discounted cash flow (DCF) analysis.");
            println!("This helps determine if the future cash flows from a project or investment will be worth more than the capital outlay needed to fund the project or investment in the present.");
            println!("The cost of capital is the minimum rate needed to justify the cost of a new venture, where the discount rate is the number that needs to meet or exceed the cost of capital.\n");
            println!("Many companies calculate their weighted average cost of capital (WACC) and use it as their discount rate when budgeting for a new project.");
            println!("It only makes sense for a company to proceed with a new project if its expected revenues are larger than its expected costs—in other words, it needs to be profitable.");
            println!("The discount rate makes it possible to estimate how much the project's future cash flows would be worth in the present.\n");
            println!("An appropriate discount rate can only be determined after the firm has approximated the project's free cash flow.");
            println!("Once the firm has arrived at a free cash flow figure, this can be discounted to determine the net present value (NPV).\n");
            println!("Setting the discount rate isn't always straightforward. Even though many companies use WACC as a proxy for the discount rate, other methods are used as well.");
            println!("In situations where the new project is considerably more or less risky than the company's normal operation, ");
            println!("it may be best to add in a risk premium in case the cost of capital is undervalued or the project does not generate as much cash flow as expected.\n");
            println!("Adding a risk premium to the cost of capital and using the sum as the discount rate takes into consideration the risk of investing.");
            println!("For this reason, the discount rate is usually always higher than the cost of capital.\n");
            CostOfCapital {}.info_bottom_line();
            println!("Source: https://www.investopedia.com/ask/answers/052715/what-difference-between-cost-capital-and-discount-rate.asp");
            println!("=============");
        }
    }

    struct DiscountedCashFlow {}
    impl DiscountedCashFlow {
        fn info(&self) {
            println!("=============");
            println!("Discounted cash flow (DCF) refers to a valuation method that estimates the value of an investment using its expected future cash flows.\n");
            println!("DCF analysis attempts to determine the value of an investment today, based on projections of how much money that investment will generate in the future.\n");
            println!("It can help those considering whether to acquire a company or buy securities make their decisions.");
            println!("Discounted cash flow analysis can also assist business owners and managers in making capital budgeting or operating expenditures decisions.\n");
            println!("The purpose of DCF analysis is to estimate the money an investor would receive from an investment, adjusted for the time value of money.");
            println!("The time value of money assumes that a dollar that you have today is worth more than a dollar that you receive tomorrow because it can be invested.");
            println!("As such, a DCF analysis is useful in any situation where a person is paying money in the present with expectations of receiving more money in the future.\n");
            println!("For example, assuming a 5% annual interest rate, $1 in a savings account will be worth $1.05 in a year.");
            println!("Similarly, if a $1 payment is delayed for a year, its present value is 95 cents because you cannot transfer it to your savings account to earn interest.\n");
            println!("Discounted cash flow analysis finds the present value of expected future cash flows using a discount rate.");
            println!("Investors can use the concept of the present value of money to determine whether the future cash flows of an investment or project are greater than the value of the initial investment.\n");
            println!("If the DCF value calculated is higher than the current cost of the investment, the opportunity should be considered.");
            println!("If the calculated value is lower than the cost, then it may not be a good opportunity, or more research and analysis may be needed before moving forward with it.\n");
            println!("To conduct a DCF analysis, an investor must make estimates about future cash flows and the ending value of the investment, equipment, or other assets.\n");
            println!("If the investor cannot estimate future cash flows, or the project is very complex, DCF will not have much value and alternative models should be employed.\n");
            println!("Source and further explanations with formulas: https://www.investopedia.com/terms/d/dcf.asp");
            println!("=============");
        }
    }

    struct NetPresentValue {}
    impl NetPresentValue {
        fn info(&self) {
            println!("=============");
            println!("Net present value (NPV) is the difference between the present value of cash inflows and the present value of cash outflows over a period of time.");
            println!("NPV is used in capital budgeting and investment planning to analyze the profitability of a projected investment or project.\n");
            println!("NPV is the result of calculations that find the current value of a future stream of payments, using the proper discount rate.");
            println!("In general, projects with a positive NPV are worth undertaking while those with a negative NPV are not.\n");
            println!("If there's one cash flow from a project that will be paid one year from now, then the calculation for the NPV of the project is as follows:\n");
            println!("- NPV = (Cash Flow / (1 + i)^t) - initial investment");
            println!("- i = Required return or discount rate");
            println!("- t = Number of time periods\n");
            println!("NPV accounts for the time value of money and can be used to compare the rates of return of different projects,");
            println!("or to compare a projected rate of return with the hurdle rate required to approve an investment.\n");
            println!("The time value of money is represented in the NPV formula by the discount rate, which might be a hurdle rate for a project based on a company's cost of capital.");
            println!("No matter how the discount rate is determined, a negative NPV shows that the expected rate of return will fall short of it, meaning that the project will not create value.\n");
            println!("Source and further explanations with formulas: https://www.investopedia.com/terms/n/npv.asp");
            println!("=============");
        }
    }

    struct WACC {}
    impl WACC {
        fn info(&self) {
            println!("=============");
            println!("Weighted average cost of capital (WACC) represents a firm's average after-tax cost of capital from all sources, including common stock, preferred stock, bonds, and other forms of debt.");
            println!("WACC is the average rate that a company expects to pay to finance its assets.\n");
            println!("WACC is a common way to determine required rate of return (RRR) because it expresses, in a single number, the return that both bondholders and shareholders demand to provide the company with capital.");
            println!("A firm's WACC is likely to be higher if its stock is relatively volatile or if its debt is seen as risky because investors will require greater returns.\n");
            println!("WACC is the discount rate that a company uses to estimate its net present value.");
            println!("WACC is also important when analyzing the potential benefits of taking on projects or acquiring another business.");
            println!("For example, if the company believes that a merger will generate a return higher than its cost of capital, then it's likely a good choice for the company.");
            println!("If its management anticipates a return lower than what their own investors are expecting, then they'll want to put their capital to better use.\n");
            println!("As the majority of businesses run on borrowed funds, the cost of capital becomes an important parameter in assessing a firm's potential for net profitability.");
            println!("WACC measures a company's cost to borrow money. The WACC formula uses both the company's debt and equity in its calculation.\n");
            println!("In most cases, a lower WACC indicates a healthy business that's able to attract investors at a lower cost.");
            println!("By contrast, a higher WACC usually coincides with businesses that are seen as riskier and need to compensate investors with higher returns.");
            println!("If a company only obtains financing through one source—say, common stock—then calculating its cost of capital would be relatively simple.");
            println!("If investors expected a rate of return of 10% to purchase shares, the firm's cost of capital would be the same as its cost of equity: 10%.\n");
            println!("The same would be true if the company only used debt financing. For example, if the company paid an average yield of 5% on its outstanding bonds, its cost of debt would be 5%. This is also its cost of capital.\n");
            println!("Source and further explanations with formulas: https://www.investopedia.com/terms/w/wacc.asp");
            println!("=============");
        }
    }
}