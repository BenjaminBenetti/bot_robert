use actix_web::{post, web, HttpResponse, Responder};
use bot_robert_lib::transfer::{SlashCommandTransfer};
use bot_robert_lib::converter::slack_response_to_transfer;
use std::time::Instant;

/// handles incoming slash command from slack.
#[post("/api/slack/cmd")]
pub async fn slash_command(slash_cmd: web::Form<SlashCommandTransfer>) -> impl Responder {
    let now = Instant::now();
    let response = bot_robert_lib::slash_command::process_command(&slash_cmd.user_name, &slash_cmd.text).await;
    let transfer = slack_response_to_transfer::convert(&response);
    let http_response = HttpResponse::Ok().json(&transfer);
    println!("Slash command processed in: {}ms \n Response: \n{:?}", now.elapsed().as_millis(), transfer);
    return http_response;
}
