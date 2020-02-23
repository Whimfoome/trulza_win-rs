mod memory;
mod offsets;
mod helpers;
mod features;
use memory::{ PID, BASE, inject };

fn main() {

    inject("Counter-Strike: Global Offensive", "client_panorama.dll");

    unsafe {
        println!("pId: {}", PID);
        println!("client_panorama.dll: {}", BASE);
        println!("");

        features::bhop::ignite(true, BASE);
        features::flash::ignite(true, BASE);
        features::glow::ignite(true, BASE);
        features::radar::ignite(true, BASE);
        features::trigger::ignite(true, BASE);
    }

    // Reading line, else the application closes
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).expect("Failed to read line");

}