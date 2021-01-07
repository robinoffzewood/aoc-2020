use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let str_in = fs::read_to_string("input.txt").expect("Error in reading file");
    let mut result: usize = 0;
    for line in str_in.lines() {
        let expr = Expression::from_string(line, false);
        result += expr.evaluate();
    }
    println!(
        "Part 1: The sum of the resulting values of each line = {}",
        result
    );

    result = 0; // reset
    for line in str_in.lines() {
        let expr = Expression::from_string(line, true);
        result += expr.evaluate();
    }
    println!(
        "Part 2: The sum of the resulting values of each line = {}",
        result
    );

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_11() {
        let expr = Expression::from_string("1 + 2 * 3 + 4 * 5 + 6", false);
        assert_eq!(71, expr.evaluate());
    }
    #[test]
    fn tst_12() {
        let expr = Expression::from_string("1 + (2 * 3) + (4 * (5 + 6))", false);
        assert_eq!(51, expr.evaluate());
    }
    #[test]
    fn tst_13() {
        let expr = Expression::from_string("2 * 3 + (4 * 5)", false);
        assert_eq!(26, expr.evaluate());
    }
    #[test]
    fn tst_14() {
        let expr = Expression::from_string("5 + (8 * 3 + 9 + 3 * 4 * 3)", false);
        assert_eq!(437, expr.evaluate());
    }
    #[test]
    fn tst_15() {
        let expr = Expression::from_string("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false);
        assert_eq!(12240, expr.evaluate());
    }
    #[test]
    fn tst_16() {
        let expr =
            Expression::from_string("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false);
        assert_eq!(13632, expr.evaluate());
    }

    #[test]
    fn tst_21() {
        let expr = Expression::from_string("1 + 2 * 3 + 4 * 5 + 6", true);
        assert_eq!(231, expr.evaluate());
    }
    #[test]
    fn tst_22() {
        let expr = Expression::from_string("1 + (2 * 3) + (4 * (5 + 6))", true);
        assert_eq!(51, expr.evaluate());
    }
    #[test]
    fn tst_23() {
        let expr = Expression::from_string("2 * 3 + (4 * 5)", true);
        assert_eq!(46, expr.evaluate());
    }
    #[test]
    fn tst_24() {
        let expr = Expression::from_string("5 + (8 * 3 + 9 + 3 * 4 * 3)", true);
        assert_eq!(1445, expr.evaluate());
    }
    #[test]
    fn tst_25() {
        let expr = Expression::from_string("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true);
        assert_eq!(669060, expr.evaluate());
    }
    #[test]
    fn tst_26() {
        let expr = Expression::from_string("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true);
        assert_eq!(23340, expr.evaluate());
    }
}

enum Operand {
    Val(usize),            // In case we have directly a digit
    Expr(Box<Expression>), // In case we must evaluate a sub expression to get the value, it's containing the expression id.
}

#[derive(PartialEq)]
enum Operator {
    Add,
    Mul,
}

struct Expression {
    left: Operand,
    operator: Operator,
    right: Operand,
}

impl Expression {
    // return the root node of the tree, containing all the expressions contained in @str_in
    fn from_string(str_in: &str, add_precedes: bool) -> Self {
        let mut char_idx: usize = 0;
        // Start with beginning of the input string, creating the first candidate root node.
        let left = Expression::parse_operand(str_in, &mut char_idx, add_precedes);
        let mut root = Expression::parse_operation(str_in, &mut char_idx, left, add_precedes);
        // While the remaining string is not empty, continue in creating a new "root" node, and assigning its left value to the previous root node id.
        while str_in.chars().nth(char_idx).is_some() {
            let new_root = Expression::parse_operation(
                &str_in,
                &mut char_idx,
                Operand::Expr(Box::new(root)),
                add_precedes,
            );
            root = new_root;
        }
        root
    }

    fn parse_operation(
        str_in: &str,
        char_idx: &mut usize,
        left: Operand,
        add_precedes: bool,
    ) -> Expression {
        let operator = Expression::parse_operator(str_in, char_idx);
        let right = Expression::parse_operand(str_in, char_idx, add_precedes);
        // What is coming next depends on the next symbol.
        // If the '+' has a higher precedence on '*' (bool and_precedes = true) AND we just had a '*' and a '+' is coming next
        //   -> build a new child operation (so it get evaluated first), taking the current "right" operand as its "left" operand.
        //
        // Else, if we have a ')' it means we must return the current operation, because it won't have any child.
        // Otherwise, we must build a new parent expression, and consider its left operand as the current operation.
        let operation;
        if add_precedes && operator == Operator::Mul && Expression::next_is_a('+', str_in, char_idx)
        {
            let child_op = Expression::parse_operation(str_in, char_idx, right, add_precedes);
            return Expression {
                left,
                operator,
                right: Operand::Expr(Box::new(child_op)),
            };
        } else {
            operation = Expression {
                left,
                operator,
                right,
            };
            if Expression::next_is_a(')', str_in, char_idx) {
                *char_idx += 1;
                //println!(" -> End of sub expression");
                return operation;
            } else if str_in.chars().nth(*char_idx) == None {
                // end of line
                return operation;
            }
            //println!(" -> New parent expression");
            return Expression::parse_operation(
                str_in,
                char_idx,
                Operand::Expr(Box::new(operation)),
                add_precedes,
            );
        }
    }

    fn parse_operand(str_in: &str, char_idx: &mut usize, add_precedes: bool) -> Operand {
        // Consume any whitespace
        while str_in.chars().nth(*char_idx) == Some(' ') {
            *char_idx += 1;
        }
        if Expression::next_is_a_digit(str_in, char_idx) {
            let val = str_in.chars().nth(*char_idx).unwrap().to_digit(10).unwrap();
            *char_idx += 1;
            return Operand::Val(val as usize);
        } else if Expression::next_is_a('(', str_in, char_idx) {
            *char_idx += 1;
            //println!(" -> New sub expression");
            let left = Expression::parse_operand(str_in, char_idx, add_precedes);
            let sub_operation = Expression::parse_operation(str_in, char_idx, left, add_precedes);
            return Operand::Expr(Box::new(sub_operation));
        } else {
            println!("Error in parsing!! Expecting a digit or '('");
            return Operand::Val(0);
        }
    }

    fn parse_operator(str_in: &str, char_idx: &mut usize) -> Operator {
        // Consume any whitespace
        while str_in.chars().nth(*char_idx) == Some(' ') {
            *char_idx += 1;
        }
        match str_in.chars().nth(*char_idx) {
            Some('+') => {
                *char_idx += 1;
                return Operator::Add;
            }
            Some('*') => {
                *char_idx += 1;
                return Operator::Mul;
            }
            _ => {
                println!("Error in parsing!! Expecting operator '+' or '*'");
                return Operator::Add;
            }
        }
    }

    fn next_is_a_digit(str_in: &str, char_idx: &mut usize) -> bool {
        // Consume any whitespace
        while str_in.chars().nth(*char_idx) == Some(' ') {
            *char_idx += 1;
        }
        if let Some(next_char) = str_in.chars().nth(*char_idx) {
            if next_char.is_digit(10) {
                return true;
            }
        }
        return false;
    }

    fn next_is_a(c: char, str_in: &str, char_idx: &mut usize) -> bool {
        // Consume any whitespace
        while str_in.chars().nth(*char_idx) == Some(' ') {
            *char_idx += 1;
        }
        if let Some(next_char) = str_in.chars().nth(*char_idx) {
            if next_char == c {
                return true;
            }
        }
        return false;
    }

    fn evaluate(&self) -> usize {
        let left = Expression::get_term(&self.left);
        let right = Expression::get_term(&self.right);
        match self.operator {
            Operator::Add => left + right,
            Operator::Mul => left * right,
        }
    }

    fn get_term(op: &Operand) -> usize {
        match op {
            Operand::Val(x) => x.clone(),
            Operand::Expr(x) => x.evaluate(),
        }
    }
}
