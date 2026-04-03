use std::env;
use std::process::Command;

#[derive(Debug)]
pub struct Darwin {
    hostname: String,
    os: String,
    kernel: String,
    cpu: String,
    memory: String,
}

impl Darwin {
    pub fn new(hostname: String, os: String, kernel: String, cpu: String, memory: String) -> Self {
        Self {
            hostname,
            os,
            kernel,
            cpu,
            memory,
        }
    }

    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }

    pub fn get_os(&self) -> &str {
        &self.os
    }

    pub fn get_kernel(&self) -> &str {
        &self.kernel
    }

    pub fn get_cpu(&self) -> &str {
        &self.cpu
    }

    pub fn get_memory(&self) -> &str {
        &self.memory
    }
}

pub fn get_os_info() -> Option<String> {
    let output = Command::new("sw_vers").arg("-productName").output().ok()?;

    let product_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()?;

    let product_version = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("sw_vers").arg("-buildVersion").output().ok()?;

    let build_version = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("uname").arg("-m").output().ok()?;

    let machine = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Some(format!(
        "{} {} ({}) {}",
        product_name, product_version, build_version, machine
    ))
}

fn get_host_info() -> Option<String> {
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let output = Command::new("hostname").output().ok()?;
    let host = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let hostname = format!("{}@{}", user, host);

    Some(hostname)
}

fn chip_name() -> String {
    let out = std::process::Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .unwrap();

    let raw = String::from_utf8_lossy(&out.stdout);

    if raw.contains("Apple") {
        raw.trim().to_string()
    } else {
        "Apple Silicon".to_string()
    }
}

pub fn show_info() {
    let hostname = get_host_info();

    if let Some(hostname) = &hostname {
        println!("\x1b[0m {}", hostname);
    }

    if let Some(os) = get_os_info() {
        println!("\x1b[34mOS:\x1b[0m {}", os);
    }

    if let Some(hostname) = hostname {
        println!("\x1b[34mHost:\x1b[0m {}", hostname);
    }

    if let Some(chip) = Some(chip_name()) {
        println!("\x1b[34mCPU:\x1b[0m {}", chip);
    }
}
