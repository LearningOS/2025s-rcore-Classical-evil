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
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    use crate::mm::translated_byte_buffer;
    use crate::task::current_user_token;

    use crate::timer::get_time_us;
    let us = get_time_us();
    let mut buffers = translated_byte_buffer(current_user_token(), ts as *const u8, 16);
    let sec = us / 1_000_000;
    let usec =  us % 1_000_000;

    buffers[0][0] = sec as u8;
    buffers[0][1] = (sec >> 8) as u8;
    buffers[0][2] = (sec >> 16) as u8;
    buffers[0][3] = (sec >> 24) as u8;
    buffers[0][4] = (sec >> 32) as u8;
    buffers[0][5] = (sec >> 40) as u8;
    buffers[0][6] = (sec >> 48) as u8;
    buffers[0][7] = (sec >> 56) as u8;

    buffers[0][8] = usec as u8;
    buffers[0][9] = (usec >> 8) as u8;
    buffers[0][10] = (usec >> 16) as u8;
    buffers[0][11] = (usec >> 24) as u8;
    buffers[0][12] = (usec >> 32) as u8;
    buffers[0][13] = (usec >> 40) as u8;
    buffers[0][14] = (usec >> 48) as u8;
    buffers[0][15] = (usec >> 56) as u8;

    //                 buffers[0][0].into()
    // unsafe {
    //     *ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
#[allow(unused)]
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    use super::TRACE;
     use crate::task::TASK_MANAGER;
     match trace_request {
         0 => {
                if TASK_MANAGER.read((id >> 12) << 12) {
                    use crate::task::current_user_token;
                    use crate::mm::translated_byte_buffer;
                    let buffers = translated_byte_buffer(current_user_token(), id as *const u8, 1);
                    buffers[0][0].into()
                }
                else {-1}                
         }
         1 => {
            if TASK_MANAGER.write((id >> 12) << 12) {
                use crate::task::current_user_token;
                use crate::mm::translated_byte_buffer;
                let mut buffers = translated_byte_buffer(current_user_token(), id as *const u8, 1);
                buffers[0][0] = data as u8;
                0
            }
            else {-1}  
             
         }
         2 => {
             unsafe{TRACE[TASK_MANAGER.get_current_task()][id]}
         }
         _ => -1
     }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
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


