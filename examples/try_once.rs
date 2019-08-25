use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

use rand::Rng;

use cap_model::addressing::*;
use cap_model::caller::*;
use cap_model::data::*;
use cap_model::*;

const N: usize = 3;

fn main() {
    println!("Start");
    let mut rng = rand::thread_rng();

    let (tx_addr, rx_addr) = channel();

    // Spawn Addressing Thread
    thread::spawn(move || {
        addressing(rx_addr);
    });

    // Spawn

    let ref_id_01 = "Ref#01".to_string();
    let req_body_01 = "ReqBody#01".to_string();

    for _ in 0..N {
        let r: u64 = rng.gen();
        let r100 = r % 1000 + 100;

        let a_ref_id = ref_id_01.clone();
        let a_req_body = req_body_01.clone();
        let a_tx = tx_addr.clone();
        thread::spawn(move || {
            println!("Spawn caller {} for {}", r100, &a_ref_id);
            let resp_body = caller(a_ref_id, a_req_body, N, a_tx);
            println!("Get RespBody: {}", resp_body);
        });
    }

    sleep!(8192);
}
