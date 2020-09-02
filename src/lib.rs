/// Hello from Magrathea
pub fn answer() -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deep_thought_test() {
        assert_eq!(answer(), 42);
    }
}
