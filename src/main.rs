use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn go(client: &Client, code: &str, depth: u32) {
    let url = match code {
        "" => "https://atcddd.fhi.no/atc_ddd_index/".to_string(),
        _ => format!("https://atcddd.fhi.no/atc_ddd_index/?code={code}&showdescription=no"),
    };
    let response = client.get(url).send().expect("send").text().expect("text");

    let selector = match depth {
        0 => Selector::parse("#content > div:nth-child(5) > div:nth-child(2) > p").unwrap(),
        1 => Selector::parse("#content > p:nth-child(4)").unwrap(),
        2 => Selector::parse("#content > p:nth-child(6)").unwrap(),
        3 => Selector::parse("#content > p:nth-child(8)").unwrap(),
        4 => Selector::parse("#content > ul > table").unwrap(),
        _ => panic!("Invalid depth encountered"),
    };

    if let Some(inner) = Html::parse_document(&response).select(&selector).next() {
        if depth < 4 {
            let a_selector = Selector::parse("a").unwrap();
            for a in inner.select(&a_selector) {
                let name = a.text().collect::<Vec<_>>().join("");
                let href = a.value().attr("href").expect("No <a>");

                let inner_code = href
                    .split("&")
                    .find(|s| s.starts_with("./?code="))
                    .and_then(|s| s.split("=").nth(1))
                    .unwrap_or("")
                    .to_owned();

                println!("{inner_code}\t{name}");
                go(client, &inner_code, depth + 1);
            }
        } else {
            let row_selector = Selector::parse("tr").unwrap();
            let td_selector = Selector::parse("td").unwrap();

            for row in inner.select(&row_selector).skip(1) {
                let tds = row.select(&td_selector).collect::<Vec<_>>();
                let inner_code = tds[0]
                    .text()
                    .collect::<String>()
                    .trim()
                    .replace("\u{a0}", " ");
                let name = tds[1]
                    .text()
                    .collect::<String>()
                    .trim()
                    .replace("\u{a0}", " ");

                if !inner_code.is_empty() && !name.is_empty() {
                    println!("{inner_code}\t{name}");
                }
            }
        }
    } else {
        eprintln!("Could not find inner element of {code}.");
    }
}

fn main() {
    let client = Client::new();
    go(&client, "", 0);
}
