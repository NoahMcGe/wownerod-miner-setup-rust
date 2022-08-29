extern crate execute;
extern crate reqwest;
extern crate scraper;
use std::io;
use std::env;
use std::io::Write;
use std::process;
use std::process::{Command, Stdio};
use execute::{Execute, shell};
mod menu_1;
mod webget;


fn main() {
	println!("\n
	Loading version from...
	https://git.wownero.com/wownero/wownero/releases");
	menu_1::clear_screen();
	let (wownerod_version,wownerod_download_name) = webget::wowgetvar();
    menu_1::run(&wownerod_version);
    //println!("\n\n    version: {} \n    download_path: {}",wownerod_version,wownerod_download_name);
    let mut input_x = String::new();
    while input_x.trim() != "Q" {
		input_x.clear();
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input_x).unwrap();
		if input_x.trim() == "Q" || input_x.trim() == "q"{
			println!("    Exiting Now...");
			process::exit(0x0100);
			}
		else if input_x.trim() == "W" || input_x.trim() == "w"{
			println!("
    https://git.wownero.com/wownero/wownero/releases
				");
			menu_1::run(&wownerod_version);
			}
		else if input_x.trim() == "H" || input_x.trim() == "h"{
			menu_1::help();
			menu_1::run(&wownerod_version);
			}
		else if input_x.trim() == "A" || input_x.trim() == "a" {
			menu_1::about();
			menu_1::run(&wownerod_version);
			}
		else if input_x.trim() == "1" { //1 sudo journalctl -u miner
			let mut command = shell("sudo journalctl -u miner");
			command.stdout(Stdio::piped());
			let output = command.execute_output().unwrap();
			println!("{}", String::from_utf8(output.stdout).unwrap());
			println!("Finished...
				");
			menu_1::run(&wownerod_version);
			}
		else if input_x.trim() == "2" { //2 sudo systemctl disable miner && sudo systemctl stop miner
			let mut command = shell("sudo systemctl disable miner && sudo systemctl stop miner");
			command.stdout(Stdio::piped());
			let output = command.execute_output().unwrap();
			println!("{}", String::from_utf8(output.stdout).unwrap());
			println!("Finished...
				");
			menu_1::run(&wownerod_version);
			}
		else if input_x.trim() == "5" { //5 sudo nano /etc/systemd/system/miner.service
			let mut command = shell("sudo nano /etc/systemd/system/miner.service");
			let output = command.execute_output().unwrap();
			println!("{}", String::from_utf8(output.stdout).unwrap());
			println!("Finished...
				");
			menu_1::run(&wownerod_version);
			}
		else if input_x.trim() == "4" { //4 Setup Daemon/Configure Miner 'Requires Keys' (Automine on boot)
			print!("Input Wallet ID: ");
			let mut input_walletid = String::new();
			input_walletid.clear();
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut input_walletid).unwrap();
			input_walletid = input_walletid[0..input_walletid.len() - 1].to_string();
			print!("Input Secret Key: ");
			let mut input_spendkey = String::new();
			input_spendkey.clear();
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut input_spendkey).unwrap();
			input_spendkey = input_spendkey[0..input_spendkey.len() - 1].to_string();
			print!("Mining Threads Amount: ");
			let mut input_mine_thread = String::new();
			input_mine_thread.clear();
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut input_mine_thread).unwrap();
			input_mine_thread = input_mine_thread[0..input_mine_thread.len() - 1].to_string();
			let mut command = shell(r#"sudo bash -c 'echo "[Unit]" > /etc/systemd/system/miner.service'"#);
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			command = shell(r#"sudo bash -c 'echo "Description=Monero Miner" >> /etc/systemd/system/miner.service'"#);
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			command = shell(r#"sudo bash -c 'echo "After=network.target" >> /etc/systemd/system/miner.service'"#);
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			command = shell(r#"sudo bash -c 'echo "[Service]" >> /etc/systemd/system/miner.service'"#);
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			let mut exec_start = String::from(r#"sudo bash -c 'echo "ExecStart=wownerod --non-interactive --start-mining "#);
			exec_start.push_str(&input_walletid);
			exec_start.push_str(&" --spendkey ");
			exec_start.push_str(&input_spendkey);
			exec_start.push_str(&r#" --mining-threads "#);
			exec_start.push_str(&input_mine_thread);
			exec_start.push_str(&r#"" >> /etc/systemd/system/miner.service'"#);
			command = shell(&exec_start);
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			command = shell(r#"sudo bash -c 'echo "User=root" >> /etc/systemd/system/miner.service'"#);
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			command = shell(r#"sudo bash -c 'echo "[Install]" >> /etc/systemd/system/miner.service'"#);
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			command = shell(r#"sudo bash -c 'echo "WantedBy=multi-user.target" >> /etc/systemd/system/miner.service'"#);
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			command = shell("sudo bash -c 'systemctl enable miner.service'");
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			command = shell("sudo bash -c 'systemctl start miner.service'");
			command.stdout(Stdio::piped());
			let _output = command.execute_output().unwrap();
			println!("Finished Daemon Setup
				");
			menu_1::run(&wownerod_version);
			}
		else if input_x.trim() == "3" { //3
			//fetch username
			let user_name = match env::var_os("USER") {
				Some(v) => v.into_string().unwrap(),
				None => panic!("$USER is not set")
				};
			println!("    Got username: {}", user_name);
			//make folder /home/$USER/wownero
			let  mut mkdir_wownero = Command::new("mkdir");
			let mut file_path = String::from("/home/" ); 
			file_path.push_str(&user_name);
			file_path.push_str("/Downloads");
			println!("    Trying to Create Path {}", file_path);
			mkdir_wownero.arg(&file_path);
			mkdir_wownero.status().expect("process failed to execute");
			//download wownerod to wownero folder.
			let mut download_wownerod = Command::new("wget");
			let  download_path = String::from("https://git.wownero.com/attachments/".to_owned()+&wownerod_download_name); //grab download path from git site
			println!("    Trying to Download Wownero for Debain ");
			download_wownerod.arg(String::from(&download_path));
			download_wownerod.status().expect("process failed to execute");
			//moving the downloaded file
			println!("    Trying to Move Wownero file to /home/{}/Downloads ",user_name);
			let mut move_path = String::from("mv e9e6fa73-9e3a-4391-af04-64fba8cc6d9e");//make a dynamic var that is grabbed  from download path
			move_path.push_str(" /home/");
			move_path.push_str(&user_name);
			move_path.push_str("/Downloads/wownero_v");
			move_path.push_str(&wownerod_version);
			move_path.push_str("_amd64.deb");
			let mut command = shell(&move_path);
			command.stdout(Stdio::piped());
			let output = command.execute_output().unwrap();
			println!("{}", String::from_utf8(output.stdout).unwrap());
			// sudo apt install file
			let mut move_path = String::from("sudo apt install  /home/");
			move_path.push_str(&user_name);
			move_path.push_str("/Downloads/wownero_v");
			move_path.push_str(&wownerod_version);
			move_path.push_str("_amd64.deb");
			command = shell(&move_path);
			command.stdout(Stdio::piped());
			let output = command.execute_output().unwrap();
			println!("{}", String::from_utf8(output.stdout).unwrap());
			println!("    Install has finished, Wownerod is installed.");
			menu_1::run(&wownerod_version);
			}
		else{
			print!("    Invalid option...
				
    Input: ");
		}
		}
}
