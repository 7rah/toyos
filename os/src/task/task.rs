use super::TaskContext;

#[derive(Copy, Clone, Debug)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}

impl TaskControlBlock {
    pub fn zero_init() -> Self {
        TaskControlBlock {
            task_status: TaskStatus::Uninit,
            task_cx: TaskContext::zero_init(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    Uninit,
    Ready,
    Running,
    Exited,
}
