use std::sync::{Arc, Mutex};

pub type RefID = String;
pub type ReqBody = String;
pub type RespBody = String;
pub type AMOS = Arc<Mutex<Option<RespBody>>>;

#[derive(Debug, Clone)]
pub struct AddressingMsg {
    pub ref_id: RefID,
    pub req_body: ReqBody,
    pub num_org: usize,
    pub tx_caller: AMOS,
}
