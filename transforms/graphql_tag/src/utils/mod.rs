// libs
use swc_common::{BytePos, FileName::Anon, SourceFile};
use swc_core::atoms::Atom;
use swc_ecma_ast::*;
use swc_ecma_parser::parse_file_as_expr;

const SOURCE: &str = "(definitions) => {
  const names = {};
  return definitions.filter(definition => {
    if (definition.kind !== 'FragmentDefinition') {
      return true;
    }
    const name = definition.name.value;
    if (names[name]) {
      return false;
    } else {
      names[name] = true;
      return true;
    }
  });
}";

pub fn add_unique_fn_to_program(program: &mut Program, unique_fn_name: String) {
    let source_file = SourceFile::new(Anon, false, Anon, SOURCE.into(), BytePos(1));

    let expr_result = parse_file_as_expr(
        &source_file,
        Default::default(),
        Default::default(),
        Default::default(),
        &mut vec![],
    )
    .expect("failed to create unique function");

    match program {
        Program::Module(program) => program.body.insert(
            1,
            ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                span: program.span,
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![VarDeclarator {
                    span: program.span,
                    name: Pat::Ident(Ident::new(Atom::from(unique_fn_name), program.span).into()),
                    init: Some(expr_result),
                    definite: true,
                }],
            })))),
        ),
        Program::Script(program) => program.body.insert(
            1,
            Stmt::Decl(Decl::Var(Box::new(VarDecl {
                span: program.span,
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![VarDeclarator {
                    span: program.span,
                    name: Pat::Ident(Ident::new(Atom::from(unique_fn_name), program.span).into()),
                    init: Some(expr_result),
                    definite: true,
                }],
            }))),
        ),
    }
}
