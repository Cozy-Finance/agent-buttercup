# Agent Smith

Agent Smith is a flexible, Rust-based framework for conducting computationally-intensive agent-based simulations in an EVM environment.

The simulation engine is built on top of [revm](https://github.com/bluealloy/revm), enabling fast EVM execution without any I/O overhead.
It was originally inspired by work done on [Arbiter](https://github.com/primitivefinance/arbiter), but ultimately took a very different approach to the design of the core simulation loop and agents.

## Why?

The simulation engine enables DeFi researchers/engineers to leverage a quantitative approach to:
- Evaluating the effects of new protocol mechanisms on market outcomes
- Optimizing risk settings, model parameters and incentives
- Stress-testing the robustness of a protocol against adversarial economic attacks and market environments

## High-Level Architecture

The core components of the simulation engine are as follows:

- **State**, an object which keeps track of:
1. EVM state inside a revm instance
2. Any non-EVM state inside a separate "World" object

- **Agents**, who in each step, decide what to do based on the current state and send transactions to update the EVM or world state.

- **Time Policy**, which determines how block numbers/timestamps progress in each step.

- **Summary Generators**, which compute summary statistics from state and write them to a file for analysis.

- **Manager**, the core object responsible for coordinating the state, agents, time policy and summarizers.
The manager effectively runs a loop. In each step:

1. Agents are processed in parallel across multiple threads and asked to send the manager any updates
2. Updates are executed against the EVM and world state
3. Update results (Did the EVM tx revert?, etc.) are temporarily cached in state
4. Agents are given the ability to read state and update any local state they track
5. Summary generators are run and statistics are written to the file
6. The time policy is called and the EVM block number/timestamp are updated

![Simulation manager architecture](docs/assets/architecture.png)


## Repo Structure

The repo is set-up as a workspace with four crates:

- [`binder`](crates/binder): Tool for generating Rust bindings and some additional metadata from contract artifacts
- [`bindings`](crates/bindings): Rust contract bindings called inside the simulation
- [`simulate`](crates/simulate): Core logic of simulation engine
- [`simulations`](crates/simulations): Concrete implementations of the simulation engine for the Cozy protocol

## Installation

```
git clone https://github.com/Cozy-Finance/simulation-engine
cd simulation-engine
```

## Generating contract bindings

To interact with your smart contracts inside the simulation, you can generate Rust bindings from the artifacts.
To do so, use the [`binder`](crates/binder/main.rs) CLI:

```
cargo run --bin binder -- --abis <abis_path> --bindings <bindings_path>
```

Check out [`bind.sh`](bind.sh) for an example script which uses [forge](https://github.com/foundry-rs/foundry/tree/master/forge) to build a project and output bindings to [`crates/bindings`](crates/bindings).


## Usage

Setting up and running a simulation requires the following steps:

### Defining non-EVM state

The [`World`](crates/simulate/src/state/world.rs) trait object is a general data structure responsible for holding any non-EVM state which your agents need to access or update. 
`World` has an associated type `WorldUpdateData`, which it takes as an input to its `execute` method.

A concrete example of what you may want to include here are CEX asset prices.
In that case, `WorldUpdateData` could simply be a struct with a timestamp, asset and the new price.

In general, assuming your world is fairly complex, you will want `WorldUpdateData` to be an enum with different variants for different world updates.
An example implementation of this design for the Cozy protocol is [here](crates/simulations/src/cozy/world.rs).

### Defining a time policy

The [`TimePolicy`](crates/simulate/src/time_policy.rs) trait object defines how block numbers and timestamps progress in the EVM.
A concrete implementation is responsible for:
1. Outputting a the new block number and timestamp on each simulation step.
2. Defining when it terminates (i.e. sufficient time or number of blocks have passed).

The `FixedBlockTimePolicy` implementation moves a fixed amount of time or blocks per step.


### Defining summary generators

The [`Summarizer`](crates/simulate/src/summarizer/mod.rs) and associated `SummaryGenerator` trait objects are responsible for computing metrics/statistics you want to collect from the EVM or world state on each step.

At the end of each step, all generators are run and the jsons are written to the `Summarizer`'s specified output file.
All generators must output a [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html).
Remember to register each generator you want to use with `summarizer.register_summary_generator(generator)`.

An example implementation for the Cozy protocol is [here](crates/simulations/src/cozy/summary_generators/).

### Defining agents

The [`Agent`](crates/simulate/src/agent/mod.rs) trait object defines an agent in the simulation.

The following methods define the core behavior of an agent which gets called on each step of the simulation:

1. `step`: Read EVM and world state, compute the actions you want to take and send updates to the manager for execution
2. `resolve_step`: Read state and potentially update any of your local state based on the results of last step

There are analogous methods, `activation_step` and `resolve_activation_step`, which are only called when the agent is first registered with the manager.
The activation step can be used to run any EVM or world set-up prior to the core simulation loop.

For example implementations of agents in the Cozy protocol, check [here](crates/simulations/src/cozy/agents/).

#### Sending updates

Note the signature of `step`:
```rust
fn step(&mut self, state: &SimState<U, W>, channel: AgentChannel<U>)
```
The agent only gets read-only access to the state. To execute actions against state, the agent sends updates to the manager for execution via the channel:

```rust
let token_approval = build_call_tx(
    agent_address,
    token_address,
    token_contract.encode_function("approve", (approved_address, U256::MAX))
);
let price_update = CexPrice::new(timestamp, price);

channel.send(SimUpdate::Evm(token_approval));
channel.send(SimUpdate::World(price_update));
```

The agent can optionally tag updates with a string:
```rust
let token_approval_tag = format!("{} token approval", self.address);
channel.send_with_tag(
    SimUpdate::Evm(token_approval),
    token_approval_tag.into()
);
```
Using a tag allows the agent to easily read the result of their update in the resolve step:
```rust
let my_update_results = state.update_results.get(&self.address).unwrap();
let token_approval_result = my_update_results.get(token_approval_tag);
```

### Defining a manager and kicking off simulation loop

The [`SimManager`](crates/simulate/src/manager.rs) is the key object responsible for running the simulation.  

Given a definition of `World`, `TimePolicy` and `Summarizer`, you can initiate the manager as follows:

```rust
let sim_state = SimState::new(world);
let sim_manager = SimManager::new(
    sim_state,
    Box::new(time_policy),
    summarizer
);
```

Next, you will want to activate all your agents:

```rust
for agent in agents {
    sim_manager.activate_agent(agent)?;
}
```

Note the order in which you activate agents introduces on the order of which the agent's `activation_steps` are run.

Finally, you can kick off your simulation:
```rust
sim_manager.run_sim()?;
```

An example of running simulations of the Cozy protocol is [here](crates/simulations/src/cozy/runner.rs).