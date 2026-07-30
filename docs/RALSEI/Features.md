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
**NOTE: *Ralsei* also mandates that all match blocks that are a part of synthesizable code must use a catch-all (`_`) case to prevent unwanted latches from forming.** *(TBD: Specific attribute to prevent Ralsei from mandating a catch-all statement in a synthesizable match block, although Rust's compiler will still check if all enum variants are matched or not. The tentative name for this attribute is `#[ralsei([)allow_no_default_case)]`)*
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

```
##### DIV (`/`) -
The **DIV** operation (`/`) is an operation which takes in two numerical inputs, performs arithmetic division, and returns their quotient. The two inputs must be of the same type and width, and the quotient will also share the same type and width.
Upon zero division, the operation will return all bits set to 1, and *Ralsei* will print a warning (if enabled). **\[NOTE: This logic is based on the RISC-V integer division specification\]**

Supported Type Combinations -
- `Int<W>` / `Int<W>` -> `Int<W>`
- `UInt<W>` / `UInt<W>` -> `UInt<W>`

Usage -
```rust

```
##### MOD (`%`) -
The **MOD** operation (`%`) is an operation which takes in two numerical inputs, performs arithmetic division, and returns their remainder. The two inputs must be of the same type and width, and the remainder will also share the same type and width.
Upon zero division, the operation will return the divisor directly, and *Ralsei* will print a warning (if enabled). **\[NOTE: This logic is based on the RISC-V integer division specification\]**

Supported Type Combinations -
- `Int<W>` % `Int<W>` -> `Int<W>`
- `UInt<W>` % `UInt<W>` -> `UInt<W>`

Usage -
```rust

```
#### Comparison -
##### Equals (`==`) -
The **Equals** operation (`==`) is an operation that takes in two inputs, compares them, and outputs a single Bit signal indicating if it's equal (HIGH, `1`) or not equal (LOW, `0`).
##### Not Equals (`!=`) -
##### Less Than (`<`) -
##### Greater Than (`>`) -
##### Less Than or Equal To (`<=`) -
##### Greater Than or Equal To (`>=`) -
#### Vector Manipulation -
##### Bitwise Shift Left (`<<`) -
##### Bitwise Shift Right (`>>`) -
##### Slicing (`[]`) -
##### Concatenation (`concat!()`) -
### Functions and Methods -

### Macros -
