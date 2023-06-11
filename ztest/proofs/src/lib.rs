#![no_std]            
use gstd::{prelude::*,msg,ActorId};
use hello_world_io::{IoProofofWaste};


static mut PROOF: Option<HashMap<ActorId,String>> = None;
//static mut PROOF: Option<IoProofofWaste> = None;
//------------------------------------------------------------

#[no_mangle]
extern "C" fn init(){
    //let init_message: String = msg::load().expect("Can't load the incoming message");
    //unsafe { GREETING = Some(init_message)}; 
}

//firma
#[no_mangle]
extern "C" fn handle(){
    let message: String = msg::load().expect("Can't decode `InputMessage`");
    let tmg = IoProofofWaste {
        who: msg::source(),
        ipfshash: message,
    };

    let actual_proofs = unsafe { PROOF.as_ref().expect("The contract is not initialized")};
    unsafe { PROOF = Some(message)};

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




