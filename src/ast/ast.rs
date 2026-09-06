enum ast{
    BinaryExpressions{
        left: Box<ast>,
        right: Box<ast>,
        operator: Operator,
    },

    UnaryExpression{
        operator: Operator,
        operand: Box<ast>,
    },

    Let{
        name: String,
        value: Box<ast>,
    },

    Print{
        value: Box<ast>,
    },

    Number{
        value: f64,
    },

    String{
        value: String,
    },

    Bool{
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


impl ast{
    pub fn print(&self) {
        match self {
            ast::BinaryExpressions { left, right, operator } => {
                left.print();
                right.print();
                println!("Operator: {:?}", operator);
            }
            ast::UnaryExpression { operator, operand } => {
                operand.print();
                println!("Operator: {:?}", operator);
            }
            ast::Let { name, value } => {
                println!("Name: {}", name);
                value.print();
            }
            ast::Print { value } => {
                value.print();
            }
            ast::Number { value } => {
                println!("Number: {}", value);
            }
            ast::String { value } => {
                println!("String: {}", value);
            }
            ast::Bool { value } => {
                println!("Bool: {}", value);
            }
        }
    }

}
