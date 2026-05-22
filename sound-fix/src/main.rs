/// For the main file, I am gonna to: 
/// 1, first call i2c.rs to find the related driver
/// 2, fix the register sequence by calling tas2781.rs

mod i2c;
mod tas2781;

fn main() {
    if let Some(bus) = i2c::find_tias2781_device() {
        println!("Found ! {:?}", bus);
        tas2781::configure(bus);
    }  else {
        print!("Not found");
    }  


}
