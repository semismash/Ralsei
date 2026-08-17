### 1. Creating a Module -
In *Ralsei*, modules are to be defined in the following manner.
1) A regular Rust struct is to be instantiated.
2) The struct is then to be marked with the `#[ralsei(module)]` attribute, so that *Ralsei* can detect it as a module.
3) The struct may have fields of any type, including native Rust types and types from external crates. However, these fields are ignored in the netlist. Only the fields of type `Input<PortType>` and `Output<PortType>` (where `PortType` is a *Ralsei* specific logic type which implements `Ported` trait, making it able to connect to a module port).
4) Parameters can be defined using **const generic** parameters, while local parameters can be defined in the `impl` block of the module as const variables.
5) The `#[ralsei(module)]` attribute mandates that the module must implement the `Module` trait, which contains the mandatory `def_module(&self)` function that defines the logic and working of the module.
6) To instantiate the module, it can be instantiated like a regular Rust struct. Ports of the type `Input<PortType>` can be connected to external wires and buses using the `.connect_in()` method, while ports of the type `Output<PortType>` can be connected to external wires and buses using the `.connect_out()` method.
7) Modules need not contain only synthesizable code. They may contain regular non-synthesizable code too, which *Ralsei* smartly categorizes and integrates into the simulator.
#### Example usage
##### Declaring a Module -
```rust
#[ralsei(module)]
pub struct SyncFIFO
<const DATA_WIDTH: usize = 8, const FIFO_DEPTH: usize = 4> 
{
	// clock and reset
	clk:       Input<Clock>,
	rst_n:     Input<Reset>,
	// inputs
	wr_en:     Input<Bit>,
	rd_en:     Input<Bit>,
	data_in:   Input<BitVec<DATA_WIDTH>>,
	// outputs
	data_out:  Output<BitVec<DATA_WIDTH>>,
	full:      Output<Bit>,
	empty:     Output<Bit>,
}
```
##### Defining a Module -
```rust
impl<const DATA_WIDTH: usize, const FIFO_DEPTH: usize> Module for SyncFIFO<DATA_WIDTH, FIFO_DEPTH> {
	// local param as const variable
	const PTR_WIDTH: usize = FIFO_DEPTH.ilog2();
	
	// module definition and function here
	fn def_module(&self) {
		
		// mem array and pointers
		let fifo_mem: [BitVec<DATA_WIDTH>; FIFO_DEPTH];
		let wr_ptr: BitVec<{ Self::PTR_WIDTH + 1 }> = BitVec::new();
		let rd_ptr: BitVec<{ Self::PTR_WIDTH + 1 }> = BitVec::new();
		
		// status flags
		self.empty.assign(wr_ptr == rd_ptr);
		self.full.assign(
		(wr_ptr.at::<{ Self::PTR_WIDTH }>() != rd_ptr.at::<{ Self::PTR_WIDTH }>()) && 
		(wr_ptr.slice::<{ Self::PTR_WIDTH - 1 }, 0>() == rd_ptr.slice::<{ Self::PTR_WIDTH - 1 }, 0>()));
		
		// addresses
		let wr_addr = UInt::<{ Self::PTR_WIDTH }>::from_bits(wr_ptr.slice::<{ Self::PTR_WIDTH - 1 }, 0>());
		let rd_addr = UInt::<{ Self::PTR_WIDTH }>::from_bits(rd_ptr.slice::<{ Self::PTR_WIDTH - 1 }, 0>());
		
		// write logic
		#[ralsei(on_edge(posedge self.clk, negedge self.rst_n))]
		{
			if self.rst_n.active() {
				wr_ptr = BitVec::from_usize(0);
			} else if self.wr_en && !self.full {
				wr_ptr = wr_ptr + Bit::init(High);
				fifo_mem[wr_addr] = self.data_in;
			}
		}
		
		// read logic
		#[ralsei(on_edge(posedge self.clk, negedge self.rst_n))]
		{
			if self.rst_n.active() {
				rd_ptr = BitVec::from_usize(0);
				self.data_out = BitVec::from_usize(0);
			} else if self.rd_en && !self.empty {
				rd_ptr = rd_ptr + Bit::init(High);
				self.data_out = fifo_mem[rd_addr];
			}
		}
	}
}

```
##### Instantiating a Module -
```rust
// in body of testbench or another module -
let DUT: SyncFIFO;
DUT = SyncFIFO {
	// clk and rst_n
	clk:      self.tb_clk.connect_in();
	rst_n:    self.tb_rst_n.connect_in();
	// inputs
	wr_en:    self.tb_wr_en.connect_in();
	rd_en:    self.tb_rd_en.connect_in();
	data_in:  self.tb_data_in.connect_in();
	// outsputs
	data_out: self.tb_data_out.connect_out();
	full:     self.tb_full.connect_out();
	empty:    self.tb_empty.connect_out();
}
```

### 2. Creating a Testbench -
In *Ralsei*, a testbench is a specific kind of module which contains non-synthesizable code that can be used to simulate and test user defined circuits. Testbenches are to be defined in the following manner.
1) A regular Rust struct is to be instantiated.
2) The struct is then to be marked with the `#ralsei(testbench)]` attribute, so that *Ralsei* can detect it as a testbench. As of now, there can only be one testbench in the entire program.
3) The struct may have any fields, or lack fields too. Testbenches should not contain input and output ports as found in modules.
4) Marking the struct as a testbench using the `#ralsei(testbench)]` attribute necessitates implementing the `TestBench` trait for the module, which requires implementing the `test(&mut self)` function, and optionally the `setup(&mut self)` function. The `test(&mut self)` function contains the actual logic that will be run for simulation during a test, while the `setup(&mut self)` function sets up and initializes all fields before the simulation starts (i.e. initial values of the simulation at t <= 0), functioning similarly to an `initial` block in *Verilog*/*SystemVerilog*.
5) Testbenches cannot be used as regular modules, and are not synthesizable into physical circuits. They exist purely for the purpose of simulation.