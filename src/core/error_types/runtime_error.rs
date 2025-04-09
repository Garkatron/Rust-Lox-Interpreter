use std::fmt;

use crate::core::syntax::{components::expression::LoxValue, token::Token};

#[derive(Debug, Clone)]
pub enum RuntimeError {
    BadOperator(Token, String),
    BadStatement(String),
    UndefinedVariable(Token),
    RedefinedVariable(String),
    BadExpr(),
    Break(),
    BadCallable(),
    ToManyArguments(Token, usize, usize),
    NativeFunctionError(String),
    Return(LoxValue),
    BadArguments(String),
    InvalidFunction(String),
    OnlyInstancesHaveProperties(),
    UndefinedProperty(),
    CantReturnFromInitializer(),
    CantAccessPrivateMethod(),
    CantCallStaticMethodFromInstance(),
    ClassInheritFromItself(),
    SuperClassMustBeSuperAClass(),
    InvalidSuperclass(),
    UnresolvedSuper(),
    InvalidClassMember()
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::BadOperator(operator, message) => {
                write!(f, "[RUNTIME ERROR]: Invalid operator '{}' used: {}", operator, message)
            }
            RuntimeError::BadStatement(message) => {
                write!(f, "[RUNTIME ERROR]: Invalid statement: {}", message)
            }
            RuntimeError::UndefinedVariable(token) => {
                write!(f, "[RUNTIME ERROR]: Undefined variable: '{}'", token.lexeme)
            }
            RuntimeError::RedefinedVariable(name) => {
                write!(f, "[RUNTIME ERROR]: The variable '{}' has already been defined and cannot be redefined.", name)
            }
            RuntimeError::BadExpr() => {
                write!(f, "[RUNTIME ERROR]: Invalid expression.")
            }
            RuntimeError::Break() => {
                write!(f, "[RUNTIME ERROR]: 'Break' statement used outside of a loop or control flow context.")
            }
            RuntimeError::BadCallable() => {
                write!(f, "[RUNTIME ERROR]: Only functions and classes can be called as callable objects.")
            }
            RuntimeError::ToManyArguments(paren, arity , args_size) => {
                write!(f, "[RUNTIME ERROR]: In '{}': Expected {} arguments but received {}.", paren, arity, args_size)
            }
            RuntimeError::NativeFunctionError(message) => {
                write!(f, "[RUNTIME ERROR]: Error in native function: {}.", message)
            }
            RuntimeError::Return(_) => {
                write!(f, "[RUNTIME ERROR]: 'Return' statement used outside of a function.")
            }
            RuntimeError::CantReturnFromInitializer() => {
                write!(f, "[RUNTIME ERROR]: Cannot return a value from an initializer.")
            }
            RuntimeError::BadArguments(m) => {
                write!(f,"[RUNTIME ERROR]: Invalid arguments provided: {}", m)
            }
            RuntimeError::InvalidFunction(m) => {
                write!(f,"[RUNTIME ERROR]: The function '{}' is invalid or undefined.", m)
            }
            RuntimeError::OnlyInstancesHaveProperties() => {
                write!(f,"[RUNTIME ERROR]: Only instances (not classes) can have properties.")
            }
            RuntimeError::UndefinedProperty() => {
                write!(f,"[RUNTIME ERROR]: Property does not exist or is inaccessible.")
            }
            RuntimeError::CantAccessPrivateMethod() => {
                write!(f,"[RUNTIME ERROR]: Attempted to access a private method outside its class.")
            }
            RuntimeError::CantCallStaticMethodFromInstance() => {
                write!(f,"[RUNTIME ERROR]: Cannot call a static method from an instance of the class.")
            }   
            RuntimeError::ClassInheritFromItself() => {
                write!(f,"[RUNTIME ERROR]: A class can't inherit from itself.")
            }
            RuntimeError::SuperClassMustBeSuperAClass() => {
                write!(f,"[RUNTIME ERROR]: SuperClass must be a SuperClass.")

            }  
            RuntimeError::InvalidSuperclass() => {
                write!(f,"[RUNTIME ERROR]: Invalid Superclass.")
            }     
            RuntimeError::UnresolvedSuper() => {
                write!(f,"[RUNTIME ERROR]: Unresolved Super.")
            }  
            RuntimeError::InvalidClassMember() => {
                write!(f,"[RUNTIME ERROR]: Invalid class member.")
            }  
            
        }
    }
}

impl std::error::Error for RuntimeError {}
