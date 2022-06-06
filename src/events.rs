use sciter::{dispatch_script_call, EventHandler, Value};
use flume::Sender;

/// action struct
#[derive(Debug)]
pub struct Action {
    pub path: String,
    pub data: String,
    pub cb: Value,
}

/// evt struct
pub struct Evt {
    pub sender: Sender<Action>,
}

impl Evt {
    /// fetch event provide a way for front-end and back-end to exchange data.
    pub fn fetch(&mut self, path: String, data: String, cb: Value) {
        let _res = self.sender.send(Action {
            path,
            data,
            cb,
        });
    }
    /// exit event can exit the app form front-end
    pub fn exit(&mut self) {
        std::process::exit(0);
    }
}

// impl sciter event
impl EventHandler for Evt {
    dispatch_script_call! {
        fn fetch(String,String,Value);
        fn exit();
    }
}