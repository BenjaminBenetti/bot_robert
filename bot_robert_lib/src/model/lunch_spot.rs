use serde::{Serialize, Deserialize};
pub static LUNCH_SPOT_COLLECTION: &str = "lunch_spots";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LunchSpot {
    pub name: String,
    pub score: i32,
    pub open_monday: bool,
    pub open_tuesday: bool,
    pub open_wednesday: bool,
    pub open_thursday: bool,
    pub open_friday: bool
}

impl LunchSpot {
    pub fn new(name: String,
               score: i32,
               open_monday: bool,
               open_tuesday: bool,
               open_wednesday: bool,
               open_thursday: bool,
               open_friday: bool) -> LunchSpot
    {
        LunchSpot {
            name,
            score,
            open_monday,
            open_tuesday,
            open_wednesday,
            open_thursday,
            open_friday
        }
    }
}