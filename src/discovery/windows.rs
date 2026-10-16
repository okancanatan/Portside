use super::ListeningPort;
use windows_sys::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable,
    TCP_TABLE_OWNER_PID_LISTENER,
};
use windows_sys::Win32::Networking::WinSock::AF_INET;



pub fn discover_ports() -> std::io::Result<Vec<ListeningPort>> {
    let mut size: u32 =0;
    let status = unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut size,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_LISTENER, 
            0,
            )
    };

    println!("Windows Status: {}",status);
    println!("Required BUffer sze is {}", size);
    Ok(Vec::new())
}
