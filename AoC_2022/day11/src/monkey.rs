
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
    operation: Operation,
    test_value: i32,
    t_monkey: i32,
    f_monkey: i32,
    pub items: Vec<i32>,
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

    /// Monkey inspects the first item that they are holding.
    ///
    /// Return: Returns the id of the monkey that it wants to throw its current
    /// item to. If returns -1, there is no item to inspect.
    pub fn inspect(&mut self) -> i32 {
        match self.items.get_mut(0) {
            Some(val) => {
                let mut current_val: i32 = val.clone();
                
                if self.operation.val == -1 {
                    // Complete operation
                    match &self.operation.instruction {
                        Instruction::ADD => current_val += current_val,
                        Instruction::MULTIPLY => current_val *= current_val,
                        Instruction::NONE => panic!("Missing operation for monkey!")
                    }
                } else {
                    // Complete operation
                    match &self.operation.instruction {
                        Instruction::ADD => current_val += self.operation.val,
                        Instruction::MULTIPLY => current_val *= self.operation.val,
                        Instruction::NONE => panic!("Missing operation for monkey!")
                    }
                }

                current_val /= 3;

                // Change the item value the monkey is currently holding to the
                // new current value.
                *val = current_val.clone();

                // Perform test
                let test_result: bool = current_val % self.test_value == 0;
                if test_result == true {
                    return self.t_monkey;
                } else {
                    return self.f_monkey;
                }
            },
            None => return -1
        }
    }

    // This might be better off not being a member function
    pub fn throw(&mut self, destination_monkey: i32, monkeys_vec: &mut Vec<Monkey>) {
        let item = self.items.remove(0);
        // monkeys_vec[destination_monkey].items.push(item);
        let m_dest = monkeys_vec.get_mut(destination_monkey as usize).unwrap();
        m_dest.items.push(item);
        
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
    pub val: i32 // If val is -1, then the operation is performed on the old value
}

impl Operation {
    pub fn new() -> Self {
        Self {
            instruction: Instruction::NONE,
            val: 0
        }
    }
}
