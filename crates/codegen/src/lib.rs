use target_lexicon::Triple;

pub fn identify_architecture() -> String {
    Triple::from(Triple::host()).to_string()
}

#[cfg(test)]
mod tests {
    use target_lexicon::{Aarch64Architecture, Architecture, Triple};

    #[test]
    fn test_identify_architecture() {
        let triple = Triple::from(Triple::host());
        assert_eq!(
            triple.architecture,
            Architecture::Aarch64(Aarch64Architecture::Aarch64)
        );
    }
}
