# Assembly Visualizer

This repo contains a small interpreter-ish for ARM assembly that exposes a `State` object after every execution, and a React app that visualizes that state per instruction. It uses fake values and does not actually execute the assembly. It powers a React app that can take any assembly code and visualize it cleanly, showing how values move around and how the computer actually executes assembly.

This is meant as a learning project. I am using it to learn assembly, Rust, and how to create interpreters. It's definitely not "optimized." It's a tiny work in progress.