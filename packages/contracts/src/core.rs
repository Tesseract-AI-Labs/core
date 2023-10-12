#![no_std]
#![allow(unused_attributes)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod patient_state;
mod constants;

use patient_state::PatientInfo;
use constants::*;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum PermissionType {
    Create,
    View,
    Delete
}

#[multiversx_sc::contract]
pub trait TesseractContract {
    #[init]
    fn init(&self) {
        let caller = self.blockchain().get_caller();
        self.admin().set(&caller);
    }

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

        // Access control
        let caller = self.blockchain().get_caller();
        require!(self.is_authorized(caller, PermissionType::Create), "Not authorized");

        let timestamp = self.blockchain().get_block_timestamp();

        let id = self.hasher();

        let newPatient = PatientInfo{
            id: id.clone(),
            name,
            dob,
            address,
            sex,
            tickets: ManagedVec::new()
        };

        self.patient_info(id).set(&newPatient);
    } 

    /// @title Delete Patient Endpoint
    /// @notice Deletes a patient record identified by the given ID.
    /// @param id The unique identifier of the patient to be deleted.
    /// @dev This endpoint is protected with access control, 
    /// allowing only authorized entities to delete patient records. 
    /// An error is thrown if the caller is not authorized or if 
    /// the patient record does not exist.
    #[endpoint]
    fn delete_patient(
        &self, 
        id: ManagedByteArray<32>
    ) {
        // Access control: Ensure the caller has the necessary permissions
        let caller = self.blockchain().get_caller();
        require!(self.is_authorized(caller, PermissionType::Delete), "Not authorized");

        // Existence check: Ensure the patient record exists before deletion
        require!(self.patient_info(id.clone()).is_empty() == false, "Patient does not exist");

        // Deletion: Remove the patient record from storage
        self.patient_info(id).clear();
    }
    
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


    /// @title Authorize Endpoint
    /// @notice Grants a specified permission to an entity.
    /// @param entity The address of the entity to be authorized.
    /// @param permission The type of permission to grant.
    /// @return An SCResult indicating success or failure.
    /// @dev Only an admin can execute this function. An AuthorizationChanged event is emitted upon successful authorization.
    #[endpoint]
    fn authorize(
        &self, 
        entity: ManagedAddress, 
        permission: PermissionType
    ) {
        let caller = self.blockchain().get_caller();
        require!(self.is_admin(caller), "Not Admin");
        
        self.permissions(&entity).set(&permission);
        self.authorization_event(entity);
    }

    /// @title Is Authorized
    /// @notice Checks whether a given entity has a specified permission.
    /// @param entity The address of the entity to check.
    /// @param permission The type of permission to check for.
    /// @return A boolean indicating whether the entity has the specified permission.
    #[endpoint]
    fn is_authorized(
        &self,
        entity: ManagedAddress,
        permission: PermissionType
    ) -> bool {
        if PermissionType::Create == self.permissions(&entity).get() {
            true
        } else {
            false
        }
    }

    /// @title Is Admin Endpoint
    /// @notice Checks whether the given entity is the admin.
    /// @param entity The address of the entity to check.
    /// @return A boolean indicating whether the entity is the admin.
    #[endpoint]
    fn is_admin(
        &self,
        entity: ManagedAddress
    ) -> bool {
        if self.admin().get() == entity {
            true
        } else {
            false
        }
    }
    
    /// @title Get Seed
    /// @notice Generates a random u32 value using a new instance of RandomnessSource.
    /// @return A random u32 value.
    fn getSeed(&self) -> u32 {
        let mut rand_source = RandomnessSource::new();
        rand_source.next_u32()
    }

    /// @title Hasher
    /// @notice Generates a Keccak256 hash from a random seed.
    /// @return A ManagedByteArray of 32 bytes representing the hash.
    fn hasher(&self) -> ManagedByteArray<32> {
        let value: u32 = self.getSeed();
        let bytes = value.to_le_bytes();
        let buffer = ManagedBuffer::from(&bytes);
        let hash = self.crypto().keccak256(&buffer);
        hash
    }

    /// @title Get Patient Info View
    /// @notice Provides access to patient information based on a unique identifier.
    /// @param id The unique identifier of the patient.
    /// @return A SingleValueMapper containing the patient information associated with the given id.
    #[view(getPatientInfo)]
    #[storage_mapper("patientInfo")]
    fn patient_info(
        &self,
        id: ManagedByteArray<32>
    ) -> SingleValueMapper<PatientInfo<Self::Api>>;


    /// @title Permissions Storage Mapper
    /// @notice Maps an entity's address to its permission type.
    /// @param entity The address of the entity.
    /// @return A single value mapper containing the permission type of the entity.
    #[storage_mapper("permissions")]
    fn permissions(
        &self, 
        entity: &ManagedAddress
    ) -> SingleValueMapper<Self::Api, PermissionType>;

    /// @title Admin Storage Mapper
    /// @notice Holds the address of the admin.
    /// @return A single value mapper containing the admin's address.
    #[storage_mapper("admin")]
    fn admin(&self) -> SingleValueMapper<ManagedAddress>;


    /// @title Authorization Changed Event
    /// @notice Emitted whenever an entity's authorization status changes.
    /// @param entity The address of the entity.
    /// @param permission The type of permission granted or revoked.
    /// @param authorized True if authorized, false otherwise.
    #[event("AuthorizationChanged")]
    fn authorization_event(&self, entity: ManagedAddress);

}