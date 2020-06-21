use std::io::Read;

struct Quote {
    symbol: String,
    expiration: String,
    change: f32,
    last: f32,
    high: f32,
    low: f32,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut resp = reqwest::blocking::get("https://www.cboe.com/products/futures/vx-cboe-volatility-index-vix-futures")?;
        
        let mut content = String::new();
        resp.read_to_string(&mut content)
            .expect("Something went wrong reading the string");


        let table = table_extract::Table::find_first(&content).unwrap();
        for row in &table {

            let symbol_html = row.get("<b>Symbol</b>").unwrap_or("<a>N/A</a>");
            let change_html = row.get("<b>Change</b>").unwrap_or("<span>N/A</span>");

            let parsed_symbol = scraper::Html::parse_fragment(symbol_html);
            let parsed_change = scraper::Html::parse_fragment(change_html);

            let symbol_value = parsed_symbol.select(&scraper::Selector::parse("a")
                .unwrap())
                .next()
                .unwrap()
                .inner_html();

            let change_value = parsed_change.select(&scraper::Selector::parse("span")
                .unwrap())
                .next()
                .unwrap()
                .inner_html();

            let quote = Quote {
                symbol: symbol_value,
                expiration: row.get("<b>Expiration</b>").unwrap_or("N/A").to_string(),
                last: row.get("<b>Last</b>").unwrap_or("N/A").to_string().parse::<f32>().unwrap(),
                change: change_value.to_string().parse::<f32>().unwrap(),
                high: row.get("<b>High</b>").unwrap_or("N/A").parse::<f32>().unwrap(),
                low: row.get("<b>Low</b>").unwrap_or("N/A").parse::<f32>().unwrap(),
            };

            println!(
                "{} for {} was last priced at {} with a change of {}. The high was {} and the low was {}.\n",
                quote.symbol, quote.expiration, quote.last, quote.change, quote.high, quote.low
                );
        }
        Ok(())
}

