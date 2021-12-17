# ic rust call

demo for how to use rust call the canister

## build

get caller's did file:

```sh
sudo cargo r > caller/caller.did
```

get caller's wasm file:
```sh
sudo cargo build --release --target wasm32-unknown-unknown

sudo ic-cdk-optimizer target/wasm32-unknown-unknown/release/caller.wasm -o target/wasm32-unknown-unknown/release/opt.wasm
```

## run

```sh
# deploy
sudo dfx deploy --no-wallet

# counter
dfx canister --no-wallet call counter get           
(0 : nat)
dfx canister --no-wallet call counter add '(10:nat)'
(10 : nat)
dfx canister --no-wallet call counter msgCaller     
(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")
dfx canister --no-wallet call counter set '(4)'
()
dfx canister --no-wallet call counter get           
(4 : nat)
dfx canister --no-wallet call counter inc      
()
dfx canister --no-wallet call counter get           
(5 : nat)

# caller
 dfx canister --no-wallet id counter
rrkah-fqaaa-aaaaa-aaaaq-cai
dfx canister --no-wallet call caller get "(principal \"$(dfx canister --no-wallet id counter)\")"
(5 : nat)
dfx canister --no-wallet call caller setCounter '(principal "rrkah-fqaaa-aaaaa-aaaaq-cai")'
()
dfx canister --no-wallet call caller add '(10:nat)'(15 : nat)
dfx canister --no-wallet call caller msgCaller     
(principal "rwlgt-iiaaa-aaaaa-aaaaa-cai")
dfx canister --no-wallet call caller set '(33:nat)'
()
dfx canister --no-wallet call caller get "(principal \"$(dfx canister --no-wallet id counter)\")"
(33 : nat)
dfx canister --no-wallet call caller inc           ()
dfx canister --no-wallet call caller get "(principal \"$(dfx canister --no-wallet id counter)\")"
(34 : nat)