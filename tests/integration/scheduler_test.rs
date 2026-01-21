use bbean_core::config::SchedulerConfig;
use bbean_core::task::{Scheduler, Task, TaskPriority, ValidatedTask};

#[tokio::test]
async fn test_scheduler_enqueue_dequeue() {
    let config = SchedulerConfig {
        max_queue_size: 100,
        task_timeout_secs: 60,
        max_retries: 3,
        batch_size: 10,
    };
    let scheduler = Scheduler::new(config);
    scheduler.start().await.unwrap();

    let task = Task::new("test-model", vec![1, 2, 3]);
    let validated = ValidatedTask {
        inner: task,
        priority: TaskPriority::Normal,
        validated_at: chrono::Utc::now(),
    };

    let receipt = scheduler.enqueue(validated).await.unwrap();
    assert_eq!(receipt.status, bbean_core::task::TaskStatus::Queued);
    assert_eq!(scheduler.queue_len().await, 1);

    let batch = scheduler.dequeue_batch().await;
    assert_eq!(batch.len(), 1);
    assert_eq!(scheduler.queue_len().await, 0);
}

#[tokio::test]
async fn test_scheduler_capacity_limit() {
    let config = SchedulerConfig {
        max_queue_size: 2,
        task_timeout_secs: 60,
        max_retries: 3,
        batch_size: 10,
    };
    let scheduler = Scheduler::new(config);
    scheduler.start().await.unwrap();