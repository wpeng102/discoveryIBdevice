use libudev::Device;
use std::io;

fn main() {
    match discover_ibs() {
        Ok(_) => {}
        Err(e) => {
            println!("error: {}", e)
        }
    }
}

fn discover_ibs() -> io::Result<()> {
    let device_debug_log = |device: &Device| {
        println!("SysPath - {:?}", device.syspath());
        for p in device.properties() {
            println!("Property - {:?} - {:?}", p.name(), p.value());
        }
        for a in device.attributes() {
            println! {"attribute - {:?} - {:?}", a.name(), a.value()}
        }
    };

    let context = libudev::Context::new()?;
    let mut enumerator = libudev::Enumerator::new(&context)?;
    enumerator.match_subsystem("infiniband")?;

    let devices = enumerator.scan_devices()?;

    for device in devices {
        let slot = match device.parent() {
            Some(parent) => match parent.property_value("PCI_SLOT_NAME") {
                None => {
                    println!("no slot");
                    String::new()
                }
                Some(p) => match p.to_str() {
                    Some(s) => s.to_string(),
                    None => String::new(),
                },
            },
            None => String::new(),
        };

        println!("slot: {}", slot);
        device_debug_log(&device);
    }
    Ok(())
}
