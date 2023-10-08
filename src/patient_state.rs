multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct PatientInfo<M: ManagedTypeApi> {
    pub id: ManagedByteArray<M, 32>,
    pub name: ManagedBuffer<M>,
    pub dob: ManagedBuffer<M>,
    pub address: ManagedAddress<M>,
    pub sex: ManagedBuffer<M>,
    // pub tickets: ManagedByteArray<M, 32>
}

impl<M> PatientInfo<M>
where
    M: ManagedTypeApi
{
        
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Ticket<M: ManagedTypeApi> {
    pub id: ManagedByteArray<M, 32>,
    pub data: ManagedBuffer<M>,
    pub result: ManagedBuffer<M>,
    pub validation: ManagedBuffer<M>
}