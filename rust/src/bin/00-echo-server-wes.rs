use protohackers::tcp_accept_and_spawn;
use protohackers::echo_server;

fn main() {
    tcp_accept_and_spawn(([0,0,0,0], 7).into(), echo_server).unwrap();
}
