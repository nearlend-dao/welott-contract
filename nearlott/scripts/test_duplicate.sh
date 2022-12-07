set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=dev-1670172000695-69146865022481
CONTRACT_ACC=dev-1670172000695-69146865022481
OWNER=dev-1670172000695-69146865022481
TREASURY_ACC=dev-1670172000695-69146865022481
INJECTOR_ACC=dev-1670172000695-69146865022481
OPERATOR_ACC=dev-1670172000695-69146865022481

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