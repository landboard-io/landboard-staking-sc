elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone,)]
pub enum State {
    Live,
    Paused,
}
