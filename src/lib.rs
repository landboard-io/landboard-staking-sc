#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const EGLD_IN_WEI: u64 = 1_000_000_000_000_000_000u64;

#[elrond_wasm::derive::contract]
pub trait Staking {
    #[init]
    fn init(&self, reward_token_id: TokenIdentifier, stake_token_id: TokenIdentifier, reward_rate: BigUint){
        require!(
            reward_token_id.is_valid_esdt_identifier(),
            "invalid reward_token_id"
        );
        require!(
            stake_token_id.is_valid_esdt_identifier(),
            "invalid stake_token_id"
        );

        self.reward_token_id().set(&reward_token_id);
        self.stake_token_id().set(&stake_token_id);
        self.reward_rate().set(&reward_rate);
    }

    /// endpoint
    // #[payable("*")]
    // #[endpoint(stake)]
    // fn stake(&self, #[payment_token] stake_token_id: TokenIdentifier, #[payment_amount] stake_amount: BigUint) {
    //     require!(stake_token_id == self.stake_token_id().get(), "invalid stake_token_id");

    //     let caller = self.blockchain().get_caller();

    //     // update all factors
    //     self.update_reward(caller);

    //     self.total_supply().update(|v| *v += & stake_amount
    //     );
    //     self.balances(&caller).update(|v| *v += &stake_amount);
    // }

    // #[endpoint(unstake)]
    // fn unstake(&self, #[var_args] opt_unstake_amount: OptionalValue<BigUint>) {
    //     let caller = self.blockchain().get_caller();

    //     require!(self.balances(&caller).get() > BigUint::zero(), "zero balance");

    //     // update all factors
    //     self.update_reward(caller);

    //     // if unstake_amount is not given, unstake all staked balance
    //     let unstake_amount = match opt_unstake_amount {
    //         OptionalValue::Some(v) => {
    //             require!(self.balances(&caller).get() >= v, "unstake_amount cannot be greater than balance");
    //             v
    //         },
    //         OptionalValue::None => self.balances(&caller).get()
    //     };

    //     self.total_supply().update(|v| *v -= & unstake_amount
    //     );
    //     self.balances(&caller).update(|v| *v -= & unstake_amount
    //     );

    //     require!(self.blockchain().get_sc_balance(&self.stake_token_id().get(), 0) >= unstake_amount, "not enough staking tokens in smart contract");
        
    //     self.send().direct(&caller, &self.stake_token_id().get(), 0, &unstake_amount, &[]);
    // }

    // #[endpoint(claimReward)]
    // fn claim_reward(&self) {
    //     let caller = self.blockchain().get_caller();

    //     // update all factors
    //     self.update_reward(caller);

    //     let reward = self.rewards(&caller).get();

    //     self.rewards(&caller).set(BigUint::zero());

    //     require!(self.blockchain().get_sc_balance(&self.reward_token_id().get(), 0) >= reward, "not enough rewarding tokens in smart contract");
        
    //     self.send().direct(&caller, &self.reward_token_id().get(), 0, &reward, &[]);
    // }

    /// view

    #[view(getRewardPerToken)]
    fn get_reward_per_token(&self) -> BigUint {
        return if self.total_supply().get() == BigUint::zero() {
            BigUint::zero()
        } else {
            let temp = &BigUint::from(self.blockchain().get_block_timestamp() - self.last_update_time().get()) * &self.reward_rate().get() * &BigUint::from(EGLD_IN_WEI);
            &temp / &self.total_supply().get() + &self.reward_per_token_stored().get()
        }
    }

    #[view(getEarned)]
    fn get_earned(&self, user_address: &ManagedAddress) -> BigUint {
        let temp = self.balances(user_address).get() * (&self.get_reward_per_token() - &self.user_reward_per_token_paid(user_address).get());
        &temp / &BigUint::from(EGLD_IN_WEI) +
        &self.rewards(user_address).get()
    }

    /// private
    fn update_reward(&self, user_address: &ManagedAddress) {
        self.reward_per_token_stored().set(&self.get_reward_per_token());
        self.last_update_time().set(self.blockchain().get_block_timestamp());
        
        self.rewards(user_address).set(self.get_earned(user_address));
        self.user_reward_per_token_paid(user_address).set(&self.reward_per_token_stored().get());
    }

    /// storage

    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getStakingTokenId)]
    #[storage_mapper("stake_token_id")]
    fn stake_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getRewardRate)]
    #[storage_mapper("reward_rate")]
    fn reward_rate(&self) -> SingleValueMapper<BigUint>;

    #[view(getLastUpdateTime)]
    #[storage_mapper("last_update_time")]
    fn last_update_time(&self) -> SingleValueMapper<u64>;

    #[view(getRewardPerTokenStored)]
    #[storage_mapper("reward_per_token_stored")]
    fn reward_per_token_stored(&self) -> SingleValueMapper<BigUint>;

    //

    #[view(getUserRewardPerTokenPaid)]
    #[storage_mapper("user_reward_per_token_paid")]
    fn user_reward_per_token_paid(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getReward)]
    #[storage_mapper("rewards")]
    fn rewards(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getTotalSupply)]
    #[storage_mapper("total_supply")]
    fn total_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getBalance)]
    #[storage_mapper("balances")]
    fn balances(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
