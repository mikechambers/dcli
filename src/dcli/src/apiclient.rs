/*
* Copyright 2020 Mike Chambers
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

use crate::error::Error;
use crate::response::drs::{check_destiny_response_status, HasDestinyResponseStatus};
use crate::utils::print_verbose;
use reqwest::Url;

const DESTINY_API_KEY: &str = env!("DESTINY_API_KEY");

pub struct ApiClient {
    pub print_url: bool,
}

impl ApiClient {
    pub fn new(print_url: bool) -> ApiClient {
        ApiClient { print_url }
    }

    pub async fn call(&self, url: &str) -> Result<reqwest::Response, Error> {
        let url = Url::parse(&url).unwrap();

        print_verbose(&format!("{}", url), self.print_url);

        let client = reqwest::Client::new();

        let response = match client
            .get(url)
            .header("X-API-Key", DESTINY_API_KEY)
            .send()
            .await {
                Ok(e) => {
                    println!("ApiClient.call response {:#?}", e);
                    println!("ApiClient.call response {:#?}", e.text().await?);
                    return Err(Error::ParameterParseFailure);
                },
                Err(e) => {
                    println!("ApiClient.call Error {:#?}", e);
                    return Err(Error::from(e));
                }
            }; //this either returns a reqwest::Response for an Error which is returned

        Ok(response)
    }

    pub async fn call_and_parse<T: serde::de::DeserializeOwned + HasDestinyResponseStatus>(
        &self,
        url: &str,
    ) -> Result<T, Error> {
        let r1 = match self.call(url).await {
                Ok(e) => e,
                Err(e) => {
                    println!("ApiClient.call_and_parse JSON Parse Error {:#?}", e);
                    return Err(Error::from(e));
                },
        };
        
       let r =  match r1.json::<T>().await {
            Ok(e) => e,
            Err(e) => {
                println!("ApiClient.call_and_parse JSON Parse Error {:#?}", e);
                return Err(Error::from(e));
            },
        };

        check_destiny_response_status(r.get_status())?;

        Ok(r)
    }
}
