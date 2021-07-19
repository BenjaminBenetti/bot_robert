use actix_web::{post, web, HttpResponse, Responder};
use regex::Regex;
use crate::api::slack::transfer::SlashCommandTransfer;

/// handles incoming slash command from slack.
#[post("/api/slack/cmd")]
pub async fn slash_command(slash_cmd: web::Form<SlashCommandTransfer>) -> impl Responder {
    let response_text: String;
    let robert_regex = Regex::new(r"^[rR]obert.*");

    match robert_regex {
        Ok(re) => {
            if re.is_match(&slash_cmd.user_name) {
                response_text = String::from("I'm talking to my self!");
            }
            else {
                response_text = format!(
                    "Slash command is {}. Args {}. User id {}",
                    slash_cmd.command,
                    slash_cmd.text,
                    slash_cmd.user_name);
            }
        }
        _ => {
            response_text = String::from("Uh-O ... some this went wrong :(");
        }
    }

    HttpResponse::Ok().body(response_text)
}
