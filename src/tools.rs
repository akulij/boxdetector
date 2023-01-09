use winapi::shared::minwindef::HKEY;

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
            let fnptr: fn(HANDLE, &mut bool) -> u32 =
                std::mem::transmute(fnptr);
            if fnptr(GetCurrentProcess(), &mut res) != 0 {
                return res;
            } else {
                return false;
            }
        }
    }
    false
}

#[cfg(windows)]
pub fn regkey_exists(hkey: HKEY, regkey: &str) -> bool {
    use winapi::shared::winerror::ERROR_SUCCESS;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::winnt::KEY_READ;
    use winapi::um::winnt::KEY_WOW64_64KEY;
    use winapi::um::winreg::RegCloseKey;
    use winapi::um::winreg::RegOpenKeyExA;

    unsafe {
        let mut regkey_h: HKEY = std::mem::zeroed();

        let mut access_keys = KEY_READ;
        if is_wow64() {
            access_keys |= KEY_WOW64_64KEY;
        }

        let regkey =
            std::ffi::CString::new(regkey).expect("error creating cstring");
        let ret =
            RegOpenKeyExA(hkey, regkey.as_ptr(), 0, access_keys, &mut regkey_h);
        if ret as u32 == ERROR_SUCCESS {
            RegCloseKey(regkey_h);

            true
        } else {
            false
        }
    }
}

#[cfg(windows)]
pub fn regkey_value_contains(
    hkey: HKEY,
    regkey: &str,
    value: &str,
    containable: &str,
) -> Option<bool> {
    use winapi::shared::winerror::ERROR_SUCCESS;
    use winapi::um::winnt::KEY_READ;
    use winapi::um::winnt::KEY_WOW64_64KEY;
    use winapi::um::winreg::RegOpenKeyExA;
    use winapi::um::winreg::RegQueryValueA;

    unsafe {
        let mut regkey_h: HKEY = std::mem::zeroed();

        let mut access_keys = KEY_READ;
        if is_wow64() {
            access_keys |= KEY_WOW64_64KEY;
        }

        let ret =
            RegOpenKeyExA(hkey, regkey.as_ptr(), 0, access_keys, &mut regkey_h);
        if ret as u32 == ERROR_SUCCESS {
            let value_container = String::with_capacity(1024);

            Some(false)
        } else {
            None
        }
    }
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

    #[test]
    fn test_regkey_exists() {
        use winapi::um::winreg::HKEY_LOCAL_MACHINE;

        assert!(!regkey_exists(
            HKEY_LOCAL_MACHINE,
            "SOFTWARE\\does not exists"
        ));

        assert!(regkey_exists(HKEY_LOCAL_MACHINE, "SOFTWARE"));
    }
}
