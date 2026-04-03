pub mod darwin;
pub mod linux;

fn main() {
    linux::show_info();
    darwin::show_info();
}
