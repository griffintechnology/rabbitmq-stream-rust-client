use std::{fmt::Debug, sync::Arc};

use super::metrics::{MetricsCollector, NopMetricsCollector};

#[derive(Clone)]
pub struct ClientOptions {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub v_host: String,
    pub heartbeat: u32,
    pub max_frame_size: u32,
    pub collector: Arc<dyn MetricsCollector>,
}

impl Debug for ClientOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientOptions")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("user", &self.user)
            .field("password", &self.password)
            .field("v_host", &self.v_host)
            .field("heartbeat", &self.heartbeat)
            .field("max_frame_size", &self.max_frame_size)
            .finish()
    }
}
impl Default for ClientOptions {
    fn default() -> Self {
        ClientOptions {
            host: "localhost".to_owned(),
            port: 5552,
            user: "guest".to_owned(),
            password: "guest".to_owned(),
            v_host: "/".to_owned(),
            heartbeat: 60,
            max_frame_size: 1048576,
            collector: Arc::new(NopMetricsCollector {}),
        }
    }
}
