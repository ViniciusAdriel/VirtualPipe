fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_bundled_translations("app/lang/");

    slint_build::compile_with_config("app/ui/main.slint", config)
        .unwrap();
}