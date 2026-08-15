Given are some of the features of ***Ralsei***, listed below -

### Primitive Types -

*Ralsei* features types that mirror languages such as *Verilog* and *SystemVerilog* to keep naming as consistent as possible. Following are the types that *Ralsei* uses -
##### Logic - 
**Logic** is a fundamental type (similar to a non-vector `logic` in *SystemVerilog*, or `reg`/`wire` in *Verilog*). It represents one of four states - `0`, `1`, `X` or `Z`, and can be synthesized by the synthesizer into either a register or a wire depending on the usage.
The four states representable by Logic are as follows -
- `0` - Zero (LOW)
- `1` - One (HIGH)
- `X` - Unknown State/Don't Care
- `Z` - High Impedance State
It is defined as -
```rust
pub struct Logic {
	// private fields...
}
```
##### LogicVec -
**LogicVec** (Logic Vector, `LogicVec<W>`) is a type which acts similar to a `logic` vector in *SystemVerilog*, essentially functioning as a bus containing multiple different contiguous **Logic** cells. **LogicVec**s can be used with synthesizable operations, either with other **Logic**s, **LogicVec**s, or other types like **numerical types** (to be described later), although only with certain methods and restrictions.
It is defined as -
```rust
// W: width of the logic vector
pub struct LogicVec<const W: usize> {
	// private fields...
}
```
The width of a **LogicVec** is constant at compile time and cannot be changed dynamically at runtime.
##### Bit -
**Bit** is a single-width type (similar to **Logic**), but unlike **Logic**, **Bit** can strictly represent just two states - `0` and `1`, similar to the `bit` datatype in *SystemVerilog*. 
It is defined as -
```rust
pub struct Bit {
	// private fields...
}
```
##### BitVec -
**BitVec** (Bit Vector, `BitVec<W>`) is a type that indicates a bus of multiple contiguous **Bit**s. Just like **Bit** and unlike it's **LogicVec** counterpart, **BitVec** can strictly represent either `0` or `1` in each of its cells. **BitVecs** can be used with synthesizable operations, with other **Bit**s, **BitVec**s, or numerical types, more flexibly compared to Logic types.
It is defined as -
```rust
// W: width of the bit vector
pub struct BitVec<const W: usize> {
	// private fields...
}
```
Similar to **LogicVec**, the width of a **BitVec** is fixed as compile time and cannot be changed dynamically at runtime.

### Structural Types -

*Ralsei* uses multiple structural types apart from the aforementioned ones, which it uses to synthesize circuits. The structural types included in *Ralsei* are given as follows -
##### Input and Output -
**Input** (`Input<PortType>`) is a type that indicates an Input port for a module. Here, `PortType` is a generic type which only accepts types which implement the `Ported` trait (i.e. types which can be connected as an Input/Output port). The **Input** type can only be used properly as a port type when defining a module struct (as done using the `#[ralsei(module)])` attribute as stated later), and cannot normally be used with any meaningful functionality in other parts of the code.
**Output** (`Output<PortType>`) is a type that indicates the Output port for a module. Similarly to **Input**, **Output** can only be used properly for a struct that is defined as a module use the `#[ralsei(module)]` attribute.
The `.connect_in()` and `.connect_out()` methods (which are defined for all types that implement `Ported`) are required to connect an variable to the module's port. `.connect_in()` connects the variable to the input port, while `.connect_out()` connects a variable to the output port. **An important note is that `.connect_out()` binds the module's physical output directly to the receiver variable, meaning that its value can be sampled separately later on.**
Example -
```rust
#[ralsei(module)]
pub struct Concat32to16 {
	in_a: Input<BitVec<32>>,
	in_b: Input<BitVec<16>>,
	out: Output<BitVec<48>>,
} // assume Module trait has been implemented for this struct already

// under simulation code
#[ralsei(testbench)]
pub struct MyTestBench {}
impl TestBench for MyTestBench {  // mandatory trait for testbench struct
	fn test(&self) {  // testbench code runs in this function
		let vec_a: BitVec<32> = BitVec::<32>::new();
		let vec_b: BitVec<16> = BitVec::<16>::new();
		let vec_c: BitVec<48> = BitVec::<48>::new();
	
		// .connect_in() internally casts the PortType to Input<PortType> and plugs it into the port of the instantiated module. Same applies to .connect_out(), which casts from PortType to Output<PortType>
		let DUT: Concat32to16 = 
			Concat32to16 {
				in_a: vec_a.connect_in(),
				in_b: vec_b.connect_in(),
				out: vec_c.connect_out(),
			}
		// other simulation code
	}
}
```
In addition, if a port isn't strictly either an **Input** or an **Output** port, the **InOut** (Input-Output, `InOut<PortType>`) type can be used, allowing the port to either drive or be driven. *(More info TBA later.)*
### Inbuilt Types -

In addition to the existing primitive types, *Ralsei* also includes some in-built types which can be defined by the user separately, but are included within the crate along with associated methods for consistency and ease of use.
##### Clock -
**Clock** is a type based on **Bit** which serves as a unified clock signal which can be used for clock triggered circuits. By default, it is set to trigger on positive edge (`posedge`), although the clock can be set to trigger on negative edge (`negedge`) too, or custom behavior.
##### Reset -
**Reset** is a type based on **Bit** which serves as a unified reset signal for respective circuits. By default, it triggers on active low reset, and when used asynchronously, it triggers on negative edge `negedge`. However, similar to **Clock**, custom behavior may be set for it too.
#### Numerical Types -
These types can be used to perform math operations like addition, multiplication, etc., and be used with their respective operations. **BitVec**s can and must be cast explicitly to the respective numerical type before an operation is performed on them.
##### Int -
**Int** (Integer, `Int<W>`) is a custom-width numerical type defined on **BitVec**, which can be used to perform numerical operations. It represents a **signed integer**, and operations performed on it automatically sign-extend and map to signed integer types. **Int** types can only operate with other **Int**s directly, unless a different type is explicitly cast to an **Int** first.
Here, `W` represents the width of the **Int** in bits. Certain operations may allow operations between Integers of different widths and have specific behavior, while others outright deny it.
Usage -
```rust
// exact syntax TBA, pseudocode below
let int_a: Int<8> = Int::from(56); // initialized int of width = 8 and value = +56
let int_b: Int<13> = Int::from(-28324); // initialized int of width = 13 and value = -28324
let int_c: Int<4> = Int::from(33); // ERROR: size of given int value (33) > 2 ** (W - 1) - 1 (2^4 = 16; must be in between -8 and +8)
let int_empty: Int<4> = Int::<4>::new(); // initializes an empty Integer
```
##### UInt -
**UInt** (Unsigned Integer, `UInt<W>`) is a custom-width numerical type defined on **BitVec**, which can be used to perform numerical operations. Unlike **Int**, it represents an **unsigned integer**, and operations performed on it automatically zero-extend and map to unsigned integer types. **UInt** types can only operate with other **UInt**s directly, unless casting is done to **UInt** from the other type.
Here, `W` represents the width of the **UInt** in bits. Certain operations may allow operations between unsigned integers of different widths and have specific behavior, while others outright deny it.
Usage -
```rust
// exact syntax TBA, pseudocode below
let uint_a: UInt<9> = UInt::from(173); // initialized int of width = 9 and value = 173
let uint_b: UInt<15> = UInt::from(-234); // ERROR: expected unsigned integer, got signed
let uint_c: UInt<5> = UInt::from(54); // ERROR: size of given int value (54) > 2 ** W - 1 (2^5 = 32)
```

### Language Grammar and Syntax -

As *Ralsei* is a Rust crate and works entirely within Rust's compile-time and runtime itself, a large number of existing Rust features and keywords are repurposed to be used for design, synthesis and simulation by *Ralsei*, many of which are analogous to *Verilog/SystemVerilog* features.
#### Module Instantiation for Structs via the `#[ralsei(module)]` attribute and the `ralsei::Module` Trait -
*Ralsei* allows defining structs in code as normal, and requires them for creating a hardware module. However, while normal Rust structs can be defined, *Ralsei* will only detect it as a hardware module if the `#[ralsei(module)]` attribute is used with the struct, and if it follows a specific format. If the attribute is used but the format is not followed, a compile-time error will be raised.
Example usage -
```rust
#[ralsei(module)]
pub struct HalfAdder {
	in_a: Input<Bit>, // use Input<T> for Input ports
	in_b: Input<Bit>,
	out_sum: Output<Bit>, // use Output<T> for Output ports
	out_carry: Output<Bit>,
}

impl Module for HalfAdder { // the #[ralsei(module)] attribute will ensure this trait is implemented by the user
	fn def_module(&self) { // main logic of module implemented here
		// re-evaluate keeping it as self.in_a and self.in_b; i.e. whether auto-deref to be implemented or not
		let sum: Bit = self.in_a ^ self.in_b; // declare a temp var for assigment
		self.out_sum.assign(sum);
		self.out_carry.assign(self.in_a & self.in_b); // or do directly too
	}
}
```
#### Const Generics as a replacement for `parameter`, and `const` values as a replacement for `localparam` -
As *Ralsei* modules are defined in structs, they can use Rust's const generic and local const mechanisms to define module-specific values that are similar to `parameter` and `localparam` in *Verilog*.
`parameters` can be emulated fall in *Ralsei* by using Rust's const generic parameters and defining them as a part of the module. A single module may have multiple const generic parameters, as there is no limit to the number of parameters a module may have. These const generic parameters can possess any Rust specific data-type or even custom types defined by the user/other crates, as Ralsei's synthesizer will not consider the datatype of the const parameter unless it is specific used for synthesis. In addition, const generic module parameters can be set to have a default value, unless explicitly specified by the user during instantiation. Similarly, `localparams` can be emulated by using struct-specific `const` variables, defined within an `impl` block of the module.
Example Usage -
```rust
#[ralsei(module)]
pub struct Mux2Way<const BIT_WIDTH: usize = 32> {
	in_a: Input<LogicVec<BIT_WIDTH>>,
	in_b: Input<LogicVec<BIT_WIDTH>>,
	out: Output<LogicVec<BIT_WIDTH>>,
}

impl Mux2Way {
	const LOCALPARAM_1: u8 = 67;
	const LOCALPARAM_2: bool = true;
}

impl Module for Mux2Way {
	fn def_module(&self) {
		// module implementation logic here...
	}
}
```
#### Enums as a means of modelling explicit states -
Rust's powerful enum system it very flexible and straightforward to model discrete states that *Ralsei* uses during synthesis. 
To make a Rust enum be recognized and synthesizable by Ralsei, the `#[ralsei(enum)]` attribute must be used. In addition, to make the said enum's variants map to certain user-given values, the `#[ralsei(repr(Type))]` attribute must be used, along with assigning the values directly via ` = <given value>` in front of the enum variant.
Example usage -
```rust
#[ralsei(enum)]
#[ralsei(repr(Int<8>))]
enum ColorState {
	Black = 0,
	Red = 1,
	Green = 2,
	Blue = 3,
	Yellow = 4,
	Cyan = 5,
	Magenta = 6,
	White = 255,
}
```
Just like regular Rust enums, *Ralsei*-recognized enums also conform to much of Rust's existing mechanisms such as exhaustive matching and even tuple/struct enum variants (assuming that they match with the `#[ralsei(repr(Type))]` attribute, if set for the enum.
**NOTE: *Ralsei* also mandates that all match blocks that are a part of synthesizable code must use a catch-all (`_`) case to prevent unwanted latches from forming.** *(TBD: Specific attribute to prevent Ralsei from mandating a catch-all statement in a synthesizable match block, although Rust's compiler will still check if all enum variants are matched or not. The tentative name for this attribute is `#[ralsei(allow_no_default_case)]`)*
Example usage -
```rust
#[ralsei(enum)]
enum ParseAction {
	AddVal(u8),
	DelVal,
	Submit,
	Cancel,
}

impl Ported for ParseAction {
	// Ported impl code...
}

#[ralsei(module)]
struct Parser {
	parse_action: Input<ParseAction>,
	action: Output<BitVec<2>>,
	has_char: Output<Bit>,
	char_val: Output<UInt<8>>,
}

impl Module for Parser {
	fn def_module(&self) {
		let action: BitVec<2>;
		let has_char: Bit = Bit::from_bool(false);
		let char_val: UInt<8> = UInt::<8>::from_u8(0);
		match self.parse_action {
			AddVal(val) => {
				action = BitVec::<2>::from(0b00);
				has_char = Bit::from_bool(true);
				char_val = UInt::<8>::from_u8(val);
			},
			DelVal => {
				action = BitVec::<2>::from(0b01);
			},
			Submit => {
				action = BitVec::<2>::from(0b10);
			},
			Cancel => {
				action = BitVec::<2>::from(0b11);
			},
			_ => {
				action = BitVec::<2>::from(0b00);
			},
		}
		self.action.assign(action);
		self.has_char.assign(has_char);
		self.char_val.assign(char_val);
	}
}
```
#### Edge-triggered behavior using the `#[ralsei(on_edge(...))]` attribute
*Ralsei* allows for clock triggered behavior using the `#[ralsei(on_edge(...))]` attribute, hence allowing the synthesized code block to execute once per edge as stated in the attribute.
The attribute syntax is as follows -
- Attribute Name - `#[ralsei(on_edge(...))]`
- Keywords -
	- `posedge` - Evaluated at positive (rising) edge of the signal
	- `negedge` - Evaluated at negative (falling) edge of the signal
	- `anyedge` - Evaluated at either the positive or negative edge of the signal (or a general change in the signal)
- The rules for `posedge`, `negedge`, and `anyedge` transitions are as follows -
	- `posedge` transition -
		- `0` -> `1`
		- `0` -> `X`
		- `0` -> `Z`
		- `X` -> `1`
		- `Z` -> `1`
	- `negedge` transition -
		- `1` -> `0`
		- `1` -> `X`
		- `1` -> `Z`
		- `X` -> `0`
		- `Z` -> `0`
	- `anyedge` transition -
		- Any valid `posedge` or `negedge` case
- After the keyword, the name of the signal is to be specified. The signal must be of either `Logic` or `Bit` type. If a different type is used, then the synthesizer will reject it and raise a compile-time error. (**NOTE: Clock and Reset type signals are allowed, as they are wrappers around the Bit type.**)
- If multiple signals are to be detected, then the specified signals are to be separated with commas (e.g. `#[ralsei(on_edge(posedge clk, negedge rst_n))])`. The synthesizer will not raise an error if a signal is repeated multiple times (although it's not recommended).
When the attribute is use, certain rules are enforced and must be conformed too for the code within -
- The attribute can only be used just before a scope. This scope is effectively treated as an `always` block in Verilog.
- All assignments (`=`) are non-blocking. This means that they are evaluated at the end of the signal edge (i.e. any signal edge specified in the parameter list).
- Temporary local variables cannot be declared in the scope. *Ralsei* will verify that no local variables have been declared in the scope.
Example Usage -
```rust
#[ralsei(module)]
pub struct DFF {
	clk: Input<Clock>,
	rst_n: Input<Reset>,
	d: Input<Logic>,
	q: Output<Logic>,
}

impl Module for DFF {
	fn def_module(&self) {
		#[ralsei(on_edge(posedge self.clk, negedge self.rst_n))]
		{
			if self.rst_n.active() {
				self.q = Logic::init(High);
			} else {
				self.q = self.d;
			}
		}
	}
}
```
### Operations -

*Ralsei* features a variety of operations for simulation and synthesis, many of which are present in existing HDLs, along with some extra *Ralsei* specific ones for added functionality and convenience.
The operations supported by *Ralsei* are listed as follows -
#### Logical -
##### NOT (`!`) -
The **NOT** operation (denoted by `!`) is an operation defined in *Ralsei* which inverts the signal as the output. It is defined on Logic, Bit, and Numerical types, returning the corresponding type as the output port.
Given below is the truth table for the NOT operation -

| Input | Output |
| ----- | ------ |
| 0     | 1      |
| 1     | 0      |
| X     | X      |
| Z     | X      |
**Note: In the above table, X and Z only apply for Logic and LogicVec types, as Bit and Numerical Types are guaranteed not to have unknown values in synthesis.**

Supported Type Combinations -
- !`Logic` -> `Logic`
- !`LogicVec<W>` -> `LogicVec<W>`
- !`Bit` -> `Bit` (including derivatives like `Clock` and `Reset`)
- !`BitVec<W>` -> `BitVec<W>`
- !`Int<W>` -> `Int<W>`
- !`UInt<W>` -> `UInt<W>`
**NOTE: For future reference, unless specified otherwise, all given type combinations in the documentation are to be assumed as commutative and associative.**

Usage -
```rust
// exact syntax TBA
// pseudocode - 
let a: Logic = Logic::new();
let b: Logic = Logic::new();
b.assign(!a); // output is NOT of the 'a' logic
let c: LogicVec<4> = LogicVec::<4>::new(); // width = 4
let d: LogicVec<4> = LogicVec::<4>::new();
d.assign(!c); // output (every corresponding bit in c is inverted to the respective bit in d)
let e: LogicVec<7> = LogicVec::<7>::new();
let f: LogicVec<11> = LogicVec::<11>::new();
f.assign(!e); // incorrect (bus widths do not match)
```
##### AND (`&`) -
The **AND** operation (denoted by `&`) is an operation which takes two inputs and outputs the AND result of them. If the input is a bus, the corresponding bits have the AND operation performed with each other. The two inputs must have the same width for the operation to be valid.
Given below is the truth table for the AND operation -

| **Input 1** | **Input 2** | **Output** |
| ----------- | ----------- | ---------- |
| 0           | 0           | 0          |
| 0           | 1           | 0          |
| 1           | 0           | 0          |
| 1           | 1           | 1          |
| X/Z         | 0           | 0          |
| X/Z         | 1           | X          |
| 0           | X/Z         | 0          |
| 1           | X/Z         | X          |
| X/Z         | X/Z         | X          |
Supported Type Combinations -
- `Logic` & `Logic` -> `Logic`
- `LogicVec<W>` & `LogicVec<W>` -> `LogicVec<W>`
- `Bit` & `Bit` -> `Bit` (including derivatives like `Clock` and `Reset`)
- `Logic` & `Bit` -> `Logic`
- `BitVec<W>` & `BitVec<W>` -> `BitVec<W>`
- `BitVec<W>` & `LogicVec<W>` -> `LogicVec<W>`
- `Int<W>` & `Int<W>` -> `Int<W>`
- `UInt<W>` & `UInt<W>` -> `UInt<W>`

Usage -
```rust
let a: Logic::init(Low);
let b: Logic::init(High);
let c: Logic::new();
c.assign(a & b);
let p: LogicVec::<4>::from_usize(8);
let q: LogicVec::<4>::from_usize(5);
let s: LogicVec::<4>::from_usize(0);
let s: LogicVec::<4>::new();
s.assign(p & q & r);
let m: LogicVec::<6>::from_usize(7);
let n: LogicVec::<7>::from_usize(6);
let o: LogicVec::<5>::new();
o.assign(m & n); // ERROR: any gates that are used in the same operation MUST be of the same or equivalent types and MUST be the same width
```
##### OR (`|`) -
The **OR** operation (denoted by `|`) is an operation which takes two inputs and outputs the OR result of them. If the input is a bus, the corresponding bits have the OR operation performed with each other. The two inputs must have the same width for the operation to be valid.
Given below is the truth table for the OR operation -

| Input A | Input B | Output |
| ------- | ------- | ------ |
| 0       | 0       | 0      |
| 0       | 1       | 1      |
| 1       | 0       | 1      |
| 1       | 1       | 1      |
| X/Z     | 0       | X      |
| X/Z     | 1       | 1      |
| 0       | X/Z     | X      |
| 1       | X/Z     | 1      |
| X/Z     | X/Z     | X      |
Supported Type Combinations -
- `Logic` | `Logic` -> `Logic`
- `LogicVec<W>` | `LogicVec<W>` -> `LogicVec<W>`
- `Bit` | `Bit` -> `Bit` (including derivatives like `Clock` and `Reset`)
- `Logic` | `Bit` -> `Logic`
- `BitVec<W>` | `BitVec<W>` -> `BitVec<W>`
- `BitVec<W>` | `LogicVec<W>` -> `LogicVec<W>`
- `Int<W>` | `Int<W>` -> `Int<W>`
- `UInt<W>` | `UInt<W>` -> `UInt<W>`

Usage -
```rust
let a: Logic::init(Low);
let b: Logic::init(High);
let c: Logic::new();
c.assign(a | b);
let p: LogicVec::<4>::from_usize(8);
let q: LogicVec::<4>::from_usize(5);
let s: LogicVec::<4>::from_usize(0);
let s: LogicVec::<4>::new();
s.assign(p | q | r);
let m: LogicVec::<6>::from_usize(7);
let n: LogicVec::<7>::from_usize(6);
let o: LogicVec::<5>::new();
o.assign(m | n); // ERROR: any gates that are used in the same operation MUST be of the same or equivalent types and MUST be the same width
```
##### XOR (`^`) -
The **XOR** operation (denoted by `^`) is an operation which takes two inputs and outputs the XOR result of them. If the input is a bus, the corresponding bits have the XOR operation performed with each other. The two inputs must have the same width for the operation to be valid.
Given below is the truth table for the XOR operation -

| Input A | Input B | Output |
| ------- | ------- | ------ |
| 0       | 0       | 0      |
| 0       | 1       | 1      |
| 1       | 0       | 1      |
| 1       | 1       | 0      |
| X/Z     | 0       | X      |
| X/Z     | 1       | X      |
| 0       | X/Z     | X      |
| 1       | X/Z     | X      |
| X/Z     | X/Z     | X      |
Supported Type Combinations -
- `Logic` ^ `Logic` -> `Logic`
- `LogicVec<W>` ^ `LogicVec<W>` -> `LogicVec<W>`
- `Bit` ^ `Bit` -> `Bit` (including derivatives like `Clock` and `Reset`)
- `Logic` ^ `Bit` -> `Logic`
- `BitVec<W>` ^ `BitVec<W>` -> `BitVec<W>`
- `BitVec<W>` ^ `LogicVec<W>` -> `LogicVec<W>`
- `Int<W>` ^ `Int<W>` -> `Int<W>`
- `UInt<W>` ^ `UInt<W>` -> `UInt<W>`

Usage -
```rust
let a: Logic::init(Low);
let b: Logic::init(High);
let c: Logic::new();
c.assign(a | b);
let p: LogicVec::<4>::from_usize(8);
let q: LogicVec::<4>::from_usize(5);
let s: LogicVec::<4>::from_usize(0);
let s: LogicVec::<4>::new();
s.assign(p | q | r);
let m: LogicVec::<6>::from_usize(7);
let n: LogicVec::<7>::from_usize(6);
let o: LogicVec::<5>::new();
o.assign(m | n); // ERROR: any gates that are used in the same operation MUST be of the same or equivalent types and MUST be the same width
```
#### Arithmetic -
##### ADD (`+`) -
The **ADD** operation (`+`) is an operation which takes in two numerical inputs, performs arithmetic addition, and returns the sum of them. The two inputs must have the same width and format, otherwise a compile-time error will be raised.
Addition of the same two numeric types outputs the corresponding type with a bit width expansion of +1. This is done to account for any leftover carry (the MSB of the new Integer is the carry bit by default). This does not apply for `Numeric Type` + `Bit` additions.

Supported Type Combinations -
- `Int<W>` + `Int<W>` -> `Int<{W + 1}>`
- `UInt<W>` + `UInt<W>` -> `UInt<{W + 1}>`
- `Int<W>` + `Bit` -> `Int<W>`
- `UInt<W>` + `Bit` -> `UInt<W>`

Usage -
```rust
let addend_a: Int<32> = Int::<32>::from_isize(13423);
let addend_b: Int<32> = Int::<32>::from_i32(-23219);
let sum = addend_a + addend_b;
let out: Int<33> = Int::<33>::new(); // width of sum is width of addends + 1
out.assign(sum);
let p: UInt<32> = UInt::<32>::from_u32(3432);
let q: Int<16> = Int::<16>::from_isize(299);
let r: UInt<33> = UInt::<33>::new();
r.assign(p + q); // ERROR: types and widths must match (or be a valid combination)
let m: UInt<32> = UInt::<32>::new();
let n: Bit = Bit::init(1);
let o: UInt<32> = UInt::<32>::new();
o.assign(m + n); // this works as it is incrementation, input and output bit width is the same
```
##### SUB (`-`) -
The **SUB** operation (`-`) is an operation which takes in two numerical inputs, performs arithmetic subtraction, and returns the difference of them. The two inputs must have the same width and format, otherwise a compile-time error will be raised. Unlike addition, it is NOT commutative.
Similar to addition, subtraction of the same two numeric types outputs the corresponding type with a bit width expansion of +1. This is done to account for any overflow (the MSB of the new Integer is the overflow bit by default). This does not apply for `Numeric Type` - `Bit` subtractions.

Supported Type Combinations -
- `Int<W>` - `Int<W>` -> `Int<{W + 1}>`
- `UInt<W>` - `UInt<W>` -> `UInt<{W + 1}>`
- \*`Int<W>` - `Bit` -> `Int<W>`
- \*`UInt<W>` - `Bit` -> `UInt<W>`
*\*Not commutative*

Usage -
```rust
let minuend: Int<32> = Int::<32>::from_isize(13423);
let subtrahend: Int<32> = Int::<32>::from_i32(-23219);
let difference = minuend - subtrahend;
let out: Int<33> = Int::<33>::new(); // width of difference is width of terms + 1
out.assign(difference);
let p: UInt<32> = UInt::<32>::from_u32(3432);
let q: Int<16> = Int::<16>::from_isize(299);
let r: UInt<33> = UInt::<33>::new();
r.assign(p - q); // ERROR: types and widths must match (or be a valid combination)
let m: UInt<32> = UInt::<32>::new();
let n: Bit = Bit::init(0);
let o: UInt<32> = UInt::<32>::new();
o.assign(m - n); // this works as it is decrementation, input and output bit width is the same
```
##### MUL (`*`) -
The **MUL** operation (`*`) is an operation which takes in two numerical inputs, performs arithmetic multiplication, and returns the product of them. The two inputs must be of the same type, but need not be of the same width. The result from multiplication is of the same type, and a bit width that of A + B (where A and B are the widths of the input types).

Supported Type Combinations -
- `Int<A>` * `Int<B>` -> `Int<{A + B}>`
- `UInt<A>` * `UInt<B>` -> `UInt<{A + B}>`

Usage -
```rust
let multiplicand: Int<16> = Int::<16>::from_i32(400);
let multiplier: Int<16> = Int::<16>::from_i32(-25);
let product = multiplicand * multiplier;
let out: Int<32> = Int::<32>::new(); // width of product is A + B (16 + 16 = 32)
out.assign(product);

let p: UInt<32> = UInt::<32>::from_u32(1234);
let q: UInt<8> = UInt::<8>::from_u8(12);
let r: UInt<40> = UInt::<40>::new();
r.assign(p * q); // ERROR: types and widths must match exactly for multiplication operands

let m: UInt<8> = UInt::<8>::from_u8(5);
let n: UInt<8> = UInt::<8>::from_u8(10);
let o: UInt<8> = UInt::<8>::new();
o.assign(m * n); // ERROR: width mismatch, product yields UInt<16>, cannot assign to UInt<8>

```
##### DIV (`/`) -
The **DIV** operation (`/`) is an operation which takes in two numerical inputs, performs arithmetic division, and returns their quotient. The two inputs must be of the same type and width, and the quotient will also share the same type and width.
Upon zero division, the operation will return all bits set to 1, and *Ralsei* will print a warning (if enabled). **\[NOTE: This logic is based on the RISC-V integer division specification\]**

Supported Type Combinations -
- `Int<W>` / `Int<W>` -> `Int<W>`
- `UInt<W>` / `UInt<W>` -> `UInt<W>`

Usage -
```rust
let dividend: UInt<32> = UInt::<32>::from_u32(5000);
let divisor: UInt<32> = UInt::<32>::from_u32(25);
let quotient = dividend / divisor;
let out: UInt<32> = UInt::<32>::new(); // width of quotient matches input width W (32)
out.assign(quotient);

let p: Int<32> = Int::<32>::from_i32(500);
let q: Int<32> = Int::<32>::from_i32(0); // division by zero case
let r: Int<32> = Int::<32>::new();
r.assign(p / q); // Note: returns saturated 0xFFFFFFFF (all bits high) following RISC-V standards

let m: Int<16> = Int::<16>::from_i32(100);
let n: UInt<16> = UInt::<16>::from_u32(5);
let o: Int<16> = Int::<16>::new();
o.assign(m / n); // ERROR: cannot perform division across mismatched numeric type domains (Int vs UInt)

```
##### MOD (`%`) -
The **MOD** operation (`%`) is an operation which takes in two numerical inputs, performs arithmetic division, and returns their remainder. The two inputs must be of the same type and width, and the remainder will also share the same type and width.
Upon zero division, the operation will return the divisor directly, and *Ralsei* will print a warning (if enabled). **\[NOTE: This logic is based on the RISC-V integer division specification\]**

Supported Type Combinations -
- `Int<W>` % `Int<W>` -> `Int<W>`
- `UInt<W>` % `UInt<W>` -> `UInt<W>`

Usage -
```rust
let numerator: Int<32> = Int::<32>::from_i32(-5);
let denominator: Int<32> = Int::<32>::from_i32(3);
let remainder = numerator % denominator;
let out: Int<32> = Int::<32>::new(); // width matches input width W (32)
out.assign(remainder); // signed remainder sign follows numerator, yields -2

let p: UInt<16> = UInt::<16>::from_u32(45);
let q: UInt<16> = UInt::<16>::from_u32(0); // modulo by zero case
let r: UInt<16> = UInt::<16>::new();
r.assign(p % q); // Note: returns original dividend value (45) following RISC-V standards

let m: UInt<32> = UInt::<32>::from_u32(10);
let n: UInt<16> = UInt::<16>::from_u32(3);
let o: UInt<32> = UInt::<32>::new();
o.assign(m % n); // ERROR: width mismatch, operands must be the exact same width W

```
#### Comparison -
##### Equals (`==`) -
The **Equals** operation (`==`) is an operation that takes in two inputs, compares them, and outputs a single Bit (or Logic) signal indicating if it's equal (HIGH, `1`) or not equal (LOW, `0`). The two vectors must be of the same width by default, otherwise *Ralsei* will raise an error.
It is analogous to the logic for XNOR.
Truth Table (per cell) -

| Input A | Input B | Output |
| ------- | ------- | ------ |
| 0       | 0       | 1      |
| 0       | 1       | 0      |
| 1       | 0       | 0      |
| 1       | 1       | 1      |
| X/Z     | 0       | X      |
| X/Z     | 1       | X      |
| 0       | X/Z     | X      |
| 1       | X/Z     | X      |
| X/Z     | X/Z     | X      |
Supported Type Combinations -
- `Logic` == `Logic` -> `Logic`
- `Logic` == `Bit` -> `Logic`
- `LogicVec<W>` == `LogicVec<W>` -> `Logic`
- `LogicVec<W>` == `BitVec<W>` -> `Logic`
- `Bit` == `Bit` -> `Bit`
- `BitVec<W>` == `BitVec<W>` -> `Bit`
- `Int<W>` == `Int<W>` -> `Bit`
- `Int<W>` == `UInt<W>` -> `Bit`
- `UInt<W>` == `UInt<W>` -> `Bit`
- `Int<W>` == `LogicVec<W>` -> `Logic`
- `UInt<W>` == `LogicVec<W>` -> `Logic`
- `Int<W>` == `BitVec<W>` -> `Bit`
- `UInt<W>` == `BitVec<W>` -> `Bit`

Usage -
```rust
let a: BitVec<32> = BitVec::<32>::from_usize(0xDEADBEEF);
let b: BitVec<32> = BitVec::<32>::from_usize(0xDEADBEEF);
let is_equal = a == b;
let out: Bit = Bit::new(); // comparisons always collapse to a single structural 2-state Bit
out.assign(is_equal);

let val_int: Int<16> = Int::<16>::from_i32(-4);
let val_uint: UInt<16> = UInt::<16>::from_u32(4);
let flag: Bit = Bit::new();
flag.assign(val_int == val_uint); // ERROR: cannot compare across different type structures (Int vs UInt)

let p: LogicVec<8> = LogicVec::<8>::new();
let q: LogicVec<12> = LogicVec::<12>::new();
let match_flag: Bit = Bit::new();
match_flag.assign(p == q); // ERROR: comparison requires matching widths, cannot compare width 8 to width 12

```
##### Not Equals (`!=`) -
The **Not Equals** operation (`!=`) is an operation that takes in two inputs, compares them, and outputs a single Bit (or Logic) signal indicating if it's not equal (HIGH, `1`) or equal (LOW, `0`). The two vectors must be of the same width by default, otherwise *Ralsei* will raise an error.
It is analogous to the logic for XOR.
Truth Table (per cell) -

| Input A | Input B | Output |
| ------- | ------- | ------ |
| 0       | 0       | 0      |
| 0       | 1       | 1      |
| 1       | 0       | 1      |
| 1       | 1       | 0      |
| X/Z     | 0       | X      |
| X/Z     | 1       | X      |
| 0       | X/Z     | X      |
| 1       | X/Z     | X      |
| X/Z     | X/Z     | X      |
Supported Type Combinations -
- `Logic` == `Logic` -> `Logic`
- `Logic` == `Bit` -> `Logic`
- `LogicVec<W>` == `LogicVec<W>` -> `Logic`
- `LogicVec<W>` == `BitVec<W>` -> `Logic`
- `Bit` == `Bit` -> `Bit`
- `BitVec<W>` == `BitVec<W>` -> `Bit`
- `Int<W>` == `Int<W>` -> `Bit`
- `Int<W>` == `UInt<W>` -> `Bit`
- `UInt<W>` == `UInt<W>` -> `Bit`
- `Int<W>` == `LogicVec<W>` -> `Logic`
- `UInt<W>` == `LogicVec<W>` -> `Logic`
- `Int<W>` == `BitVec<W>` -> `Bit`
- `UInt<W>` == `BitVec<W>` -> `Bit`

Usage -
```rust
let state_a: UInt<8> = UInt::<8>::from_u8(12);
let state_b: UInt<8> = UInt::<8>::from_u8(15);
let is_not_equal = state_a != state_b;
let out: Bit = Bit::new(); // inequality always collapses to a single structural 2-state Bit
out.assign(is_not_equal);

let m: BitVec<4> = BitVec::<4>::from_usize(5);
let n: LogicVec<4> = LogicVec::<4>::from_usize(5);
let flag: Bit = Bit::new();
flag.assign(m != n); // Note: Allowed due to safe multi-state boundary crossing rule, reduces to 2-state Bit

let x: Int<8> = Int::<8>::from_i32(-1);
let y: Int<16> = Int::<16>::from_i32(-1);
let err_flag: Bit = Bit::new();
err_flag.assign(x != y); // ERROR: width mismatch, inputs must be explicitly resized to match before comparing

```
##### Less Than (`<`) -
The **Less Than** operation (`<`) is an operation that takes in two inputs, compares them, and outputs a single Bit signal indicating if the first operand is strictly lesser than (HIGH, `1`) the second operand or not (LOW, `0`). The two vectors must be of the same width by default, otherwise *Ralsei* will raise an error.

Supported Type Combinations -
- `Int<W>` < `Int<W>` -> `Bit`
- `UInt<W>` < `UInt<W>` -> `Bit`

Usage -
```rust
let val_a: Int<16> = Int::<16>::from_i32(-45);
let val_b: Int<16> = Int::<16>::from_i32(12);
let is_less = val_a < val_b;
let out: Bit = Bit::new(); // collapse to a single structural Bit flag
out.assign(is_less); // yields true (1) because -45 < 12 is signed true

let p: UInt<32> = UInt::<32>::from_u32(100);
let q: UInt<16> = UInt::<16>::from_u16(200);
let flag: Bit = Bit::new();
flag.assign(p < q); // ERROR: width mismatch, magnitude operands must be the exact same width

let m: BitVec<8> = BitVec::<8>::from_usize(5);
let n: BitVec<8> = BitVec::<8>::from_usize(10);
let err_flag: Bit = Bit::new();
err_flag.assign(m < n); // ERROR: magnitude comparisons are strictly forbidden on raw BitVec vectors
```
##### Greater Than (`>`) -
The **Greater Than** operation (`>`) is an operation that takes in two inputs, compares them, and outputs a single Bit signal indicating if the first operand is strictly greater than (HIGH, `1`) the second operand or not (LOW, `0`). The two vectors must be of the same width by default, otherwise *Ralsei* will raise an error.

Supported Type Combinations -
- `Int<W>` > `Int<W>` -> `Bit`
- `UInt<W>` > `UInt<W>` -> `Bit`

Usage -
```rust
let val_a: UInt<8> = UInt::<8>::from_u8(55);
let val_b: UInt<8> = UInt::<8>::from_u8(20);
let is_greater = val_a > val_b;
let out: Bit = Bit::new();
out.assign(is_greater); // yields true (1)

let p: Int<32> = Int::<32>::from_i32(-10);
let q: Int<32> = Int::<32>::from_i32(-5);
let flag: Bit = Bit::new();
flag.assign(p > q); // yields false (0) because -10 is not greater than -5 in signed space

let m: Int<16> = Int::<16>::from_i32(5);
let n: UInt<16> = UInt::<16>::from_u16(2);
let err_flag: Bit = Bit::new();
err_flag.assign(m > n); // ERROR: cannot perform magnitude operations across mismatched types (Int vs UInt)
```
##### Less Than or Equal To (`<=`) -
The **Less Than or Equal To** operation (`<=`) is an operation that takes in two inputs, compares them, and outputs a single Bit signal indicating if the first operand is lesser than or equal to (HIGH, `1`) the second operand or not (LOW, `0`). The two vectors must be of the same width by default, otherwise *Ralsei* will raise an error.

Supported Type Combinations -
- `Int<W>` <= `Int<W>` -> `Bit`
- `UInt<W>` <= `UInt<W>` -> `Bit`

Usage -
```rust
let val_a: Int<32> = Int::<32>::from_i32(500);
let val_b: Int<32> = Int::<32>::from_i32(500);
let is_less_equal = val_a <= val_b;
let out: Bit = Bit::new();
out.assign(is_less_equal); // yields true (1)

let p: UInt<8> = UInt::<8>::from_u8(12);
let q: UInt<8> = UInt::<8>::from_u8(10);
let flag: Bit = Bit::new();
flag.assign(p <= q); // yields false (0)
```
##### Greater Than or Equal To (`>=`) -
The **Greater Than or Equal To** operation (`>=`) is an operation that takes in two inputs, compares them, and outputs a single Bit signal indicating if the first operand is lesser than or equal to (HIGH, `1`) the second operand or not (LOW, `0`). The two vectors must be of the same width by default, otherwise *Ralsei* will raise an error.

Supported Type Combinations -
- `Int<W>` >= `Int<W>` -> `Bit`
- `UInt<W>` >= `UInt<W>` -> `Bit`

Usage -
```rust
let val_a: UInt<12> = UInt::<12>::from_u16(400);
let val_b: UInt<12> = UInt::<12>::from_u16(400);
let is_greater_equal = val_a >= val_b;
let out: Bit = Bit::new();
out.assign(is_greater_equal); // yields true (1)

let p: Int<8> = Int::<8>::from_i32(-12);
let q: Int<8> = Int::<8>::from_i32(-15);
let flag: Bit = Bit::new();
flag.assign(p >= q); // yields true (1) because -12 is greater than or equal to -15
```
#### Vector Manipulation -
##### Bitwise Shift Left (`<<`) -
The **Bitwise Shift Left** operation (`<<`) shifts all the bits within the first operand towards the left by the amount dictated by the second operand (which is of type `UInt`). The two vectors need not be of the same width, and the result only depends on the width of the vector being shifted. The shifted space is always padded with zeros.

Supported Type Combinations -
- `LogicVec<W>` << `UInt<Q>` -> `LogicVec<W>`
- `BitVec<W>` << `UInt<Q>` -> `BitVec<W>`
- `Int<W>` << `UInt<Q>` -> `Int<W>`
- `UInt<W>` << `UInt<Q>` -> `UInt<W>`

Usage -
```rust
let target: BitVec<16> = BitVec::<16>::from_usize(0x00FF);
let amount: UInt<4> = UInt::<4>::from_u8(4);
let shifted = target << amount;
let out: BitVec<16> = BitVec::<16>::new(); // width depends strictly on the vector being shifted (16)
out.assign(shifted); // yields 0x0FFF (shifted left by 4, padded with zeros)

let num: UInt<32> = UInt::<32>::from_u32(5);
let shift_amt: UInt<8> = UInt::<8>::from_u8(2);
let result: UInt<32> = UInt::<32>::new(); // widths of target and amount do not need to match
result.assign(num << shift_amt); // yields 20 (5 multiplied by 2^2)

let bad_target: Int<8> = Int::<8>::from_i32(2);
let bad_amount: Int<4> = Int::<4>::from_i32(1);
let err_out: Int<8> = Int::<8>::new();
err_out.assign(bad_target << bad_amount); // ERROR: shift amount operand must strictly be of type UInt
```
##### Bitwise Shift Right (`>>`) -
The **Bitwise Shift Right** operation (`>>`) shifts all the bits within the first operand towards the right by the amount dictated by the second operand (which is of type `UInt`). The two vectors need not be of the same width, and the result only depends on the width of the vector being shifted.
**NOTE: Bitwise Shift Right always zero extends by default. However, if the vector to be shifted is a signed integer (`Int<W>`), it will be sign extended instead.**

Supported Type Combinations -
- `LogicVec<W>` >> `UInt<Q>` -> `LogicVec<W>`
- `BitVec<W>` >> `UInt<Q>` -> `BitVec<W>`
- `Int<W>` >> `UInt<Q>` -> `Int<W>`
- `UInt<W>` >> `UInt<Q>` -> `UInt<W>`

Usage -
```rust
// unsigned Vector Logical Right Shift (zero-extneded)
let raw_vector: BitVec<8> = BitVec::<8>::from_usize(0b11001100);
let amt: UInt<4> = UInt::<4>::from_u8(2);
let logical_shift = raw_vector >> amt;
let out_logical: BitVec<8> = BitVec::<8>::new();
out_logical.assign(logical_shift); // yields 0b00110011 (padded with zero bits from the left)

// Signed Integer Arithmetic Right Shift (sign-extended)
let signed_num: Int<8> = Int::<8>::from_i32(-64); // Binary: 0b11000000
let amt_signed: UInt<4> = UInt::<4>::from_u8(2);
let arithmetic_shift = signed_num >> amt_signed;
let out_arithmetic: Int<8> = Int::<8>::new();
out_arithmetic.assign(arithmetic_shift); // yields -16 (Binary: 0b11110000, MSB sign bit is copied down)
```
##### Indexing (`[]`) -
**Indexing** is an operation which is defined specifically for Vector types (`LogicVec<W>`, `BitVec<W>`, `Int<W>`, `UInt<W>`), allowing to index a specific cell from the vector and use accordingly. All indexing in *Ralsei* is zero-indexing.
There are three ways to index a vector in *Ralsei* -
1. Using the `.at::<INDEX>()` method defined for all vector types, to index at compile time.
2. Using the `.index(idx: UInt<W>)` method for runtime indexing (synthesized into a MUX). **NOTE: .index() ensures that specifically for `BitVec<W>` types and its derivatives the width of the vector 'W' equals 2 to the power of the width of the index literal 'Q' (i.e. W == 2\*\*Q). However, this is not checked for `LogicVec<W>`.** 
3. Using the `[]` operator to index a specific cell from the vector, either synthesizes to compile-time or runtime. (Determined by the `#[ralsei(module)]` attribute macro at compile-time, and evaluates with the aforementioned rules.)
The method/operation returns the type of cell that the vector is composed of. If the requested index is/can be out of bounds, then a compiler error is raised and synthesis fails. However, for **LogicVec** indexing, if it is at runtime, it returns `X` instead.

<!-- Developer's note: To get past the hard indexing limit of W from BitVec<W> mismatching with 2 ** Q of UInt<Q>, concatenation/truncating the vector to match the expected width can be used to get past it. -->

Supported Type Combinations -
- `LogicVec<W>`\[`UInt<Q>`\] -> `Logic`
- `BitVec<W>`\[`UInt<Q>`\] -> `Bit`
- `Int<W>`\[`UInt<Q>`\] -> `Bit`
- `UInt<W>`\[`UInt<Q>`\] -> `Bit`

Usage -
```rust
let vec: LogicVec<64> = LogicVec::<64>::new();
const IDX: usize = 17;
let cell: Logic = Logic::new();
cell.assign(vec.at::<IDX>()); // IDX provided, hardwiring bit 17 from vec to cell

let bus: BitVec<4> = BitVec::<4>::new();
let idx: UInt<2> = UInt::<2>::from_u8(2);
let cell: Bit = Bit::new();
cell.assign(bus.index(idx)); // W = 4; Q = 2; W = 2 ** Q; hence evaluates

let vec_2: Int<16> = Int::<16>::new();
let cell_2: Bit = Bit::new();
cell.assign(vec_2.at::<23>()); // IDX value 23 is greater than max indexable value of 15, hence causing out-of-bounds compiler error

let vec_3: UInt<18> = UInt::<18>::from_usize(67);
let index: UInt<4> = UInt::<4>::from_u8(9);
cell.assign(vec_3[index]); // runtime indexing done here as constant value is not passed directly into index; ERROR: as width of index 'Q' (4) when raised to power of 2 (2**Q = 16) does not match width of vector 'W' (18)
```
##### Slicing -
**Slicing** is an operation which is defined specifically for Vector types (`LogicVec<W>`, `BitVec<W>`, `Int<W>`, `UInt<W>`), allowing to slice a specific range of cells from the vector and use accordingly. Just like in indexing, slices in *Ralsei* is zero-indexed.
There are three ways to slice a vector in *Ralsei* -
1. Static part-select - Using the `.slice::<UPPER, LOWER>()` method on a vector type to slice it statically at compile-time. (**NOTE: UPPER and LOWER are both inclusive.**)
2. Indexed part-select (upwards) - Using the `.slice_up::<COUNT>::(idx: UInt<Q>)` method on a vector type at runtime, to slice up `COUNT` cells from `idx`. (**NOTE: Raises compile time error if (2\*\*Q + COUNT - 2 >= W**)
3. Indexed part-select (downwards) - Using the `.slice_down::<COUNT>::(idx: UInt<Q>)` method on a vector type at runtime, to slice down `COUNT` cells from `idx`. (**NOTE: Raises compile time error if COUNT <= 1**)
The methods return a vector of the same type but a different width depending on the number of bits sliced . If the method goes out of bounds at compile-time, an error is raised and synthesis fails. But if the method (after synthesis) goes out of bounds at runtime, the extra sliced cells are filled with a specific value based on the type of vector sliced -
- If it is a **LogicVec** (or its derivatives), the extra sliced cells are filled with the `X` value.
- If it is a **BitVec** (or its derivatives), the extra sliced cells are filled with the `0` value by default.

Supported Type Combinations - (**NOTE: The `[:]` and `[±:]` syntax is used here simply for ease of representation, and does not represent actual syntax in the language.**)
- `LogicVec<W>`\[`UPPER`:`LOWER`\] -> `LogicVec<{UPPER-LOWER+1}>`
- `LogicVec<W>`\[`UInt<Q>`±:`COUNT`\] -> `LogicVec<COUNT>`
- `BitVec<W>`\[`UPPER`:`LOWER`\] -> `BitVec<{UPPER-LOWER+1}>`
- `BitVec<W>`\[`UInt<Q>`±:`COUNT`\] -> `BitVec<COUNT>`
- `Int<W>`\[`UPPER`:`LOWER`\] -> `Int<{UPPER-LOWER+1}>`
- `Int<W>`\[`UInt<Q>`±:`COUNT`\] -> `Int<COUNT>`
-  `UInt<W>`\[`UPPER`:`LOWER`\] -> `UInt<{UPPER-LOWER+1}>`
- `UInt<W>`\[`UInt<Q>`±:`COUNT`\] -> `UInt<COUNT>`

Example Usage -
```rust
let instruction: BitVec<32> = BitVec::<32>::from_usize(0x02A58533);
let rd: BitVec<5> = BitVec::<5>::new();
rd.assign(instruction.slice::<11, 7>()); 

let bad_vec: UInt<16> = UInt::<16>::new();
let out_err: UInt<8> = UInt::<8>::new();
out_err.assign(bad_vec.slice::<19, 12>()); // ERROR: 19 out of bounds for W=16

let reg_file: BitVec<8> = BitVec::<8>::new();
let idx_up: UInt<3> = UInt::<3>::from_u8(2);
let window_up: BitVec<2> = BitVec::<2>::new();
window_up.assign(reg_file.slice_up::<2>(idx_up)); 

let wide_bus: LogicVec<10> = LogicVec::<10>::new();
let idx_overflow: UInt<4> = UInt::<4>::from_u8(9);
let window_logic: LogicVec<4> = LogicVec::<4>::new();
window_logic.assign(wide_bus.slice_up::<4>(idx_overflow)); // runtime overflow fills extra bits with X

let error_bus: BitVec<16> = BitVec::<16>::new();
let idx_bad: UInt<4> = UInt::<4>::from_u8(5);
let window_bad: BitVec<4> = BitVec::<4>::new();
window_bad.assign(error_bus.slice_up::<4>(idx_bad)); // ERROR: 2^4 + 4 - 2 = 18 >= W (16)

let data_bus: Int<32> = Int::<32>::new();
let idx_down: UInt<4> = UInt::<4>::from_u8(2);
let window_down: Int<3> = Int::<3>::new();
window_down.assign(data_bus.slice_down::<3>(idx_down)); 

let small_bus: UInt<16> = UInt::<16>::new();
let idx_ok: UInt<3> = UInt::<3>::from_u8(4);
let window_fail: UInt<1> = UInt::<1>::new();
window_fail.assign(small_bus.slice_down::<1>(idx_ok)); // ERROR: COUNT <= 1
```
##### Concatenation (`concat!()`) -
**Concatenation** is an operation which takes in multiple different wires/buses and groups them into a single contiguous vector. The order in which these elements are joined together depends on the order of which they are passed into the macro (passed MSB first). The elements passed must be of the same type or same class of types, and the returned vector is of the corresponding type and sum of all the widths as described below.

Supported Type Combinations - (**NOTE: In the given combinations, `Bit` and `Logic` are optional in most of them, and are just included to help deduce the type in the documentation. `Logic` or `LogicVec<W>` can be included in any parameter list, but doing so would implicitly cast it to `LogicVec<N>` regardless of the other types.**)
- `concat!(Logic, Logic, Logic, ...N)` -> `LogicVec<N>`
- `concat!(Logic, Bit, ...N)` -> `LogicVec<N>`
- `concat!(Bit, Bit, Bit, ...N` -> `BitVec<N>`
- `concat!(Logic, LogicVec<A>, LogicVec<B>, ...)` -> `LogicVec<{1 + A + B + ...}>`
- `concat!(Bit, BitVec<A>, BitVec<B>, ...)` -> `BitVec<{1 + A + B + ...}>`
- `concat!(Bit, Int<A>, Int<B>, ...)` -> `Int<{1 + A + B + ...}>`
- `concat!(Bit, UInt<A>, UInt<B>, ...)` -> `UInt<{1 + A + B + ...}>`
- ...other combinations possible but not mentioned...

Example Usage -
```rust
use ralsei::path::to::concat; // path to be added

let sign_bit: Bit = Bit::init(1);
let imm_10_1: BitVec<10> = BitVec::<10>::new();
let imm_11: Bit = Bit::init(0);
let imm_19_12: BitVec<8> = BitVec::<8>::new();
let immediate: BitVec<20> = BitVec::<20>::new();
immediate.assign(concat!(sign_bit, imm_10_1, imm_11, imm_19_12)); 

let internal_flag: Bit = Bit::init(1);
let register_bus: BitVec<8> = BitVec::<8>::new();
let external_pin: Logic = Logic::new(); // 4-state wire
let boundary_bus: LogicVec<10> = LogicVec::<10>::new();
boundary_bus.assign(concat!(external_pin, internal_flag, register_bus)); 

let val_signed: Int<8> = Int::<8>::new();
let val_unsigned: UInt<8> = UInt::<8>::new();
let out_err: UInt<16> = UInt::<16>::new();
out_err.assign(concat!(val_signed, val_unsigned)); 
// ERROR: Int and UInt belong to different class structures and hence cannot mix directly
```
### Functions and Methods -
### Macros -
*Ralsei* provides a large number of in-built macros which can be used for a variety of purposes to help with synthesis, operations, and reducing boilerplate. Following are the macros that are defined in *Ralsei*.
##### `generate!()` -
The `generate!()` macro is a powerful macro provided by *Ralsei* which smartly generates synthesizable code at compile time (such as via loops, conditions, and match cases), helping avoid repetitive boilerplate and optimizing code greatly.
The macro can be used in combination with existing Rust control structures such as `if-else` statements, `for` loops (with compile-time known conditions), and `match` statements, which will allow the macro to automatically generate synthesizable (or non-synthesizable) structures.
Control structures in the macro which are used to generate new synthesizable blocks can be given a specific name or a **label** via the  `#[ralsei(gen_lbl = 'label_name')]` attribute. This makes it so that during synthesis, the names given to these generated modules and blocks match that of the naming. Doing so is optional, as not giving a label will allow *Ralsei* to auto-generate names, based on the same format as that of *SystemVerilog*.
The following are the use-cases of the `generate!()` macro in *Ralsei* -
1) Generate-conditional (via `if-else` branches) - Rust's `if-else` conditional statements can be used to conditionally generate certain synthesizable blocks and structures. While using `if-else` blocks in a `generate!()` macro body, the condition must be fully evaluable at compile time.
   In addition, Rust's `if let` syntax may also be used to leverage Rust's pattern matching feature.
   Example use-case -
```rust
const PIPELINED: bool = false;

// inside module definition 
generate!(
	#[ralsei(gen_lbl = 'sequential')]
	if PIPELINED {
		#[on_edge(posedge(self.clk))]
		{
			data_out = data_in_a + data_in_b;
		}
	} 
	#[ralsei(gen_lbl = 'combinational')]
	else {
		data_out = data_in_a + data_in_b;
	}
);

enum InstType {
	Reg(usize),
	Imm(usize),
	Nop
}

// if let usecase 
const inst_type: InstType = Reg(5);
generate!(
	if let Reg(reg_no) = inst_type {
		// do something
	} else {
		// do something else
	}
);
```
2) Generate-replication (via `for` loops) - A Rust `for` loop can be used within the `generate!()` macro to repeatedly generate a synthesizable structure, module, or connection multiple times repeatedly. However, doing so requires the bounds of the loop to be known and evaluable at compile time. 
   Example use-case -
```rust
const DFF_COUNT: usize = 32;

// inside module definition
generate!(
	#[ralsei(gen_lbl = 'gen_shift_reg')]
	for i in 0..DFF_COUNT {
		if i==0 {
			let inst_0 = DFF {
				clk: self.clk.connect_in(),
				d:   self.data_in.connect_in(),
				q:   pipe[0].connect_out(),
			};
		} else {
			let inst_n = DFF {
				clk: self.clk.connect_in(),
				d:   pipe[i - 1].connect_in(),
				q:   pipe[i].connect_out(),
			};
		}
	}
);
```
3) Generate-case (via `match` statement) - Rust's `match` statements can be used to safely generate safe branches that correspond to distinct hardware layout configurations, based on discrete values which are evaluable at compile time. 
   Example use-case -
```rust
enum AluType {
	Bypass,
	Simple,
	Complex
}

const ALU_TYPE: AluType = Bypass;

// inside module definition
generate!(
	match ALU_TYPE {
		#[ralsei(gen_lbl = 'basic_alu')]
		Simple => {
			alu_out.assign(in_a + in_b);
		},
		#[ralsei(gen_lbl = 'advanced_alu')]
		Complex => {
			let dsp_inst = ComplexDspCore {
				a: in_a.connect_in(),
				b: in_b.connect_in(),
				y: self.alu_out.connect_out(),
			}
		},
		#[ralsei(gen_lbl = 'bypass_alu')]
		_ => {
			alu_out.assign = in_a;
		}
	}
);
```
##### `concat!()` - 
The `concat!()` macro is a operational macro provided by *Ralsei* which concatenates bits and buses of multiple different widths into a single bus of the combined width of all buses. For more information about the concatenate macro, please refer [[##### Concatenation (`concat!()`) -|here]].
### Attributes -
As *Ralsei* is tightly integrated with Rust while still having syntax, features, and requiring verification similar to that of *SystemVerilog*, it uses a combination of multiple different custom Rust attributes to verify the structure of the program and it's *Ralsei*-specific syntax and semantics.
All *Ralsei* attributes are in the format `#[ralsei(...)]` (where `...` is replaced with the actual name of the attribute).
Here are the list of the attributes that *Ralsei* introduces, and their function and usage -
- `#[ralsei(module)]`
- `#[ralsei(testbench)]`
- `#[ralsei(function)]`
- `#[ralsei(packed)]`
- `#[ralsei(inline)]`
- `#[ralsei(enum)]`
- `#[ralsei(repr(Type))]`
- `#[ralsei(allow_no_default_case)]`
- `#[ralsei(on_edge(...))]`
- `#[ralsei(rename = 'custom_name')]`
- `#[ralsei(protected)]`
- `#[ralsei(tb_init)]`
- `#[ralsei(gen_lbl = 'label_name')]`
