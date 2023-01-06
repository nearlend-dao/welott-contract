set -e
NETWORK=testnet
SUFFIX=testnet

MASTER_ACC=lamns1.$NETWORK
CONTRACT_ACC=welott40.$MASTER_ACC
OWNER=$MASTER_ACC
TREASURY_ACC=lamns1.$NETWORK
INJECTOR_ACC=lamns1.$NETWORK
OPERATOR_ACC=lamns1.$NETWORK

export NEAR_ENV=$NETWORK

echo "############### CONFIG OPERATOR AND TREASURY AND INJECTOR ADDRESS #####################"
near call $CONTRACT_ACC --accountId=$OWNER set_operator_and_treasury_and_injector_addresses '{
    "_injector_address":"'$INJECTOR_ACC'",
    "_operator_address": "'$OPERATOR_ACC'",
    "_treasury_address": "'$TREASURY_ACC'"
}'


echo "####################### GET CONFIG CONTRACT #########################"
near view $CONTRACT_ACC get_config ''


echo "############### CONFIG MAX NUMBER TICKET BUY #####################"

near call $CONTRACT_ACC --accountId=$OWNER set_max_number_tickets_per_buy '{
    "_max_number_tickets_per_buy":100
}'

echo "####################### GET CONFIG CONTRACT #########################"
near view $CONTRACT_ACC get_config ''