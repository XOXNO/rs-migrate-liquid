ADDRESS=erd1qqqqqqqqqqqqqpgqfq0yn2v5ejl42wqx5a8g0fsgq4j8pujpah0stdg9y7
PROXY=https://devnet-gateway.xoxno.com
PROJECT="./output/migrate.wasm"

# Token identifiers
SEGLD_SC=erd1qqqqqqqqqqqqqpgqvg8r5yavkyhu6rmmkgqzgsduzheg2fk7v5ysrypdex
HEGLD_SC=erd1qqqqqqqqqqqqqpgq2udp46dvs4cvp4urak39t2fqxp7t3lpzv5ysec452j
HSEGLD_SC=erd1qqqqqqqqqqqqqpgq4q8w5kr0mnyhtydm23mcgqsu5d4yxym4v5ysxl7cat

# Smart contract addresses
SEGLD="str:SEGLD-cc1abc"
HEGLD="str:HEGLD-ae8054"
HSEGLD="str:HSEGLD-dfbd96"
XOXNO_LIQUID_SC=erd1qqqqqqqqqqqqqpgqc2d2z4atpxpk7xgucfkc7nrrp5ynscjrah0scsqc35

# NFT ticker
NFT_TICKER="str:HLSR-2096d3"

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} \
    --arguments ${SEGLD} ${HEGLD} ${HSEGLD} ${SEGLD_SC} ${HEGLD_SC} ${HSEGLD_SC} ${XOXNO_LIQUID_SC} ${NFT_TICKER} \
    --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=D || return

    echo "New smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "Upgrade smart contract address: ${ADDRESS}"
    mxpy contract upgrade ${ADDRESS} --bytecode=${PROJECT} \
    --metadata-payable-by-sc \
    --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=600000000 --send --proxy=${PROXY} --chain="D" || return
}