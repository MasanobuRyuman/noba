// https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html
#[derive(Debug)]
pub enum Error {
    Network(String),
    Other(String),
}