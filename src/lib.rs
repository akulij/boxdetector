#[cfg(windows)]
#[inline(always)]
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
