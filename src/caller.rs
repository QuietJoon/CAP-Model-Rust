use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

use crate::constant::*;
use crate::data::*;
use crate::util::*;

pub fn caller(
    ref_id: RefID,
    req_body: ReqBody,
    num_org: usize,
    tx_addressing: Sender<AddressingMsg>,
) -> RespBody {
    if DEBUG {println!("Start caller for {}", &ref_id);}
    let (tx_caller, rx_caller) = channel();

    let addressing_msg = AddressingMsg {
        ref_id: ref_id.clone(),
        req_body: req_body.clone(),
        num_org: num_org,
        tx_caller: tx_caller.clone(),
    };

    send_until_success(addressing_msg, tx_addressing);

    // TODO: Find an appropriate value
    sleep!(128);
    let resp_body = recv_until_success(rx_caller);
    if DEBUG {println!("End caller for {}", &ref_id);}

    resp_body
}
