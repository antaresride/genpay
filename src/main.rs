fn main() {
    #[cfg(feature = "use_cranelift")]
    println!("{}", codegen::identify_architecture_aarch64());
}
