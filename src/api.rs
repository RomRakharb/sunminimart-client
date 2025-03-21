use crate::State;
use reqwest::Client;
use serde_json::json;

enum Action {
    None,
    GetProduct,
    Sale,
}

impl State {
    async fn add_product(&self, barcode: String, name: String, retail: u32) -> reqwest::Result<()> {
        let json = json!({
            "barcode": barcode,
            "name": name,
            "retail": retail,
        });

        let client = Client::new();
        let _ = client
            .post(self.setting.database_url.clone())
            .json(&json)
            .send()
            .await?;
        Ok(())
    }

    async fn delete_product(&self, barcode: String) {}
    async fn stock_product(&self) {}
    async fn get_all_product(&self) {}
    async fn get_price(&self) {}
    async fn sell(&self) {}
}
