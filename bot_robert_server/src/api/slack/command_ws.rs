use actix_web::{post, web, HttpResponse, Responder};
use crate::api::slack::transfer::SlashCommandTransfer;

/// handles incoming slash command from slack.
#[post("/api/slack/cmd")]
pub async fn slash_command(slash_cmd: web::Form<SlashCommandTransfer>) -> impl Responder {
    HttpResponse::Ok().body(bot_robert_lib::slash_command::process_command(&slash_cmd.user_name, &slash_cmd.text))
}
