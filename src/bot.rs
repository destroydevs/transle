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

use std::sync::Arc;
use std::vec;

use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};

use crate::manager::Manager;

pub struct TransleBot;

impl TransleBot {
    pub async fn start(bot: Bot, manager: Manager) {
        let manager = Arc::new(manager);

        let handler = Update::filter_inline_query().branch(dptree::endpoint(
            move |bot: Bot, q: InlineQuery| {
                let manager = Arc::clone(&manager);

                async move {
                    //let from = q.from;
                    let text = q.query;

                    if text.trim().is_empty() {
                        return respond(());
                    }

                    let response = match Self::get_resp(text, &manager).await {
                        Some(s) => s,
                        None => return respond(()),
                    };

                    let result = InlineQueryResultArticle::new(
                        "1",
                        "Translation",
                        InputMessageContent::Text(InputMessageContentText::new(&response)),
                    ).description(response);

                    let results = vec![InlineQueryResult::Article(result)];

                    let _ = bot.answer_inline_query(q.id, results).send().await;

                    respond(())
                }
            },
        ));

        Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    pub async fn get_resp(resp: String, manager: &Manager) -> Option<String> {
        let resp = manager.send_request(resp).await;

        if let Some(r) = resp {
            let choices = r.choices;
            let choice = choices.get(0);

            if let Some(c) = choice {
                Some(c.message.content.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}
