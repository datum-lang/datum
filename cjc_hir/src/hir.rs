#[derive(Debug)]
pub struct Expr<'hir> {
    pub kind: ExprKind<'hir>,
}

#[derive(Debug)]
pub enum ExprKind<'hir> {
    Call(&'hir Expr<'hir>, &'hir [Expr<'hir>]),
}
