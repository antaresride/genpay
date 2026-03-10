//register the allocator inside your library file, but only during tests
#[cfg(all(test, feature = "use_mimalloc"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "use_mimalloc")]
pub fn print_stats() {
    // This allows you to see how much memory LLVM used
    mimalloc::MiMalloc::stat_print();
}

pub mod cranelift;
pub mod llvm;

pub trait LinkerBackend {
    fn link(&self, output_path: &str) -> std::io::Result<()>;
    // Add other shared methods here
}
