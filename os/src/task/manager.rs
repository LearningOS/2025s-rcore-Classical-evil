//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // println!("fetch pid:{}", self.ready_queue[0].pid.0);
        self.ready_queue.pop_front()
    }

    ///
    pub fn stride(&mut self) -> Option<Arc<TaskControlBlock>> {
        // println!("stride pid:{}", self.ready_queue[0].pid.0);
        let mut min_stride = 0x1000;
        let mut min_index = 0;
        let mut i = 0;
        for _item in &self.ready_queue {
            let task_inner = self.ready_queue[i].inner_exclusive_access();
            if min_stride > task_inner.stride {
                min_stride = task_inner.stride;
                min_index = i;
            }

            i += 1;
        }

        self.ready_queue.swap(min_index, i - 1); 
        self.ready_queue.pop_back()
    }
    
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

#[allow(unused)]
static mut FLAG: bool = false;
pub fn stride() -> Option<Arc<TaskControlBlock>> {

    TASK_MANAGER.exclusive_access().fetch()
    
    // match unsafe{FLAG} {
    //     false => {unsafe{FLAG = true}; TASK_MANAGER.exclusive_access().fetch()},
    //     true  => TASK_MANAGER.exclusive_access().stride(),
    // }
}
