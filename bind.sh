#!/bin/bash

cargo build -p binder

if [ -d "contracts" ];
then
    rm -r "contracts"
fi

forge clean

echo "Installing cozy contracts ... "
forge install https://github.com/Cozy-Finance/cozy-protocol-v2 --no-commit
forge install https://github.com/Cozy-Finance/cozy-models-v2-simulation --no-commit
forge install https://github.com/Cozy-Finance/cozy-triggers-v2-simulation --no-commit
forge install https://github.com/gnosis/canonical-weth --no-commit
forge install https://github.com/Cozy-Finance/cozy-simulation-contracts --no-commit

echo "Building contract artifacts ... "
for d in ./contracts/*/ ; do (cd "$d" && [ -d "test" ] && mv "test" "test_ignore"); done
for d in ./contracts/*/ ; do (cd "$d" && [ -d "script" ] && mv "script" "script_ignore"); done
for d in ./contracts/*/ ; do (cd "$d" && FOUNDRY_PROFILE='default' forge build); done

echo "Removing some artifacts ... "
rm -r "contracts/cozy-protocol-v2/out/structs"

echo "Building rust bindings ... "
cargo run --bin binder -- --abis "contracts/cozy-triggers-v2-simulation/out" --bindings "crates/bindings" --mod-name "cozy_triggers" --overwrite
cargo run --bin binder -- --abis "contracts/cozy-models-v2-simulation/out" --bindings "crates/bindings" --mod-name "cozy_models" --overwrite
cargo run --bin binder -- --abis "contracts/cozy-protocol-v2/out" --bindings "crates/bindings" --mod-name "cozy_protocol" --overwrite
cargo run --bin binder -- --abis "contracts/canonical-weth/out" --bindings "crates/bindings" --mod-name "weth" --overwrite
cargo run --bin binder -- --abis "contracts/cozy-simulation-contracts/out" --bindings "crates/bindings" --mod-name "cozy_simulation" --overwrite