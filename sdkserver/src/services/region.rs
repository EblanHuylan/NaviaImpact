use common::encryption;
use prost::Message;
use proto::{QueryCurrRegionHttpRsp, QueryRegionListHttpRsp, RegionInfo, RegionSimpleInfo};
use serde_json::json;

pub const QUERY_REGION_LIST_ENDPOINT: &str = "/query_region_list";
pub const QUERY_CUR_REGION_ENDPOINT: &str = "/query_cur_region";

const EC2B: &[u8] = include_bytes!("../../initial_key.ec2b");
const KEY: &[u8] = include_bytes!("../../initial_key.bin");
const GLOBAL_CONFIG: &str = r#"{"codeSwitch":[4334],"sdkenv":"2","checkdevice":"false","loadPatch":"false","showexception":"false","regionConfig":"pm|fk|add","downloadMode":"0"}"#;

fn xor(bytes: &mut [u8], key: &[u8]) {
    for i in 0..bytes.len() {
        bytes[i] ^= key[i % key.len()];
    }
}

#[tracing::instrument]
pub async fn query_region_list() -> String {
    let mut client_custom_config = GLOBAL_CONFIG.as_bytes().to_vec();
    xor(&mut client_custom_config, KEY);

    let rsp = QueryRegionListHttpRsp {
        retcode: 0,
        region_list: vec![RegionSimpleInfo {
            r#type: String::from("DEV_PUBLIC"),
            dispatch_url: String::from("http://127.0.0.1:21000/query_cur_region"),
            name: String::from("os_navia"),
            title: String::from("NaviaImpact"),
        }],
        client_secret_key: EC2B.to_vec(),
        client_custom_config_encrypted: client_custom_config,
        ..Default::default()
    };

    let mut buf = Vec::new();
    rsp.encode(&mut buf).unwrap();

    rbase64::encode(&buf)
}

#[tracing::instrument]
pub async fn query_cur_region() -> String {
    let rsp = QueryCurrRegionHttpRsp {
        retcode: 0,
        region_info: Some(RegionInfo {
            gateserver_ip: String::from("127.0.0.1"),
            gateserver_port: 22101,
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut buf = Vec::new();
    rsp.encode(&mut buf).unwrap();

    let content = rbase64::encode(&encryption::rsa_encrypt(&buf));
    let sign = rbase64::encode(&encryption::rsa_sign(&buf));

    json!({
        "content": content,
        "sign": sign
    })
    .to_string()
}
