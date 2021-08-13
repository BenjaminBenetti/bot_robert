use async_trait::async_trait;

#[async_trait]
pub trait Task {
    // run the task.
    async fn run(&mut self);

    // kill the task. Stops task if running and prevents any future runs caused by events.
    async fn kill(&mut self);
}