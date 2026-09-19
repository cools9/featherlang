enum Ast {
    BinaryExpressions {
        left: Box<Ast>,
        right: Box<Ast>,
        operator: Operator,
    },

    UnaryExpression {
        operator: Operator,
        operand: Box<Ast>,
    },

    Let {
        name: String,
        value: Box<Ast>,
    },

    Print {
        value: Box<Ast>,
    },

    Number {
        value: f64,
    },

    String {
        value: String,
    },

    Bool {
        value: bool,
    },
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Ast {
    pub fn print(&self) {
        match self {
            Ast::BinaryExpressions {
                left,
                right,
                operator,
            } => {
                left.print();
                right.print();
                println!("Operator: {:?}", operator);
            }
            Ast::UnaryExpression { operator, operand } => {
                operand.print();
                println!("Operator: {:?}", operator);
            }
            Ast::Let { name, value } => {
                println!("Name: {}", name);
                value.print();
            }
            Ast::Print { value } => {
                value.print();
            }
            Ast::Number { value } => {
                println!("Number: {}", value);
            }
            Ast::String { value } => {
                println!("String: {}", value);
            }
            Ast::Bool { value } => {
                println!("Bool: {}", value);
            }
        }
    }
}
