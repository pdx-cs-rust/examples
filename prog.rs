#[derive(Debug)]
struct CpuState {
    r: [i32;2],
    pc: usize,
}

type Op<'a> = &'a dyn Fn(CpuState) -> CpuState;

impl CpuState {
    fn run(mut self, prog: &[Op]) -> i32 {
        while self.pc < prog.len() {
            self = prog[self.pc](self);
        }
        self.r[0]
    }
}

fn log(msg: &str, state: &CpuState) {
    println!("{}: {:?}", msg, state);
}

#[allow(unused)]
fn add(mut state: CpuState) -> CpuState {
    log("add_start", &state);
    state.r[0] += state.r[1];
    state.pc += 1;
    log("add_end", &state);
    state
}

#[allow(unused)]
fn sub(mut state: CpuState) -> CpuState {
    log("sub_start", &state);
    state.r[0] -= state.r[1];
    state.pc += 1;
    log("sub_end", &state);
    state
}

#[allow(unused)]
fn rst(_state: CpuState) -> CpuState {
    let state = CpuState {
        r: [0; 2],
        pc: 0,
    };
    log("reset", &state);
    state
}

fn make_jmp(target: usize) -> Box<dyn Fn(CpuState) -> CpuState> {
    Box::new(move |mut state| {
        state.pc = target;
        log("jmp", &state);
        state
    })
}


fn main() {
    let state = CpuState { r: [0, 1], pc: 0 };
    let add: Op = &add;
    let sub: Op = &sub;
    let _rst: Op = &rst;
    let jmp3: Op = &make_jmp(3);
    let prog = [jmp3, add, add, sub, add];
    println!("{}", state.run(&prog));
}
