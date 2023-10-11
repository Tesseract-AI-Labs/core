#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod patient_state;
mod constants;

use patient_state::PatientInfo;
use constants::*;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub enum PermissionType {
    Admin,
    Moderator
}

#[multiversx_sc::contract]
pub trait TesseractContract {
    #[init]
    fn init(&self) {}

    /// @title Create Patient Endpoint
    /// @notice Creates a new patient record in the contract.
    /// @param name The name of the patient.
    /// @param dob The date of birth of the patient.
    /// @param address The address of the patient.
    /// @param sex The gender of the patient.
    /// @return A result containing the patient ID or an error.
    /// @dev This function requires the caller to be authorized for the Create permission.
    /// It generates a new patient ID using a hash function, creates a new PatientInfo instance,
    /// and sets it in the contract's storage using the patient_info storage mapper.
    /// The timestamp and caller's address are recorded as the creation time and creator of the patient record.
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

        let caller = self.blockchain().get_caller();

        let timestamp = self.blockchain().get_block_timestamp();

        let id = self.hasher();

        // let newPatient = PatientInfo{
        //     id: id.clone(),
        //     name,
        //     dob,
        //     address,
        //     sex
        // };

        // self.patient_info(id).set(&newPatient);
    } 

    #[endpoint]
    fn delete_patient(
        &self, 
        id: ManagedByteArray<32>
    ) {
        require!(self.patient_info(id.clone()).is_empty() == false, "Patient does not exist");
        self.patient_info(id).clear();
    }

    #[endpoint]
    fn update_patient(
        &self,
        id: ManagedByteArray<32>
    ) {}

    
    #[endpoint]
    fn create_ticket(
        &self,
        patient_id: i64,
        hash_id: i64
    ) {
        
    }

    #[endpoint]
    fn add_model(
        &self
    ) {}

    #[endpoint]
    fn update_model(
        &self,
        id: ManagedByteArray<32>
    ) {}


    #[endpoint]
    fn authorize(
        &self,
        entity: ManagedAddress,
        permission: PermissionType
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
        let hash = self.crypto().keccak256(&buffer);
        hash
    }


    #[view(getPatientInfo)]
    #[storage_mapper("patientInfo")]
    fn patient_info(
        &self,
        id: ManagedByteArray<32>
    ) -> SingleValueMapper<PatientInfo<Self::Api>>;


    #[storage_mapper("permissions")]
    fn permissions(
        &self, 
        entity: &ManagedAddress
    ) -> SingleValueMapper<PermissionType>;


}