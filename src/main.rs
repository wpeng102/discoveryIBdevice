use libudev::Device;

fn main() {
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
        device_debug_log(&device);
    }
}
