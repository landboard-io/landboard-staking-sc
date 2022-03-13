##### - configuration - #####
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallets/test-wallet.pem"

################################################
TOKEN_ID="SVEN-4b35b0"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

REWARD_RATE=1000000000000000000             # 1 SVEN per second
MIN_STAKE_LIMIT=100000000000000000000       # 100 SVEN
LOCKING_TIMESTAMP=10                        # 10 seconds

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

### view
getRewardPerToken() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardPerToken"
}

get_earned() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardPerToken" --arguments ${CALLER_ADDRESS_HEX}
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