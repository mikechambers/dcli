/*
* Copyright 2021 Mike Chambers
* https://github.com/mikechambers/dcli
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of
* this software and associated documentation files (the "Software"), to deal in
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do
* so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use reqwest::header::{HeaderMap, HeaderValue, CONNECTION};
use reqwest::{Client, Url};

use crate::error::Error;
use crate::response::drs::{
    check_destiny_response_status, IsDestinyAPIResponse,
};
use crate::utils::print_verbose;

const DESTINY_API_KEY: &str = env!("DESTINY_API_KEY");
const API_TIMEOUT: u64 = 10; //seconds

//this makes sure that the env variable isnt set, but empty
static_assertions::const_assert!(!DESTINY_API_KEY.is_empty());

pub struct ApiClient {
    pub verbose: bool,
    client: Client,
}

impl ApiClient {
    pub fn new(verbose: bool) -> Result<ApiClient, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(
            "Keep-Alive",
            HeaderValue::from_static("timeout=10, max=1000"),
        );
        headers.insert("X-API-Key", HeaderValue::from_static(DESTINY_API_KEY));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(API_TIMEOUT))
            .build()?;

        Ok(ApiClient { verbose, client })
    }

    pub async fn call(&self, url: &str) -> Result<reqwest::Response, Error> {
        let url = Url::parse(&url).unwrap();

        print_verbose(&format!("{}", url), self.verbose);

        let response = self
            .client
            .get(url)
            //.header("X-API-Key", DESTINY_API_KEY)
            .send()
            .await?; //this either returns a reqwest::Response for an Error which is returned

        Ok(response)
    }

    pub async fn call_and_parse<
        T: serde::de::DeserializeOwned + IsDestinyAPIResponse,
    >(
        &self,
        url: &str,
    ) -> Result<T, Error> {
        let body = match self.call(url).await {
            Ok(e) => {
                //println!("{:?}", e.headers());
                e.text().await?
            }
            Err(e) => return Err(e),
        };

        if self.verbose {
            let len = body.chars().count();
            const MAX: usize = 200;
            let limit = std::cmp::min(len, MAX);

            println!(
                "---------Begin API response : First {}  chars---------",
                limit
            );
            println!("{}", &body[..limit]);
            println!("---------End API response---------");
        }

        //we split the parsing from the request so we can capture the body and
        //print it out if we need to
        let r = serde_json::from_str::<T>(&body)?;

        check_destiny_response_status(r.get_status())?;

        Ok(r)
    }

    pub async fn call_post(
        &self,
        url: &str,
        post_data: &str,
    ) -> Result<reqwest::Response, Error> {
        let url = Url::parse(&url).unwrap();

        print_verbose(&format!("{}", url), self.verbose);

        let response = self
            .client
            .post(url)
            .body(post_data.to_string())
            .send()
            .await?; //this either returns a reqwest::Response for an Error which is returned

        Ok(response)
    }

    pub async fn call_post_and_parse<
        T: serde::de::DeserializeOwned + IsDestinyAPIResponse,
    >(
        &self,
        url: &str,
        post_data: &str,
    ) -> Result<T, Error> {
        let body = match self.call_post(url, post_data).await {
            Ok(e) => {
                //println!("{:?}", e.headers());
                e.text().await?
            }
            Err(e) => return Err(e),
        };

        if self.verbose {
            let len = body.chars().count();
            const MAX: usize = 200;
            let limit = std::cmp::min(len, MAX);

            println!(
                "---------Begin API response : First {}  chars---------",
                limit
            );
            println!("{}", &body[..limit]);
            println!("---------End API response---------");
        }

        //we split the parsing from the request so we can capture the body and
        //print it out if we need to
        let r = serde_json::from_str::<T>(&body)?;

        check_destiny_response_status(r.get_status())?;

        Ok(r)
    }
}
