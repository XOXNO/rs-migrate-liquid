use crate::proxy_segld;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();
pub const WAD: u64 = 1_000_000_000_000_000_000;

#[multiversx_sc::module]
pub trait UtilsModule: crate::storage::StorageModule {
    fn get_segld_exchange_rate(&self) -> BigUint {
        let rate = self
            .tx()
            .to(self.segld_sc().get())
            .typed(proxy_segld::LiquidStakingProxy)
            .get_exchange_rate()
            .returns(ReturnsResult)
            .sync_call_readonly();

        rate
    }

    fn shares_to_egld(&self, shares: &BigUint) -> BigUint {
        let wad = BigUint::from(WAD);
        let fx = self.get_segld_exchange_rate();
        fx * shares / wad
    }
}
