
mod decode;

fn main() {
  decode::decode(std::env::args().nth(1).unwrap());
}
