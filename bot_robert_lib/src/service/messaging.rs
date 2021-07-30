use crate::model::SlackResponse;
use crate::converter::slack_response_to_transfer;
use actix_web;
use std::error::Error;

static SLACK_MSG_POST_URL: &str = "https://slack.com/api/chat.postMessage";

pub async fn post_message_to_channel(login_token: &str, channel_id: &str, msg: SlackResponse) -> Result<(), Box<dyn Error>> {
    let client = actix_web::client::Client::default();
    let mut msg_to_send = msg.clone();

    msg_to_send.channel = Some(channel_id.to_string());
    let msg_json = serde_json::to_string(&slack_response_to_transfer::convert(&msg_to_send))?;

    do_post(client, &login_token.to_string(), &msg_json).await?;
    Ok(())
}

async fn do_post(client: actix_web::client::Client, login_token: &String, post_body: &String) -> Result<(), Box<dyn Error>> {
    return match client.post(SLACK_MSG_POST_URL)
        .header("Content-type", "application/json")
        .header("Authorization", format!("Bearer {}", login_token))
        .send_body(post_body).await {
        Ok(_) => Ok(()),
        Err(error) => {
            println!("Failed to post to slack with error: {:?}", error);
            Err(Box::new(error))
        }
    }
}