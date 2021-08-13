use crate::model::SlackResponse;
use crate::converter::slack_response_to_transfer;
use std::error::Error;
use crate::service::slack_api_client;

static SLACK_MSG_POST_URL: &str = "https://slack.com/api/chat.postMessage";

pub async fn post_message_to_channel(channel_id: &str, msg: SlackResponse) -> Result<(), Box<dyn Error>> {
    let mut msg_to_send = msg.clone();

    msg_to_send.channel = Some(channel_id.to_string());
    let msg_json = serde_json::to_string(&slack_response_to_transfer::convert(&msg_to_send))?;

    slack_api_client::do_post(SLACK_MSG_POST_URL, &msg_json).await?;
    Ok(())
}