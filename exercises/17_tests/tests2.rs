// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let a: f32 = 2.0;
        let b = 3.0;

        assert_eq!(a, b - 1.0);
    }
}
