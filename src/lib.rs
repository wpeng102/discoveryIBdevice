pub fn is_dpu(device_id: &str) -> bool {
    device_id.starts_with("0xa2") || device_id.starts_with("0xc2")
}
