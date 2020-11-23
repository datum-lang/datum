use crate::neat::Namespace;

pub fn struct_function(struct_func_def: &cjc_parser::StructFuncDef, namespace: &mut Namespace) -> bool {
    let mut success = true;

    parameters(&struct_func_def.params, namespace);

    success
}

pub fn parameters(params: &Vec<(cjc_lexer::Loc, Option<cjc_parser::Parameter>)>, namespace: &mut Namespace) {
    for (loc, p) in params {
        let p = match p {
            Some(p) => p,
            None => {
                continue;
            }
        };

        namespace.resolve_type(&p.ty);
    }
}