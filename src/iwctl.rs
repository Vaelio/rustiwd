use crate::networks::Network;
use crate::devices::Device;
use std::process::Command;
use std::time::{Instant, Duration};


struct IwController<'a> {
    devices: Vec<Device>,
    networks: Vec<Network>,
    selected_device: &'a Device,
    last_scan: Instant,
}

impl IwController<'_> {
    fn scan(&self) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("iwctl")
                .args(["station", &self.selected_device.name(), "scan"])
                .output()?;

        Ok(())
    }

    fn get_networks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.last_scan.elapsed().as_secs() > 10 {
            self.scan();
            self.last_scan = Instant::now();
        }

        let output = Command::new("iwctl")
                .args(["station", &self.selected_device.name(), "get-networks", "rssi-dbms"])
                .output()?;
        /* TODO: PARSE OUTPUT */
        Ok(())
    }
}
