use rusty_watcher::add;

mod video;
fn main() {
    println!(
        "Hello, world!\n 2 * 2 = {}\n 2 + 2 = {}",
        video::multiply(2, 2),
        add(2, 2)
    );
}
