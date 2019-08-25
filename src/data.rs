use std::sync::mpsc::*;

pub type RefID = String;
pub type ReqBody = String;
pub type RespBody = String;

#[derive(Debug, Clone)]
pub struct AddressingMsg {
    pub ref_id: RefID,
    pub req_body: ReqBody,
    pub num_org: usize,
    pub tx_caller: Sender<RespBody>,
}
