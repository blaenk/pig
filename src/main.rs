extern crate zmq;

fn main() {
    let mut ctx = zmq::Context::new();
    let mut socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:5555").unwrap();

    let lang = zmq::Message::from_slice(b"ruby").unwrap();
    socket.send_msg(lang, zmq::SNDMORE).unwrap();

    let code = zmq::Message::from_slice(b"puts 'testing'").unwrap();
    socket.send_msg(code, 0).unwrap();

    let highlighted = socket.recv_string(0).unwrap().unwrap();
    println!("highlighted:\n{}", highlighted);
}
