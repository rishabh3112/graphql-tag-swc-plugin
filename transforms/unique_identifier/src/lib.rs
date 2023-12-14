// libs
use swc_ecma_ast::Ident;
use swc_ecma_visit::VisitMut;

pub struct UniqueIdentifierVisitor {
    pub identifier: String,
    pub count: i64,
}

impl UniqueIdentifierVisitor {
    pub fn new() -> Self {
        Self {
            identifier: "unique".into(),
            count: 0,
        }
    }
}

impl VisitMut for UniqueIdentifierVisitor {
    fn visit_mut_ident(&mut self, node: &mut Ident) {
        if node.sym.as_str() == self.identifier {
            self.count = self.count + 1;
        }
    }
}
