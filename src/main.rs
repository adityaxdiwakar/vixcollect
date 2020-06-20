use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut resp = reqwest::blocking::get("https://www.cboe.com/products/futures/vx-cboe-volatility-index-vix-futures")?;
        
        let mut content = String::new();
        resp.read_to_string(&mut content)
            .expect("Something went wrong reading the string");


        let table = table_extract::Table::find_first(&content).unwrap();
        for row in &table {
            println!(
                "{} for {} was last priced at {} with a change of {}. The high was {} and the low was {}, settlement price ended being {} with a volume of {}",
                row.get("<b>Symbol</b>").unwrap_or("N/A"),
                row.get("<b>Expiration</b>").unwrap_or("N/A"),
                row.get("<b>Last</b>").unwrap_or("N/A"),
                row.get("<b>Change</b>").unwrap_or("N/A"),
                row.get("<b>High</b>").unwrap_or("N/A"),
                row.get("<b>Low</b>").unwrap_or("N/A"),
                row.get("<b>Settlement</b>").unwrap_or("N/A"),
                row.get("<b>Volume</b>").unwrap_or("0")
            );
        }
        Ok(())
}

