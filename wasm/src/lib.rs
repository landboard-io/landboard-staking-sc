////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    simple_staking
    (
        claimReward
        getBalance
        getEarned
        getLastStakeTimes
        getLastUpdateTime
        getLockingTimestamp
        getMinStakeLimit
        getPaused
        getReward
        getRewardPerWei
        getRewardPerWeiStored
        getRewardRate
        getRewardTokenId
        getStakingTokenId
        getTotalSupply
        getUserRewardPerWeiPaid
        setLockingTimestamp
        setMinStakeLimit
        setPaused
        setRewardRate
        setRewardTokenId
        setStakeTokenId
        stake
        unstake
        withdraw
    )
}

elrond_wasm_node::wasm_empty_callback! {}
