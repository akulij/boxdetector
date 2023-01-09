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
