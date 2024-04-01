use std::path::PathBuf;

use relm4::prelude::*;

pub mod ui;

use ui::usb::*;

fn main() -> anyhow::Result<()> {
    // custom panic handler setup
    human_panic::setup_panic!(human_panic::metadata!());

    // the function usb_detect_all() requires sudo permissions
    // this is to let UsbHandler access to the usb ports
    if let Ok(devices) = detect_usb::usb_detect_all() {
        println!("Number: {}", devices.len());

        for device in devices {
            println!("{:?}", device);
        }
    } else {
        eprintln!("Unable to detect USB devices.");
    }

    Ok(())
}
