#!/bin/bash

forge clean

forge install https://github.com/Cozy-Finance/cozy-protocol-v2 --no-commit
forge install https://github.com/Cozy-Finance/cozy-models-v2-simulation --no-commit
forge install https://github.com/Cozy-Finance/cozy-triggers-v2-simulation --no-commit

forge build --skip script test

cargo run -- --abis "../../contracts/cozy-triggers-v2-simulation/out" --bindings "../bindings" --mod-name "cozy_triggers" --overwrite
cargo run -- --abis "../../contracts/cozy-models-v2-simulation/out" --bindings "../bindings" --mod-name "cozy_models" --overwrite
cargo run -- --abis "../../contracts/cozy-protocol-v2/out" --bindings "../bindings" --mod-name "cozy_protocol" --overwrite