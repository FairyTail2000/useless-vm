use std::convert::TryFrom;

#[repr(usize)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Instruction {
	ADD = 0,
	SUB = 1,
	MOV = 2,
	MUL = 3,
	DIV = 4,
	JMP = 5,
	PUSH = 6,
	POP = 7,
	EXIT = 8,
	L0 = 9,
	L1 = 10,
	SHIFTRR = 11,
	SHIFTRL = 12,
	LEA = 13,
	OwnAddress = 14,
	NOOP = 15,
}

impl TryFrom<usize> for Instruction {
	type Error = ();

	fn try_from(v: usize) -> Result<Self, Self::Error> {
		match v {
			x if x == Instruction::ADD as usize => Ok(Instruction::ADD),
			x if x == Instruction::SUB as usize => Ok(Instruction::SUB),
			x if x == Instruction::MOV as usize => Ok(Instruction::MOV),
			x if x == Instruction::MUL as usize => Ok(Instruction::MUL),
			x if x == Instruction::DIV as usize => Ok(Instruction::DIV),
			x if x == Instruction::JMP as usize => Ok(Instruction::JMP),
			x if x == Instruction::PUSH as usize => Ok(Instruction::PUSH),
			x if x == Instruction::POP as usize => Ok(Instruction::POP),
			x if x == Instruction::EXIT as usize => Ok(Instruction::EXIT),
			x if x == Instruction::L0 as usize => Ok(Instruction::L0),
			x if x == Instruction::L1 as usize => Ok(Instruction::L1),
			x if x == Instruction::SHIFTRL as usize => Ok(Instruction::SHIFTRL),
			x if x == Instruction::SHIFTRR as usize => Ok(Instruction::SHIFTRR),
			x if x == Instruction::LEA as usize => Ok(Instruction::LEA),
			x if x == Instruction::OwnAddress as usize => Ok(Instruction::OwnAddress),
			x if x == Instruction::NOOP as usize => Ok(Instruction::NOOP),
			_ => Err(()),
		}
	}
}

impl Into<u8> for Instruction {
	fn into(self) -> u8 {
		match self {
			Instruction::ADD => 0,
			Instruction::SUB => 1,
			Instruction::MOV => 2,
			Instruction::MUL => 3,
			Instruction::DIV => 4,
			Instruction::JMP => 5,
			Instruction::PUSH => 6,
			Instruction::POP => 7,
			Instruction::EXIT => 8,
			Instruction::L0 => 9,
			Instruction::L1 => 10,
			Instruction::SHIFTRR => 11,
			Instruction::SHIFTRL => 12,
			Instruction::LEA => 13,
			Instruction::OwnAddress => 14,
			Instruction::NOOP => 15
		}
	}
}

struct CPU {
	register_0: i128,
	register_1: i128,
	register_2: i128,
	register_3: i128,
	register_4: i128,
	register_5: i128,
	register_6: i128,
	register_7: i128,
	register_8: i128,
	register_9: i128,
	stack: Vec<i128>,
	program: Vec<u8>,
	counter: usize
}

impl CPU {
	fn new() -> Self {
		return CPU {
			register_0: 0,
			register_1: 0,
			register_2: 0,
			register_3: 0,
			register_4: 0,
			register_5: 0,
			register_6: 0,
			register_7: 0,
			register_8: 0,
			register_9: 0,
			stack: Vec::with_capacity(u16::MAX as usize),
			program: Vec::with_capacity(u16::MAX as usize),
			counter: 0
		}
	}

	fn execute(&mut self) {
		while self.counter < self.program.len() {
			match Instruction::try_from(self.program[self.counter] as usize) {
				Ok(ins) => {
					match ins {
						Instruction::ADD => {
							match self.register_0.checked_add(self.register_1) {
								Some(val) => self.register_2 = val,
								None => {
									eprintln!("Overflowing subscription!");
									return;
								}
							}
						}
						Instruction::SUB => {
							match self.register_0.checked_sub(self.register_1) {
								Some(val) => self.register_2 = val,
								None => {
									eprintln!("Overflowing subscription!");
									return;
								}
							}
						}
						Instruction::MOV => {

						}
						Instruction::MUL => {
							match self.register_0.checked_mul(self.register_1) {
								Some(val) => self.register_2 = val,
								None => {
									eprintln!("Overflowing multiplication!");
									return;
								}
							}
						}
						Instruction::DIV => {
							match self.register_0.checked_div(self.register_1) {
								Some(val) => self.register_2 = val,
								None => {
									eprintln!("Div by zero!");
									return;
								}
							}
						}
						Instruction::JMP => {
							match usize::try_from(self.register_2) {
								Ok(val) => self.counter = val,
								Err(e) => {
									eprintln!("Couldn't set register 2's value into counter! {}", e);
								}
							}
							continue;
						}
						Instruction::PUSH => {
							self.stack.push(self.register_0);
							self.stack.push(self.register_1);
							self.stack.push(self.register_2);
							self.stack.push(self.register_3);
							self.stack.push(self.register_4);
							self.stack.push(self.register_5);
							self.stack.push(self.register_6);
							self.stack.push(self.register_7);
							self.stack.push(self.register_8);
							self.stack.push(self.register_9);
						}
						Instruction::POP => {
							self.register_9 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_9 from stack!");
									return;
								}
							};
							self.register_8 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_8 from stack!");
									return;
								}
							};
							self.register_7 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_7 from stack!");
									return;
								}
							};
							self.register_6 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_6 from stack!");
									return;
								}
							};
							self.register_5 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_5 from stack!");
									return;
								}
							};
							self.register_4 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_4 from stack!");
									return;
								}
							};
							self.register_3 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_3 from stack!");
									return;
								}
							};
							self.register_2 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_2 from stack!");
									return;
								}
							};
							self.register_1 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_1 from stack!");
									return;
								}
							};
							self.register_0 = match self.stack.pop() {
								Some(val) => val,
								None => {
									eprintln!("Couldn't pop register_0 from stack!");
									return;
								}
							};
						}
						Instruction::EXIT => {
							eprintln!("Exit");
							return;
						}
						Instruction::L0 => {
							self.register_0 = 0;
						}
						Instruction::L1 => {
							self.register_0 = 1;
						}
						Instruction::SHIFTRR => {
							let reg0 = self.register_0;
							let reg1 = self.register_1;
							let reg2 = self.register_2;
							let reg3 = self.register_3;
							let reg4 = self.register_4;
							let reg5 = self.register_5;
							let reg6 = self.register_6;
							let reg7 = self.register_7;
							let reg8 = self.register_8;
							let reg9 = self.register_9;
							self.register_0 = reg9;
							self.register_1 = reg0;
							self.register_2 = reg1;
							self.register_3 = reg2;
							self.register_4 = reg3;
							self.register_5 = reg4;
							self.register_6 = reg5;
							self.register_7 = reg6;
							self.register_8 = reg7;
							self.register_9 = reg8;
						}
						Instruction::SHIFTRL => {
							let reg0 = self.register_0;
							let reg1 = self.register_1;
							let reg2 = self.register_2;
							let reg3 = self.register_3;
							let reg4 = self.register_4;
							let reg5 = self.register_5;
							let reg6 = self.register_6;
							let reg7 = self.register_7;
							let reg8 = self.register_8;
							let reg9 = self.register_9;
							self.register_0 = reg1;
							self.register_1 = reg2;
							self.register_2 = reg3;
							self.register_3 = reg4;
							self.register_4 = reg5;
							self.register_5 = reg6;
							self.register_6 = reg7;
							self.register_7 = reg8;
							self.register_8 = reg9;
							self.register_9 = reg0;
						}
						Instruction::LEA => {
							self.register_0 = match usize::try_from(self.register_2) {
								Ok(idx) => {
									self.program[idx]
								}
								Err(e) => {
									eprintln!("{}", e);
									return;
								}
							} as i128;

						}
						Instruction::OwnAddress => {
							self.register_0 = self.counter as i128;
						}
						Instruction::NOOP => {}
					};
				},
				Err(_) => {
					eprintln!("Couldn't decode instruction! {}", self.program[self.counter]);
					return;
				}
			};

			self.counter += 1;
		}
	}
}


fn main() {
	let mut cpu = CPU::new();
	for _ in 0..100 {
		cpu.program.push(Instruction::NOOP.into());
	}

	cpu.program.push(Instruction::L1.into());
	cpu.program.push(Instruction::SHIFTRR.into());
	cpu.program.push(Instruction::L1.into());
	cpu.program.push(Instruction::ADD.into());
	cpu.program.push(Instruction::L0.into());
	cpu.program.push(Instruction::SHIFTRL.into());
	cpu.program.push(Instruction::ADD.into());
	cpu.program.push(Instruction::L0.into());
	cpu.program.push(Instruction::SHIFTRL.into());
	cpu.program.push(Instruction::L0.into());
	cpu.program.push(Instruction::ADD.into());
	cpu.program.push(Instruction::L0.into());
	cpu.program.push(Instruction::SHIFTRL.into());
	cpu.program.push(Instruction::L0.into());
	cpu.program.push(Instruction::SHIFTRL.into());
	cpu.program.push(Instruction::SHIFTRL.into());


	cpu.program.push(Instruction::OwnAddress.into());
	cpu.program.push(Instruction::SHIFTRR.into());
	cpu.program.push(Instruction::ADD.into());
	cpu.program.push(Instruction::LEA.into());
	cpu.program.push(Instruction::EXIT.into());
	cpu.execute();
}
