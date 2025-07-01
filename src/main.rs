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

use std::{env, panic::{self}};

use crate::bot::TransleBot;

mod bot;
mod manager;

#[tokio::main]
async fn main() {

    panic::set_hook(Box::new(|info| {
        println!("PANIC HERE: {:?}", info.location());
        println!("PAYLOAD: \n{:?}", info.payload());
    }));

    let ms_token = match env::var("MISTRAL_TOKEN") {
        Ok(s) => s,
        Err(e) => {
               panic!("MISTRAL TOKEN IN ENV NOT SET {}",e);
        }
    };

    let agent_id = match env::var("MISTRAL_AGENT_ID") {
        Ok(s) => s,
        Err(e) => {
               panic!("MISTRAL AGENT ID IN ENV NOT SET {}",e);
        }
    };

    let bot_token = match env::var("BOT_TOKEN") {
        Ok(s) => s,
        Err(e) => {
               panic!("TELEGRAM BOT TOKEN IN ENV NOT SET {}",e);
        }
    };

    let manager = manager::Manager::new(
        ms_token,
        agent_id,
    );

    let bot = teloxide::Bot::new(bot_token);

    let _ = TransleBot::start(bot, manager).await;
}
