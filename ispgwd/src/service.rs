//! Future service experiment

use std::thread;
use std::time::Instant;

use crate::net::*;
use crate::prelude::*;

#[derive(Debug, Clone)]
struct IspState {
    #[allow(dead_code)]
    // ISP name
    name: String,

    // Interface associated with ISP
    interface: NetworkInterface,

    // IPv4 gateway ISP
    gateway: Option<Gateway>,

    #[allow(dead_code)]
    // Priority of ISP, lower is more deseriable
    priority: u32,

    // Instant when this ISP becamse the active gateway
    became_active: Option<Instant>,

    #[allow(dead_code)]
    // Instant when this ISP went bad
    became_bad: Option<Instant>,
}

#[derive(Debug, Clone)]
enum ServiceState {
    Discovery,
    Monitor,
}

#[derive(Debug, Clone)]
struct ServiceFsmState {
    // FSM state
    state: ServiceState,

    // Vector of ISP state
    isps: Vec<IspState>,

    // Current default gateway
    default_gw: Option<Gateway>,

    // Is the internet reachable via default gateway
    gw_good: bool,

    // Current active ISP.  Index into isps
    active_isp: Option<usize>,

    // count of iterations
    count: u32,
}

/// Service struct
#[derive(Debug, Clone)]
pub struct Service {
    #[allow(dead_code)]
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
                name: isp.name.to_owned(),
                interface: get_interface(&isp.interface)?,
                priority: isp.priority,
                gateway: get_isp_gateway(&isp.interface)?,
                became_active: None,
                became_bad: None,
            };
            isps.push(isp_state);
        }

        let poll_duration = std::time::Duration::from_millis(config.poll_duration_ms);
        let default_gw = get_default_gw()?;

        Ok(Service {
            config,
            poll_duration,
            inner: ServiceFsmState {
                state: ServiceState::Discovery,
                isps,
                default_gw,
                gw_good: false,
                active_isp: None,
                count: 0,
            },
        })
    }

    /// Start and run the service
    pub fn run(&mut self) -> Result<(), AppError> {
        debug!("Running a service: {:#?}", self);

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
        debug!("Found gateway: {:#?}", self.inner.default_gw);

        // update all the interface info
        for (i, isp) in self.inner.isps.iter_mut().enumerate() {
            isp.interface = get_interface(&isp.interface.name)?;
            isp.gateway = get_isp_gateway(&isp.interface.name)?;
            if isp.gateway == self.inner.default_gw {
                isp.became_active = Some(Instant::now());
                self.inner.active_isp = Some(i);
            }
        }

        // check internet reachability
        let mut good_host = 0;
        let mut bad_host = 0;
        // make this an odd number
        let internet_hosts = [
            "8.8.8.8", // google DNS
            "1.1.1.1", // cloudflare DNS
            "9.9.9.9", // quad9
        ];
        for host in internet_hosts {
            match ping_addr(host)? {
                true => good_host += 1,
                false => bad_host += 1,
            }
        }

        info!("good_host: {}, bad_host: {}", good_host, bad_host);
        self.inner.gw_good = good_host > bad_host;
        info!(
            "good_host: {}, bad_host: {}, gw_good: {}",
            good_host, bad_host, self.inner.gw_good
        );

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
