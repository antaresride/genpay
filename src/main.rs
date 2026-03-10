#[cfg(feature = "use_mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    #[cfg(feature = "use_cranelift")]
    println!("{}", codegen::identify_architecture_aarch64());
    println!("GenPayLabs!");
}
