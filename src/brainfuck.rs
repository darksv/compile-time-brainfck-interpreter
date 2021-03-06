use crate::const_vec::ConstVec;

#[derive(Eq, PartialEq)]
pub(crate) enum Op {
    DecPtr,
    IncPtr,
    Dec,
    Inc,
    Print,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

pub(crate) const fn parse(program: &str) -> &'static [Op] {
    let mut idx = 0;
    let program = program.as_bytes();
    let mut ops = ConstVec::new();
    let mut loop_starts = ConstVec::new();
    loop_starts.push(0);
    while idx < program.len() {
        let byte = program[idx];
        idx += 1;

        let op = match byte {
            b'>' => Op::IncPtr,
            b'<' => Op::DecPtr,
            b'+' => Op::Inc,
            b'-' => Op::Dec,
            b'.' => Op::Print,
            b',' => Op::Input,
            b'[' => {
                loop_starts.push(ops.len());
                Op::LoopStart(0)
            }
            b']' => {
                let start = match loop_starts.pop() {
                    Some(x) => x,
                    None => panic!(),
                };
                let end = ops.len();
                ops[start] = Op::LoopStart(end + 1);
                Op::LoopEnd(start)
            }
            _ => continue,
        };
        ops.push(op);
    }
    ops.as_slice()
}

pub(crate) const fn run<const OPS: &'static [Op], const INPUT: &'static str>() -> &'static str {
    fn rt() -> &'static str {
        const { panic!("runtime execution not supported") }
    }

    unsafe { std::intrinsics::const_eval_select((), run_ct::<OPS, INPUT>, rt) }
}

const fn run_ct<const OPS: &'static [Op], const INPUT: &'static str>() -> &'static str {
    let mut output_buffer = ConstVec::<i8>::new();
    let input = INPUT.as_bytes();
    let mut input_ptr = 0;

    let mut memory = ConstVec::<i8>::new();
    memory.push(0);
    let mut ptr = 0;
    let mut ip = 0;

    while ip < OPS.len() {
        let op = &OPS[ip];
        ip += 1;
        match op {
            Op::IncPtr => {
                ptr += 1;
                if ptr == memory.len() {
                    memory.push(0);
                }
            }
            Op::DecPtr => ptr -= 1,
            Op::Inc => memory[ptr] = memory[ptr].wrapping_add(1),
            Op::Dec => memory[ptr] = memory[ptr].wrapping_sub(1),
            Op::Print => output_buffer.push(memory[ptr]),
            Op::Input => {
                memory[ptr] = if input_ptr < input.len() {
                    input[input_ptr] as i8
                } else {
                    -1
                };
                input_ptr += 1;
            }
            Op::LoopStart(end) => {
                if memory[ptr] == 0 {
                    ip = *end;
                }
            }
            Op::LoopEnd(start) => {
                if memory[ptr] != 0 {
                    ip = *start;
                }
            }
        }
    }

    let buffer = output_buffer.as_slice();
    unsafe {
        std::str::from_utf8_unchecked(
            &*std::ptr::slice_from_raw_parts(
                buffer.as_ptr().cast(),
                buffer.len(),
            )
        )
    }
}
