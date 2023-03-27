
/// Structure representing a monkey
///
/// * `id`: An id for each monkey
/// * `items`: A vector of items that each monkey is holding
/// * `operation`: The operation to be completed
/// * `test_value`: Value to test which monkey the item to be thrown to
/// * `t_monkey`: The monkey's id that should be thrown to if the test value is true
/// * `f_monkey`: The monkey's id that should be thrown to if the test value is false
pub struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Operation,
    test_value: i32,
    t_monkey: i32,
    f_monkey: i32
}

impl std::fmt::Debug for Monkey {

    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("Monkey id: {}\n", self.id);
        print!("Items: {:?}\n", self.items);
        print!("Operations: {:#?}\n", self.operation);
        print!("Divisible By Value: {}\n", self.test_value);
        print!("True monkey id: {}\n", self.t_monkey);
        print!("False monkey id: {}\n\n\n", self.f_monkey);
        Ok(())   
    }
}


impl Monkey {
    pub fn new(a_id: i32, a_items: Vec<i32>, a_operation: Operation,
        a_test_value: i32, a_t_monkey: i32, a_f_monkey: i32) -> Self {

        Self { 
            id: a_id, 
            items: a_items,
            operation: a_operation,
            test_value: a_test_value,
            t_monkey: a_t_monkey,
            f_monkey: a_f_monkey
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    NONE,
    ADD,
    MULTIPLY
}

impl Instruction {
    pub fn parse(s: &str) -> Self {
        match s {
            "*" => return Self::MULTIPLY,
            "+" => return Self::ADD,
            _ => panic!("invalid instruction '{s}'")
        }
    }
}

#[derive(Debug)]
pub struct Operation {
    pub instruction: Instruction,
    pub val: i32
}

impl Operation {
    pub fn new() -> Self {
        Self {
            instruction: Instruction::NONE,
            val: 0
        }
    }
}
