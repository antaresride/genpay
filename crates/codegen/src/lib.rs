use target_lexicon::Triple;

pub fn identify_architecture_aarch64() -> String {
    Triple::host().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_identify_architecture_aarch64() {
        let arquitecture: &'static str = "aarch64-unknown-linux-gnu";
        assert_eq!(identify_architecture_aarch64().to_string(), arquitecture);
    }
}
