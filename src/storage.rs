multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getSegld)]
    #[storage_mapper("segld")]
    fn segld(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getSegldSc)]
    #[storage_mapper("segld_sc")]
    fn segld_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getHegld)]
    #[storage_mapper("hegld")]
    fn hegld(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getHegldSc)]
    #[storage_mapper("hegld_sc")]
    fn hegld_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getHsegld)]
    #[storage_mapper("hsegld")]
    fn hsegld(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getHsegldSc)]
    #[storage_mapper("hsegld_sc")]
    fn hsegld_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getLiquidSc)]
    #[storage_mapper("liquid_sc")]
    fn liquid_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getNftTicker)]
    #[storage_mapper("nft_ticker")]
    fn nft_ticker(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getPendingSegld)]
    #[storage_mapper("pending_segld")]
    fn pending_segld(&self) -> SingleValueMapper<BigUint>;

    #[view(getVirtualEgldAdded)]
    #[storage_mapper("virtual_egld_added")]
    fn virtual_egld_added(&self) -> SingleValueMapper<BigUint>;
}
