use chrono::prelude::*;
enum OptionType {
    Call,
    Put,
}

struct StockOption<'a> {
    option_type: &'a OptionType,
    ticker: &'a str,
    strike_price: f32,
    expiration_date: &'a chrono::NaiveDate,
}

impl StockOption<'_> {
    fn new<'a>(
        option_type: &'a OptionType,
        ticker: &'a str,
        strike_price: f32,
        expiration_date: &'a NaiveDate,
    ) -> StockOption<'a> {
        StockOption {
            option_type,
            ticker,
            strike_price,
            expiration_date,
        }
    }
}

struct StockOptionChains {
    count: u32,
    num_strikes: u32,
    option_type: OptionType,
    ticker: String,
    current_price: f32,
    expiration_date: chrono::NaiveDate,
}

// impl Iterator for StockOptionChains<'a> {
//     type Item = StockOption;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;
//         self.current_price += 0.05;
//         if self.count > self.num_strikes {
//             None
//         } else {
//             let stock_option = StockOption::new(
//                 self.option_type,
//                 self.ticker,
//                 self.current_price,
//                 self.expiration_date,
//             );
//             Some(stock_option)
//         }
//     }
// }
