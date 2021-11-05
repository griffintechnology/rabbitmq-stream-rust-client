#[derive(Clone, Debug)]
pub struct ClientOptions {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub v_host: String,
    pub heartbeat: u32,
    pub max_frame_size: u32,
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
        }
    }
}
