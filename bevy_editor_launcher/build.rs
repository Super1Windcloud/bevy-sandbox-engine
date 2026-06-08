//! Build script for embedding the Windows executable icon.

fn main() {
    println!("cargo:rerun-if-changed=assets/app.ico");

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/app.ico");
        res.compile()
            .expect("failed to compile Windows icon resources");
    }
}
