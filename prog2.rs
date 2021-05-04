struct State {
    r: [i32;4],
    pc: usize,
}

type Op = Box<dyn Fn(State) -> State>;

impl State {
    fn run(mut self, prog: &[Op]) -> i32 {
        while self.pc < prog.len() {
            let pc = self.pc;
            self = prog[pc](self);
            self.pc += 1;
        }
        self.r[0]
    }
}

fn add(mut state: State) -> State {
    state.r[0] += state.r[1];
    state
}

fn sub(mut state: State) -> State {
    state.r[0] -= state.r[1];
    state
}

fn mov(from: usize, to: usize) -> Op {
    Box::new(move |mut state: State| -> State {state.r[to] = state.r[from]; state})
}


fn main() {
    let state = State { r: [0, 1, 0, 0], pc: 0 };
    let add = || Box::new(add);
    let sub = || Box::new(sub);
    let prog = [add(), mov(0, 1), add(), mov(0, 1), add(), mov(3,1), sub()];
    println!("{}", state.run(&prog));
}
