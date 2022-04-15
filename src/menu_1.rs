
pub fn run() {
	print!("- Menu -
    Q. Quit
    W. Wownero Git link
    H. Help
    A. About
    1. View Full Log (Daemon)
    2. Disable Miner (Daemon)
    - Setup - 
    3. Download Wownerod Verison 'v{}' (~/Downloads)
    4. Setup Daemon/Configure Miner 'Requires Keys' (Automine on boot)
    5. Edit Daemon
    Input: ", wow_return());
}
pub fn wow_return() -> &'static str {
	let wownerod_version = "0.10.1.0";
	return wownerod_version;
}
pub fn help() {
	println!("
    Must be a sudo user
    Run Step 3 before Step 4
    Wowlet keys are required for Step 4
    This is for wownerod (not XMRIG)
    Designed to run on a Linux headless server (Mainly Debian & Forks)

    Service Location:
    /etc/systemd/system/miner.service

    How Do I turn it off?
    sudo systemctl disable miner
    sudo systemctl stop miner
	");
}
pub fn about() {
	println!("
    Written: in Rust, a little app to speed the setup of my miners.
    By: Noah , New ideas are appreciated! noahm1611@gmail.com :D
	");
}