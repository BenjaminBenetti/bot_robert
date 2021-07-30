use actix_web::{post, Responder, HttpResponse, web::Form};
use bot_robert_lib::transfer::SlackFormPayloadTransfer;
use bot_robert_lib::model::SlackResponse;
use bot_robert_lib::converter::slack_response_to_transfer;
use std::time::Instant;

#[post("/api/slack/action")]
pub async fn inbound_slack_action(slack_payload: Form<SlackFormPayloadTransfer>) -> impl Responder {
    // Slack will send multiple different requests to this endpoint.
    let now = Instant::now();

    match bot_robert_lib::slash_command::process_action(&slack_payload).await {
        Ok((res, url)) => {
            if let Some(url) = url {
                send_reply_to_slack(&res, &url).await;
            }
            println!("Inbound slack action processed in: {}ms", now.elapsed().as_millis());
            HttpResponse::Ok()
        },
        Err(error) => {
            println!("Failed to process inbound response with error: {:?}", error);
            HttpResponse::InternalServerError()
        }
    }
}

async fn send_reply_to_slack(res: &SlackResponse, url: &String) {
    let client = actix_web::client::Client::default();
    let res_json = serde_json::to_string(&slack_response_to_transfer::convert(&res));

    if let Ok(res_json) = res_json {
        match client.post(url).send_body(res_json).await {
            Ok(_) => {}
            Err(error) => println!("Failed to send response to slack with error: {:?}", &error)
        }
    }
    else {
        println!("RIP");
    }
}