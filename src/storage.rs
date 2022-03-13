elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::State;

#[elrond_wasm::module]
pub trait StorageModule {
    /// chnage
    
    #[only_owner]
    #[endpoint(setRewardTokenId)]
    fn set_reward_token_id(&self, reward_token_id: TokenIdentifier) {
        require!(
            reward_token_id.is_valid_esdt_identifier(),
            "invalid reward_token_id"
        );

        self.reward_token_id().set(&reward_token_id);
    }

    #[only_owner]
    #[endpoint(setStakeTokenId)]
    fn set_stake_token_id(&self, stake_token_id: TokenIdentifier) {
        require!(
            stake_token_id.is_valid_esdt_identifier(),
            "invalid stake_token_id"
        );

        self.stake_token_id().set(&stake_token_id);
    }

    #[only_owner]
    #[endpoint(setRewardRate)]
    fn set_reward_rate(&self, reward_rate: BigUint) {
        self.reward_rate().set(&reward_rate);
    }

    #[only_owner]
    #[endpoint(setMinStakeLimit)]
    fn set_min_stake_limit(&self, min_stake_limit: BigUint) {
        self.min_stake_limit().set(&min_stake_limit);
    }

    #[only_owner]
    #[endpoint(setLockingTimestamp)]
    fn set_locking_timestamp(&self, locking_timestamp: u64) {
        self.locking_timestamp().set(&locking_timestamp);
    }

    #[only_owner]
    #[endpoint(setState)]
    fn set_state(&self, state: State) {
        self.state().set(state);
    }

    /// storage

    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getStakingTokenId)]
    #[storage_mapper("stake_token_id")]
    fn stake_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // in wei
    #[view(getRewardRate)]
    #[storage_mapper("reward_rate")]
    fn reward_rate(&self) -> SingleValueMapper<BigUint>;

    #[view(getLastUpdateTime)]
    #[storage_mapper("last_update_time")]
    fn last_update_time(&self) -> SingleValueMapper<u64>;

    #[view(getRewardPerWeiStored)]
    #[storage_mapper("reward_per_wei_stored")]
    fn reward_per_wei_stored(&self) -> SingleValueMapper<BigUint>;

    // 

    #[view(getMinStakeLimit)]
    #[storage_mapper("min_stake_limit")]
    fn min_stake_limit(&self) -> SingleValueMapper<BigUint>;

    #[view(getLockingTimestamp)]
    #[storage_mapper("locking_timestamp")]
    fn locking_timestamp(&self) -> SingleValueMapper<u64>;

    #[view(getState)]
    #[storage_mapper("state")]
    fn state(&self) -> SingleValueMapper<State>;

    // stats

    #[view(getTotalSupply)]
    #[storage_mapper("total_supply")]
    fn total_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getUserRewardPerWeiPaid)]
    #[storage_mapper("user_reward_per_wei_paid")]
    fn user_reward_per_wei_paid(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getReward)]
    #[storage_mapper("rewards")]
    fn rewards(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getBalance)]
    #[storage_mapper("balances")]
    fn balances(&self, user_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getLastStakeTimes)]
    #[storage_mapper("last_stake_times")]
    fn last_stake_times(&self, user_address: &ManagedAddress) -> SingleValueMapper<u64>;
}