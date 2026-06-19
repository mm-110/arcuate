//! Python-specific implementation of `SourceCodeAnalyzer` using `rustpython_parser`.

use crate::domain::entities::definition_kind::DefinitionKind;
use crate::domain::entities::documented_construct::DocumentedConstruct;
use crate::domain::entities::source_file_analysis::SourceFileAnalysis;
use crate::domain::ports::parser_error::ParserError;
use crate::domain::ports::source_code_analyzer::SourceCodeAnalyzer;
use rustpython_parser::{ast, Parse};

pub struct PythonParser;

impl SourceCodeAnalyzer for PythonParser {
    fn supports_extension(&self, ext: &str) -> bool {
        ext == "py"
    }

    fn analyze(&self, source_code: &str) -> Result<SourceFileAnalysis, ParserError> {
        let stmts = ast::Suite::parse(source_code, "<source>")
            .map_err(|e| ParserError::SyntaxError { message: e.to_string(), line: None })?;
        Ok(SourceFileAnalysis {
            top_level_doc: module_doc(&stmts),
            documented_constructs: constructs_from_stmts(&stmts),
        })
    }
}

fn constructs_from_stmts(stmts: &[ast::Stmt]) -> Vec<DocumentedConstruct> {
    stmts.iter().filter_map(construct_from_stmt).collect()
}

fn methods_from_class_body(stmts: &[ast::Stmt]) -> Vec<DocumentedConstruct> {
    stmts.iter().filter_map(method_from_stmt).collect()
}

fn method_from_stmt(stmt: &ast::Stmt) -> Option<DocumentedConstruct> {
    match stmt {
        ast::Stmt::FunctionDef(f) => {
            let name = f.name.as_str();
            if name.starts_with('_') && name != "__init__" {
                return None;
            }
            Some(DocumentedConstruct {
                name: f.name.to_string(),
                kind: DefinitionKind::Function,
                signature: Some(build_signature(&f.name, &f.args, f.returns.as_deref(), false)),
                docstring: module_doc(&f.body),
                source_line_range: None,
                nested_constructs: vec![],
            })
        }
        ast::Stmt::AsyncFunctionDef(f) => {
            let name = f.name.as_str();
            if name.starts_with('_') && name != "__init__" {
                return None;
            }
            Some(DocumentedConstruct {
                name: f.name.to_string(),
                kind: DefinitionKind::Function,
                signature: Some(build_signature(&f.name, &f.args, f.returns.as_deref(), true)),
                docstring: module_doc(&f.body),
                source_line_range: None,
                nested_constructs: vec![],
            })
        }
        _ => None,
    }
}

fn construct_from_stmt(stmt: &ast::Stmt) -> Option<DocumentedConstruct> {
    match stmt {
        ast::Stmt::FunctionDef(f) => {
            if f.name.as_str().starts_with('_') {
                return None;
            }
            Some(DocumentedConstruct {
                name: f.name.to_string(),
                kind: DefinitionKind::Function,
                signature: Some(build_signature(&f.name, &f.args, f.returns.as_deref(), false)),
                docstring: module_doc(&f.body),
                source_line_range: None,
                nested_constructs: vec![],
            })
        }
        ast::Stmt::AsyncFunctionDef(f) => {
            if f.name.as_str().starts_with('_') {
                return None;
            }
            Some(DocumentedConstruct {
                name: f.name.to_string(),
                kind: DefinitionKind::Function,
                signature: Some(build_signature(&f.name, &f.args, f.returns.as_deref(), true)),
                docstring: module_doc(&f.body),
                source_line_range: None,
                nested_constructs: vec![],
            })
        }
        ast::Stmt::ClassDef(c) => Some(DocumentedConstruct {
            name: c.name.to_string(),
            kind: DefinitionKind::Class,
            signature: Some(class_signature(&c.name, &c.bases)),
            docstring: module_doc(&c.body),
            source_line_range: None,
            nested_constructs: methods_from_class_body(&c.body),
        }),
        _ => None,
    }
}

fn module_doc(stmts: &[ast::Stmt]) -> Option<String> {
    if let Some(ast::Stmt::Expr(e)) = stmts.first()
        && let ast::Expr::Constant(c) = e.value.as_ref()
        && let ast::Constant::Str(s) = &c.value
    {
        Some(s.to_string())
    } else {
        None
    }
}

fn build_signature(
    name: &ast::Identifier,
    args: &ast::Arguments,
    returns: Option<&ast::Expr>,
    is_async: bool,
) -> String {
    let params: Vec<String> = args.args.iter()
        .filter(|a| a.def.arg.as_str() != "self")
        .map(|a| {
            let param_name = a.def.arg.to_string();
            match &a.def.annotation {
                Some(ann) => format!("{}: {}", param_name, expr_to_type_str(ann)),
                None => param_name,
            }
        }).collect();
    let return_annotation = returns
        .map(|r| format!(" -> {}", expr_to_type_str(r)))
        .unwrap_or_default();
    let prefix = if is_async { "async def" } else { "def" };
    format!("{prefix} {name}({}){}:", params.join(", "), return_annotation)
}

fn expr_to_type_str(expr: &ast::Expr) -> String {
    match expr {
        ast::Expr::Name(n) => n.id.to_string(),
        ast::Expr::Attribute(a) => format!("{}.{}", expr_to_type_str(&a.value), a.attr),
        ast::Expr::Subscript(s) => format!("{}[{}]", expr_to_type_str(&s.value), expr_to_type_str(&s.slice)),
        ast::Expr::BinOp(b) => format!("{} | {}", expr_to_type_str(&b.left), expr_to_type_str(&b.right)),
        ast::Expr::Tuple(t) => t.elts.iter().map(|e| expr_to_type_str(e)).collect::<Vec<_>>().join(", "),
        ast::Expr::Constant(c) => match &c.value {
            ast::Constant::None => "None".to_string(),
            _ => "_".to_string(),
        },
        _ => "_".to_string(),
    }
}

fn class_signature(name: &ast::Identifier, bases: &[ast::Expr]) -> String {
    if bases.is_empty() {
        format!("class {}:", name)
    } else {
        let base_names: Vec<String> = bases.iter().filter_map(expr_to_name).collect();
        format!("class {}({}):", name, base_names.join(", "))
    }
}

fn expr_to_name(expr: &ast::Expr) -> Option<String> {
    match expr {
        ast::Expr::Name(n) => Some(n.id.to_string()),
        ast::Expr::Attribute(a) => {
            let value = expr_to_name(&a.value)?;
            Some(format!("{}.{}", value, a.attr))
        }
        _ => None,
    }
}
