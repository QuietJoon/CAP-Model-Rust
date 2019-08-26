use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

use rand::Rng;

use cap_model::addressing::*;
use cap_model::caller::*;
use cap_model::constant::*;
use cap_model::data::*;
use cap_model::util::*;
use cap_model::*;

const TRIAL: usize = 3000;
const ORG: usize = 3;

fn main() {
    println!("Start");
    let mut rng = rand::thread_rng();

    let (tx_addr, rx_addr) = channel();
    let (tx_resp, rx_resp) = channel();
    let (tx_end, rx_end) = channel();

    // Spawn Addressing Thread
    thread::spawn(move || {
        addressing(rx_addr);
    });

    // Spawn many call request
    for tn in 0..TRIAL {
        for org in 0..ORG {
            let r: u64 = rng.gen();
            let r100 = r % 100;
            let a_ref_id = format!("RefID#{:04}", tn);
            let a_req_body = format!("ReqBody#{:04}", tn);
            let a_tx = tx_addr.clone();
            let a_tx_resp = tx_resp.clone();
            let a_tx_end = if tn == TRIAL-1 && org == ORG-1 {
                Some(tx_end.clone())
            } else {
                None
            };

            thread::spawn(move || {
                sleep!(r100);
                if DEBUG {
                    println!("Spawn caller {} for {}", org, &a_ref_id);
                }
                let resp_body = caller(a_ref_id, a_req_body, ORG, a_tx);
                let res = format!("Caller {} gets RespBody: {}", org, resp_body);
                if DEBUG {
                    println!("{}", res);
                }
                send_until_success(res, a_tx_resp);
                if tn == TRIAL-1 && org == ORG-1 {
                    eprintln!("End Calling");
                    send_until_success((),a_tx_end.unwrap());
                }
            });
        }
    }

    println!("Wait");
    let _ = rx_end.recv();
    eprintln!("End Wait");
    sleep!(1000);
    /*
    let mut idx = 0;
    let mut contents: Vec<String> = Vec::with_capacity(TRIAL * ORG);

    println!("Start Receiving");
    loop {
        let res = rx_resp.recv();
        if res.is_ok() {
            contents.push(res.unwrap());
            idx += 1;
        }
        if idx >= TRIAL * ORG {
            break;
        }
    }

    println!("Print all");
    idx = 0;
    for r in contents {
        println!("{:04}: {}", idx, r);
        idx += 1;
    }
    */
}
