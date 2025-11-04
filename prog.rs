/// State of CPU simulation.
#[derive(Debug)]
struct CpuState {
    /// Our CPU has two 32-bit data registers.
    r: [i32;2],
    /// Our CPU has a program counter.
    pc: usize,
}

/// Type of an "operation": that is, a machine instruction.
type Op<'a> = &'a dyn Fn(CpuState) -> CpuState;

impl CpuState {
    /// Run the program `prog` prog
    /// until the program counter exits
    /// the program memory.
    fn run(mut self, prog: &[Op]) -> i32 {
        while self.pc < prog.len() {
            self = prog[self.pc](self);
        }
        self.r[0]
    }
}

/// Show the state of the CPU.
fn log(msg: &str, state: &CpuState) {
    println!("{}: {:?}", msg, state);
}

/// Add `r1` to `r0`.
#[allow(unused)]
fn add(mut state: CpuState) -> CpuState {
    log("add_start", &state);
    state.r[0] += state.r[1];
    state.pc += 1;
    log("add_end", &state);
    state
}

/// Subtract `r1` from `r0`.
#[allow(unused)]
fn sub(mut state: CpuState) -> CpuState {
    log("sub_start", &state);
    state.r[0] -= state.r[1];
    state.pc += 1;
    log("sub_end", &state);
    state
}

/// Reset the CPU.
#[allow(unused)]
fn rst(_state: CpuState) -> CpuState {
    let state = CpuState {
        r: [0; 2],
        pc: 0,
    };
    log("reset", &state);
    state
}

/// Create a jump instruction with the particular
/// target address. This function is not a machine
/// instruction itself: it produces one when called.
fn make_jmp(mut target: String) -> impl Fn(CpuState) -> CpuState {
    |mut state: CpuState| {
        state.pc = target;
        log(&format!("jmp {}", target), &state);
        state
    }
}


/// Demo program.
fn main() {
    let state = CpuState { r: [0, 1], pc: 0 };
    let add: Op = &add;
    let sub: Op = &sub;
    let _rst: Op = &rst;
    let jmp3: Op = &make_jmp(3);
    let prog = [jmp3, add, add, sub, add];
    println!("{}", state.run(&prog));
}
