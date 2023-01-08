#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_sandboxed() {
        assert!(!is_sandboxed());
    }
}
