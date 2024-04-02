use rusb::{
    Context, Device, DeviceDescriptor, DeviceHandle, DeviceList, Error, Language, PrimaryLanguage,
    UsbContext,
};
use std::time::Duration;

pub fn usb_detect_all() -> Result<Vec<(u8, String, String, String)>, rusb::Error> {
    let context = Context::new()?;

    let device_list = context.devices()?;
    let mut devices_info = Vec::new();

    for device in device_list.iter() {
        let handle_device = device.open()?;
        let device_info = get_device_info(&handle_device)?;
        devices_info.push(device_info);
    }

    Ok(devices_info)
}

pub fn detect_control_hub(information: (u8, String, String, String)) -> bool {
    let manufacturer = information.1.to_lowercase();
    let product = information.2.to_lowercase();

    manufacturer == "rev robotics" && product.starts_with("control hub")
}

fn get_device_info<T: UsbContext>(
    handle: &DeviceHandle<T>,
) -> Result<(u8, String, String, String), rusb::Error> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    let active_configurations = handle.active_configuration()?;
    let manufacturer: String;
    let product: String;
    let serial_number: String;

    if !languages.is_empty() {
        let language = languages[0];
        manufacturer = handle
            .read_manufacturer_string(language, &device_desc, timeout)
            .unwrap_or("Not found".to_string());
        product = handle
            .read_product_string(language, &device_desc, timeout)
            .unwrap_or("Not found".to_string());
        serial_number = handle
            .read_serial_number_string(language, &device_desc, timeout)
            .unwrap_or("Not found".to_string());
    } else {
        (manufacturer, product, serial_number) = (
            "Unable to find".to_string(),
            "Unable to find".to_string(),
            "Unable to find".to_string(),
        );
    }

    Ok((active_configurations, manufacturer, product, serial_number))
}
