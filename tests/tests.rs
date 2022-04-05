use interpreter::interpret::interpret;
use interpreter::types::ByteCode;

#[cfg(test)]
mod interpreter_tests {
    use super::*;
    use ByteCode::*;

    #[test]
    fn err_no_value_loaded() {
        assert!(
            interpret(vec![Return]).is_err(),
            "Should return StackUnderflow error"
        );
    }

    #[test]
    fn load_value_and_return() {

        let val = 1;
        
        let load_value_and_return = vec![LoadVal(val), Return];

        assert_eq!(
            interpret(load_value_and_return).unwrap().value,
            val,
            "Should return the loaded value"
        );
    }

    #[test]
    fn load_two_values_multiply_return() {

        let val_first = 2;
        let val_second = 4;

        let load_two_values_multiply_return = vec![LoadVal(val_first), LoadVal(val_second), Multiply, Return];

        assert_eq!(
            interpret(load_two_values_multiply_return)
                .unwrap()
                .value,
            val_first * val_second,
            "Should return {} * {} = {}",
            val_first,
            val_second,
            val_first * val_second
        );
    }

    #[test]
    fn load_two_values_divide_return() {

        let val_first = 4;
        let val_second = 2;

        let load_two_values_multiply_return = vec![LoadVal(val_first), LoadVal(val_second), Divide, Return];

        assert_eq!(
            interpret(load_two_values_multiply_return)
                .unwrap()
                .value,
            val_first / val_second,
            "Should return {} / {} = {}",
            val_first,
            val_second,
            val_first / val_second
        );
    }

    #[test]
    fn load_two_values_subtract_return() {

        let val_first = 4;
        let val_second = 2;

        let load_two_values_multiply_return = vec![LoadVal(val_first), LoadVal(val_second), Subtract, Return];

        assert_eq!(
            interpret(load_two_values_multiply_return)
                .unwrap()
                .value,
            val_first - val_second,
            "Should return {} - {} = {}",
            val_first,
            val_second,
            val_first - val_second
        );
    }

    #[test]
    fn write_value() {

        let val = 2;
        let var_name = 'c';

        let write_value = vec![LoadVal(val), WriteVar(var_name), Return];
        let write_value_result = interpret(write_value).unwrap();

        assert_eq!(
            write_value_result.name,
            Some(var_name),
            "Should load value {} into variable {} -> this expression should resolve to Some({})",
            val,
            var_name,
            var_name
        );
        assert_eq!(
            write_value_result.value, val,
            "Should load value {} into variable {} -> this expression should resolve to {}",
            val, var_name, val
        );
    }

    #[test]
    fn arithmetic_written_values() {

        let x = 'x';
        let y = 'y';

        let val_x_first = 1;
        let val_y_first = 2;

        let arithmetic_written_values = vec![
            // Load 1
            LoadVal(val_x_first),
            // Write x = 1
            WriteVar(x),
            // Load 2
            LoadVal(val_y_first),
            // Write y = 2
            WriteVar(y),
            // Read x = 1
            ReadVar(x),
            // Load 1
            LoadVal(1),
            // Add (will apply to last 2 values in stack) -> 1 + 1 = 2 (new value in stack)
            Add,
            // Read y = 2
            ReadVar(y),
            // multiply -> 2 * 2 = 4 (new value in stack)
            Multiply,
            // Load 3
            LoadVal(3),
            // Subtract -> 4 - 3 = 1 (new value in stack)
            Subtract,
            // Return the result
            Return,
        ];

        assert_eq!(
            interpret(arithmetic_written_values).unwrap().value,
            1,
            "Should apply the sequence of arithmetic operations using the memory on the stack and then return the result"
        );
    }
}
