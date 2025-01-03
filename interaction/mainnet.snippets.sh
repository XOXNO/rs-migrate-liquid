ADDRESS=erd1qqqqqqqqqqqqqpgqc0jp2q280xaccqszxwsh5cyl2hv35g79ah0sk4zu5n
PROXY=https://gateway.xoxno.com
PROJECT="./output/migrate.wasm"

# Token identifiers
SEGLD_SC=erd1qqqqqqqqqqqqqpgq4gzfcw7kmkjy8zsf04ce6dl0auhtzjx078sslvrf4e
HEGLD_SC=erd1qqqqqqqqqqqqqpgq35qkf34a8svu4r2zmfzuztmeltqclapv78ss5jleq3
HSEGLD_SC=erd1qqqqqqqqqqqqqpgqxmn4jlazsjp6gnec95423egatwcdfcjm78ss5q550k

# Smart contract addresses
SEGLD=str:SEGLD-3ad2d0
HEGLD=str:HEGLD-d61095
HSEGLD=str:HSEGLD-c13a4e
XOXNO_LIQUID_SC=erd1qqqqqqqqqqqqqpgq6uzdzy54wnesfnlaycxwymrn9texlnmyah0ssrfvk6

# NFT ticker
NFT_TICKER=str:HLSR-374950

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} \
    --arguments ${SEGLD} ${HEGLD} ${HSEGLD} ${SEGLD_SC} ${HEGLD_SC} ${HSEGLD_SC} ${XOXNO_LIQUID_SC} ${NFT_TICKER} \
    --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return

    echo "New smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "Upgrade smart contract address: ${ADDRESS}"
    mxpy contract upgrade ${ADDRESS} --bytecode=${PROJECT} \
    --arguments ${SEGLD} ${HEGLD} ${HSEGLD} ${SEGLD_SC} ${HEGLD_SC} ${HSEGLD_SC} ${XOXNO_LIQUID_SC} ${NFT_TICKER} \
    --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return
}

unDelegate() {
    mxpy --verbose contract call ${ADDRESS} \
    --function="unDelegate" \
    --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=50000000 \
    --proxy=${PROXY} \
    --chain=1 \
    --send || return
}

withdraw() {
    NONCE=0xf5
    mxpy --verbose contract call ${ADDRESS} \
    --function="withdraw" \
    --arguments ${NONCE} \
    --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=50000000 \
    --proxy=${PROXY} \
    --chain=1 \
    --send || return
}

getVirtualEgldAdded() {
    mxpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function="getVirtualEgldAdded"
}

getPendingSegld() {
    mxpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function="getPendingSegld"
}