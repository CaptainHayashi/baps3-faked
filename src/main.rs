#![feature(phase)]

extern crate docopt;
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;


docopt!(Args deriving Show, "
ury-faked, a dummy BAPS3 service daemon

Usage:
    ury-faked PROFILE [-H HOST] [-p PORT]
    ury-faked --help

Options:
    -H, --host=HOST  The listening host.  [default: 127.0.0.1]
    -p, --port=PORT  The listening port.  [default: 1350]
    -h, --help       Show this message.
", flag_port: u16)

fn main() {
    let args: Args = Args::docopt().decode()
                                   .unwrap_or_else(|e| e.exit());
    println!("{}", args);
}
