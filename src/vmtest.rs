const DISK_SIZE_MIN: u32 = 60; // GB

#[cfg(windows)]
pub fn is_inside_vm() -> bool {
    let mut flag = false;

    flag |= check_common_vm();

    flag |= check_vmware();

    flag
}

fn check_common_vm() -> bool {
    let mut flag = false;

    flag |= check_disk_size();
    flag |= check_sleep_patch();
    flag |= check_cpu_count();
    flag |= check_native_vhd_boot();

    flag
}

fn calc_disk_gb_size(bps: u32, spc: u32, clusters: u32) -> f32 {
    bps as f32 / 1024.0 * spc as f32 / 1024.0 * clusters as f32 / 1024.0
}
fn check_disk_size() -> bool {
    use winapi::um::fileapi::GetDiskFreeSpaceA;

    let mut sectors: u32 = 0;
    let mut bytes_per_sector: u32 = 0;
    let mut free_clusters: u32 = 0;
    let mut total_clusters: u32 = 0;

    unsafe {
        if GetDiskFreeSpaceA(
            "C:\\\0".as_ptr() as *const i8,
            &mut sectors,
            &mut bytes_per_sector,
            &mut free_clusters,
            &mut total_clusters,
        ) != 0
        {
            calc_disk_gb_size(bytes_per_sector, sectors, total_clusters) <= DISK_SIZE_MIN as f32
        } else {
            false
        }
    }
}

fn check_sleep_patch() -> bool {
    use winapi::um::synchapi::Sleep;
    use winapi::um::sysinfoapi::GetTickCount;

    unsafe {
        let start_time = GetTickCount();

        Sleep(500);

        GetTickCount() - start_time <= 450
    }
}

fn check_cpu_count() -> bool {
    use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};

    let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };

    unsafe {
        GetSystemInfo(&mut sys_info);
    }

    sys_info.dwNumberOfProcessors < 2
}

fn check_native_vhd_boot() -> bool {
    use winapi::um::libloaderapi::GetModuleHandleA;
    use winapi::um::libloaderapi::GetProcAddress;

    let mut is_native = false;

    unsafe {
        let func_native = GetProcAddress(
            GetModuleHandleA("kernel32".as_ptr() as *const i8),
            "IsNativeVhdBoot".as_ptr() as *const i8,
        );
        if func_native as u32 != 0 {
            let fnptr = func_native as *const ();
            let fnptr: fn(&mut bool) -> () = std::mem::transmute(fnptr);
            fnptr(&mut is_native);
        }

        is_native
    }
}
