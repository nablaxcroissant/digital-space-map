//use reqwest::blocking as rq;
use std::fs;
use scraper::{Html, Selector, ElementRef};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Grant {
    amount: String,
    currency: String,
    researcher: String,
}

fn main() {
    let mut results: Vec<Grant> = Vec::new();

    let page = fs::read_to_string("./data400.html").unwrap();

    let fragment = Html::parse_document(&page);

    //print!("{:?}", fragment.html());

    let table_selector = Selector::parse("table#ag-results").unwrap();
    let tbody = Selector::parse("tbody").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    
    let table = fragment.select(&table_selector).next().unwrap();
    let body = table.select(&tbody).next().unwrap();

    let _= body.select(&row_selector)
        .filter(|el| {
            //print!("{:?}", el.value().attr("class"));
            if let Some(class) = el.value().attr("class") {
                !(class == "hide")
            } else {
                true
            }
        })
        .for_each(|tr|{
            if let Some(grant) = get_datapoint(tr){
                results.push(grant)
            }
        });
    //print!("{:?}", results);

    let mut wtr = csv::Writer::from_path("data2.csv").unwrap();
    for grant in results.iter(){
        wtr.serialize(grant).unwrap();
    }
   

}

fn get_datapoint(tr: ElementRef<'_>) -> Option<Grant>{
    let entry_selector = Selector::parse("span").unwrap();
    let amount_selector = Selector::parse("span.amount-currency").unwrap();
    let currency_selector = Selector::parse("span.currency").unwrap();

    let mut researchers = tr.select(&entry_selector);
    let _ = researchers.next()?;
    let _ = researchers.next()?;
    let researcher = researchers.next()?.text().next()?.to_string();
    let amount = tr.select(&amount_selector).next()?.text().next()?.to_string();
    let currency = tr.select(&currency_selector).next()?.text().next()?.to_string();
    Some(Grant{
        amount,
        currency,
        researcher,
    })
}