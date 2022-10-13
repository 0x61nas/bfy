use crate::bf_interpreter::interpreter::Interpreter;
use console::Term;

pub struct Repl {
    pub interpreter: Interpreter,
    pub term: Term,
    pub history: Vec<String>,
    pub loop_body: String,
    pub loop_depth: usize,
}

/// The REPL prompt
pub const PROMPT: &str = "bf-interpreter> ";
/// History file name
pub const HISTORY_FILE: &str = "bf-interpreter-history.bfr";
/// The command prefix
pub const COMMAND_PREFIX: &str = "!";

/// Tests :D
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bf_interpreter::cell::Cell;
    use pretty_assertions::assert_eq;

    #[test]
    fn nested_loop_level_1() {
        let term = Term::stdout();
        let interpreter = Interpreter::new(4, vec![], term);

        let mut repl = Repl::new(interpreter);

        repl.process("++".to_string());
        repl.process("[>++".to_string());
        repl.process("[>+<-]".to_string());
        repl.process("<-]".to_string());

        let cells = &repl.interpreter.cells;

        assert_eq!(cells[0], Cell::default_cell(&vec![]));
        assert_eq!(cells[1], Cell::default_cell(&vec![]));
        assert_eq!(cells[2], Cell::new(4, &vec![]));
    }

    #[test]
    fn nested_loop_level_2() {
        let term = Term::stdout();
        let interpreter = Interpreter::new(4, vec![], term);

        let mut repl = Repl::new(interpreter);

        repl.process("++".to_string());
        repl.process("[>++".to_string());
        repl.process("[>+<-]".to_string());
        repl.process("[>++".to_string());
        repl.process("[>+<-]".to_string());
        repl.process("<-]".to_string());
        repl.process("<-]".to_string());

        let cells = &repl.interpreter.cells;

        assert_eq!(cells[0], Cell::default_cell(&vec![]));
        assert_eq!(cells[1], Cell::default_cell(&vec![]));
        assert_eq!(cells[2], Cell::new(4, &vec![]));
    }

    #[test]
    fn print_my_first_name() {
        let term = Term::stdout();
        let interpreter = Interpreter::new(10, vec![], term);

        let mut repl = Repl::new(interpreter);

        let code = "++++ ++++ 8
        [
            >++++
            [
            >++ A
            >+++ a
            >++++
            >+ space
            <<<<-
        ]

        >>>>>>++
        [
            <<<-
            >>>-
        ]

        <<<<<<<-
        ]
        >>+. Print cell 2: A
        <<++++
            [
            >+++
            [
            >+++
            <-
        ]
        >++
        <<-
        ]
        >>+. Print n
            <<+++
            [
            >+++
            [
            >-
            <-
        ]
        >-
            <<-
        ]
        >>-. Print n
            <<++++++
            [
            >>+++
            <<-
        ]
        >>. Print s"
            .to_string()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        for line in code {
            repl.process(line);
        }

        assert_eq!(repl.interpreter.cells[0], Cell::default_cell(&vec![]));
        assert_eq!(repl.interpreter.cells[1], Cell::default_cell(&vec![]));
        assert_eq!(repl.interpreter.cells[2], Cell::new(115, &vec![]));
        assert_eq!(repl.interpreter.cells[3], Cell::new(96, &vec![]));
        assert_eq!(repl.interpreter.cells[4], Cell::new(112, &vec![]));
        assert_eq!(repl.interpreter.cells[5], Cell::new(32, &vec![]));
    }

    #[test]
    fn print_my_first_name_in_one_command() {
        let term = Term::stdout();
        let interpreter = Interpreter::new(10, vec![], term);

        let mut repl = Repl::new(interpreter);

        let code = "++++++++[>++++[>++>+++>++++>+<<<<-]>>>>>>++[<<<->>>-]<<<<<<<-]>>+.\
        <<++++[>+++[>+++<-]>++<<-]>>+.<<+++[>+++[>-<-]>-<<-]>>-.<<++++++[>>+++<<-]>>."
            .to_string();

        repl.process(code);

        assert_eq!(repl.interpreter.cells[0], Cell::default_cell(&vec![]));
        assert_eq!(repl.interpreter.cells[1], Cell::default_cell(&vec![]));
        assert_eq!(repl.interpreter.cells[2], Cell::new(115, &vec![]));
        assert_eq!(repl.interpreter.cells[3], Cell::new(96, &vec![]));
        assert_eq!(repl.interpreter.cells[4], Cell::new(112, &vec![]));
        assert_eq!(repl.interpreter.cells[5], Cell::new(32, &vec![]));
    }

    #[test]
    fn print_hello_world() {
        let term = Term::stdout();
        let interpreter = Interpreter::new(10, vec![], term);

        let mut repl = Repl::new(interpreter);

        let _ = "[ This program prints \"Hello World!\" and a newline to the screen, its
                length is 106 active command characters. [It is not the shortest.]
                ]
                ++++++++               Set Cell #0 to 8
                [
                    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
                    [                   as the cell will be cleared by the loop
                        >++             Add 2 to Cell #2
                        >+++            Add 3 to Cell #3
                        >+++            Add 3 to Cell #4
                        >+              Add 1 to Cell #5
                        <<<<-           Decrement the loop counter in Cell #1
                    ]                   Loop until Cell #1 is zero; number of iterations is 4
                    >+                  Add 1 to Cell #2
                    >+                  Add 1 to Cell #3
                    >-                  Subtract 1 from Cell #4
                    >>+                 Add 1 to Cell #6
                    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
                    <-                  Decrement the loop Counter in Cell #0
                ]                       Loop until Cell #0 is zero; number of iterations is 8

                The result of this is:
                Cell no :   0   1   2   3   4   5   6
                Contents:   0   0  72 104  88  32   8
                Pointer :   ^

                >>.                     Cell #2 has value 72 which is 'H'
                >---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
                +++++++..+++.           Likewise for 'llo' from Cell #3
                >>.                     Cell #5 is 32 for the space
                <-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
                <.                      Cell #3 was set to 'o' from the end of 'Hello'
                +++.------.--------.    Cell #3 for 'rl' and 'd'
                >>+.                    Add 1 to Cell #5 gives us an exclamation point
                >++.                    And finally a newline from Cell #6
            "
        .to_string()
        .split("\n")
        .for_each(|s| repl.process(s.to_string()));
    }
}
