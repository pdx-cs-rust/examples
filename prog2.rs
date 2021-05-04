struct CpuState {
    r: [i32;4],
    pc: usize,
}

type Op = Box<dyn Fn(CpuState) -> CpuState>;

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

fn add() -> Op {
    Box::new(|mut state: CpuState| -> CpuState {
        state.r[0] += state.r[1];
        state
    })
}

fn sub() -> Op {
    Box::new(|mut state: CpuState| -> CpuState {
        state.r[0] -= state.r[1];
        state
    })
}

fn mov(from: usize, to: usize) -> Op {
    Box::new(move |mut state: CpuState| -> CpuState {
        state.r[to] = state.r[from];
        state
    })
}

fn main() {
    let state = CpuState { r: [0, 1, 0, 0], pc: 0 };
    let prog = &[add(), mov(0, 1), add(), mov(0, 1), add(), mov(3, 1), sub()];
    println!("{}", state.run(prog));
}
