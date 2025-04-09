use std::cell::RefCell;
use std::{fmt, rc::Rc, sync::atomic::Ordering};

use crate::core::fuctions::lox_callable::LoxCallable;
use crate::core::fuctions::lox_function::LoxFunction;
use crate::core::oop::lox_class::LoxClass;
use crate::core::oop::lox_instance::LoxInstance;
use crate::core::{error_types::runtime_error::RuntimeError, syntax::token::Token};
use std::sync::atomic::AtomicUsize;
static NEXT_ID: AtomicUsize = AtomicUsize::new(2);
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
    Super {
        keyword: Token,
        method: Token
    },
    Grouping {
        id: usize,
        expression: Box<Expr>,
    },
    Literal {
        id: usize,
        value: LoxValue,
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
pub enum LoxValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Rc<dyn LoxCallable>),
    LoxFunction(Rc<LoxFunction>),
    LoxInstance(Rc<RefCell<LoxInstance>>),
    LoxClass(LoxClass),
    Nil,
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Boolean(b1), LoxValue::Boolean(b2)) => b1 == b2,
            (LoxValue::Number(n1), LoxValue::Number(n2)) => n1 == n2,
            (LoxValue::String(s1), LoxValue::String(s2)) => s1 == s2,
            (LoxValue::Nil, LoxValue::Nil) => true,
            _ => false,
        }
    }
}

impl Eq for LoxValue {}

impl LoxValue {
    pub fn is_callable(&self) -> bool {
        matches!(self, LoxValue::Callable(_))
    }
    pub fn return_fn_if_callable(&self) -> Option<Rc<dyn LoxCallable>> {
        match self {
            LoxValue::Callable(fun) => Some(fun.clone()),
            LoxValue::LoxFunction(fun) => Some(fun.clone()),
            LoxValue::LoxClass(fun) => Some(Rc::new(fun.clone())),
            _ => None, 
        }
    }
}

impl Hash for LoxValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LoxValue::Boolean(b) => b.hash(state),
            LoxValue::Number(n) => {
                n.to_bits().hash(state);
            }
            LoxValue::String(s) => s.hash(state),
            LoxValue::Nil => state.write_u8(0), 
            LoxValue::Callable(_) => {
                panic!("No se puede hacer hash de un Callable");
            }
            LoxValue::LoxInstance(_) => {
                panic!("No se puede hacer hash de un Instance");
            }
            LoxValue::LoxClass(_) => {
                panic!("No se puede hacer hash de un Class");
            }
            LoxValue::LoxFunction(_) => {
                panic!("No se puede hacer hash de un Function");
            }
        }
    }
}
pub trait Visitor<R> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, RuntimeError>;
    fn visit_call(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> Result<R, RuntimeError>;
    fn visit_grouping(&mut self, expression: &Expr) -> Result<R, RuntimeError>;
    fn visit_literal(&mut self, value: &LoxValue) -> Result<R, RuntimeError>;
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
    fn visit_super(&mut self, keyword: &Token, keyword: &Token) -> Result<R, RuntimeError>;
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
            Expr::Super { keyword, method } => {
                visitor.visit_super(keyword, method)
            }
        }
    }

    pub fn new_id() -> usize {
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
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
            Expr::This { keyword , ..} => {
                write!(f,"This {}", keyword)
            }
            Expr::Super { keyword, method } => {
                write!(f,"Super {} {}", keyword, method)
            }
        }
    }
}

impl fmt::Debug for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxValue::Number(n) => write!(f, "Number({})", n),
            LoxValue::String(s) => write!(f, "String({:?})", s),
            LoxValue::Boolean(b) => write!(f, "Boolean({})", b),
            LoxValue::Callable(_) => write!(f, "Callable()"),
            LoxValue::LoxFunction(_) => write!(f, "Callable(<function>)"),
            LoxValue::Nil => write!(f, "Nil"),
            LoxValue::LoxInstance(i) => {
                write!(f, "LoxInstance({})", i.borrow().lox_class.name)
            }
            LoxValue::LoxClass(c) => {
                write!(f, "LoxClass({})", c.name)
            }
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxValue::Number(n) => write!(f, "Number({})", n),
            LoxValue::String(s) => write!(f, "String({})", s),
            LoxValue::Boolean(b) => write!(f, "Boolean({})", b),
            LoxValue::Callable(_d) => {
                write!(f, "LoxCallable()")
            }
            LoxValue::Nil => write!(f, "nil"),
            LoxValue::LoxInstance(i) => {
                write!(f, "LoxInstance({})", i.borrow().lox_class.name)
            }
            LoxValue::LoxClass(c) => {
                if let Some(s) = &c.super_class {
                    write!(f, "LoxClass({}) <- LoxSuper({})", c.name, s.name)
                } else {
                    write!(f, "LoxClass({})", c.name)
                }
            }
            LoxValue::LoxFunction(ff) => {
                write!(f, "LoxFunction({})" , ff)
            }
        }
    }
}
