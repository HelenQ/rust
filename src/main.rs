use crate::garden::vegetables::Asparagus;
pub mod garden;
fn main() {
    let plnt = Asparagus {};
    println!("I'm growing {:?}!", plnt);
}
