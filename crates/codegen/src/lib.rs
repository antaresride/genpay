#[cfg(feature = "use_cranelift")]
use target_lexicon::Triple;

#[cfg(feature = "use_cranelift")]
pub fn identify_architecture_aarch64() -> String {
    Triple::host().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_identify_architecture_aarch64() {
        let detected = identify_architecture_aarch64();
        let valid_triples = ["aarch64-unknown-linux-gnu", "aarch64-apple-darwin"];
        assert!(
            valid_triples.contains(&detected.as_str()),
            "Detected '{}' is not a valid aarch64 triple",
            detected
        );
    }
}
