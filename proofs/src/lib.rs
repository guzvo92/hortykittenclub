#![no_std]            
use gstd::{prelude::*,msg,ActorId};
use hello_world_io::{IoProofofWaste};


static mut PROOF: Option<IoProofofWaste> = None;
//------------------------------------------------------------

#[no_mangle]
extern "C" fn init(){
    //let init_message: String = msg::load().expect("Can't load the incoming message");
    //unsafe { GREETING = Some(init_message)}; 
}

//firma
#[no_mangle]
extern "C" fn handle(){
    let message: IoProofofWaste = msg::load().expect("Can't decode `InputMessage`");
    let actual_proofs = unsafe { PROOF.as_ref().expect("The contract is not initialized")};

}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to share the metahash");
}

#[no_mangle]
extern "C" fn state() {
    let state = unsafe {PROOF.as_ref().expect("The contract is not initialized")};
    msg::reply(state, 0).expect("Unable to share the state");
}




