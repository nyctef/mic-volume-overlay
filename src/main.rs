#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::Duration;

use eframe::egui;
use windows::core::Result;
use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::System::Com::{CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER};
use windows::Win32::{Media::Audio::*, System::Com::CoCreateInstance};

mod meter;
use meter::meter;

fn main() -> Result<()> {
    unsafe {
        CoInitialize(None)?;

        let get_level = get_audio_meter()?;

        run_ui(get_level);

        CoUninitialize();
        Ok(())
    }
}

unsafe fn get_audio_meter() -> Result<impl Fn() -> f32> {
    // TODO: do we need to worry about these objects getting dropped
    // when they go out of scope, making the returned fn invalid?
    //
    // alternatively, do we need to manually drop anything?
    let device_enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;
    let endpoint = device_enumerator.GetDefaultAudioEndpoint(eCapture, eConsole)?;
    let info: IAudioMeterInformation = endpoint.Activate(CLSCTX_INPROC_SERVER, None)?;
    let get_level = move || info.GetPeakValue().unwrap_or(-1_f32);
    Ok(get_level)
}

fn run_ui(get_level: impl Fn() -> f32 + 'static) {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(50.0, 240.0)),
        ..Default::default()
    };
    eframe::run_simple_native("My egui App", options, move |ctx, frame| {
        frame.set_always_on_top(true);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.add(meter(get_level()));
                },
            );

            ctx.request_repaint_after(Duration::from_millis(100));
        });
    })
    .expect("Failed to run app");
}
