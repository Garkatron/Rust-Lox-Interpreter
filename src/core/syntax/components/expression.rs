use std::cell::RefCell;
use std::{fmt, rc::Rc, sync::atomic::Ordering};

use crate::core::lox_class::LoxClass;
use crate::core::lox_function::LoxFunction;
use crate::core::lox_instance::LoxInstance;
use crate::core::{error_types::runtime_error::RuntimeError, lox_callable::LoxCallable, syntax::token::Token};
use std::sync::atomic::AtomicUsize;
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
use std::hash::Hasher;
use std::hash::Hash;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Expr {
    Binary {
        id: usize,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Logical {
        id: usize,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        id: usize,
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>
    },
    Grouping {
        id: usize,
        expression: Box<Expr>,
    },
    Literal {
        id: usize,
        value: LiteralValue,
    },
    This {
        id: usize,
        keyword: Token
    },
    Unary {
        id: usize,
        operator: Token,
        right: Box<Expr>,
    },
    Comma {
        id: usize,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Ternary {
        id: usize,
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Variable {
        id: usize,
        name: Token,
        value: Box<Expr>,
    },
    Assing {
        id: usize,
        name: Token,
        value: Box<Expr>,
    },
}

#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Rc<dyn LoxCallable>),
    LoxFunction(Rc<LoxFunction>),
    LoxInstance(Rc<RefCell<LoxInstance>>),
    LoxClass(LoxClass),
    Nil,
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralValue::Boolean(b1), LiteralValue::Boolean(b2)) => b1 == b2,
            (LiteralValue::Number(n1), LiteralValue::Number(n2)) => n1 == n2,
            (LiteralValue::String(s1), LiteralValue::String(s2)) => s1 == s2,
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            _ => false,
        }
    }
}

impl Eq for LiteralValue {}

impl LiteralValue {
    pub fn is_callable(&self) -> bool {
        matches!(self, LiteralValue::Callable(_))
    }
    pub fn return_fn_if_callable(&self) -> Option<Rc<dyn LoxCallable>> {
        match self {
            LiteralValue::Callable(fun) => Some(fun.clone()),
            LiteralValue::LoxFunction(fun) => Some(fun.clone()),
            LiteralValue::LoxClass(fun) => Some(Rc::new(fun.clone())),
            _ => None, 
        }
    }
}

impl Hash for LiteralValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LiteralValue::Boolean(b) => b.hash(state),
            LiteralValue::Number(n) => {
                n.to_bits().hash(state);
            }
            LiteralValue::String(s) => s.hash(state),
            LiteralValue::Nil => state.write_u8(0), 
            LiteralValue::Callable(_) => {
                panic!("No se puede hacer hash de un Callable");
            }
            LiteralValue::LoxInstance(_) => {
                panic!("No se puede hacer hash de un Instance");
            }
            LiteralValue::LoxClass(_) => {
                panic!("No se puede hacer hash de un Class");
            }
            LiteralValue::LoxFunction(_) => {
                panic!("No se puede hacer hash de un Function");
            }
        }
    }
}
pub trait Visitor<R> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, RuntimeError>;
    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> Result<R, RuntimeError>;
    fn visit_grouping(&mut self, expression: &Expr) -> Result<R, RuntimeError>;
    fn visit_literal(&mut self, value: &LiteralValue) -> Result<R, RuntimeError>;
    fn visit_comma(&mut self, left: &Expr, right: &Expr) -> Result<R, RuntimeError>;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<R, RuntimeError>;
    fn visit_ternary(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<R, RuntimeError>;
    fn visit_variable(&mut self, name: &Token, value: &Expr) -> Result<R, RuntimeError>;
    fn visit_assing(&mut self, name: &Token, value: &Expr) -> Result<R, RuntimeError>;
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, RuntimeError>;
    fn visit_get(&mut self, name: &Token, object: &Expr) -> Result<R, RuntimeError>;
    fn visit_set(&mut self, object: &Expr, name: &Token, value: &Expr) -> Result<R, RuntimeError>;
    fn visit_this(&mut self, keyword: &Token) -> Result<R, RuntimeError>;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, RuntimeError> {
        match self {
            Expr::Binary {
                left,
                operator,
                right, ..
            } => visitor.visit_binary(left, operator, right),
            Expr::Grouping { expression,.. } => visitor.visit_grouping(expression),
            Expr::Literal { value, .. } => visitor.visit_literal(value),
            Expr::Unary { operator, right,.. } => visitor.visit_unary(operator, right),
            Expr::Comma { left, right ,..} => visitor.visit_comma(left, right),
            Expr::Ternary {
                condition,
                then_branch,
                else_branch, ..
            } => visitor.visit_ternary(condition, then_branch, else_branch),
            Expr::Variable { name , value, ..} => {visitor.visit_variable(name, value)}
            Expr::Assing { name, value, .. } => {
                visitor.visit_assing(name, value)
            }
            Expr::Logical { left, operator, right, .. } => {
                visitor.visit_logical(left, operator, right)
            }
            Expr::Call { callee, paren, arguments , ..} => {
                visitor.visit_call(callee, paren, arguments)
            }
            Expr::Get { object, name } => {
                visitor.visit_get(name, object)
            }
            Expr::Set { object, name, value } => {
                visitor.visit_set(object, name, value)
            }
            Expr::This { keyword, .. } => {
                visitor.visit_this(keyword)
            }
        }
    }

    pub fn new_id() -> usize {
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    }
}   


impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Callable(_d) => {
                write!(f, "{:?}", "Function")
            }
            LiteralValue::Nil => write!(f, "nil"),
            LiteralValue::LoxInstance(i) => {
                write!(f, "Instance of {}", i.borrow().lox_class.name)
            }
            LiteralValue::LoxClass(c) => {
                write!(f, "Class {}", c.name)
            }
            LiteralValue::LoxFunction(ff) => {
                write!(f, "Fucntion {:?}", ff)
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right, ..
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping { expression, .. } => {
                write!(f, "(group {})", expression)
            }
            Expr::Literal { value, .. } => {
                write!(f, "{}", value)
            }
            Expr::Unary { operator, right, .. } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            Expr::Comma { left, right, .. } => {
                write!(f, "({}, {})", left, right)
            }
            Expr::Ternary {
                condition,
                then_branch,
                else_branch, ..
            } => {
                write!(f, "({}) ? {} : {}", condition, then_branch, else_branch)
            }
            Expr::Variable { name , ..} => {
                write!(f, "(var {} = {})", name.lexeme, "")
            }
            Expr::Assing { name, value, .. } => {
                write!(f, "({} = {})", name, value)
            }
            Expr::Logical { left, operator, right, .. } => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expr::Call { callee, paren, arguments, .. } => {
                write!(f, "{}({} {:?})", callee, paren, arguments)
            },
            Expr::Get { object, name } => {
                write!(f, "{}, {:?}", name, object)
            }
            Expr::Set { object, name, value } => {
                write!(f, "{}, {}, {:?}", value, name, object)
            }
            Expr::This { keyword } => {
                write!(f,"This {}", keyword)
            }
        }
    }
}

impl fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "Number({})", n),
            LiteralValue::String(s) => write!(f, "String({:?})", s),
            LiteralValue::Boolean(b) => write!(f, "Boolean({})", b),
            LiteralValue::Callable(_) => write!(f, "Callable()"),
            LiteralValue::LoxFunction(_) => write!(f, "Callable(<function>)"),
            LiteralValue::Nil => write!(f, "Nil"),
            LiteralValue::LoxInstance(i) => {
                write!(f, "LoxInstance {}", i.borrow().lox_class.name)
            }
            LiteralValue::LoxClass(c) => {
                write!(f, "LoxClass {}", c.name)
            }
        }
    }
}

