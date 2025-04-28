//! File and filesystem-related syscalls
#[allow(unused)]
use crate::fs::{open_file, OpenFlags, Stat, linkat, find_inode_id, get_some, unlinkat};
use crate::mm::{translated_byte_buffer, translated_str, UserBuffer};
use crate::task::{current_task, current_user_token};

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_write", current_task().unwrap().pid.0);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_read", current_task().unwrap().pid.0);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        trace!("kernel: sys_read .. file.read");
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    trace!("kernel:pid[{}] sys_open", current_task().unwrap().pid.0);
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
   
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        // println!("name:{}", find_inode_id(path.as_str()));
        // get_some(path.as_str());
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    trace!("kernel:pid[{}] sys_close", current_task().unwrap().pid.0);
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

/// YOUR JOB: Implement fstat.
#[allow(unused)]
pub fn sys_fstat(fd: usize, st: *mut Stat) -> isize {
    trace!(
        "kernel:pid[{}] sys_fstat NOT IMPLEMENTED",
        current_task().unwrap().pid.0
    );


    use crate::fs::StatMode;


    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(osinode) = &inner.fd_table[fd] {
        let inode_id = osinode.get_inode_id();
        let link = inode_id >> 20 ;
        let mut f = 0 as u32;
        let l = link as u32;
        // println!("file:{}", (inode_id >> 12) & 0xf);
        let file = match ((inode_id >> 12) & 0xf) {
            0 => {f = 0o040000;StatMode::DIR},
            1 => {f = 0o100000;StatMode::FILE},
            _ => {StatMode::NULL},
        };
        let inode_id = (inode_id) & (0xfff);
        
        let inode_id = inode_id as u64;

        // 获取目标缓冲区
        let buffer_size = core::mem::size_of::<Stat>();
        let mut buffers = translated_byte_buffer(
            token,
            st as *const u8,
            buffer_size
        );

        // println!("inode_id:{}", inode_id);
        // buffers[0][0] = sec as u8;
        // buffers[0][1] = (sec >> 8) as u8;
        // buffers[0][2] = (sec >> 16) as u8;
        // buffers[0][3] = (sec >> 24) as u8;
        // buffers[0][4] = (sec >> 32) as u8;
        // buffers[0][5] = (sec >> 40) as u8;
        // buffers[0][6] = (sec >> 48) as u8;
        // buffers[0][7] = (sec >> 56) as u8;
    
        buffers[0][8] = inode_id as u8;
        buffers[0][9] = (inode_id >> 8) as u8;
        buffers[0][10] = (inode_id >> 16) as u8;
        buffers[0][11] = (inode_id >> 24) as u8;
        buffers[0][12] = (inode_id >> 32) as u8;
        buffers[0][13] = (inode_id >> 40) as u8;
        buffers[0][14] = (inode_id >> 48) as u8;
        buffers[0][15] = (inode_id >> 56) as u8;

        buffers[0][16] = f as u8;
        buffers[0][17] = (f >> 8) as u8;
        buffers[0][18] = (f >> 16) as u8;
        buffers[0][19] = (f >> 24) as u8;

        buffers[0][20] = l as u8;
        buffers[0][21] = (l >> 8) as u8;
        buffers[0][22] = (l >> 16) as u8;
        buffers[0][23] = (l >> 24) as u8;

        0
    }
    else {
        -1
    }
    

}

/// YOUR JOB: Implement linkat.
pub fn sys_linkat(old_name: *const u8, new_name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_linkat NOT IMPLEMENTED",
        current_task().unwrap().pid.0
    );

    let token = current_user_token();
    let old = translated_str(token, old_name);
    let new = translated_str(token, new_name);
    linkat(old.as_str(), new.as_str())
}

/// YOUR JOB: Implement unlinkat.
pub fn sys_unlinkat(name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_unlinkat NOT IMPLEMENTED",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let name = translated_str(token, name);
    unlinkat(name.as_str())
}
