pub mod cranelift;
pub mod llvm;

pub trait LinkerBackend {
    fn link(&self, output_path: &str) -> std::io::Result<()>;
    // Add other shared methods here
}
