use reqwest;
use serde_json::json;
use std::collections::HashMap;

enum Action {
    None,
    GetProduct,
    Sale,
}

async fn add_product(
    barcode: String,
    name: String,
    cost: u32,
    retail: u32,
    wholesale: u32,
) -> reqwest::Result<()> {
    let json = json!({
        "barcode": barcode,
        "name": name,
        "cost": cost,
        "retail": retail,
        "wholesale": wholesale
    });

    let client = reqwest::Client::new();
    let response = client.post("").json(&json).send().await?;
    Ok(())
}
