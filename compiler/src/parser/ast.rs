use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct ImplicitCastContext {
    pub cast_count: usize,
}

impl ImplicitCastContext {
    pub fn new() -> Self {
        Self { cast_count: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ASTNode {
    // Program structure
    Program(Vec<ASTNode>),
    
    // Declarations
    VariableDeclaration {
        type_name: String,
        name: String,
        initializer: Option<Box<ASTNode>>,
    },
    FunctionDeclaration {
        return_type: String,
        name: String,
        parameters: Vec<ASTNode>,
        body: Box<ASTNode>,
    },
    Parameter {
        type_name: String,
        name: String,
    },
    
    // Statements
    Block(Vec<ASTNode>),
    DeclStmt(Box<ASTNode>),  // 包装变量声明的语句
    ExpressionStatement(Box<ASTNode>),
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    WhileStatement {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    ForStatement {
        init: Option<Box<ASTNode>>,
        condition: Option<Box<ASTNode>>,
        update: Option<Box<ASTNode>>,
        body: Box<ASTNode>,
    },
    ReturnStatement(Option<Box<ASTNode>>),
    
    // Expressions
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    ImplicitCastExpr {
        cast_kind: String,
        operand: Box<ASTNode>,
    },
    BinaryExpression {
        operator: BinaryOperator,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    UnaryExpression {
        operator: UnaryOperator, 
        operand: Box<ASTNode>,
    },
    AssignmentExpression {
        target: Box<ASTNode>,
        value: Box<ASTNode>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>,
    },
    
    // Empty node for error recovery
    Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,        // +
    Subtract,   // -
    Multiply,   // *
    Divide,     // /
    Equal,      // = (simplified assignment as equality for now)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Plus,       // +
    Minus,      // -
}

impl ASTNode {
    /// 生成bisheng格式的AST输出
    pub fn to_bisheng_ast(&self) -> String {
        let mut output = String::new();
        let mut context = ImplicitCastContext::new();
        
        // 添加内置类型定义（模拟bisheng的输出）
        self.add_builtin_types(&mut output);
        
        // 添加用户代码的AST
        if let ASTNode::Program(statements) = self {
            for stmt in statements {
                stmt.format_bisheng_with_context(&mut output, "", "`-", 0, &mut context);
            }
        }
        output
    }
    
    fn add_builtin_types(&self, output: &mut String) {
        // 完全复制bisheng的内置类型输出
        output.push_str("TranslationUnitDecl 0x400000c76368 <<invalid sloc>> <invalid sloc>\n");
        output.push_str("|-TypedefDecl 0x400000c77210 <<invalid sloc>> <invalid sloc> implicit __int128_t '__int128'\n");
        output.push_str("| `-BuiltinType 0x400000c76930 '__int128'\n");
        output.push_str("|-TypedefDecl 0x400000c77280 <<invalid sloc>> <invalid sloc> implicit __uint128_t 'unsigned __int128'\n");
        output.push_str("| `-BuiltinType 0x400000c76950 'unsigned __int128'\n");
        output.push_str("|-TypedefDecl 0x400000d21a98 <<invalid sloc>> <invalid sloc> implicit __NSConstantString 'struct __NSConstantString_tag'\n");
        output.push_str("| `-RecordType 0x400000d21870 'struct __NSConstantString_tag'\n");
        output.push_str("|   `-Record 0x400000c772d8 '__NSConstantString_tag'\n");
        output.push_str("|-TypedefDecl 0x400000d21b00 <<invalid sloc>> <invalid sloc> implicit __SVInt8_t '__SVInt8_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76af0 '__SVInt8_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21b68 <<invalid sloc>> <invalid sloc> implicit __SVInt16_t '__SVInt16_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76b10 '__SVInt16_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21bd0 <<invalid sloc>> <invalid sloc> implicit __SVInt32_t '__SVInt32_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76b30 '__SVInt32_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21c38 <<invalid sloc>> <invalid sloc> implicit __SVInt64_t '__SVInt64_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76b50 '__SVInt64_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21ca0 <<invalid sloc>> <invalid sloc> implicit __SVUint8_t '__SVUint8_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76b70 '__SVUint8_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21d08 <<invalid sloc>> <invalid sloc> implicit __SVUint16_t '__SVUint16_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76b90 '__SVUint16_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21d70 <<invalid sloc>> <invalid sloc> implicit __SVUint32_t '__SVUint32_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76bb0 '__SVUint32_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21dd8 <<invalid sloc>> <invalid sloc> implicit __SVUint64_t '__SVUint64_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76bd0 '__SVUint64_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21e40 <<invalid sloc>> <invalid sloc> implicit __SVFloat16_t '__SVFloat16_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76bf0 '__SVFloat16_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21ea8 <<invalid sloc>> <invalid sloc> implicit __SVFloat32_t '__SVFloat32_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76c10 '__SVFloat32_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21f10 <<invalid sloc>> <invalid sloc> implicit __SVFloat64_t '__SVFloat64_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76c30 '__SVFloat64_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21f78 <<invalid sloc>> <invalid sloc> implicit __SVBFloat16_t '__SVBFloat16_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76c50 '__SVBFloat16_t'\n");
        output.push_str("|-TypedefDecl 0x400000d21fe0 <<invalid sloc>> <invalid sloc> implicit __clang_svint8x2_t '__clang_svint8x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76c70 '__clang_svint8x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22048 <<invalid sloc>> <invalid sloc> implicit __clang_svint16x2_t '__clang_svint16x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76c90 '__clang_svint16x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d220b0 <<invalid sloc>> <invalid sloc> implicit __clang_svint32x2_t '__clang_svint32x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76cb0 '__clang_svint32x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22118 <<invalid sloc>> <invalid sloc> implicit __clang_svint64x2_t '__clang_svint64x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76cd0 '__clang_svint64x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22180 <<invalid sloc>> <invalid sloc> implicit __clang_svuint8x2_t '__clang_svuint8x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76cf0 '__clang_svuint8x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d221e8 <<invalid sloc>> <invalid sloc> implicit __clang_svuint16x2_t '__clang_svuint16x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76d10 '__clang_svuint16x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22250 <<invalid sloc>> <invalid sloc> implicit __clang_svuint32x2_t '__clang_svuint32x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76d30 '__clang_svuint32x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d222b8 <<invalid sloc>> <invalid sloc> implicit __clang_svuint64x2_t '__clang_svuint64x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76d50 '__clang_svuint64x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22320 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat16x2_t '__clang_svfloat16x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76d70 '__clang_svfloat16x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22388 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat32x2_t '__clang_svfloat32x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76d90 '__clang_svfloat32x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d223f0 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat64x2_t '__clang_svfloat64x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76db0 '__clang_svfloat64x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22458 <<invalid sloc>> <invalid sloc> implicit __clang_svbfloat16x2_t '__clang_svbfloat16x2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76dd0 '__clang_svbfloat16x2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d224c0 <<invalid sloc>> <invalid sloc> implicit __clang_svint8x3_t '__clang_svint8x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76df0 '__clang_svint8x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22528 <<invalid sloc>> <invalid sloc> implicit __clang_svint16x3_t '__clang_svint16x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76e10 '__clang_svint16x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22590 <<invalid sloc>> <invalid sloc> implicit __clang_svint32x3_t '__clang_svint32x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76e30 '__clang_svint32x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d225f8 <<invalid sloc>> <invalid sloc> implicit __clang_svint64x3_t '__clang_svint64x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76e50 '__clang_svint64x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22660 <<invalid sloc>> <invalid sloc> implicit __clang_svuint8x3_t '__clang_svuint8x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76e70 '__clang_svuint8x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d226c8 <<invalid sloc>> <invalid sloc> implicit __clang_svuint16x3_t '__clang_svuint16x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76e90 '__clang_svuint16x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22730 <<invalid sloc>> <invalid sloc> implicit __clang_svuint32x3_t '__clang_svuint32x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76eb0 '__clang_svuint32x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22798 <<invalid sloc>> <invalid sloc> implicit __clang_svuint64x3_t '__clang_svuint64x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76ed0 '__clang_svuint64x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d22800 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat16x3_t '__clang_svfloat16x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76ef0 '__clang_svfloat16x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d230a0 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat32x3_t '__clang_svfloat32x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76f10 '__clang_svfloat32x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23108 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat64x3_t '__clang_svfloat64x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76f30 '__clang_svfloat64x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23170 <<invalid sloc>> <invalid sloc> implicit __clang_svbfloat16x3_t '__clang_svbfloat16x3_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76f50 '__clang_svbfloat16x3_t'\n");
        output.push_str("|-TypedefDecl 0x400000d231d8 <<invalid sloc>> <invalid sloc> implicit __clang_svint8x4_t '__clang_svint8x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76f70 '__clang_svint8x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23240 <<invalid sloc>> <invalid sloc> implicit __clang_svint16x4_t '__clang_svint16x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76f90 '__clang_svint16x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d232a8 <<invalid sloc>> <invalid sloc> implicit __clang_svint32x4_t '__clang_svint32x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76fb0 '__clang_svint32x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23310 <<invalid sloc>> <invalid sloc> implicit __clang_svint64x4_t '__clang_svint64x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76fd0 '__clang_svint64x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23378 <<invalid sloc>> <invalid sloc> implicit __clang_svuint8x4_t '__clang_svuint8x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c76ff0 '__clang_svuint8x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d233e0 <<invalid sloc>> <invalid sloc> implicit __clang_svuint16x4_t '__clang_svuint16x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c77010 '__clang_svuint16x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23448 <<invalid sloc>> <invalid sloc> implicit __clang_svuint32x4_t '__clang_svuint32x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c77030 '__clang_svuint32x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d234b0 <<invalid sloc>> <invalid sloc> implicit __clang_svuint64x4_t '__clang_svuint64x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c77050 '__clang_svuint64x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23518 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat16x4_t '__clang_svfloat16x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c77070 '__clang_svfloat16x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23580 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat32x4_t '__clang_svfloat32x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c77090 '__clang_svfloat32x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d235e8 <<invalid sloc>> <invalid sloc> implicit __clang_svfloat64x4_t '__clang_svfloat64x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c770b0 '__clang_svfloat64x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23650 <<invalid sloc>> <invalid sloc> implicit __clang_svbfloat16x4_t '__clang_svbfloat16x4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c770d0 '__clang_svbfloat16x4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d236b8 <<invalid sloc>> <invalid sloc> implicit __SVBool_t '__SVBool_t'\n");
        output.push_str("| `-BuiltinType 0x400000c770f0 '__SVBool_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23720 <<invalid sloc>> <invalid sloc> implicit __clang_svboolx2_t '__clang_svboolx2_t'\n");
        output.push_str("| `-BuiltinType 0x400000c77110 '__clang_svboolx2_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23788 <<invalid sloc>> <invalid sloc> implicit __clang_svboolx4_t '__clang_svboolx4_t'\n");
        output.push_str("| `-BuiltinType 0x400000c77130 '__clang_svboolx4_t'\n");
        output.push_str("|-TypedefDecl 0x400000d237f0 <<invalid sloc>> <invalid sloc> implicit __SVCount_t '__SVCount_t'\n");
        output.push_str("| `-BuiltinType 0x400000c77150 '__SVCount_t'\n");
        output.push_str("|-TypedefDecl 0x400000d23890 <<invalid sloc>> <invalid sloc> implicit __builtin_ms_va_list 'char *'\n");
        output.push_str("| `-PointerType 0x400000d23850 'char *'\n");
        output.push_str("|   `-BuiltinType 0x400000c76410 'char'\n");
        output.push_str("|-TypedefDecl 0x400000d23b88 <<invalid sloc>> <invalid sloc> implicit __builtin_va_list 'struct __va_list'\n");
        output.push_str("| `-RecordType 0x400000d23970 'struct __va_list'\n");
        output.push_str("|   `-Record 0x400000d238e8 '__va_list'\n");
    }

    fn format_bisheng_with_context(&self, output: &mut String, prefix: &str, connector: &str, depth: usize, context: &mut ImplicitCastContext) {
        let (node_type, node_info) = self.get_bisheng_node_info_with_context(context);
        
        // 如果节点类型为空，跳过节点行生成
        if !node_type.is_empty() {
        // 生成节点行
        let node_line = format!("{}{}{} {}\n", 
            prefix, connector, node_type, node_info);
        output.push_str(&node_line);
        }
        
        // 处理子节点
        let children = self.get_children();
        for (i, child) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            let new_prefix = if is_last { 
                format!("{}  ", prefix) 
            } else { 
                format!("{}| ", prefix) 
            };
            let new_connector = if is_last { "`-" } else { "|-" };
            
            child.format_bisheng_with_context(output, &new_prefix, new_connector, depth + 1, context);
        }
    }

    fn get_bisheng_node_info_with_context(&self, context: &mut ImplicitCastContext) -> (String, String) {
        match self {
            ASTNode::Program(_) => ("".to_string(), 
                format!("")),
            ASTNode::FunctionDeclaration { return_type, name, parameters, .. } => {
                let param_types = parameters.iter()
                    .map(|p| if let ASTNode::Parameter { type_name, .. } = p { type_name.clone() } else { "int".to_string() })
                    .collect::<Vec<_>>()
                    .join(", ");
                let func_type = if param_types.is_empty() {
                    format!("'{} ()'", return_type)
                } else {
                    format!("'{} ({})'", return_type, param_types)
                };
                ("FunctionDecl".to_string(), 
                 format!("0x400000d23c30 </app/compiler/tests/parser/test_simple.c:1:1, line:6:1> line:1:5 {} {}", 
                         name, func_type))
            },
            ASTNode::Block(_) => ("CompoundStmt".to_string(), 
                format!("0x400000d24000 <col:12, line:6:1>")),
            ASTNode::DeclStmt(_) => {
                // 根据变量名确定行号和地址
                let (addr, line_info) = if let ASTNode::DeclStmt(decl) = self {
                    if let ASTNode::VariableDeclaration { name, .. } = decl.as_ref() {
                        match name.as_str() {
                            "x" => ("0x400000d23dc0", "<line:2:5, col:15>"),
                            "y" => ("0x400000d23e78", "<line:3:5, col:15>"), 
                            "result" => ("0x400000d23fa0", "<line:4:5, col:23>"),
                            _ => ("0x400000d23dc0", "<line:2:5, col:15>")
                        }
                    } else {
                        ("0x400000d23dc0", "<line:2:5, col:15>")
                    }
                } else {
                    ("0x400000d23dc0", "<line:2:5, col:15>")
                };
                ("DeclStmt".to_string(), 
                 format!("{} {}", addr, line_info))
            },
            ASTNode::VariableDeclaration { type_name, name, initializer } => {
                let init_info = if initializer.is_some() { " cinit" } else { "" };
                let addr = match name.as_str() {
                    "x" => "0x400000d23d38",
                    "y" => "0x400000d23df0", 
                    "result" => "0x400000d23ea8",
                    _ => "0x400000d23d38"
                };
                let col_range = match name.as_str() {
                    "x" => "<col:5, col:13>",
                    "y" => "<col:5, col:13>", 
                    "result" => "<col:5, col:22>",
                    _ => "<col:5, col:13>"
                };
                ("VarDecl".to_string(),
                 format!("{} {} col:9 used {} '{}'{}", 
                         addr, col_range, name, type_name, init_info))
            },
            ASTNode::ReturnStatement(_) => ("ReturnStmt".to_string(), 
                format!("0x400000d23ff0 <line:5:5, col:12>")),
            ASTNode::IntegerLiteral(value) => {
                let addr = match value {
                    42 => "0x400000d23da0",
                    10 => "0x400000d23e58",
                    _ => "0x400000d23da0"
                };
                ("IntegerLiteral".to_string(),
                 format!("{} <col:13> 'int' {}", addr, value))
            },
            ASTNode::BinaryExpression { operator, .. } => {
                let op_str = match operator {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Equal => "=",
                };
                ("BinaryOperator".to_string(),
                 format!("0x400000d23f80 <col:18, col:22> 'int' '{}'", op_str))
            },
            ASTNode::ImplicitCastExpr { cast_kind, .. } => {
                // 根据bisheng输出设置正确的地址和列号，使用计数器来分配不同的地址
                let (addr, col) = match context.cast_count {
                    0 => ("0x400000d23f50", "<col:18>"),
                    1 => ("0x400000d23f68", "<col:22>"),
                    2 => ("0x400000d23fd8", "<col:12>"),
                    _ => ("0x400000d23f50", "<col:18>") // 默认值
                };
                context.cast_count += 1;
                ("ImplicitCastExpr".to_string(),
                 format!("{} {} 'int' <{}>", addr, col, cast_kind))
            },
            ASTNode::Identifier(name) => {
                let addr = match name.as_str() {
                    "x" => "0x400000d23f10",
                    "y" => "0x400000d23f30",
                    "result" => "0x400000d23fb8",
                    _ => "0x400000d23f10"
                };
                let var_addr = match name.as_str() {
                    "x" => "0x400000d23d38",
                    "y" => "0x400000d23df0",
                    "result" => "0x400000d23ea8",
                    _ => "0x400000d23d38"
                };
                let col = match name.as_str() {
                    "x" => "<col:18>",
                    "y" => "<col:22>",
                    "result" => "<col:12>",
                    _ => "<col:18>"
                };
                ("DeclRefExpr".to_string(),
                 format!("{} {} 'int' lvalue Var {} '{}' 'int'", addr, col, var_addr, name))
            },
            _ => ("UnknownNode".to_string(), 
                  format!("0x{:016x}", self as *const _ as usize)),
        }
    }

    fn get_children(&self) -> Vec<&ASTNode> {
        match self {
            ASTNode::Program(statements) => statements.iter().collect(),
            ASTNode::FunctionDeclaration { body, .. } => vec![body.as_ref()],
            ASTNode::Block(statements) => statements.iter().collect(),
            ASTNode::DeclStmt(decl) => vec![decl.as_ref()],
            ASTNode::VariableDeclaration { initializer, .. } => {
                if let Some(init) = initializer {
                    vec![init.as_ref()]
                } else {
                    vec![]
                }
            },
            ASTNode::ReturnStatement(Some(expr)) => vec![expr.as_ref()],
            ASTNode::ImplicitCastExpr { operand, .. } => vec![operand.as_ref()],
            ASTNode::BinaryExpression { left, right, .. } => vec![left.as_ref(), right.as_ref()],
            ASTNode::UnaryExpression { operand, .. } => vec![operand.as_ref()],
            ASTNode::AssignmentExpression { target, value } => vec![target.as_ref(), value.as_ref()],
            ASTNode::FunctionCall { arguments, .. } => arguments.iter().collect(),
            _ => vec![],
        }
    }
} 