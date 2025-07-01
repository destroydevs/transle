/*
   Copyright 2025 Evgeny K.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

pub struct Manager {
    mistral_token: String,
    agent_id: String,
}

impl Manager {
    pub fn new(token: String, agent_id: String) -> Self {
        Manager {
            mistral_token: token,
            agent_id,
        }
    }

    pub async fn send_request(&self, content: String) -> Option<Response> {
        let request = Request {
            agent_id: self.agent_id.to_string(),
            messages: vec![RequestMessage {
                role: "user".to_string(),
                content: content.clone(),
            }],
            max_tokens: content.len(),
        };

        let client = reqwest::Client::builder().build().unwrap();

        let mut post = client.post("https://api.mistral.ai/v1/agents/completions");

        let mut headers = HeaderMap::new();

        headers.append(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.append("Accept", HeaderValue::from_str("application/json").unwrap());

        post = post.headers(headers);

        post = post.bearer_auth(self.mistral_token.clone());

        post = post.json(&request);

        let post_request = post.build().unwrap();

        let response = client.execute(post_request).await.unwrap();

        if response.status().is_success() {
            let json = response.json::<Response>().await.unwrap();

            Some(json)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choices {
    index: i32,
    pub message: ResponseMessage,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    agent_id: String,
    messages: Vec<RequestMessage>,
    max_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub choices: Vec<Choices>,
}
