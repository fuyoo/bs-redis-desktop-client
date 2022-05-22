use sciter::{dispatch_script_call, EventHandler, Value};
use flume::Sender;
#[derive(Debug)]
pub struct Action{
    pub path: String,
    pub data: String,
    pub cb:Value
}

pub struct Evt {
    pub sender: Sender<Action>
}


impl Evt {
    pub fn fetch(&mut self, path: String, data: String, cb: Value) {
        let _res = self.sender.send(Action{
            path,
            data,
            cb
        });
    }
    pub fn exit(&mut self) {
        std::process::exit(0);
    }
}

impl EventHandler for Evt {
    dispatch_script_call! {
        fn fetch(String,String,Value);
        fn exit();
    }
}