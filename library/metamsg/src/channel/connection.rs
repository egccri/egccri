use crate::message::Message;
use crate::Who;

pub struct Connection {
    socket: Who,
    in_bound: Vec<Message>,
    out_bound: Vec<Message>,
}
