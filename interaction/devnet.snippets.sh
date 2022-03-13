##### - configuration - #####
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallets/test-wallet.pem"

################################################
TOKEN_ID="SVEN-4b35b0"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

REWARD_RATE=100000000000000000000             # 100 SVEN per second
MIN_STAKE_LIMIT=100000000000000000000       # 100 SVEN
LOCKING_TIMESTAMP=10                        # 10 seconds
PAUSED=1

CALLER_ADDRESS="erd1dl8ucerztz80eqtvs2u35vj5pckle3h3mnuce5fctyzxp4d74dfqwy7ntn"
CALLER_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"

################################################
ADDRESS=$(erdpy data load --key=address-devnet)
TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
################################################

deploy() {
    erdpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=50000000 \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_ID_HEX} ${REWARD_RATE} ${MIN_STAKE_LIMIT} ${LOCKING_TIMESTAMP} \
    --send \
    --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} \
    --metadata-payable \
    --metadata-payable-by-sc \
    --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}
}

unstake() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="unstake" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

claimReward() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="claimReward" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLive() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setPaused" \
    --arguments 0 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setPaused() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setPaused" \
    --arguments 1 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

withdraw() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="withdraw" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

### view
getRewardPerWei() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardPerWei"
}

getEarned() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getEarned" --arguments ${CALLER_ADDRESS_HEX}
}

getPaused() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getPaused"
}

getRewardRate() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardRate"
}

getTotalSupply() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getTotalSupply"
}

getUserRewardPerWeiPaid() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getUserRewardPerWeiPaid" --arguments ${CALLER_ADDRESS_HEX}
}

getReward() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getReward" --arguments ${CALLER_ADDRESS_HEX}
}

getBalance() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getBalance" --arguments ${CALLER_ADDRESS_HEX}
}

getLastStakeTimes() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getLastStakeTimes" --arguments ${CALLER_ADDRESS_HEX}
}