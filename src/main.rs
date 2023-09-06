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

        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;

        // todo: handle case where the mic we care about isn't set as the default?
        let endpoint = device_enumerator.GetDefaultAudioEndpoint(eCapture, eConsole)?;

        let info: IAudioMeterInformation = endpoint.Activate(CLSCTX_INPROC_SERVER, None)?;

        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(320.0, 240.0)),
            ..Default::default()
        };

        eframe::run_simple_native("My egui App", options, move |ctx, frame| {
            frame.set_always_on_top(true);

            egui::CentralPanel::default().show(ctx, |ui| {
                let level = info.GetPeakValue().unwrap_or(-1_f32);
                ui.add(meter(level));

                ctx.request_repaint_after(Duration::from_millis(100));
            });
        })
        .expect("Failed to run app");

        // loop {
        //     dbg!(info.GetPeakValue()?);
        //     std::thread::sleep(std::time::Duration::from_millis(100));
        // }

        // TODO: do we need to drop anything?

        CoUninitialize();
        Ok(())
    }
}
