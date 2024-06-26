slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    use soloud::*;
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let sl = Soloud::default().unwrap();
        let mut wav = audio::Wav::default();
        wav.load_mem(include_bytes!("audio.wav")).unwrap();

        let ui_handle: slint::Weak<AppWindow> = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
            sl.play(&wav);
            while sl.voice_count() > 0 {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });

    ui.run()
}
