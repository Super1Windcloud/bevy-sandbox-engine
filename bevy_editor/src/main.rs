use bevy_sandbox_engine::App;

fn main() {
    bevy_sandbox_engine::locale_env::normalize_process_locale();
    App::new().run();
}
