use std::collections::HashMap;

type Int = i32;
type Cache = HashMap<String, ([Int; 4], usize)>;

#[derive(Debug)]
struct Alu<'a> {
    pub vars: [Int; 4],
    pub input: &'a [Int],
    pub mp: usize,
    program: &'a [Instruction],
    pc: usize,
    cache: &'a mut Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Param {
    Var(char),
    Num(Int),
}

impl From<&str> for Param {
    fn from(s: &str) -> Self {
        if let Ok(num) = s.parse() {
            Param::Num(num)
        } else {
            Param::Var(s.chars().next().unwrap())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inp(char),
    Add(char, Param),
    Mul(char, Param),
    Div(char, Param),
    Mod(char, Param),
    Eql(char, Param),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            ["inp", a] => Instruction::Inp(a.chars().next().unwrap()),
            ["add", a, b] => Instruction::Add(a.chars().next().unwrap(), Param::from(*b)),
            ["mul", a, b] => Instruction::Mul(a.chars().next().unwrap(), Param::from(*b)),
            ["div", a, b] => Instruction::Div(a.chars().next().unwrap(), Param::from(*b)),
            ["mod", a, b] => Instruction::Mod(a.chars().next().unwrap(), Param::from(*b)),
            ["eql", a, b] => Instruction::Eql(a.chars().next().unwrap(), Param::from(*b)),
            _ => panic!("Invalid instruction: {}", s),
        }
    }
}

impl<'a> Alu<'a> {
    fn new(input: &'a [Int], program: &'a [Instruction], cache: &'a mut Cache) -> Self {
        let mut alu = Alu {
            vars: [0, 0, 0, 0],
            input,
            mp: 0,
            program,
            pc: 0,
            cache,
        };
        alu.reset_vars();
        alu
    }

    fn reset_vars(&mut self) {
        // self.vars = [0, 0, 0, 0];
        self.mp = 0;
        self.pc = 0;
    }

    fn set_mem(&mut self, mem: &'a [Int]) {
        self.input = mem;
        self.reset_vars();
    }

    fn read(&mut self) -> Int {
        if self.mp >= self.input.len() {
            panic!(
                "Out of bounds, mp = {}, but len is {}",
                self.mp,
                self.input.len()
            );
        }
        let value = self.input[self.mp];
        self.mp += 1;
        value
    }

    fn resolve(&self, param: Param) -> Int {
        match param {
            Param::Var(v) => self.var(v),
            Param::Num(n) => n,
        }
    }

    fn var(&self, var: char) -> Int {
        match var {
            'w' => self.vars[0],
            'x' => self.vars[1],
            'y' => self.vars[2],
            'z' => self.vars[3],
            _ => panic!("Invalid variable: {}", var),
        }
    }

    fn set_var(&mut self, var: char, value: Int) {
        match var {
            'w' => self.vars[0] = value,
            'x' => self.vars[1] = value,
            'y' => self.vars[2] = value,
            'z' => self.vars[3] = value,
            _ => panic!("Invalid variable: {}", var),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        // let vars_before = self.vars;
        match instruction {
            Instruction::Inp(a) => {
                // put current state into cache so we can start from here next time
                // the key is the input up until the memory counter (mc)
                // only cache if we're not at the last input
                if self.mp < self.input.len() {
                    let key = self
                        .input
                        .iter()
                        .take(self.mp)
                        .map(|x| x.to_string())
                        .collect::<String>();
                    if !self.cache.contains_key(&key) {
                        // println!("cache: {} -> {:?}", key, self.vars);
                        self.cache.insert(key, (self.vars, self.pc));
                    }
                }

                let result = self.read();
                self.set_var(a, result);
            }
            Instruction::Add(a, b) => {
                let result = self.var(a) + self.resolve(b);
                self.set_var(a, result);
            }
            Instruction::Mul(a, b) => {
                let result = self.var(a) * self.resolve(b);
                self.set_var(a, result);
            }
            Instruction::Div(a, b) => {
                let result = self.var(a) / self.resolve(b);
                self.set_var(a, result);
            }
            Instruction::Mod(a, b) => {
                let result = self.var(a) % self.resolve(b);
                self.set_var(a, result);
            }
            Instruction::Eql(a, b) => {
                let result = if self.var(a) == self.resolve(b) { 1 } else { 0 };
                self.set_var(a, result);
            }
        };
        // let vars_after= self.vars;
        // println!("{:?}: {:?} -> {:?}", instruction, vars_before, vars_after);
    }

    pub fn run(&mut self) {
        self.reset_vars();
        // The cache mechanism should work in such a way, that when we start the program with a
        // certain memory input, that intermediate results at any read operation are cached.
        // We should be able to iterate over the input, and find the longest chain of inputs in the
        // cache. The result should be a tuple that contains the program counter and the state of
        // the registers up to that point.

        let mut key = String::new();
        for i in 0..(self.input.len() - 1) {
            // parse the i32 at that position into a single digit, considering they should only be
            // 1-9
            let char = self.input[i].to_string().chars().nth(0).unwrap();
            key.push(char);
            if !self.cache.contains_key(&key) {
                // println!("cache miss: {}", key);
                break;
            }
            let (vars, pc) = self.cache[&key];
            // println!("Found cache: {} -> {:?}", key, vars);
            self.vars = vars;
            self.pc = pc;
            self.mp = i + 1;
        }
        //
        // println!("Starting from pc = {}", self.pc);
        while self.pc < self.program.len() {
            print!("{}. {:?} \t->\t {:?} \t->\t", self.pc, self.vars, self.program[self.pc]);
            self.execute(self.program[self.pc]);
            println!("{:?}", self.vars);
            self.pc += 1;
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| Instruction::from(line)).collect()
}

fn create_input(val: i64) -> Vec<Int> {
    let mut input = Vec::new();
    let mut val = val;
    while val > 0 {
        input.push((val % 10) as Int);
        val /= 10;
    }
    input.reverse();
    input
}

/// Considering that our puzzle input program may have many instructions that really do not
/// influence the output of the program, we could benefit from simplifying the instructions, by
/// removing any instructions that do not influence the output of the program.
fn simplify(program: &[Instruction], inputs: &[i64]) -> Vec<Instruction> {
    let input = create_input(inputs[0]);
    let mut cache = Cache::new();
    let mut alu = Alu::new(&input, program, &mut cache);
    let mut simplified = Vec::new();
    for instruction in program {
        let vars_before = alu.vars;
        alu.execute(*instruction);
        let vars_after = alu.vars;
        if vars_before != vars_after {
            simplified.push(*instruction);
        }
    }
    simplified
}

/// Approach 2: Speeding up
/// -----------------------
/// Thinking about the assignment, we need to find the biggest number. This means that we should
/// perhaps start with the biggest number with 14 characters, and move down from that.
///
/// If we consider that the change in output between 99_999_999_999_999 and 99_999_999_999_998 is
/// only affected by the instructions following the last input instruction, we can cache the
/// contents of the registers for the last input instruction, and only run the instructions that
/// follow that instruction.
pub fn main_2() {
    let step = 10;
    let program = parse(include_str!("../../input/day24.txt"));
    let mut serial: i64 = 99_999_999_999_999;
    let mut cache = Cache::new();

    while serial >= 11_111_111_111_111 {
        let input = create_input(serial);
        // if the input contains a zero, skip
        if input.iter().any(|&x| x == 0) {
            serial -= step;
            continue;
        }
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        if alu.vars[3] == 0 {
            println!("Found serial: {}", serial);
            break;
        }
        println!("{}: {}", serial, alu.vars[3]);
        serial -= step;
    }
}

/// Approach 3: Reverse engineering the 'z' register
/// ------------------------------------------------
/// We need to look for inputs that set the z register to zero. 
/// Perhaps there's a way to find clues in the code to figure out what variance leads to having
/// zero in the z register.
///
/// Taking a look at the last lines of the puzzle input, we can deduce some facts:
///
/// add z y - This means that the value of y is added to z, so (y + z) == 0
/// mul y x - This means y is multiplied by x
///
/// --- oh wait ---
///
/// Looking at the lines of the program, we see some patterns that keep coming back, after each
/// inp occurrence. Let's line them up:
///
/// inp w    inp w    inp w    inp w    inp w     inp w     inp w
/// mul x 0  mul x 0  mul x 0  mul x 0  mul x 0   mul x 0   mul x 0
/// add x z  add x z  add x z  add x z  add x z   add x z   add x z
/// mod x 26 mod x 26 mod x 26 mod x 26 mod x 26  mod x 26  mod x 26
/// div z 1  div z 1  div z 1  div z 1  div z 26  div z 26  div z 26
/// add x 13 add x 15 add x 15 add x 11 add x -16 add x -11 add x -6
/// eql x w  eql x w  eql x w  eql x w  eql x w   eql x w   eql x w
/// eql x 0  eql x 0  eql x 0  eql x 0  eql x 0   eql x 0   eql x 0
/// mul y 0  mul y 0  mul y 0  mul y 0  mul y 0   mul y 0   mul y 0
/// add y 25 add y 25 add y 25 add y 25 add y 25  add y 25  add y 25
/// mul y x  mul y x  mul y x  mul y x  mul y x   mul y x   mul y x
/// add y 1  add y 1  add y 1  add y 1  add y 1   add y 1   add y 1
/// mul z y  mul z y  mul z y  mul z y  mul z y   mul z y   mul z y
/// mul y 0  mul y 0  mul y 0  mul y 0  mul y 0   mul y 0   mul y 0
/// add y w  add y w  add y w  add y w  add y w   add y w   add y w
/// add y 5  add y 14 add y 15 add y 16 add y 8   add y 9   add y 2
/// mul y x  mul y x  mul y x  mul y x  mul y x   mul y x   mul y x
/// add z y  add z y  add z y  add z y  add z y   add z y   add z y
///
/// There are only 3 values different in every part of the program, also the mod x 26 is weird.
/// Anyway, the invariants to the program are the input digit, the state of the registers, and
/// those 3 values that can be different. 
///
/// If we can figure out which state we need from the input and registers, in order to have z be
/// zero, we can reverse engineer the program.
///
/// - w is only written to by inp
/// - z is a carry
/// - x and y are set to zero each iteration.
/// so we basically have a program with 3 parameters (those varying values) which has inputs 
/// (digit, carry) and has a certain output z.
///
/// if we can figure out how manipulating the digit and the parameters affects z, we have a chance
/// at simplifying the program.
fn subprog(a: Int, b: Int, c: Int) -> Vec<Instruction> {
    vec![
        Instruction::Inp('w'),
        Instruction::Mul('x', Param::Num(0)),
        Instruction::Add('x', Param::Var('z')),
        Instruction::Mod('x', Param::Num(26)),
        Instruction::Div('z', Param::Num(a)),
        Instruction::Add('x', Param::Num(b)),
        Instruction::Eql('x', Param::Var('w')),
        Instruction::Eql('x', Param::Num(0)),
        Instruction::Mul('y', Param::Num(0)),
        Instruction::Add('y', Param::Num(25)),
        Instruction::Mul('y', Param::Var('x')),
        Instruction::Add('y', Param::Num(1)),
        Instruction::Mul('z', Param::Var('y')),
        Instruction::Mul('y', Param::Num(0)),
        Instruction::Add('y', Param::Var('w')),
        Instruction::Add('y', Param::Num(c)),
        Instruction::Mul('y', Param::Var('x')),
        Instruction::Add('z', Param::Var('y')),
    ]
}

fn sub_solutions(search_z: Int, a: Int, b: Int, c: Int) -> Vec<(i64, Int)> {
    let mut solutions = Vec::new();
    for w in 1..=9 {
        // inp w
 
        // mul x 0
        // add x z
        // mod x 26
        // ----------- x = orig_z % 26
        // div z (a)
        // ----------- z = orig_z / (a) ------> 1 or 26
        //
        // add x (b)
        // eql x w
        // eql x 0
        // ----------- if x + (b) == w, then x = 0
        // ----------- if (orig_z % 26) + (b) == w, then x = 0
        // ----------- if x + (b) != w, then x = 1
        // ----------- if (orig_z % 26) + (b) != w, then x = 1
        //
        // mul y 0
        // add y 25
        // mul y x
        // add y 1
        // mul z y
        // y becomes (x * 25) + 1
        // what's in z gets multiplied by y
        // ------------ z = z * ((x * 25) + 1)
        //
        // mul y 0
        // add y w
        // add y (c)
        // mul y x
        // y becomes (w + c) * x
        // ------------ z = z + ((w + c) * x)
        //
        // add z y
        //
        // ===================================================================
        // To reverse engineer, we're searching for orig_z and w that lead to:
        // search_z = z + ((w + c) * x)
        // // we don't know x yet, but it's either 0 or 1 so:
        let z_if_x_0 = search_z;
        let mut z_if_x_1 = search_z - (w + c);
        // ------------ z = z * ((x * 25) + 1) -- leads to
        // let z_if_x_0 = z_if_x_0;
        z_if_x_1 = z_if_x_1 / 26;
        println!("z_if_x_0: {}, z_if_x_1: {}", z_if_x_0, z_if_x_1);
        //
        // ----------- z = orig_z / (a) ------> 1 or 26
        let orig_z_if_x_0 = z_if_x_0 * (a);
        let orig_z_if_x_1 = z_if_x_1 * (a);
        println!("orig_z_if_x_0: {}, orig_z_if_x_1: {}", orig_z_if_x_0, orig_z_if_x_1);
        //
        // ----------- if (orig_z % 26) + (b) == w, then x = 0 -- leads to
        let orig_z_mod_26_if_x_0 = w - b;
        solutions.push((w as i64, w - b));
        // ----------- if (orig_z % 26) + (b) != w, then x = 1
        if orig_z_if_x_1 % 26 + b != w {
            // solutions.push((w as i64, orig_z_if_x_1));
        }

        // -------------------------------------------------------------
        // attempt 2
        // -------------------------------------------------------------
        // Z mod 26 = w - b makes x 0

        // If a == 26, we can control z after setting x separately by multiplying by 26
        // ... So z is actually 26d + e if a == 26
        // And e controls the digit and d the z output. 
        // e = (w - b)
        // d = search_z * 26

        // if a == 1, z after setting x, is z
        // 

        // If x = 0, z stays z
        // If x = 1, z is z * 26 + c*w


        // If a == 1, out_z = z*26 - c*w

        // reserve
    }
    println!("search_z: {} ABC: {},{},{} solutions: {:?}", search_z, a, b, c, solutions);
    solutions
}

pub fn main() {
    let progs = vec![
        (1, 13, 5),
        (1, 15, 14),
        (1, 15, 15),
        (1, 11, 16),
        (26, -16, 8),
        (26, -11, 9),
        (26, -6, 2),
        (1, 11, 13),
        (1, 10, 16),
        (26, -10, 6),
        (26, -8, 6),
        (26, -11, 9),
        (1, 12, 11),
        (26,-15,5)
    ];

    let mut step = progs.len() - 1;
    let search_z = 0;

    let prog = progs[step - 1];
    let solutions = sub_solutions(24 + (10 * 26), prog.0, prog.1, prog.2);

    let mut i = 1;
    for (digit, z) in solutions {
        let input = create_input(digit);
        let mut cache = Cache::new();
        let program = subprog(prog.0, prog.1, prog.2);

        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.vars[3] = z;
        println!("{}. in:{} / z:{} ---> ", i, digit, alu.vars[3]);
        alu.run();
        println!("<----- z: {}", alu.vars[3]);
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test inp instruction
    /// inp a - Read an input value and write it to variable a.
    #[test]
    fn test_inp() {
        let program = vec![Instruction::Inp('x')];
        let input = vec![5];
        let mut cache = Cache::new();
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[1], 5);
    }

    /// Test add instruction
    #[test]
    fn test_add() {
        let program = vec![Instruction::Inp('x'), Instruction::Add('x', Param::Num(3))];
        let input = vec![5];
        let mut cache = Cache::new();

        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[1], 8);
    }

    /// Test mul instruction
    #[test]
    fn test_mul() {
        let program = vec![Instruction::Inp('x'), Instruction::Mul('x', Param::Num(3))];
        let input = vec![5];
        let mut cache = Cache::new();
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[1], 15);
    }

    /// Test div instruction
    #[test]
    fn test_div() {
        let program = vec![Instruction::Inp('x'), Instruction::Div('x', Param::Num(2))];
        let input = vec![7];
        let mut cache = Cache::new();
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[1], 3);
    }

    /// Test mod instruction
    #[test]
    fn test_mod() {
        let program = vec![Instruction::Inp('x'), Instruction::Mod('x', Param::Num(2))];
        let input = vec![5];
        let mut cache = Cache::new();
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[1], 1);
    }

    /// Test eql instruction
    #[test]
    fn test_eql() {
        let program = vec![Instruction::Inp('x'), Instruction::Eql('x', Param::Num(7))];
        let input = vec![7];
        let mut cache = Cache::new();
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[1], 1);

        // test inequality
        let input = vec![5];
        alu.set_mem(&input);
        alu.run();
        assert_eq!(alu.vars[1], 0);
    }

    /// test parsing of instructions
    #[test]
    fn test_parse() {
        let input = "inp a\nadd a 3\nmul a 2";
        let instructions = parse(input);
        assert_eq!(
            instructions,
            vec!(
                Instruction::Inp('a'),
                Instruction::Add('a', Param::Num(3)),
                Instruction::Mul('a', Param::Num(2)),
            )
        );
    }

    /// test example 1
    #[test]
    fn test_example_1() {
        let program = parse("inp x\nmul x -1");
        let input = vec![5];
        let mut cache = Cache::new();
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[1], -5);
    }

    /// test example 2
    #[test]
    fn test_example_2() {
        let input = r#"inp z
inp x
mul z 3
eql z x"#;
        let program = parse(input);
        let input = vec![2, 6];
        let mut cache = Cache::new();
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[3], 1);

        let input = vec![3, 6];
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[3], 0);
    }

    /// test example 3
    #[test]
    fn test_example_3() {
        let input = r#"inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2"#;
        let program = parse(input);
        let input = vec![11];
        let mut cache = Cache::new();
        let mut alu = Alu::new(&input, &program, &mut cache);
        alu.run();
        assert_eq!(alu.vars[0], 1);
        assert_eq!(alu.vars[1], 0);
        assert_eq!(alu.vars[2], 1);
        assert_eq!(alu.vars[3], 1);
    }
}
