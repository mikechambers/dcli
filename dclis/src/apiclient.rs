use reqwest::Url;

const DESTINY_API_KEY: &str = env!("DESTINY_API_KEY");

pub struct ApiCallError {
    pub message: String,
    pub _error_type: ApiCallErrorType,
}

pub enum ApiCallErrorType {
    Request,
    Parse,
}

pub struct ApiClient {
    //TODO: add verbose / print URL
}

impl ApiClient {
    pub fn new() -> ApiClient {
        ApiClient {}
    }

    pub async fn call_api(&self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        let url = Url::parse(&url).unwrap();

        println!("{}", url);

        let client = reqwest::Client::new();

        let resp = match client
            .get(url)
            .header("X-API-Key", DESTINY_API_KEY)
            .send()
            .await
        {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

        Ok(resp)
    }
}
