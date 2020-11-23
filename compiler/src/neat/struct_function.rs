use crate::neat::Namespace;
use cjc_hir::Parameter;

pub fn struct_function(struct_func_def: &cjc_parser::StructFuncDef, namespace: &mut Namespace) -> bool {
    let mut success = true;

    let _params = resolve_params(&struct_func_def.params, namespace);
    // println!("{:?}", params);

    success
}

pub fn resolve_params(parameters: &Vec<(cjc_lexer::Loc, Option<cjc_parser::Parameter>)>, namespace: &mut Namespace) -> Vec<Parameter> {
    let mut params = Vec::new();
    for (loc, p) in parameters {
        let p = match p {
            Some(p) => p,
            None => {
                continue;
            }
        };

        namespace.resolve_type(&p.ty);

        params.push(Parameter {
            location: *loc,
            name: p.get_name()
        })
    }
    return params;
}