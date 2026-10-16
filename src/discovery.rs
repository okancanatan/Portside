use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct ListeningPort {
    pub addr: SocketAddr,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

// Windows Implementation

#[cfg(target_os = "windows")]
mod windows;

pub fn discover_ports() -> std::io::Result<Vec<ListeningPort>> {
    #[cfg(target_os = "windows")]
    {
        windows::discover_ports()
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Port discovery is not implemented for this platform.",
        ))
    }
}