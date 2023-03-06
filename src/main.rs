extern crate winapi_util;

use winapi_util::winapi::um::processthreadsapi::GetCurrentProcess;
use winapi_util::winapi::shared::minwindef::FILETIME;
use winapi_util::winapi::um::sysinfoapi::{GetSystemTimeAsFileTime, GetTickCount64};

fn main() {
    let mut idle_time = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let mut kernel_time = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let mut user_time = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let mut uptime = 0;

    unsafe {
        GetSystemTimeAsFileTime(&mut idle_time);
        GetCurrentProcess();
        GetProcessTimes(
            GetCurrentProcess(),
            &mut dummy,
            &mut dummy,
            &mut kernel_time,
            &mut user_time,
        );
        uptime = GetTickCount64() / 1000;
    }

    let uptime_hours = uptime / 3600;
    let uptime_minutes = (uptime % 3600) / 60;
    let uptime_seconds = uptime % 60;

    println!(
        "System uptime: {} hours, {} minutes, {} seconds",
        uptime_hours, uptime_minutes, uptime_seconds
    );
}
