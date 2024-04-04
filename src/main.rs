use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use relm4::prelude::*;

pub mod ui;

use ui::usb::*;

use crate::detect_usb::detect_control_hub;

// when app component init is done (fully initialized), set to true
pub static READY: AtomicBool = AtomicBool::new(false);

// check if app is ready
pub fn is_ready() {
    READY.load(Ordering::Relaxed);
}

lazy_static::lazy_static! {
    static ref GLOBAL_CSS: String = format!("");
}

fn main() -> anyhow::Result<()> {
    // custom panic handler setup
    human_panic::setup_panic!(human_panic::metadata!());

    // the function usb_detect_all() requires sudo permissions
    // this is to let UsbHandler access to the usb ports
    if let Ok(devices) = detect_usb::usb_detect_all() {
        println!("Number: {}", devices.len());

        for device in devices {
            println!("{:?}", device);
            println!("{}", detect_control_hub(device));
        }
    } else {
        println!("Unable to detect USB devices.");
    }

    Ok(())
}
