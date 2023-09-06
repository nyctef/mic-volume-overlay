use windows::core::{ComInterface, Result};
use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::System::Com::{CoInitialize, CoUninitialize, CLSCTX_INPROC_SERVER};
use windows::Win32::{Media::Audio::*, System::Com::CoCreateInstance};

fn main() -> Result<()> {
    unsafe {
        CoInitialize(None)?;

        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;

        // todo: handle case where the mic we care about isn't set as the default?
        let endpoint = device_enumerator.GetDefaultAudioEndpoint(eCapture, eConsole)?;

        let info: IAudioMeterInformation = endpoint.Activate(CLSCTX_INPROC_SERVER, None)?;

        loop {
            dbg!(info.GetPeakValue()?);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // TODO: do we need to drop anything?

        CoUninitialize();
        Ok(())
    }
}
