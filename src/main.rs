#[cfg(feature = "use_mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    println!("{}", codegen::identify_architecture_aarch64());
}
