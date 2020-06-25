mod memory;
mod offsets;
mod helpers;
mod features;
use memory::{ PID, BASE, inject };

fn main() {

    inject("Counter-Strike: Global Offensive", "client_panorama.dll");

    unsafe {
        println!("pId: {}", &PID);
        println!("client_panorama.dll: {}", &BASE);
        println!("");

        features::bhop::ignite(true);
        features::flash::ignite(true);
        features::glow::ignite(true);
        features::radar::ignite(true);
        features::trigger::ignite(true);
    }

    // Reading line or the application will close
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).expect("Failed to read line");

}