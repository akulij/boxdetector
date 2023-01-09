#[cfg(windows)]
pub fn is_debugging() -> bool {
    let mut debuggin_flag = false;

    debuggin_flag |= is_debugger_present();
    debuggin_flag |= check_error_debug();

    debuggin_flag
}

fn is_debugger_present() -> bool {
    use winapi::um::debugapi::IsDebuggerPresent;

    unsafe { IsDebuggerPresent() != 0 }
}

fn check_error_debug() -> bool {
    use winapi::um::debugapi::OutputDebugStringA;
    use winapi::um::errhandlingapi::{GetLastError, SetLastError};

    unsafe {
        // random error which debugger will not throw
        let error = 92;

        // push error to error stack
        SetLastError(error);

        // drops error if debugger not connected
        OutputDebugStringA("useless".as_ptr() as *const i8);

        GetLastError() == 92
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_debugger_present() {
        assert!(!is_debugger_present());
    }

    #[cfg(feature = "extra_additional")]
    #[test]
    fn test_error_debug() {
        assert!(!check_error_debug());
    }
}
