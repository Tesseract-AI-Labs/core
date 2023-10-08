#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod patient_state;
mod constants;

use patient_state::PatientInfo;
use constants::*;

#[multiversx_sc::contract]
pub trait TesseractContract {
    #[init]
    fn init(&self) {}


    // Patient Management Modules
    #[endpoint]
    fn create_patient(
        &self,
        name: ManagedBuffer,
        dob: ManagedBuffer,
        address: ManagedAddress,
        sex: ManagedBuffer,
    ) {
        require!(!name.is_empty(), INVALID_ENTRY);
        require!(!dob.is_empty(), INVALID_ENTRY);
        require!(!sex.is_empty(), INVALID_ENTRY);
        require!(address != ManagedAddress::zero(), "Address is empty");

        let timestamp = self.blockchain().get_block_timestamp();

        let id = self.hasher();

        let newPatient = PatientInfo{
            id,
            name,
            dob,
            address,
            sex,
            // ManagedByteArray::empty()
        };



    } 

    
    #[endpoint]
    fn create_entry(
        &self,
        patient_id: i64,
        hash_id: i64
    ) {
        
    }
    
    #[view(getPendingAnalysis)]
    fn get_pending_analysis(&self) {
        
    }
    
    fn getSeed(&self) -> u32 {
        let mut rand_source = RandomnessSource::new();
        rand_source.next_u32()
    }

    fn hasher(&self) -> ManagedByteArray<32> {
        let value: u32 = self.getSeed();
        let bytes = value.to_le_bytes();
        let buffer = ManagedBuffer::from(&bytes);
        let mut hash = self.crypto().keccak256(&buffer);
        hash
    }

}