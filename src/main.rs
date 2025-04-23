use reqwest::blocking::Client;
use serde_json::json;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let shenyu_url = "http://localhost:9195/shenyu/plugin/selectorAndRules";
    let l_key = "123456"; 
    let url = "http://localhost:8080"; 

    let payload = json!({
        "pluginName": "divide",
        "selectorHandler": format!("[{{\"upstreamUrl\":\"{}\"}}]", url),
        "conditionDataList": [{
            "paramType": "uri",
            "operator": "match",
            "paramValue": "/**"
        }],
        "ruleDataList": [{
            "ruleHandler": "{\"loadBalance\":\"random\"}",
            "conditionDataList": [{
                "paramType": "uri",
                "operator": "match",
                "paramValue": "/**"
            }]
        }]
    });

    let client = Client::new();
    let response = client
        .post(shenyu_url)
        .header("Content-Type", "application/json")
        .header("localKey", l_key)
        .json(&payload)
        .send()?;

    println!("status: {}", response.status());
    let response_text = response.text()?;
    println!("response: {}", response_text);

    Ok(())
}
