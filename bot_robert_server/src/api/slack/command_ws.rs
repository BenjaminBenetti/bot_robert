use actix_web::{post, web, HttpResponse, Responder};
use crate::api::slack::transfer::{SlashCommandTransfer};
use crate::api::slack::converter::slack_response_to_transfer;
use std::time::Instant;

/// handles incoming slash command from slack.
#[post("/api/slack/cmd")]
pub async fn slash_command(slash_cmd: web::Form<SlashCommandTransfer>) -> impl Responder {
    let now = Instant::now();
    let response = bot_robert_lib::slash_command::process_command(&slash_cmd.user_name, &slash_cmd.text);
    let http_response = HttpResponse::Ok().json(slack_response_to_transfer::convert(&response));
    println!("Slash command processed in: {}ms", now.elapsed().as_millis());
    return http_response;
}
