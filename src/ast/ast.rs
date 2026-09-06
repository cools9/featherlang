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


enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
