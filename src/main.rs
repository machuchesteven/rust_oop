mod classes;
mod refs;
mod borrowing;

use refs::references;
use classes::ownerships;
use crate::borrowing::main_borrowing;

fn main() {
    ownerships();
    references();
    main_borrowing();
}
