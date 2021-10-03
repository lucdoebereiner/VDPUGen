#!/bin/bash

cargo build --release

cd genscwrapper
cargo run -- --name VDPUGen --outputs 1 -a "in, e, a, f"
cd ../sc
make ugen UGEN=VDPUGen
make install UGEN=VDPUGen
cd ..
