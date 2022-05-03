mod context;
mod task;

use core::time::Duration;

pub use context::TaskContext;
use log::{info, trace};
pub use task::TaskInfo;

use self::{
    context::switch,
    task::{TaskControlBlock, TaskStatus},
};
use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT},
    link_app::{APP_NAME, APP_NUM},
    loader::{init_app_cx, UserStack, USER_STACK},
    sync::UPSafeCell,
    syscall::SyscallId,
    timer::timer_now,
};

pub struct TaskManager {
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    infos: [TaskInfo; APP_NUM],
    tasks: [TaskControlBlock; APP_NUM],
    current_task: usize,
    timestamp: Duration,
}

lazy_static::lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = TaskManager::init();
}

impl TaskManager {
    fn init() -> Self {
        let mut tasks = [TaskControlBlock::zero_init(); APP_NUM];

        let mut infos = [(); APP_NUM].map(|_| TaskInfo::zero_init());

        for (app_id, (task, info)) in tasks.iter_mut().zip(infos.iter_mut()).enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(app_id));
            task.task_status = TaskStatus::Ready;

            info.status = TaskStatus::Ready;
            info.name = APP_NAME[app_id];
            info.id = app_id;
        }

        let inner = TaskManagerInner {
            tasks,
            infos,
            timestamp: Duration::default(),
            current_task: 0,
        };
        TaskManager {
            inner: unsafe { UPSafeCell::new(inner) },
        }
    }
    pub fn get_current_task(&self) -> usize {
        let inner = self.inner.exclusive_access();
        inner.current_task
    }
    pub fn get_current_task_name(&self) -> &'static str {
        let inner = self.inner.exclusive_access();
        APP_NAME[inner.current_task]
    }
    pub fn get_current_task_status(&self) -> TaskStatus {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].task_status
    }
    pub fn get_current_task_stack(&self) -> &UserStack {
        let inner = self.inner.exclusive_access();
        &USER_STACK[inner.current_task]
    }
    pub fn get_current_task_data_section(&self) -> &[u8] {
        let inner = self.inner.exclusive_access();
        let get_base_i = |app_id| APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT;
        let app_id = inner.current_task;
        unsafe { core::slice::from_raw_parts(get_base_i(app_id) as *const u8, APP_SIZE_LIMIT) }
    }
    pub fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        let mut _unused = TaskContext::zero_init();
        drop(inner);
        TASK_MANAGER.set_timestamp(timer_now());

        unsafe { switch(&mut _unused, next_task_cx_ptr) }
        panic!("unreachable in run_first_task!");
    }
    /// Change the status of current `Running` task into `Ready`.
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
        inner.infos[current].status = TaskStatus::Ready;
    }

    /// Change the status of current `Running` task into `Exited`.
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
        inner.infos[current].status = TaskStatus::Exited;
    }

    pub fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;

        (current + 1..current + APP_NUM + 1)
            .map(|id| id % APP_NUM)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    pub fn run_next_task(&self) {
        let next = self.find_next_task();
        if let Some(next) = next {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;

            let current_name = APP_NAME[current];
            let next_name = APP_NAME[next];
            trace!("run next task! current:{current} {current_name} next:{next} {next_name}");

            inner.current_task = next;
            inner.tasks[next].task_status = TaskStatus::Running;

            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);

            unsafe { switch(current_task_cx_ptr, next_task_cx_ptr) }
        } else {
            info!("{:#?}", self.inner.exclusive_access().infos);
            panic!("All applications completed!")
        }
    }

    pub fn get_current_task_info(&self) -> TaskInfo {
        let inner = self.inner.exclusive_access();
        inner.infos[inner.current_task]
    }

    pub fn add_current_task_info_call_times(&self, syscall_id: SyscallId) -> Option<()> {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let call = &mut inner.infos[current].call;
        call.add(syscall_id)
    }

    pub fn add_current_task_user_time(&self, time: Duration) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.infos[current].add_user_time(time);
    }

    pub fn add_current_task_kernel_time(&self, time: Duration) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.infos[current].add_kernel_time(time);
    }

    pub fn get_timestamp(&self) -> Duration {
        self.inner.exclusive_access().timestamp
    }
    pub fn set_timestamp(&self, timestamp: Duration) {
        self.inner.exclusive_access().timestamp = timestamp
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// suspend current task
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}
