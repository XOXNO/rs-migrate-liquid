#![no_std]

use multiversx_sc::hex_literal::hex;
#[allow(unused_imports)]
use multiversx_sc::imports::*;
use proxy_segld::UndelegateAttributes;
pub mod proxy_liquid;
pub mod proxy_market;
pub mod proxy_segld;
pub mod storage;
pub mod utils;

const xxx: [u8; 32] = hex!("00000000000000000500aa049c3bd6dda4438a097d719d37efef2eb148cff1e1");
const trs: [u8; 32] = hex!("00000000000000000500aa049c3bd6dda4438a097d719d37efef2eb148cff1e1");

#[multiversx_sc::contract]
pub trait Migrate: crate::storage::StorageModule + crate::utils::UtilsModule {
    #[init]
    fn init(
        &self,
        segld: TokenIdentifier,
        hegld: TokenIdentifier,
        hsegld: TokenIdentifier,
        segld_sc: ManagedAddress,
        hegld_sc: ManagedAddress,
        hsegld_sc: ManagedAddress,
        xoxno_liquid_sc: ManagedAddress,
        nft_ticker: TokenIdentifier,
    ) {
        self.segld().set(segld);
        self.hegld().set(hegld);
        self.hsegld().set(hsegld);
        self.segld_sc().set(segld_sc);
        self.hegld_sc().set(hegld_sc);
        self.hsegld_sc().set(hsegld_sc);
        self.liquid_sc().set(xoxno_liquid_sc);
        self.nft_ticker().set(nft_ticker);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable('*')]
    #[endpoint]
    fn migrate(&self) {
        let payment = self.call_value().single_esdt();

        let segld = self.segld().get();
        let hegld = self.hegld().get();
        let hsegld = self.hsegld().get();

        match payment.token_identifier.clone() {
            id if id == segld => self.migrate_segld(payment),
            id if id == hegld => self.migrate_hegld(payment),
            id if id == hsegld => self.migrate_hsegld(payment),
            _ => {
                panic!("Invalid token identifier");
            }
        }
    }

    #[endpoint]
    fn un_delegate(&self) {
        let segld = self.segld().get();
        let amount = self.pending_segld().get();

        let unbond_nft = self
            .tx()
            .to(self.liquid_sc().get())
            .typed(proxy_segld::LiquidStakingProxy)
            .undelegate(OptionalValue::<ManagedAddress>::None)
            .single_esdt(&segld, 0, &amount)
            .returns(ReturnsBackTransfersSingleESDT)
            .sync_call();

        let data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_caller(),
            &unbond_nft.token_identifier,
            unbond_nft.token_nonce,
        );

        let mut contracts: ManagedVec<Self::Api, ManagedAddress> = ManagedVec::new();
        contracts.push(ManagedAddress::new_from_bytes(&xxx));
        contracts.push(ManagedAddress::new_from_bytes(&trs));

        let attributes = data.decode_attributes::<UndelegateAttributes<Self::Api>>();

        require!(
            contracts.contains(&attributes.delegation_contract),
            "Out of sync"
        );

        self.pending_segld().clear();
    }

    #[endpoint]
    fn withdraw(&self, nonce: u64) {
        let nft_ticker = self.nft_ticker().get();
        let epoch = self.blockchain().get_block_epoch();
        let data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_caller(),
            &nft_ticker,
            nonce,
        );

        let attributes = data.decode_attributes::<UndelegateAttributes<Self::Api>>();

        require!(attributes.unbond_epoch <= epoch, "Invalid epoch");

        let egld = self
            .tx()
            .to(self.liquid_sc().get())
            .typed(proxy_segld::LiquidStakingProxy)
            .withdraw()
            .single_esdt(&nft_ticker, nonce, &data.amount)
            .returns(ReturnsBackTransfersEGLD)
            .sync_call();

        self.tx()
            .to(self.liquid_sc().get())
            .typed(proxy_liquid::LiquidStakingProxy)
            .migrate_pending()
            .egld(egld)
            .sync_call();
    }

    fn migrate_segld(&self, payment: EsdtTokenPayment) {
        self.pending_segld()
            .update(|pending| *pending += &payment.amount);

        let egld_amount = self.shares_to_egld(&payment.amount);

        let xegld_payment = self
            .tx()
            .to(self.liquid_sc().get())
            .typed(proxy_liquid::LiquidStakingProxy)
            .migrate(egld_amount)
            .returns(ReturnsBackTransfersSingleESDT)
            .sync_call();

        let caller = self.blockchain().get_caller();
        self.tx().to(caller).esdt(xegld_payment).transfer();
    }

    fn migrate_hegld(&self, payment: EsdtTokenPayment) {
        let caller = self.blockchain().get_caller();
        let back_transfers = self
            .tx()
            .to(self.hegld_sc().get())
            .typed(proxy_market::MoneyMarketProxy)
            .redeem(OptionalValue::<BigUint>::None)
            .with_esdt_transfer(payment)
            .returns(ReturnsBackTransfers)
            .sync_call();

        let egld_amount = back_transfers.total_egld_amount;

        require!(egld_amount > BigUint::zero(), "Invalid token identifier");

        let xegld_payment = self
            .tx()
            .to(self.liquid_sc().get())
            .typed(proxy_liquid::LiquidStakingProxy)
            .delegate()
            .egld(egld_amount)
            .returns(ReturnsBackTransfersSingleESDT)
            .sync_call();

        require!(
            xegld_payment.amount > BigUint::zero(),
            "Invalid token identifier"
        );

        self.tx().to(caller).esdt(xegld_payment).transfer();
    }

    fn migrate_hsegld(&self, payment: EsdtTokenPayment) {
        let back_transfers = self
            .tx()
            .to(self.hsegld_sc().get())
            .typed(proxy_market::MoneyMarketProxy)
            .redeem(OptionalValue::<BigUint>::None)
            .with_esdt_transfer(payment)
            .returns(ReturnsBackTransfers)
            .sync_call();

        let segld_payment = back_transfers.esdt_payments.get(0);

        let segld = self.segld().get();

        require!(
            segld_payment.token_identifier == segld,
            "Invalid token identifier"
        );

        self.migrate_segld(segld_payment);
    }
}