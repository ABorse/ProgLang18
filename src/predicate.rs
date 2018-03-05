
use evaluator::Expression;
use evaluator::evaluate;
use lexer::Token;

#[derive(Clone)]
pub struct Predicate {
    pub operator: Token,
    pub l_hand: Expression,
    pub r_hand: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
}

impl Predicate {

    pub fn evaluate(&self) -> Expression {

        fn evaluate_oper(o: char, l: f64, r:f64) -> bool {
            match o {
                '>' => if l > r { true } else { false },
                '<' => if l < r { true } else { false },
                '=' => if l == r { true } else { false },
                _   => panic!("Unknown operator in predicate"),
            }
        }
        
        let l_val = evaluate(&self.l_hand).ok().unwrap();
        let r_val = evaluate(&self.r_hand).ok().unwrap();
        let is_true = match self.operator {
            Token::Oper(o)  => evaluate_oper(o, l_val, r_val),
            _ => panic!("not an operator")
        };
        if is_true {
            return self.if_true.clone();
        }
        return self.if_false.clone();
    }

}
