# Soroban::V1

[test account (https://laboratory.stellar.org/#account-creator?network=futurenet)]
Public Key	GB5IFWZQK5SENLTJWSGNULXPVMXLJFWNBEKKWWQ6OHIVMWG6KVDS7PIL
Secret Key	SAKOQ6DQJRIBE7O2SSBID2UEIRUDMNB4C65DZ6BVM3DHQ7ERYE62VEBY

[fund account (works if you have stellar running locally on docker)]
curl "http://localhost:8000/friendbot?addr=GB5IFWZQK5SENLTJWSGNULXPVMXLJFWNBEKKWWQ6OHIVMWG6KVDS7PIL"

[Confirm using horizon]
curl http://localhost:8000/accounts/GB5IFWZQK5SENLTJWSGNULXPVMXLJFWNBEKKWWQ6OHIVMWG6KVDS7PIL

[add testnet to global config]
stellar network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

[generate identity for testing]
stellar keys generate --global koha --network testnet

[reveal identity public key (append --no-fund to disable auto account refill using friendbot)]
stellar keys address koha

[deploy to local testnet]
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/play_soroban.wasm \
  --secret-key SBN3DHDHXYRR23BBLR3YEGDZSPVO2TDTYM7IYUH6OB5BUDTI33VRV5J7 \
  --rpc-url http://localhost:8000/soroban/rpc \
  --network-passphrase 'Standalone Network ; February 2017'

[deploy to public testnet]
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/play_soroban.wasm \
  --source koha \
  --network testnet

[contracts addr starts with C]
CCTBVGVUZR3P66BIALKZWEXCO27M3RZEL5KXGSK5TMHVNJCLA356U3GR

[invoke contract at addr]
stellar contract invoke \
  --id CCTBVGVUZR3P66BIALKZWEXCO27M3RZEL5KXGSK5TMHVNJCLA356U3GR \
  --source koha \
  --network testnet \
  -- \
  add \
  --left 4 \
  --right 7


