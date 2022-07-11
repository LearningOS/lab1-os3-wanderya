//! Process management syscalls

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{
    exit_current_and_run_next,
    suspend_current_and_run_next,
    get_current_status,
    get_current_syscall_times,
    TaskStatus, 
    get_current_start_time
};
use crate::timer::{get_time, get_time_us};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

pub fn cal_time(time: usize) -> usize {
    let sec= time / 1_000_000;
    let usec = time % 1_000_000;

    ((sec & 0xffff) * 1000 + usec / 1000) as usize
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // println!("=== sys_task_info === {}", get_time_us());
    let us = get_time_us();
    
    unsafe {
        *ti = TaskInfo {
            status: get_current_status(),
            syscall_times: get_current_syscall_times(),
            time: cal_time(us),
        };
    }
    0
}
