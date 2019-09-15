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
    }

    std::thread::spawn(|| {
        features::bhop::run();
    });
    std::thread::spawn(|| {
        features::flash::run();
    });
    std::thread::spawn(|| {
        features::glow::run();
    });
    std::thread::spawn(|| {
        features::radar::run();
    });
    std::thread::spawn(|| {
        features::skins::run();
    });
    std::thread::spawn(|| {
        features::trigger::run();
    });

    let mut string = String::new();
    std::io::stdin().read_line(&mut string).expect("Failed to read line");

}