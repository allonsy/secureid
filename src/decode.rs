
extern crate url;
extern crate base64;

const V3_BASE64_SIZE : i32 = 389;

pub fn decode(inp : String) {
    let decodedURL = url::Url::parse(&inp).unwrap();
    let mut token_val = String::new();
    for (key, value) in decodedURL.query_pairs() {
        if key == "ctfData" {
            token_val = value.into_owned();
        }
    }

    let base_token = base64::decode(token_val.as_bytes()).unwrap();
    println!("value is: {:?}", base_token);
}
