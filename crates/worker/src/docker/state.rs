use chrono::{DateTime, Utc};
use shared::models::task::{Task, TaskState};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
pub(crate) struct DockerState {
    current_task: Arc<Mutex<Option<Task>>>,
    last_started: Arc<Mutex<Option<DateTime<Utc>>>>,
    is_running: Arc<Mutex<bool>>,
}

impl DockerState {
    pub(crate) fn new() -> Self {
        Self {
            current_task: Arc::new(Mutex::new(None)),
            last_started: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub(crate) async fn set_current_task(&self, task: Option<Task>) {
        let mut current_task = self.current_task.lock().await;
        if let Some(existing_task) = &*current_task {
            if let Some(new_task) = &task {
                if existing_task.id == new_task.id
                    && existing_task.generate_config_hash() == new_task.generate_config_hash()
                {
                    return;
                }
            }
        }
        *current_task = task;
    }

    pub(crate) async fn update_task_state(&self, task_id: Uuid, state: TaskState) {
        let mut current_task = self.current_task.lock().await;
        if let Some(task) = current_task.as_mut() {
            if task.id == task_id {
                task.state = state;
            }
        } else {
            println!("No current task found when trying to update state");
        }
    }

    pub(crate) async fn get_current_task(&self) -> Option<Task> {
        let current_task = self.current_task.lock().await;
        current_task.clone()
    }

    pub(crate) async fn get_last_started(&self) -> Option<DateTime<Utc>> {
        let last_started = self.last_started.lock().await;
        *last_started
    }

    pub(crate) async fn set_last_started(&self, last_started: DateTime<Utc>) {
        let mut last_started_guard = self.last_started.lock().await;
        *last_started_guard = Some(last_started);
    }

    pub(crate) async fn get_is_running(&self) -> bool {
        let is_running = self.is_running.lock().await;
        *is_running
    }

    pub(crate) async fn set_is_running(&self, is_running: bool) {
        let mut is_running_guard = self.is_running.lock().await;
        *is_running_guard = is_running;
    }
}
