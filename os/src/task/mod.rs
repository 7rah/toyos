mod context;
mod task;

pub use context::TaskContext;
use log::{debug, trace};

use self::{
    context::switch,
    task::{TaskControlBlock, TaskStatus},
};
use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT},
    link_app::{APP_NAME, APP_NUM},
    loader::{init_app_cx, UserStack, USER_STACK},
    sync::UPSafeCell,
};

pub struct TaskManager {
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    tasks: [TaskControlBlock; APP_NUM],
    current_task: usize,
}

lazy_static::lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = TaskManager::init();
}

impl TaskManager {
    fn init() -> Self {
        let mut tasks = [TaskControlBlock::zero_init(); APP_NUM];

        for (app_id, task) in tasks.iter_mut().enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(app_id));
            task.task_status = TaskStatus::Ready;
        }

        let inner = TaskManagerInner {
            tasks,
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
        debug!("run first task");
        drop(inner);
        let mut _unused = TaskContext::zero_init();

        unsafe { switch(&mut _unused, next_task_cx_ptr) }
        panic!("unreachable in run_first_task!");
    }
    /// Change the status of current `Running` task into `Ready`.
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// Change the status of current `Running` task into `Exited`.
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
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
            panic!("All applications completed!")
        }
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
