struct CpuState {
    r: [i32;2],
    pc: usize,
}

type Op = fn(CpuState) -> CpuState;

impl CpuState {
    fn run(mut self, prog: &[Op]) -> i32 {
        while self.pc < prog.len() {
            let pc = self.pc;
            self = prog[pc](self);
            self.pc += 1;
        }
        self.r[0]
    }
}

fn add(mut state: CpuState) -> CpuState {
    state.r[0] += state.r[1];
    state
}

fn sub(mut state: CpuState) -> CpuState {
    state.r[0] -= state.r[1];
    state
}

fn main() {
    let state = CpuState { r: [0, 1], pc: 0 };
    let prog = [add, add, sub, add];
    println!("{}", state.run(&prog));
}
