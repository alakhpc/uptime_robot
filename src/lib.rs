pub mod enums;
mod error;
pub mod structs;

pub use error::Error;
use reqwest::{Client, RequestBuilder, Response};
use structs::GetMonitorsResponse;

pub struct UptimeRobot {
    api_key: String,
    client: Client,
}

impl UptimeRobot {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        Self { api_key, client }
    }

    fn api_url(path: &str) -> String {
        format!("https://api.uptimerobot.com/v2{}", path)
    }

    async fn fire(&self, req: RequestBuilder) -> Result<Response, Error> {
        req.basic_auth("api_key", Some(&self.api_key))
            .query(&[("api_key", &self.api_key)])
            .query(&[("format", "json")])
            .send()
            .await
            .map(Response::error_for_status)?
            .map_err(Into::into)
    }

    pub async fn get_monitors(&self) -> Result<GetMonitorsResponse, Error> {
        let url = Self::api_url("/getMonitors");
        let req = self.client.post(url);
        let res = self.fire(req).await?.json::<GetMonitorsResponse>().await?;
        Ok(res)
    }
}
