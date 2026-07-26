// unique node idedntifier 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeID(pub usize);

// logic, similar to verilog logic, equivalent to logic in SV
#[derive(Clone, Copy, Debug)]
struct Logic {
    id: NodeID,
}

// logic vector, similar to a vector in Verilog/SV
#[derive(Clone, Copy, Debug)]
struct LogicVec<const W: usize> {
    signal: [Logic; W],
}

struct Clock {
    clk: Logic,
}

loop {
    //... logic here
}
