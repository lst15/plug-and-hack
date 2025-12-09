mod osint;

use crate::osint::shodan;

#[tokio::main]
async fn main() {
    let shodan = shodan::Shodan::new(String::from(""));

    let filters = shodan::Filters {
        http_component: vec!["'Next.js'".into()],
        ..Default::default()
    };
    let rsp = shodan.query(filters).await;
    let total = rsp.matches.len();
    println!("{total:?}");
}
