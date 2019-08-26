// Proxy
use std::sync::{Arc, Mutex};
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
    rx_proxy: Receiver<AMOS>,
) {
    if DEBUG {
        println!("Start proxy for {}", &ref_id);
    }

    let mut count = 0;
    let mut amos_resps = Vec::with_capacity(num_org);

    loop {
        let res = rx_proxy.try_recv();
        if count >= num_org {
            break;
        } else if res.is_err() {
            // TODO: Find an appropriate value
            sleep!(1);
        } else {
            if DEBUG {
                println!("P {} receives: {}", &ref_id, &count);
            }
            amos_resps.push(res.unwrap());
            count += 1;
        }
    }

    let resp_body = interact(&ref_id, req_body);
    for amos_resp in amos_resps {
        assign_until_success(resp_body.clone(), amos_resp);
    }
    if DEBUG {
        println!("End proxy for {}", ref_id);
    }
}


fn interact(ref_id: &RefID, req_body: ReqBody) -> RespBody {
    sleep!(64);
    format!("Generates response for {} by {}", req_body, ref_id)
}
