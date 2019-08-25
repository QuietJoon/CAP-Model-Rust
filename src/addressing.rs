use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

use rand::Rng;

use crate::constant::*;
use crate::data::*;
use crate::proxy::*;
use crate::util::*;

pub fn addressing(rx_addr: Receiver<AddressingMsg>) {
    let mut the_table: HashMap<String, Sender<AMOS>> = HashMap::new();

    loop {
        let res = rx_addr.try_recv();
        if res.is_err() {
            // TODO: Find an appropriate value
            sleep!(1);
        } else {
            let addr_msg = res.unwrap();
            if DEBUG {
                println!("A received: {}", &addr_msg.ref_id);
            }

            let o_tx_proxy = the_table.get(&addr_msg.ref_id);
            if o_tx_proxy.is_some() {
                if DEBUG {
                    println!("'A sends msg to proxy {}", &addr_msg.ref_id);
                }
                let tx_proxy = o_tx_proxy.unwrap();
                send_until_success(addr_msg.tx_caller, tx_proxy.clone());
            } else {
                let (tx_proxy, rx_proxy) = channel();
                the_table.insert(addr_msg.ref_id.clone(), tx_proxy.clone());

                let a_ref_id = addr_msg.ref_id.clone();
                let a_req_body = addr_msg.req_body.clone();
                let num_org = addr_msg.num_org;
                thread::spawn(move || {
                    if DEBUG {
                        println!("A spawns proxy for {}", &a_ref_id);
                    }
                    proxy(a_ref_id, a_req_body, num_org, rx_proxy);
                });

                if DEBUG {
                    println!(" A sends msg to proxy {}", &addr_msg.ref_id);
                }
                send_until_success(addr_msg.tx_caller, tx_proxy);
            }
        }
    }
}
