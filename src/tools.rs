pub fn should_panic_release(should_me: bool) -> bool {
    if should_me {
        #[cfg(not(debug_assertions))]
        panic!("I think? this code should not be reached...");
    }

    should_me
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
