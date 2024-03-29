use rusb::{Context, Device, DeviceDescriptor, DeviceList, Error, UsbContext};

pub fn usb_detect_all(
) -> Result<Vec<(DeviceDescriptor, String, u16, u16, String, Option<String>)>, rusb::Error> {
    let context = Context::new()?;

    let device_list = context.devices()?;
    let mut devices_info = Vec::new();

    for device in device_list.iter() {
        let device_info = get_device_info(&device)?;
        devices_info.push(device_info);
    }

    Ok(devices_info)
}

fn get_device_info(
    device: &Device<Context>,
) -> Result<(DeviceDescriptor, String, u16, u16, String, Option<String>), rusb::Error> {
    let device_desc = device.device_descriptor()?;
    let product_id = device_desc.product_id();
    let vendor_id = device_desc.vendor_id();

    let serial_number_index = device_desc.serial_number_string_index();
    let manufacturer_index = device_desc.manufacturer_string_index();
    let product_index = device_desc.product_string_index();

    /*
    let serial_number = string_from_index(device, &serial_number_index);
    let manufacturer = string_from_index(device, &manufacturer_index);
    let product_name = string_from_index(device, &product_index);
    */

    let product_name = "PLACEHOLDER".to_string();
    let manufacturer = "PLACEHOLDER".to_string();
    let serial_number = Some("PLACEHOLDER".to_string());

    Ok((
        device_desc,
        manufacturer,
        vendor_id,
        product_id,
        product_name,
        serial_number,
    ))
}

fn string_from_index(device: &Device<Context>, item_index: &Option<u8>) -> Result<String, Error> {
    Ok("PLACEHOLDER".to_string())
}
