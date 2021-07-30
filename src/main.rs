use sys_info::*;

fn main() {
    let ref mem:Result<MemInfo, Error> = mem_info();
    if let Ok(ref m) = mem {
        println!("total:{}",m.total);
    } else {
        println!("{}","this is a error!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_os_type() {
        let typ = os_type().unwrap();
        assert!(typ.len() > 0);
        println!("os_type(): {}", typ);
    }

    #[test]
    pub fn test_os_release() {
        let release = os_release().unwrap();
        assert!(release.len() > 0);
        println!("os_release(): {}", release);
    }

    #[test]
    pub fn test_cpu_num() {
        let num = cpu_num().unwrap();
        assert!(num > 0);
        println!("cpu_num(): {}", num);
    }

    #[test]
    pub fn test_cpu_speed() {
        let speed = cpu_speed().unwrap();
        assert!(speed > 0);
        println!("cpu_speed(): {}", speed);
    }

    #[test]
    pub fn test_loadavg() {
        let load = loadavg().unwrap();
        println!("loadavg(): {:?}", load);
    }

    #[test]
    pub fn test_proc_total() {
        let procs = proc_total().unwrap();
        assert!(procs > 0);
        println!("proc_total(): {}", procs);
    }

    #[test]
    pub fn test_mem_info() {
        let mem = mem_info().unwrap();
        assert!(mem.total > 0);
        println!("mem_info(): {:?}", mem);
    }

    #[test]
    #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
    pub fn test_disk_info() {
        let info = disk_info().unwrap();
        println!("disk_info(): {:?}", info);
    }

    #[test]
    pub fn test_hostname() {
        let host = hostname().unwrap();
        assert!(host.len() > 0);
        println!("hostname(): {}", host);
    }

    #[test]
    #[cfg(not(windows))]
    pub fn test_boottime() {
        let bt = boottime().unwrap();
        println!("boottime(): {} {}", bt.tv_sec, bt.tv_usec);
        assert!(bt.tv_sec > 0 || bt.tv_usec > 0);
    }

    #[test]
    #[cfg(target_os = "linux")]
    pub fn test_linux_os_release() {
        let os_release = linux_os_release().unwrap();
        println!("linux_os_release(): {:?}", os_release.name)
    }
}