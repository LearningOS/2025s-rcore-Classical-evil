//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    -1
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    // use crate::mm::{
    //     MapPermission, MapType, MapArea,
    // };
    // if start % 4096 != 0 {
    //     return -1;
    // }
    // if port & !0x7 != 0 {
    //     return -1;
    // }
    // if port & 0x7 == 0 {
    //     return -1;
    // }
    // let mut i = start;
    // loop {
    //     if i > start + len {
    //         break;
    //     }
    //     if let Some(_x) = KERNEL_SPACE.exclusive_access().page_table.find_pte(i) {
    //         return -1;
    //     }
    //     i += 4096;

    // }
    // let mut inner = TASK_MANAGER.inner.exclusive_access();
    // TASK_MANAGER.get_current_taskblock().memory_set.push(MapArea::new(
    //                                 start.into(), 
    //                                 ((start + len + 4096 - 1) / 4096).into(), 
    //                                 MapType::Framed, MapPermission::from_bits(port as u8).unwrap())
    //                                 , None);

    // TASK_MANAGER.get_current_taskblock().memory_set.insert_framed_area(
    //                                 start.into(), 
    //                                 ((start + len + 4096 - 1) / 4096).into(), 
    //                                 MapType::Framed, MapPermission::from_bits(port as u8).unwrap());
  
    use crate::task::TASK_MANAGER;
    TASK_MANAGER.mmap(start, len, port)
    
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    use crate::task::TASK_MANAGER;
    TASK_MANAGER.munmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}


