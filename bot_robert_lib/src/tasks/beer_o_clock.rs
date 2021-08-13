use crate::tasks::{Task, ScheduledTask};
use crate::service::messaging;
use chrono::{Utc, DateTime, TimeZone, Weekday, Datelike, Duration, Timelike};
use chrono_tz::US::Pacific;
use async_trait::async_trait;
use crate::model::SlackResponse;

pub struct BeerOClock {
    killed: bool,
}

impl BeerOClock {
    pub fn new() -> BeerOClock {
        BeerOClock {
            killed: false
        }
    }
}

#[async_trait]
impl Task for BeerOClock {
    async fn run(&mut self) {
        let _ = messaging::post_message_to_channel("CLKSLCJU9", SlackResponse::from_string("It's Beer O Clock! :beers:")).await;
    }

    async fn kill(&mut self) {
        self.killed = true;
    }
}

impl ScheduledTask for BeerOClock {
    fn get_next_run_time(&self) -> Option<DateTime<Utc>> {
        if self.killed {
            return None
        }

        let now = Utc::now().with_timezone(&Pacific);
        let mut target = Pacific.isoywd(now.year(), now.iso_week().week(), Weekday::Fri).and_hms(16, 0, 0).with_timezone(&Utc);

        // if past friday 16:00 target next friday.
        if now.weekday() == Weekday::Sat || now.weekday() == Weekday::Sun ||
            (now.weekday() == Weekday::Fri && now.hour() >= 16) {
            target = target + Duration::weeks(1);
        }

        Some(target)
    }
}