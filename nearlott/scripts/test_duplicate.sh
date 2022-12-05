set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=dev-1669739821818-27891091064609
CONTRACT_ACC=dev-1669739821818-27891091064609
OWNER=dev-1669739821818-27891091064609
TREASURY_ACC=dev-1669739821818-27891091064609
INJECTOR_ACC=dev-1669739821818-27891091064609
OPERATOR_ACC=dev-1669739821818-27891091064609

export NEAR_ENV=$NETWORK

echo "Start Test !"

function buy_ticket() {
    near call $CONTRACT_ACC --accountId=duonghb3.testnet buy_tickets '{
        "_lottery_id": 2,
        "_ticket_numbers":[1233145,1233146]
    }' --deposit=2
}

buy_ticket &
buy_ticket &
wait
echo "End Test !"