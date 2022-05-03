use core::{fmt, time::Duration};

use super::TaskContext;
use crate::syscall::{SyscallId, MAX_SYSCALL_NUM};

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

#[derive(Debug, Clone, Copy)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub name: &'static str,
    pub call: Call,
    pub user_time: Duration,
    pub kernel_time: Duration,
}

#[derive(Clone, Copy)]
pub struct Call {
    inner: [SyscallInfo; MAX_SYSCALL_NUM],
}

impl Call {
    fn zero_init() -> Self {
        Self {
            inner: [SyscallInfo {
                id: SyscallId::Unsupported,
                times: 0,
            }; MAX_SYSCALL_NUM],
        }
    }
    pub fn add(&mut self, syscall_id: SyscallId) -> Option<()> {
        let find = |id| self.inner.iter().position(|&x| x.id == id);

        if let Some(pos) = find(syscall_id) {
            self.inner[pos].times += 1;
            Some(())
        } else if let Some(pos) = find(SyscallId::Unsupported) {
            self.inner[pos].times = 1;
            self.inner[pos].id = syscall_id;
            Some(())
        } else {
            None
        }
    }
}

impl fmt::Debug for Call {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let pos = self
            .inner
            .iter()
            .position(|&x| x.id == SyscallId::Unsupported)
            .unwrap_or(self.inner.len());
        let v = &self.inner[..pos];
        write!(f, "{:?}", v)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SyscallInfo {
    pub id: SyscallId,
    pub times: usize,
}

impl TaskInfo {
    pub fn zero_init() -> Self {
        Self {
            id: 0,
            status: TaskStatus::Uninit,
            name: "",
            call: Call::zero_init(),
            user_time: Duration::default(),
            kernel_time: Duration::default(),
        }
    }

    pub fn add_user_time(&mut self, time: Duration) {
        self.user_time += time;
    }
    pub fn add_kernel_time(&mut self, time: Duration) {
        self.kernel_time += time;
    }
}
