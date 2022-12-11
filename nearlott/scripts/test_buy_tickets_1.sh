set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=dev-1670690337621-13695714686091
CONTRACT_ACC=dev-1670690337621-13695714686091
OWNER=dev-1670690337621-13695714686091
TREASURY_ACC=dev-1670690337621-13695714686091
INJECTOR_ACC=dev-1670690337621-13695714686091
OPERATOR_ACC=dev-1670690337621-13695714686091
DECIMAL_20=00000000000000000000
DECIMAL_23=00000000000000000000000
DECIMAL_24=000000000000000000000000
GAS=300000000000000
DUONGHB3=duonghb3.testnet

export NEAR_ENV=$NETWORK

echo "Start Test !"

function buy_ticket() {
    near call $CONTRACT_ACC --accountId=$OWNER buy_tickets '{
        "_lottery_id": 1,
        "_ticket_numbers":[
          1233145,1233146,1233145,1233146,1233145,1233146,
          1233145,1233146,1233145,1233146,1233145,1233146
          ]
    }' --depositYocto=1200000000000000000000000 --gas=$GAS
}
for i in {1..40}
do
   buy_ticket
done
near view $CONTRACT_ACC view_number_tickets_per_lottery '{"_lottery_id": 1}'
near view $CONTRACT_ACC view_user_info_for_lottery_id '{"_user": "'$CONTRACT_ACC'", "_lottery_id": 1, "_cursor": 0, "_size": 1000}'

echo "End Test !"