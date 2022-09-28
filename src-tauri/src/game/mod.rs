use crossbeam_channel::{Receiver, Sender};

pub struct TauriBridge(pub Sender<u32>, pub Receiver<()>);

pub struct BevyBridge(pub Sender<()>);

pub mod counter;
