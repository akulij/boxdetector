pub fn should_panic_release(should_me: bool) -> bool {
    if should_me {
        #[cfg(not(debug_assertions))]
        panic!("I think? this code should not be reached...");
    }

    should_me
}

#[cfg(windows)]
pub fn is_wow64() -> bool {
    use winapi::um::libloaderapi::GetModuleHandleA;
    use winapi::um::libloaderapi::GetProcAddress;
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::um::winnt::HANDLE;

    let mut res = false;

    unsafe {
        let func_native = GetProcAddress(
            GetModuleHandleA("kernel32".as_ptr() as *const i8),
            "IsWow64Process".as_ptr() as *const i8,
        );
        if func_native as u32 != 0 {
            let fnptr = func_native as *const ();
            let fnptr: fn(HANDLE, &mut bool) -> u32 = std::mem::transmute(fnptr);
            if fnptr(GetCurrentProcess(), &mut res) != 0 {
                return res;
            } else {
                return false;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(debug_assertions))]
    #[test]
    #[should_panic]
    fn panic_release() {
        should_panic_release(true);
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn not_panic_release() {
        assert_eq!(should_panic_release(false), false);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn not_panic_debug() {
        assert_eq!(should_panic_release(false), false);
        assert_eq!(should_panic_release(true), true);
    }
}
