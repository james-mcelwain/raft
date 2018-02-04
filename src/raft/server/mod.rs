pub mod unix;

pub trait Server {
    fn new(id: u8) -> Self;
    fn listen(&self);
}
