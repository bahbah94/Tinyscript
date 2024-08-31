use crate::lexer::lexer::Token; 
use crate::parser::parser::ASTNode;
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    String,
    Boolean,
}

// Define `SymbolEntry` struct to hold information about each symbol
#[derive(Debug, Clone)]
pub struct SymbolEntry {
    pub name: String,
    pub typ: Type,
}

// Define `SymbolTable` struct with methods for managing symbols and scopes
#[derive(Debug,Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<String, SymbolEntry>,
    pub parent: Option<Box<SymbolTable>>,  // Reference to the parent scope's table
}

impl SymbolTable {
    // Creates a new symbol table, optionally with a parent scope
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent,
        }
    }

    // Inserts a new symbol into the current scope
    pub fn insert(&mut self, name: String, typ: Type) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            Err(format!("Symbol '{}' is already defined", name))
        } else {
            let entry = SymbolEntry { name: name.clone(), typ };
            self.symbols.insert(name, entry);
            Ok(())
        }
    }

    // Looks up a symbol in the current scope, or recursively in parent scopes
    pub fn lookup(&self, name: &str) -> Option<&SymbolEntry> {
        if let Some(entry) = self.symbols.get(name) {
            Some(entry)
        } else if let Some(ref parent) = self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }
  }

    // Define the `SemanticAnalyzer` struct to handle type checking and scope management
pub struct SemanticAnalyzer {
    scopes: Vec<SymbolTable>,  // Stack of scopes
}

impl SemanticAnalyzer {
    // Initializes the semantic analyzer with a global scope
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: vec![SymbolTable::new(None)],  // Start with a global scope
        }
    }

    // Enter a new scope by pushing a new symbol table onto the stack
    pub fn enter_scope(&mut self) {
        self.scopes.push(SymbolTable::new(Some(Box::new(self.scopes.last().unwrap().clone()))));
    }

    // Exit the current scope by popping the top symbol table off the stack
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    // Declare a new variable in the current scope
    pub fn declare_variable(&mut self, name: String, typ: Type) -> Result<(), String> {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name, typ)
    }

    // Look up a variable in the current or parent scopes
    pub fn lookup_variable(&self, name: &str) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(entry) = scope.lookup(name) {
                return Some(&entry.typ);
            }
        }
        None
    }

    // Check the types of expressions and statements in the AST
    pub fn check_types(&mut self, node: &ASTNode) -> Result<Type, String> {
        match node {
            // Handle binary operations like addition, subtraction, etc.
            ASTNode::BinaryOp(left, token, right) => {
                let left_type = self.check_types(left)?;
                let right_type = self.check_types(right)?;
                match token {
                    Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                        if left_type == Type::Integer && right_type == Type::Integer {
                            Ok(Type::Integer)
                        } else {
                            Err(format!("Type error: {:?} and {:?} are not compatible with {:?}", left_type, right_type, token))
                        }
                    }
                    Token::Equal | Token::NotEqual | Token::LessThan | Token::GreaterThan => {
                        if left_type == right_type {
                            Ok(Type::Boolean)  // Comparison operators result in a boolean
                        } else {
                            Err(format!("Type error: {:?} and {:?} cannot be compared with {:?}", left_type, right_type, token))
                        }
                    }
                    _ => Err(format!("Unknown binary operator: {:?}", token)),
                }
            }
            // Handle identifier nodes (variable usage)
            ASTNode::Identifier(name) => {
                if let Some(typ) = self.lookup_variable(name) {
                    Ok(typ.clone())
                } else {
                    Err(format!("Undeclared variable: {}", name))
                }
            }
            // Handle integer literals
            ASTNode::Integer(_) => Ok(Type::Integer),
            // Handle string literals
            ASTNode::StringLiteral(_) => Ok(Type::String),
            // Handle let statements (variable declaration)
            ASTNode::LetStmt(name, expr) => {
                let expr_type = self.check_types(expr)?;
                self.declare_variable(name.clone(), expr_type.clone())?;
                Ok(expr_type)
            }
            // Handle if statements
            ASTNode::IfStmt(condition, then_branch, else_branch) => {
                let cond_type = self.check_types(condition)?;
                if cond_type != Type::Boolean {
                    return Err("Condition of if statement must be boolean".to_string());
                }
                self.check_types(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.check_types(else_branch)?;
                }
                Ok(Type::Boolean)  // The type of the entire if statement might depend on your language's semantics
            }
            // Handle while statements
            ASTNode::WhileStmt(condition, body) => {
                let cond_type = self.check_types(condition)?;
                if cond_type != Type::Boolean {
                    return Err("Condition of while statement must be boolean".to_string());
                }
                self.check_types(body)?;
                Ok(Type::Boolean)
            }
            // Handle return statements
            ASTNode::ReturnStmt(expr) => {
                let expr_type = self.check_types(expr)?;
                Ok(expr_type)  // Return type needs to be checked against the function's declared return type
            }
            // Handle blocks of statements
            ASTNode::Block(statements) => {
                self.enter_scope();
                for stmt in statements {
                    self.check_types(stmt)?;
                }
                self.exit_scope();
                Ok(Type::Boolean)  // The type of a block could be void or unit type depending on your language
            }
            // Handle expression statements
            ASTNode::ExprStmt(expr) => {
                self.check_types(expr)
            }
            // Handle other cases as needed...
            _ => Err(format!("Unknown AST node type: {:?}", node)),
        }
    }
}