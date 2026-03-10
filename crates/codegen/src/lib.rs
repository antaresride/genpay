//register the allocator inside your library file, but only during tests
#[cfg(all(test, feature = "use_mimalloc"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "use_mimalloc")]
pub fn print_stats() {
    // libmimalloc-sys provides the raw C bindings.
    // Passing None/null_mut uses default stdout and no custom output function.
    // This allows you to see how much memory LLVM used
    unsafe {
        libmimalloc_sys::mi_stats_print_out(None, std::ptr::null_mut());
    }
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
    use std::time::Instant;

    fn heavy_allocation_task() {
        // Simulate LLVM IR generation: millions of small allocations
        let mut v = Vec::with_capacity(100_000);
        for i in 0..1_000_000 {
            v.push(format!("instruction_{}", i));
            if i % 100 == 0 {
                v.clear();
            }
        }
    }

    #[test]
    fn bench_allocator_performance() {
        let start = Instant::now();
        heavy_allocation_task();
        let duration = start.elapsed();

        let allocator_type = if cfg!(feature = "use_mimalloc") {
            "mimalloc"
        } else {
            "System"
        };
        println!(
            "\n[Result] {} allocator took: {:?}",
            allocator_type, duration
        );
        // Add this line to trigger the printout
        #[cfg(feature = "use_mimalloc")]
        print_stats();
    }
}
