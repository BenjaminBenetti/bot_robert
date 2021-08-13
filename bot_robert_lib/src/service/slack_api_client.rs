use std::error::Error;
use std::env;
use lazy_static::lazy_static;
use reqwest::RequestBuilder;

lazy_static! {
    pub static ref SLACK_API_KEY: String = {
        env::var("BOT_ROBERT_API_KEY").unwrap()
    };
}

pub async fn do_post(url: &str, post_body: &String) -> Result<(), Box<dyn Error>> {

    let client = reqwest::Client::new();

    return match set_common_headers(client.post(url))
        .body(post_body.clone()).send().await {
        Ok(_) => Ok(()),
        Err(error) => {
            println!("Failed to post to slack with error: {:?}", error);
            Err(Box::new(error))
        }
    }
}

fn set_common_headers(request: RequestBuilder) -> RequestBuilder {
    request.header("Content-type", "application/json")
        .header("Authorization", format!("Bearer {}", SLACK_API_KEY.to_string()))
}