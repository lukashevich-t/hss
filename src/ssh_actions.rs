use crate::ssh_configs;

pub fn enlist() {
    let host_names = ssh_configs::read_config_lines();
    println!("{:?}", host_names);
    // let mut cmd = Command::new("ssh.exe");
    // cmd.arg(&host_names[2]);
    // cmd.status();
}