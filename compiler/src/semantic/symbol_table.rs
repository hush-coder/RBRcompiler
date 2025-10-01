use std::collections::HashMap;
use crate::semantic::types::CType;

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub type_: CType,
    pub is_function: bool,
    pub params: Option<Vec<CType>>, // 函数参数类型
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, SymbolInfo>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { scopes: vec![HashMap::new()] }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare_variable(&mut self, name: &str, type_: CType) -> Result<(), String> {
        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(name) {
            return Err(format!("变量重复定义: {}", name));
        }
        scope.insert(name.to_string(), SymbolInfo {
            name: name.to_string(),
            type_,
            is_function: false,
            params: None,
        });
        Ok(())
    }

    pub fn declare_function(&mut self, name: &str, return_type: CType, params: &[CType]) -> Result<(), String> {
        let scope = self.scopes.first_mut().unwrap();
        if scope.contains_key(name) {
            return Err(format!("函数重复定义: {}", name));
        }
        scope.insert(name.to_string(), SymbolInfo {
            name: name.to_string(),
            type_: return_type,
            is_function: true,
            params: Some(params.to_vec()),
        });
        Ok(())
    }

    pub fn lookup_variable(&self, name: &str) -> Result<&SymbolInfo, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                if !info.is_function {
                    return Ok(info);
                }
            }
        }
        Err(format!("变量未定义: {}", name))
    }

    pub fn lookup_function(&self, name: &str) -> Result<&SymbolInfo, String> {
        let scope = self.scopes.first().unwrap();
        if let Some(info) = scope.get(name) {
            if info.is_function {
                return Ok(info);
            }
        }
        Err(format!("函数未定义: {}", name))
    }
}
