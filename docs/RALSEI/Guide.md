### 1. Creating a Module -
In *Ralsei*, modules are to be defined in the following manner.
1) A regular Rust struct is to be instantiated.
2) The struct is then to be marked with the `#[ralsei(module)]` attribute, so that *Ralsei* can detect it as a module.
3) The struct may have fields of any type, including native Rust types and types from external crates. However, these fields are ignored in the netlist. Only the fields of type `Input<PortType>` and `Output<PortType>` (where `PortType` is a *Ralsei* specific logic type which implements `Ported` trait, making it able to connect to a module port).
4) The `#[ralsei(module)]` attribute mandates that the module must implement the `Module` trait, which contains the mandatory `def_module(&self)` function that defines the logic and working of the module.
5) To instantiate the module, it can be instantiated like a regular Rust struct. Ports of the type `Input<PortType>` can be connected to external wires and buses using the `.connect_in()` method, while ports of the type `Output<PortType>` can be connected to external wires and buses using the `.connect_out()` method.
6) Modules need not contain only synthesizable code. They may contain regular non-synthesizable code too, which *Ralsei* smartly categorizes and integrates into the simulator.

### 2. Creating a Testbench -
In *Ralsei*, a testbench is a specific kind of module which contains non-synthesizable code that can be used to simulate and test user defined circuits. Testbenches are to be defined in the following manner.
1) A regular Rust struct is to be instantiated.
2) The struct is then to be marked with the `#ralsei(testbench)]` attribute, so that *Ralsei* can detect it as a testbench. As of now, there can only be one testbench in the entire program.
3) The struct may have any fields, or lack fields too. Testbenches should not contain input and output ports as found in modules.
4) Marking the struct as a testbench using the `#ralsei(testbench)]` attribute necessitates implementing the `TestBench` trait for the module, which requires implementing the `test(&mut self)` function, and optionally the `setup(&mut self)` function. The `test(&mut self)` function contains the actual logic that will be run for simulation during a test, while the `setup(&mut self)` function sets up and initializes all fields before the simulation starts (i.e. initial values of the simulation at t <= 0), functioning similarly to an `initial` block in *Verilog*/*SystemVerilog*.
5) Testbenches cannot be used as regular modules, and are not synthesizable into physical circuits. They exist purely for the purpose of simulation.