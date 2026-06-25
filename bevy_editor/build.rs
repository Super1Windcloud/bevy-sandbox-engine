//! Build script for embedding the Windows executable icon.

fn main() {
    println!("cargo:rerun-if-changed=../bevy_editor_launcher/assets/app.ico");

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../bevy_editor_launcher/assets/app.ico");
        res.compile()
            .expect("failed to compile Windows icon resources");
    }
}
