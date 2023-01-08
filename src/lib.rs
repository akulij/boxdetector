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
