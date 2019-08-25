// Proxy
use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

use crate::constant::*;
use crate::data::*;
use crate::util::*;

// Listen Sender<RespBody> for each Caller, and send resp_body by req_body
pub fn proxy(
    ref_id: RefID,
    req_body: ReqBody,
    num_org: usize,
    rx_proxy: Receiver<Sender<RespBody>>,
) {
    if DEBUG {
        println!("Start proxy for {}", &ref_id);
    }
    let resp_body = interact(&ref_id, req_body);
    let mut count = 0;

    loop {
        let res = rx_proxy.try_recv();
        if res.is_err() {
            // TODO: Find an appropriate value
            sleep!(1);
        } else {
            if count < num_org {
                if DEBUG {
                    println!("P {} receives: {}", &ref_id, &count);
                }
                send_until_success(resp_body.clone(), res.unwrap());
                count += 1;
            } else {
                break;
            }
        }
    }
    if DEBUG {
        println!("End proxy for {}", ref_id);
    }
}

fn interact(ref_id: &RefID, req_body: ReqBody) -> RespBody {
    sleep!(64);
    format!("Generates response for {} by {}", req_body, ref_id)
}
