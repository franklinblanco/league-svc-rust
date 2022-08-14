use std::fmt::{Display, Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::domain::enums::error::Error;

pub async fn perform_request<B: Serialize, R: DeserializeOwned, T: Display + Debug>(
    base_url: String,
    client: &reqwest::Client,
    method: reqwest::Method,
    path: String,
    body: Option<B>,
    expected_status_code: u16,
    _headers: Vec<(String, String)>,
) -> Result<R, Error> {
    
    let req_incomplete =
        client.request(method, format!("{url}{path}", url = base_url, path = path));
    let req_complete = match body {
        Some(b) => req_incomplete.json(&b),
        None => req_incomplete,
    };
    match req_complete.send().await {
        // Error handling here
        Ok(res) => {
            // Request sent correctly
            match res.status().as_u16() == expected_status_code {
                true => {
                    match res.json::<R>().await {
                        Ok(resp_dto) => Ok(resp_dto), //  Return correctly deserialized obj
                        Err(err) => Err(Error::CommunicatorError(err.to_string())),
                    }
                }
                false => {
                    //If status code is any other than expected
                    Err(Error::UnexpectedStatusCode(
                        expected_status_code,
                        res.status().as_u16(),
                        &res.text().await.unwrap(),
                    ))
                }
            }
        }
        Err(e) => {
            //  Request couldn't be sent
            Err(Error::NetworkError(e.to_string()))
        }
    }
}