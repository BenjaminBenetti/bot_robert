use crate::model::SlackResponse;
use crate::converter::slack_response_to_transfer;
use std::error::Error;
use crate::service::slack_api_client;
use reqwest::Response;
use crate::error::JsonDecodeError;

static SLACK_MSG_POST_URL: &str = "https://slack.com/api/chat.postMessage";

pub async fn post_message_to_channel(channel_id: &str, msg: SlackResponse) -> Result<Response, Box<dyn Error + Send>> {
    let mut msg_to_send = msg.clone();

    msg_to_send.channel = Some(channel_id.to_string());
    let decode_result = serde_json::to_string(&slack_response_to_transfer::convert(&msg_to_send));
    match decode_result {
        Err(e) => Err(Box::new(JsonDecodeError::new(e.to_string()))),
        Ok(json) => slack_api_client::do_post(SLACK_MSG_POST_URL, &json).await
    }
}