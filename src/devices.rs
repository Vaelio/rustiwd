enum DeviceMode {
    Station,
    Ap
}

pub struct Device {
    name: String,
    mac_addr: String,
    powered: bool,
    adapter: String,
    Mode: DeviceMode
}

impl Device {
    pub fn name(&self) ->&str {
        &self.name
    }
}
