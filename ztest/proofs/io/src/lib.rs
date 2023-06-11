#![no_std]                  

use gstd::{prelude::*,ActorId,Encode,Decode,TypeInfo,Debug};
use gmeta::{In,InOut, Metadata};
pub struct HelloMetadata;

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct IoProofofWaste {
    pub who: ActorId,
    pub ipfshash: String,
}

impl Metadata for HelloMetadata {
    type Init = InOut<String, ()>;
    type Handle = In<IoProofofWaste>;
    type State = String;
    type Reply = (); 
    type Signal = ();
    type Others = (); 
 }


