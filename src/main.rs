mod types;
mod utils;
mod dlse_esd;

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    
    for arg in args {
        if arg.ends_with(".py") {
            dlse_esd::DlseEsd::compile(&*arg);
        }
        else if arg.ends_with(".esd") {
            dlse_esd::DlseEsd::decompile(&*arg);
        }
        else { 
            println!("Invalid file found: {} (use the extension .py or .esd)", arg);
        }
    }
}
