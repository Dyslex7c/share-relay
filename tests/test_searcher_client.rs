use mev_share_relay::bundle::types::SearcherBundle;
use mev_share_relay::searcher::assembler::assemble_backrun_bundle;
use mev_share_relay::searcher::client::SearcherRpcClient;
use serde_json::Value;

#[test]
fn formats_valid_mev_send_bundle_json_rpc_payload() {
    let client = SearcherRpcClient::new("https://relay.mev-share.net".into());
    assert_eq!(client.relay_url, "https://relay.mev-share.net");
    let bundle = assemble_backrun_bundle([0xab; 32], vec![0x02, 0xf8, 0x6c, 0x01], 1_000, 9000);
    let raw = client.format_send_bundle_request(&bundle);
    let parsed: Value = serde_json::from_str(&raw).expect("valid json-rpc payload");
    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["method"], "mev_sendBundle");
    assert_eq!(parsed["id"], 1);
    let param = &parsed["params"][0];
    assert_eq!(param["target_block"], 1_000);
    assert_eq!(param["kickback_bps"], 9000);
    let items = param["items"].as_array().expect("items array");
    assert_eq!(items.len(), 2);
    let decoded: SearcherBundle = serde_json::from_value(param.clone()).expect("bundle round-trip");
    assert_eq!(decoded.items, bundle.items);
    assert_eq!(decoded.target_block, 1_000);
}
