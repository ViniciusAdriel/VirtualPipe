fn main() {
    slint_build::compile("gui/main.slint")
        .expect("Slint build failed");
}