mod list {
    pub struct Tasks {
        pub item: String,
    }
}

use crate::things_todo::add_activity;
use things_completed::items_completed; 

fn lets_add_task() {
    let task = list::Tasks {
        item: String::from("Tasks"),
    };

    add_activity(); 

    items_completed::remove_task();

    // crate::things_todo::add_activity(); // absolute path because we start at root crate
}
