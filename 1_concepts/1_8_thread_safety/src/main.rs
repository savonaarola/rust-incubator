use std::{cell::Cell, marker::PhantomData, rc::Rc, sync::Arc};



struct OnlySync{

}

struct OnlySend<T: Send>{
    cell: Cell<T>,
}
struct SyncAndSend ;
struct NotSyncNotSend<T>{
    rc: Rc<T>,
}


fn main() {
    let notshared = NotSyncNotSend{
        rc: Rc::new(21),
    };
    let shared = OnlySend{
        cell: Cell::new(32),
    };

    std::thread::spawn({
        let shared = shared;
        move || {
            println!("{:?}", shared.cell);
        }
    }).join().unwrap();

    // std::thread::spawn({
    //     let notshared = notshared;
    //     move || {
    //         println!(":?",notshared.rc);
    //     }
    // }).join().unwrap();
}