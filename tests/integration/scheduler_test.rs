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

    for i in 0..2 {
        let task = Task::new(format!("model-{}", i), vec![i as u8]);
        let validated = ValidatedTask {
            inner: task,
            priority: TaskPriority::Normal,
            validated_at: chrono::Utc::now(),
        };
        scheduler.enqueue(validated).await.unwrap();
    }

    let task = Task::new("overflow", vec![99]);
    let validated = ValidatedTask {
        inner: task,
        priority: TaskPriority::Normal,
        validated_at: chrono::Utc::now(),
    };
    let result = scheduler.enqueue(validated).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_scheduler_priority_ordering() {
    let config = SchedulerConfig::default();
    let scheduler = Scheduler::new(config);
    scheduler.start().await.unwrap();

    let low = ValidatedTask {
        inner: Task::new("low-model", vec![1]),
        priority: TaskPriority::Low,
        validated_at: chrono::Utc::now(),
    };
    let high = ValidatedTask {
        inner: Task::new("high-model", vec![2]),
        priority: TaskPriority::Critical,
        validated_at: chrono::Utc::now(),
    };

    scheduler.enqueue(low).await.unwrap();
    scheduler.enqueue(high).await.unwrap();

    let batch = scheduler.dequeue_batch().await;
    assert_eq!(batch[0].inner.model_id, "high-model");
}

#[tokio::test]
async fn test_task_status_tracking() {
    let config = SchedulerConfig::default();
    let scheduler = Scheduler::new(config);
    scheduler.start().await.unwrap();

    let task = Task::new("model", vec![1, 2, 3]);
    let task_id = task.id.clone();
    let validated = ValidatedTask {
        inner: task,
        priority: TaskPriority::Normal,
        validated_at: chrono::Utc::now(),
    };

    scheduler.enqueue(validated).await.unwrap();
    let status = scheduler.get_status(&task_id).await.unwrap();
    assert_eq!(status, bbean_core::task::TaskStatus::Queued);
}
