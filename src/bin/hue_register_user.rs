#![allow(unstable)]
extern crate hue;
use std::os;
use hue::errors::HueError;

#[allow(while_true)]
fn main() {
    let args = os::args();
    if args.len() != 3 {
        println!("usage : {} <devicetype> <username>", args[0]);
    } else {
        let bridge = ::hue::bridge::Bridge::discover_required();
        println!("posting user {:?}/{:?} in {:?}", args[1], args[2], bridge);
        while true {
            let r = bridge.register_user(args[1].as_slice(), args[2].as_slice());
            match r {
                Ok(r) => {
                    println!("done: {:?}", r);
                    break;
                },
                Err(HueError::BridgeError(ref error)) if error.code == 101 => {
                    println!("Push the bridge button");
                    std::io::timer::sleep(std::time::duration::Duration::seconds(5))
                },
                Err(e) => panic!(e)
            }
        }
    }
}