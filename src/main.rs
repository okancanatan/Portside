
mod discovery;

fn main() {

    match discovery::discover_ports(){
        Ok(ports) => {
            println!("Found {} ports", ports.len());
        },
        Err(error) => {
            eprint!("Discovery failed: {}", error);
        },
    }

}