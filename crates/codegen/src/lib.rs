//register the allocator inside your library file, but only during tests
#[cfg(all(test, feature = "use_mimalloc"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "use_mimalloc")]
pub fn print_stats() {
    // This allows you to see how much memory LLVM used
    mimalloc::MiMalloc::stat_print();
}

#[cfg(feature = "use_cranelift")]
use target_lexicon::Triple;

#[cfg(feature = "use_cranelift")]
pub fn identify_architecture_aarch64() -> String {
    Triple::host().to_string()
}
#[cfg(feature = "use_cranelift")]
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
