// # 1 
struct Temp<'a> {
    name: &'a str,
}


impl<'a> Temp<'a> {
    fn just_print(thing : &'a str) {
        println!("{}", thing);
    }
}

struct Something {
    age: i32,   
}

impl Something {
    fn doer(self) {
      Temp::just_print("amogh");
      println!("{}", self.age);
    }
}

// fn main() {
//     let s = Something{
//         age: 2,
//     };
//     s.doer();
    
// }

// # 2
use std::sync::Arc;
use crossbeam::channel;

struct Channel {
    receiver: channel::Receiver<Arc<i32>>,
    sender: channel::Sender<Arc<i32>>,
}

impl Channel {
    fn new() -> Channel {
        let (sender, receiver) = channel::unbounded();
        Channel {  receiver, sender }
    }

    fn sender(&self) {
        &self.sender.send(Arc::new(10));
    }

    fn receiver(&self) {
        while let Ok(_) = &self.receiver.recv() {
            println!("shit received");
            break
        }
    }
}

fn main() {
    let c = Channel::new();
    c.sender();
    c.receiver()
}
