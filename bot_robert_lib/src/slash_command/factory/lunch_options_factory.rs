use crate::slash_command::SlackResponse;

/// gets a list of lunch options as a slack response array
pub fn lunch_options() -> Vec<SlackResponse> {
    vec!(
        SlackResponse::from_string("Lido"),
        SlackResponse::from_string("Tractor"),
        SlackResponse::from_string("Bao"),
        SlackResponse::from_string("Siam"),
        SlackResponse::from_string("Red Fish Blue Fish"),
        SlackResponse::from_string("Owly"),
        SlackResponse::from_string("Hudson Market"),
        SlackResponse::from_string("Farmhouse"),
        SlackResponse::from_string("Flying Otter"),
        SlackResponse::from_string("AyoEat"),
        SlackResponse::from_string("Cafe Brio"),
        SlackResponse::from_string("King Sejong"),
        SlackResponse::from_string("10 Acres"),
        SlackResponse::from_string("The Ruby"),
        SlackResponse::from_string("Shanzee's"),
        SlackResponse::from_string("Plish Deli"),
    )
}