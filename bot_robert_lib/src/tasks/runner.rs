use crate::tasks::{BeerOClock, ScheduledTask};
use actix_rt::{Arbiter, System};
use futures::join;

/// boot system task runners
pub async fn start_system_tasks() {
    if let None = System::try_current() {
        let _ = System::new();
    }

    let arbiter = Arbiter::new();
    arbiter.spawn(async {
        let mut beer_o_clock = BeerOClock::new();

        join!(
            beer_o_clock.schedule_task()
        );
    });
}