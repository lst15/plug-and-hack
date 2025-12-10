use plug_and_hack::osint;

#[tokio::main]
async fn main() {
    let shodan = osint::shodan::Shodan::new(String::from(""));

    let filters = osint::shodan::Filters {
        http_component: vec!["'Next.js'".into()],
        ..Default::default()
    };
    let rsp = shodan.query(filters).await;
    let total = rsp.matches.len();
    println!("{total:?}");
}
