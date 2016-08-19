extern crate ieee8021504;

use ieee8021504::phy::frame::{Frame, FrameCtrl};

fn main() {
    let f = Frame::new(vec!(0,0,0,0,0x7A,5,6,0,123,100,100)).unwrap();

<<<<<<< HEAD
    //println!("Hello, world! {:?}", f.ack_requested());
    println!("security_enable {:?}", f.frame_pending());
    println!("ack_requested {:?}", f.ack_requested());
    println!("pan_id_compression {:?}", f.pan_id_compression());
//    println!("security_enable {:?}", f.FrameCtrlImpl);
    //println!("security_enable {:?}", f.());
=======
    println!("Hello, world! {:?}", f.ack_requested());
>>>>>>> 39d3d6e700428b972331406e7598cdebbd239ce2
}
