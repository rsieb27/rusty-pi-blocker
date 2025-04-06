use std::fs;
use color_eyre::eyre::Result;

#[cfg(target_family = "unix")]
pub fn stop_dns_server() -> Result<()> {
    //read the pid from the file
    let pid_str = fs::read_to_string("dns.pid")
        .map_err(|_| color_eyre::eyre::eyre!("Could not read dns.pid. Is the server running?"))?; 
    let pid: i32 = pid_str
        .trim().parse().map_err(|_| color_eyre::eyre::eyre!("Invalid PID found in dns.pid"))?;

    //send SIGTERM to the process libc
    let result = unsafe { libc::kill(pid, libc::SIGTERM ) };
    if result == 0 {
        println!("Sent SIGTERM to process {}", pid);
        fs::remove_file("dns.pid")?;
        Ok(())
    } else {
        Err(eyre::eyre!("Failed to send SIGTERM to process {}"), pid)
    }
}

#[cfg(target_family = "windows")]
pub fn stop_dns_server() -> Result<()> {
    //read the pid from file
    let pid_str = fs::read_to_string("dns.pid")
        .map_err(|_| color_eyre::eyre::eyre!("Could not read dns.pid. Is the server running?"))?;
    let pid: i32 = pid_str
        .trim().parse().map_err(|_| color_eyre::eyre::eyre!("Invalid PID found in dns.pid"))?;

    //use taskkill to stop the process 
    let status = std::process::Command::new("taskkill")
        .args(&["/PID", &pid.to_string(), "/F"])
        .status()?;
    
    if status.success() {
        println!("Stopped process {}", pid);
        fs::remove_file("dns.pid")?;
        Ok(())
    } else {
        Err(color_eyre::eyre::eyre!("Failed to stop process {}", pid))
    }
}