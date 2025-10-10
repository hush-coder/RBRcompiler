use super::basic_block::BasicBlock;
use std::fmt;

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub blocks: Vec<BasicBlock>,
    pub parameters: Vec<String>,
    pub return_type: String,
}

impl IRFunction {
    pub fn new(name: String, parameters: Vec<String>, return_type: String) -> Self {
        Self {
            name,
            blocks: Vec::new(),
            parameters,
            return_type,
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }

    pub fn get_entry_block(&self) -> Option<&BasicBlock> {
        self.blocks.first()
    }

    pub fn get_block_by_label(&self, label: &str) -> Option<&BasicBlock> {
        self.blocks.iter().find(|block| block.label == label)
    }
}

impl fmt::Display for IRFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params_str = self.parameters.join(", ");
        writeln!(f, "function {} ({}) -> {} {{", self.name, params_str, self.return_type)?;
        
        for block in &self.blocks {
            write!(f, "{}", block)?;
        }
        
        writeln!(f, "}}")?;
        Ok(())
    }
} 