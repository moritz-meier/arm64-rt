# Rust on bare-metal AArch64

A crate for running Rust on bare-metal AArch64

- Startup Code
- Muli-Core
- Exception Level EL3-EL1 NS
- Cache Maintenance
- Virtual Memory
- PSCI support
- System Timer
- ARM Performance Monitoring Unit
- ARM Coresight STM Instrumentation Trace

## Example

```
cd ./example
cargo build --target aarch64-unknown-none
cargo run --target aarch64-unknown-none
```

### Run on ZynqMP Board

Xilinx `xsdb` debugger is needed  
for example from https://github.com/DLR-FT/xilinx-nix-utils/tree/zynq-modules (`nix develop .#xilinx-lab`):


```
xsdb

connect
target 10 # (or whatever number Cortex-A53 #0 is)
# ensure board has booted into U-Boot
stop
dow ./target/aarch64-unknown-none/debug/example
con
```

Connect gdb debugger to xsdb:
```
rust-gdb ./target/aarch64-unknown-none/debug/example

target extended-remote :3001

```