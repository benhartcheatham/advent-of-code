// use std::fmt::Display;
// use std::fs;
use std::io;

// use aocutils::timeln;
// use z3::ast::{Ast, BV};
//
// #[derive(Clone, Debug)]
// struct Computer {
//     ra: i64,
//     rb: i64,
//     rc: i64,
//     rip: usize,
//     halted: bool,
//
//     opcodes: [fn(&mut Computer, u8); 7],
//     program: Vec<u8>,
//     output: Vec<i64>,
// }
//
// impl Computer {
//     fn new((ra, rb, rc): (i64, i64, i64), program: &[u8]) -> Self {
//         Computer {
//             ra,
//             rb,
//             rc,
//             rip: 0,
//             halted: false,
//             opcodes: [
//                 Self::adv,
//                 Self::bxl,
//                 Self::bst,
//                 Self::bxc,
//                 Self::out,
//                 Self::bdv,
//                 Self::cdv,
//             ],
//             program: program.to_owned(),
//             output: Vec::new(),
//         }
//     }
//
//     fn to_combo(&self, operand: u8) -> i64 {
//         match operand {
//             x if x <= 3 => x as i64,
//             4 => self.ra,
//             5 => self.rb,
//             6 => self.rc,
//             _ => panic!("invalid combo operand {}!", operand),
//         }
//     }
//
//     fn div(&mut self, operand: u8) -> i64 {
//         let numerator = self.ra;
//         let pow = self.to_combo(operand);
//         let denominator = 2_i64.pow(if pow < 0 {
//             panic!("Division by zero! (adv power: {})", pow)
//         } else {
//             pow as u32
//         });
//
//         // needs truncation?
//         numerator / denominator
//     }
//
//     fn adv(&mut self, operand: u8) {
//         self.ra = self.div(operand);
//     }
//
//     fn bxl(&mut self, operand: u8) {
//         self.rb ^= operand as i64;
//     }
//
//     fn bst(&mut self, operand: u8) {
//         self.rb = self.to_combo(operand) % 8;
//     }
//
//     fn bxc(&mut self, _operand: u8) {
//         self.rb ^= self.rc;
//     }
//
//     fn out(&mut self, operand: u8) {
//         self.output.push(self.to_combo(operand) % 8);
//     }
//
//     fn bdv(&mut self, operand: u8) {
//         self.rb = self.div(operand);
//     }
//
//     fn cdv(&mut self, operand: u8) {
//         self.rc = self.div(operand);
//     }
//
//     fn run_program(&mut self) {
//         let plen = self.program.len();
//
//         while self.rip < plen && self.rip + 1 < plen && !self.halted {
//             let (op, operand) = (self.program[self.rip], self.program[self.rip + 1]);
//
//             match op {
//                 x if x < 3 => {
//                     self.opcodes[x as usize](self, operand);
//                     self.rip += 2
//                 }
//                 3 => {
//                     if self.ra != 0 {
//                         self.rip = operand as usize;
//                     } else {
//                         self.rip += 2;
//                     }
//                 }
//                 x if x < 8 => {
//                     self.opcodes[(x - 1) as usize](self, operand);
//                     self.rip += 2
//                 }
//                 _ => self.halted = true,
//             }
//         }
//     }
//
//     // Look to "Completed 2024 day 17" git commit for reddit comment that has explanation
//     #[allow(unused_assignments)]
//     fn translate(&self) -> i64 {
//         let ctx = z3::Context::new(&z3::Config::new());
//         let opt = z3::Optimize::new(&ctx);
//         let s = BV::new_const(&ctx, "s", 64);
//         let (mut reg_a, mut reg_b, mut reg_c) = (
//             s.clone(),
//             BV::from_i64(&ctx, 0, 64),
//             BV::from_i64(&ctx, 0, 64),
//         );
//
//         // This will only work for the following program:
//         // TODO: Make a general translation routine
//         assert_eq!(
//             self.program,
//             [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0]
//         );
//         for x in &self.program {
//             // bst regA
//             reg_b = reg_a.bvsmod(&BV::from_i64(&ctx, 8, 64));
//             // bxl 7
//             reg_b ^= &BV::from_i64(&ctx, 7, 64);
//             //  cdv regB
//             reg_c = reg_a.bvsdiv(&(BV::from_i64(&ctx, 1, 64) << &reg_b));
//             // bxl 7
//             reg_b ^= &BV::from_i64(&ctx, 7, 64);
//             // bxc
//             reg_b ^= reg_c;
//             // adv 3
//             reg_a = reg_a.bvsdiv(&BV::from_i64(&ctx, 8, 64));
//             opt.assert(
//                 &(reg_b.bvsmod(&BV::from_i64(&ctx, 8, 64)))._eq(&BV::from_i64(
//                     &ctx,
//                     (*x).into(),
//                     64,
//                 )),
//             );
//         }
//
//         opt.assert(&(reg_a._eq(&BV::from_i64(&ctx, 0, 64))));
//         opt.minimize(&s);
//         assert_eq!(opt.check(&[]), z3::SatResult::Sat);
//         let res = opt
//             .get_model()
//             .unwrap()
//             .eval(&s, true)
//             .unwrap()
//             .as_i64()
//             .unwrap();
//         res
//     }
// }
//
// impl Display for Computer {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             self.output
//                 .iter()
//                 .map(|i| i.to_string())
//                 .collect::<Vec<String>>()
//                 .join(",")
//         )
//     }
// }
//
// fn part1(input: &str) -> Computer {
//     let mut lines = input.lines();
//     let mut registers = [0, 0, 0];
//
//     for reg in &mut registers {
//         *reg = lines
//             .next()
//             .unwrap()
//             .split(':')
//             .nth(1)
//             .map(|s| s.trim().parse::<i64>().unwrap())
//             .unwrap();
//     }
//
//     lines.next();
//
//     let nums = lines.next().unwrap().split(':').nth(1).unwrap();
//     let program = nums
//         .trim()
//         .split(',')
//         .map(|d| d.parse::<u8>().unwrap())
//         .collect::<Vec<u8>>();
//
//     let mut computer = Computer::new(registers.into(), &program);
//     computer.run_program();
//     computer
// }
//
// fn part2(input: &str) -> i64 {
//     let mut lines = input.lines();
//     let mut registers = [0, 0, 0];
//
//     for reg in &mut registers {
//         *reg = lines
//             .next()
//             .unwrap()
//             .split(':')
//             .nth(1)
//             .map(|s| s.trim().parse::<i64>().unwrap())
//             .unwrap();
//     }
//
//     lines.next();
//
//     let nums = lines.next().unwrap().split(':').nth(1).unwrap();
//     let program = nums
//         .trim()
//         .split(',')
//         .map(|d| d.parse::<u8>().unwrap())
//         .collect::<Vec<u8>>();
//
//     let computer = Computer::new(registers.into(), &program);
//     computer.translate()
// }

pub fn run(_benchmark: bool) -> io::Result<()> {
    // let input = fs::read_to_string("inputs/2024/day17.txt")?;
    // timeln!("part1: {}", part1(&input));
    // timeln!("part2: {}", part2(&input));

    Ok(())
}
