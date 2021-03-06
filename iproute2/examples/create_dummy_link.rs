extern crate futures;
extern crate iproute2;
extern crate tokio_core;

use std::thread::spawn;

use futures::Future;
use tokio_core::reactor::Core;

use iproute2::new_connection;

fn main() {
    // Create a netlink connection, and a handle to send requests via this connection
    let (connection, handle) = new_connection().unwrap();

    // The connection we run in its own thread
    spawn(move || Core::new().unwrap().run(connection));

    // Create a request to create the veth pair
    handle
        .link()
        .add()
        .veth("veth-rs-1".into(), "veth-rs-2".into())
        // Execute the request, and wait for it to finish
        .execute()
        .wait()
        .unwrap();
}
