
#[cfg(test)]

extern crate updated_scheme;

use updated_scheme::predicate::*;
use updated_scheme::evaluator::*;
use updated_scheme::interpreter::*;
use updated_scheme::lexer::Token;

#[test]
fn test_raw_predicate() {

    let expr_1 = Expression::Number(1.0);
    let expr_2 = Expression::Number(2.0);
    let expr_3 = Expression::Number(3.0);
    let expr_4 = Expression::Number(4.0);

    let predicate = Predicate {
        operator: Token::Oper('>'),
        l_hand: expr_1,
        r_hand: expr_2,
        if_true: expr_3,
        if_false: expr_4,
    };
    
    let result = predicate.evaluate();
    let value = match result {
        Expression::Number(a) => a,
        _ => panic!(),
    };

    assert_eq!(value, 4.0);

}


#[test]
fn test_with_multiple_variable_string() {

    let env = String::from("(define (x (+ 3 2))) (define (y 6))");
    let expr = String::from("(if (> x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 11.0);

}


//Tests of predicates
#[test]
fn test_predicate_1() {

    let env = String::from("(define (x (* 2 4))) (define (y 7)) (define (z (if (< x y) 12 (+ 1 3))))");
    let expr = String::from("(if (> x z) (* x y) (+ z y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 56.0);

}

#[test]
fn test_predicate_2() {

    let env = String::from("(define (x 5)) (define (y (* 2.5 2)))");
    let expr = String::from("(if (> x y) (* x y) (if (< x y) (+ x y) (if (= x y) x y)))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 5.0);

}

#[test]
fn test_predicate_3() {

    let env = String::from("(define (x 12)) (define (y 5)) (define (z 9))");
    let expr = String::from("(if (= (* x y) (+ x (* z y))) 1 0)");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 0.0);

}

#[test]
fn test_predicate_4() {

    let env = String::from("(define (w 17)) (define (x 45)) (define (y 54)) (define (z 99))");
    let expr = String::from("(if (< (* w x) (* y z)) (if (= (+ x y) z) z w) (* w x))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 99.0);

}
