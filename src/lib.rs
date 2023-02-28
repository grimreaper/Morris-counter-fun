pub fn do_nothing() -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_nothing() {
        assert_eq!(do_nothing(), false);
    }
}
