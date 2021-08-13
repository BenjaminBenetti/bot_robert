use crate::tasks::Task;
use async_std;
use chrono::{DateTime, Utc};
use async_trait::async_trait;

#[async_trait]
pub trait ScheduledTask: Task {

    /// schedule the task to run at some future point.
    async fn schedule_task(&mut self) {
        let next_run_time = self.get_next_run_time();

        if let Some(run_at) = next_run_time {
            if let Ok(sleep_duration) = (run_at - Utc::now()).to_std() {
                async_std::task::sleep(sleep_duration).await;
                self.run().await;
                self.schedule_task().await;
            }
            else
            {
                println!("Schedule task duration negative! aborting task");
                self.kill().await;
            }
        }
    }

    /// returns the next time at which this task should run, or None if no more runs should be performed
    fn get_next_run_time(&self) -> Option<DateTime<Utc>>;
}