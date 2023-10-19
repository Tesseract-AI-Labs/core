/// MultiversX Smart Contract Imports
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// @title Patient Information Struct
/// @notice This struct holds the information related to a patient.
/// @param M The managed type API used for handling complex data types.
/// @field id A unique identifier for the patient.
/// @field name The name of the patient.
/// @field dob The date of birth of the patient.
/// @field address The address of the patient.
/// @field sex The sex of the patient.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct PatientInfo<M: ManagedTypeApi> {
    pub id: ManagedByteArray<M, 32>,
    pub name: ManagedBuffer<M>,
    pub dob: ManagedBuffer<M>,
    pub address: ManagedAddress<M>,
    pub sex: ManagedBuffer<M>,
}
impl<M> PatientInfo<M>
where
    M: ManagedTypeApi
{
        
}

/// @title Ticket Struct
/// @notice This struct holds information related to a ticket.
/// @param M The managed type API used for handling complex data types.
/// @field id A unique identifier for the ticket.
/// @field timestamp The timestamp when the ticket was created.
/// @field analysis_result The result of the analysis associated with this ticket.
/// @field validation The validation information for the analysis result.
/// @field patient_id The unique identifier of the patient associated with this ticket.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem)]
pub struct Ticket<M: ManagedTypeApi> {
    pub id: ManagedByteArray<M, 32>,
    pub timestamp: u64,
    pub analysis_result: ManagedBuffer<M>,
    pub validation: ManagedBuffer<M>,
    pub patient_id: ManagedByteArray<M, 32>
}

/// @title Model Struct
/// @notice This struct holds information related to a model.
/// @param M The managed type API used for handling complex data types.
/// @field id A unique identifier for the model.
/// @field timestamp The timestamp when the model was created.
/// @field model_hash The hash of the model.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem)]
pub struct Model<M: ManagedTypeApi> {
    pub id: ManagedByteArray<M, 32>,
    pub timestamp: u64,
    pub model_hash: ManagedBuffer<M>
}

/// @title Metrics Struct
/// @notice This struct holds metrics related to a model or analysis.
/// @param M The managed type API used for handling complex data types.
/// @field id A unique identifier for the metrics.
/// @field timestamp The timestamp when the metrics were recorded.
/// @field accuracy The accuracy metric.
/// @field log_loss The log loss metric.
/// @field rmse The Root Mean Square Error (RMSE) metric.
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem)]
pub struct Metrics<M: ManagedTypeApi> {
    pub id: ManagedByteArray<M, 32>,
    pub timestamp: u64,
    pub accuracy: u64,
    pub log_loss: u64,
    pub rmse: u64,
}


