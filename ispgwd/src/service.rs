//! Future service experiment

use std::thread;

use crate::net::*;
use crate::prelude::*;

#[derive(Debug, Clone)]
struct IspState {
    name: String,
    interface: NetworkInterface,
    priority: u32,
}

#[derive(Debug, Clone)]
enum ServiceState {
    Discovery,
    Monitor,
}

#[derive(Debug, Clone)]
struct ServiceFsmState {
    isps: Vec<IspState>,
    default_gw: Option<Gateway>,
    state: ServiceState,
    count: u32,
}

/// Service struct
#[derive(Debug, Clone)]
pub struct Service {
    config: IspgwdConfig,
    poll_duration: std::time::Duration,
    inner: ServiceFsmState,
}

impl Service {
    /// Create a new service
    pub fn new(config: IspgwdConfig) -> Result<Service, AppError> {
        let mut isp_configs = config.isp_configs.clone();
        isp_configs.sort_by(|a, b| a.priority.cmp(&b.priority));

        let mut isps = Vec::new();
        for isp in isp_configs {
            let isp_state = IspState {
                name: isp.name,
                interface: get_interface(&isp.interface)?,
                priority: isp.priority,
            };
            isps.push(isp_state);
        }

        let poll_duration = std::time::Duration::from_millis(config.poll_duration_ms);
        let default_gw = get_default_gw()?;

        Ok(Service {
            config,
            poll_duration,
            inner: ServiceFsmState {
                isps,
                default_gw,
                state: ServiceState::Discovery,
                count: 0,
            },
        })
    }

    /// Start and run the service
    pub fn run(&mut self) -> Result<(), AppError> {
        info!("Running a service: {:#?}", self);

        loop {
            let should_stop = self.handle_state()?;
            if should_stop {
                break;
            }
            thread::sleep(self.poll_duration);
            self.inner.count += 1;
        }

        Ok(())
    }

    fn handle_discovery(&mut self) -> Result<bool, AppError> {
        info!("Discovery");

        // get default gateway
        self.inner.default_gw = get_default_gw()?;
        info!("Found gateway: {:#?}", self.inner.default_gw);

        // update all the interface info
        for isp in self.inner.isps.iter_mut() {
            isp.interface = get_interface(&isp.interface.name)?;
        }

        self.inner.state = ServiceState::Monitor;
        Ok(false)
    }

    fn handle_monitor(&mut self) -> Result<bool, AppError> {
        info!("Monitor");

        if self.inner.count > 2 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn handle_state(&mut self) -> Result<bool, AppError> {
        match self.inner.state {
            ServiceState::Discovery => self.handle_discovery(),
            ServiceState::Monitor => self.handle_monitor(),
        }
    }
}
