use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use chrono_tz::US::Pacific;
use chrono::{Datelike, Weekday, Utc};

pub static LUNCH_SPOT_COLLECTION: &str = "lunch_spots";
pub static BASE_LUNCH_SPOT_SCORE: f64 = 5f64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LunchSpot {
    pub name: String,
    pub up_voters: HashSet<String>,
    pub down_voters: HashSet<String>,
    pub open_monday: bool,
    pub open_tuesday: bool,
    pub open_wednesday: bool,
    pub open_thursday: bool,
    pub open_friday: bool
}

impl LunchSpot {
    pub fn new(name: String,
               open_monday: bool,
               open_tuesday: bool,
               open_wednesday: bool,
               open_thursday: bool,
               open_friday: bool) -> LunchSpot
    {
        LunchSpot {
            name,
            up_voters: HashSet::new(),
            down_voters: HashSet::new(),
            open_monday,
            open_tuesday,
            open_wednesday,
            open_thursday,
            open_friday
        }
    }

    /// check if this lunch spot is open today or not
    /// ### return
    /// true / false is the lunch spot open today
    pub fn is_open(&self) -> bool {
        match Utc::now().with_timezone(&Pacific).weekday() {
            Weekday::Mon => self.open_monday,
            Weekday::Tue => self.open_tuesday,
            Weekday::Wed => self.open_wednesday,
            Weekday::Thu => self.open_thursday,
            Weekday::Fri => self.open_friday,
            Weekday::Sat => true,
            Weekday::Sun => true,
        }
    }
}