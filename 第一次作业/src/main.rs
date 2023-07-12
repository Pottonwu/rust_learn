mod sequence;

mod test;
use test::sort2;
use sequence::sequence1::sort;
fn main() {
   sort('A','z');
   println!("*********************************");
   sort2('a','Z');
}

