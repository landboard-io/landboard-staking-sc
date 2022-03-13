#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();


mod storage;

#[elrond_wasm::derive::contract]
pub trait Staking:
    storage::StorageModule
{
    /*
        @param reward_rate: reward token amount created in every second
     */
    #[init]
    fn init(&self,
        reward_token_id: TokenIdentifier,
        stake_token_id: TokenIdentifier,
        reward_rate: BigUint,
        min_stake_limit: BigUint,
        locking_timestamp: u64
    ){
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

        self.min_stake_limit().set(&min_stake_limit);
        self.locking_timestamp().set(locking_timestamp);
        self.paused().set(0u32);    // live
    }

    /// endpoint
    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self, #[payment_token] stake_token_id: TokenIdentifier, #[payment_amount] stake_amount: BigUint) {
        self.require_activation();

        require!(
            stake_token_id == self.stake_token_id().get(),
            "invalid stake_token_id"
        );
        require!(
            stake_amount >= self.min_stake_limit().get(),
            "cannot stake less than min_stake_limit"
        );

        let caller = self.blockchain().get_caller();

        // update all factors
        self.update_reward(&caller);

        self.total_supply().update(|v| *v += &stake_amount
        );
        self.balances(&caller).update(|v| *v += &stake_amount);
        self.last_stake_times(&caller).set(self.blockchain().get_block_timestamp());
    }

    #[endpoint(unstake)]
    fn unstake(&self, #[var_args] opt_unstake_amount: OptionalValue<BigUint>) {
        self.require_activation();
        self.require_check_locking();

        let caller = self.blockchain().get_caller();

        require!(
            self.balances(&caller).get() > BigUint::zero(),
            "zero balance"
        );

        // update all factors
        self.update_reward(&caller);

        // if unstake_amount is not given, unstake all staked balance
        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(value) => {
                require!(
                    self.balances(&caller).get() >= value,
                    "unstake_amount cannot be greater than balance"
                );

                value
            },
            OptionalValue::None => self.balances(&caller).get()
        };

        self.total_supply().update(|v| *v -= & unstake_amount
        );
        self.balances(&caller).update(|v| *v -= & unstake_amount
        );

        require!(
            self.blockchain().get_sc_balance(&self.stake_token_id().get(), 0) >= unstake_amount,
            "not enough staking tokens in smart contract"
        );
        
        self.send().direct(&caller, &self.stake_token_id().get(), 0, &unstake_amount, &[]);
    }

    #[endpoint(claimReward)]
    fn claim_reward(&self) {
        self.require_activation();
        self.require_check_locking();

        let caller = self.blockchain().get_caller();

        // update all factors
        self.update_reward(&caller);

        let reward_amount = self.rewards(&caller).get();

        require!(
            reward_amount > BigUint::zero(),
            "nothing to claim"
        );

        self.rewards(&caller).update(|v| *v -= &reward_amount
        );

        require!(
            self.blockchain().get_sc_balance(&self.reward_token_id().get(), 0) >= reward_amount,
            "not enough rewarding tokens in smart contract"
        );
        
        self.send().direct(&caller, &self.reward_token_id().get(), 0, &reward_amount, &[]);
    }

    /// view

    #[view(getRewardPerWei)]
    fn get_reward_per_wei(&self) -> BigUint {
        let reward_per_wei_stored = self.reward_per_wei_stored().get();
        return if self.total_supply().get() == BigUint::zero() {
            reward_per_wei_stored
        } else {
            let time_delta = BigUint::from(self.blockchain().get_block_timestamp() - self.last_update_time().get());

            reward_per_wei_stored + &time_delta * &self.reward_rate().get() / &self.total_supply().get()
        }
    }

    #[view(getEarned)]
    fn get_earned(&self, user_address: &ManagedAddress) -> BigUint {
        let reward_pert_token_delta = &self.get_reward_per_wei() - &self.user_reward_per_wei_paid(user_address).get();
        
        reward_pert_token_delta * &self.balances(user_address).get() + &self.rewards(user_address).get()
    }

    /// private
    #[inline]
    fn update_reward(&self, user_address: &ManagedAddress) {
        self.reward_per_wei_stored().set(&self.get_reward_per_wei());
        self.last_update_time().set(self.blockchain().get_block_timestamp());
        
        self.rewards(user_address).set(self.get_earned(user_address));
        self.user_reward_per_wei_paid(user_address).set(&self.reward_per_wei_stored().get());
    }

    #[inline]
    fn require_activation(&self) {
        require!(
            self.paused().get() == 0u32,
            "staking is not live"
        );
    }

    #[inline]
    fn require_check_locking(&self) {
        let caller = self.blockchain().get_caller();

        require!(
            self.last_stake_times(&caller).get() + self.locking_timestamp().get() <= self.blockchain().get_block_timestamp(),
            "you cannot unstake or claim reward before locking_timestamp"
        );
    }

    /// endpoint - only_owner
    
    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self,
        #[var_args] opt_token_id: OptionalValue<TokenIdentifier>,
        #[var_args] opt_token_amount: OptionalValue<BigUint>) {
        // if token_id is not given, set it to eGLD
        let token_id = match opt_token_id {
            OptionalValue::Some(v) => v,
            OptionalValue::None => TokenIdentifier::egld()
        };
        // if token_amount is not given, set it to balance of SC - max value to withdraw
        let token_amount = match opt_token_amount {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_sc_balance(&token_id, 0)
        };

        self.send().direct(&self.blockchain().get_caller(), &token_id, 0, &token_amount, &[]);
    }
}
