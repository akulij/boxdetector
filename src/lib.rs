mod debugtest;
mod tools;
use tools::should_panic_release as panicer;
mod vmtest;

unsafe fn _is_sandboxed(flag: *mut bool) {
    // init value to use in |= operator
    *flag = false;

    // check if debug attached
    *flag |= panicer( debugtest::is_debugging() );

    // check for popular vm's
    *flag |= panicer( vmtest::is_inside_vm() );
}

#[cfg(windows)]
#[inline(always)]
#[doc = "
Returns true if program debugged or sanboxed.
Panics if detected debugger or sandbox in --release mode.
"]
pub fn is_sandboxed() -> bool {
    let sanboxed_flag: bool;

    _is_sandboxed(&mut sanboxed_flag);

    sanboxed_flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_sandboxed() {
        assert!(!is_sandboxed());
    }
}
