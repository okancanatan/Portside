use super::ListeningPort;
use windows_sys::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable,
    TCP_TABLE_OWNER_PID_LISTENER,
    MIB_TCPROW_OWNER_PID,
};
use windows_sys::Win32::Networking::WinSock::AF_INET;
use windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;


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

    let mut buffer = vec![0u32; (size as usize).div_ceil(4)];
 
    if status != ERROR_INSUFFICIENT_BUFFER {
        return Err(std::io::Error::from_raw_os_error(status as i32));
    }

    if size == 0 {
        return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Windows returned a TCP table that has no content (size of 0)",
                ));
    }
       
    let second_status = unsafe 
    {
        GetExtendedTcpTable(
            buffer.as_mut_ptr().cast(),
            &mut size,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_LISTENER,
            0,
        )
    };

    if second_status != 0 {
     
        return Err(std::io::Error::from_raw_os_error(second_status as i32));
    }





    let entry_count = buffer[0];
    let row_size = std::mem::size_of::<MIB_TCPROW_OWNER_PID>();
    println!("tcp listening entries: {}", entry_count);

    let row_size = std::mem::size_of::<MIB_TCPROW_OWNER_PID>();
    let count = entry_count as usize;
    let header_size = std::mem::size_of::<u32>();

    let required_size = count
        .checked_mul(row_size)
        .and_then(|row_size| row_size.checked_add(header_size))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "TCP table overflow",
                )
        })?;
    
    if required_size > size as usize {
        return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "TCP table is smaller than expected"
                ));
    }

    println!("Expected TCP table size {}", required_size);
    println!("tcp record size (in bytes): {}", row_size);
    println!("Windows Status: {}",status);
    println!("Required BUffer sze is {}", size);
    Ok(Vec::new())
}
