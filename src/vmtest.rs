#[cfg(windows)]
pub fn is_inside_vm() -> bool {
    let mut flag = false;

    flag |= check_common_vm();

    flag |= check_vmware();

    flag
}
