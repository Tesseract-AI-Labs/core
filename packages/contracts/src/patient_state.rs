multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct PatientInfo<M: ManagedTypeApi> {
    pub id: ManagedByteArray<M, 32>,
    pub name: ManagedBuffer<M>,
    pub dob: ManagedBuffer<M>,
    pub address: ManagedAddress<M>,
    pub sex: ManagedBuffer<M>,
    pub tickets: ManagedVec<M, Ticket<M>>
}

impl<M> PatientInfo<M>
where
    M: ManagedTypeApi
{
        
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem)]
pub struct Ticket<M: ManagedTypeApi> {
    pub id: ManagedByteArray<M, 32>,
    pub timestamp: u64,
    pub analysis_result: ManagedBuffer<M>,
    pub validation: ManagedBuffer<M>
}

