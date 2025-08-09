use crate::ir::{Arg, Expr, Fn, InnerType, Lit, Program, Type};
use crate::parser::slt::NavigableSlt;

pub fn validate(program: &Program<'_>, slt: &NavigableSlt<'_, '_>) -> usize {
    let childs = slt.childs();

    let fn_err = program
        .func
        .iter()
        .zip(childs)
        .fold(0, |acc, (f, slt)| acc + validate_fn(f, &slt));

    fn_err
}

fn validate_fn(func: &Fn<'_>, slt: &NavigableSlt<'_, '_>) -> usize {
    let mut err_cpt = 0;

    for expr in &func.body {
        #[allow(clippy::single_match)]
        match expr {
            Expr::FnCall { id, args } => {
                let Some(called_func) = slt.find_func(id) else {
                    error!("cannot find function {id} in this scope");
                    err_cpt += 1;
                    continue;
                };

                let min_args_number = if let Some(variadic) = called_func.variadic {
                    variadic
                } else {
                    called_func.args.len()
                };

                if min_args_number > args.len() {
                    error!("not enough arguments passed to call function {id}");
                    err_cpt += 1;
                    continue;
                }

                for i in 0..min_args_number {
                    let Some(ty) = get_arg_ty(&args[i], slt) else {
                        error!("unable to find the type of this expression");
                        err_cpt += 1;
                        continue;
                    };

                    if ty != called_func.args[i] {
                        error!(
                            "type mismatch for {id} argument number {i} expected `{}` and got `{}`",
                            called_func.args[i], ty
                        );
                        err_cpt += 1;
                    }
                }
            }
            _ => (),
        }
    }

    err_cpt
}

fn get_arg_ty(expr: &Arg<'_>, slt: &NavigableSlt<'_, '_>) -> Option<Type> {
    match expr {
        Arg::Lit(Lit::Int(_)) => Some(Type::Val(InnerType::Int)),
        Arg::Lit(Lit::Str(_)) => Some(Type::Val(InnerType::Str)),
        Arg::Lit(Lit::Bool(_)) => Some(Type::Val(InnerType::Bool)),
        Arg::Id(id) => slt.find_variable(id).map(|var| var.ty),
    }
}
