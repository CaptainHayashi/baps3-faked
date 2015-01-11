#![feature(plugin)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
#[plugin] #[no_link] extern crate docopt_macros;

extern crate baps3_protocol;

use std::borrow::ToOwned;
use std::error;
use std::error::Error;
use std::io;
use std::io::net::ip;
use std::sync::mpsc;

use baps3_protocol::proto;
use baps3_protocol::server;

docopt!(Args, "
baps3-faked, a dummy BAPS3 service daemon

Usage:
    baps3-faked -h
    baps3-faked [-a ADDRESS]

Options:
    -a, --address=ADDRESS  Where to listen for connections (host:port).
                           [default: localhost:1350]
    -h, --help             Show this message.
");

enum FakedError {
    Io(io::IoError),
    Send
}

impl error::FromError<io::IoError> for FakedError {
    fn from_error(err: io::IoError) -> FakedError {
        FakedError::Io(err)
    }
}

impl<T> error::FromError<mpsc::SendError<T>> for FakedError {
    fn from_error(_: mpsc::SendError<T>) -> FakedError {
        FakedError::Send
    }
}

impl error::Error for FakedError {
    fn description(&self) -> &str {
        match *self {
            FakedError::Io(ref io) => io.description(),
            FakedError::Send => "send error"
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            FakedError::Io(ref io) => io.detail(),
            _ => None
        }
    }
}

type FakedResult<A> = Result<A, FakedError>;

fn main() {
    let args: Args = Args::docopt()
                          .decode()
                          .unwrap_or_else(|e| e.exit());

    if let Err(e) = faked(args) {
        println!(
            "caught error: {} ({})",
            e.description(),
            &*e.detail().unwrap_or("no additional info".to_owned())
        );
    }
}

/// Says hi to a new connection.
fn hi(addr: ip::SocketAddr, tx: mpsc::Sender<server::Response>)
  -> FakedResult<()> {
    println!("new connection: {:?}", addr);

    try!(tx.send(server::Response::Unicast(
        addr,
        proto::Message::new("OHAI", &["baps3-faked"])
    )));
    try!(tx.send(server::Response::Unicast(
        addr,
        proto::Message::from_word("FEATURES")
    )));
    try!(tx.send(server::Response::Unicast(
        addr,
        proto::Message::new("STATE", &["Ready"])
    )));

    Ok(())
}

/// Reports on a lost connection.
fn bye(addr: ip::SocketAddr) -> FakedResult<()> {
    println!("lost connection: {:?}", addr);

    Ok(())
}

/// Reports on a client message.
fn msg(addr: ip::SocketAddr, m: proto::Message) -> FakedResult<()> {
    println!("{:?} says: {} {:?}", addr, m.word(), m.args());

    Ok(())
}

/// The function holding most of the logic for faked.
fn faked(args: Args) -> FakedResult<()> {
    let server::Server {
        request_rx,
        response_tx
    } = try!(server::Server::new(&*args.flag_address));

    for request in request_rx.iter() {
        try!(match request {
            server::ctl::Request::Hi(addr) => hi(addr, response_tx.clone()),
            server::ctl::Request::Bye(addr) => bye(addr),
            server::ctl::Request::Message(addr, m) => msg(addr, m),
            server::ctl::Request::Gone => break,
            server::ctl::Request::ServerError(e) =>
                return Err(FakedError::Io(e))
        })
    }

    Ok(())
}
