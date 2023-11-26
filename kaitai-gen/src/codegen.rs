use std::collections::HashMap;

use anyhow::Error;
use codegen::Scope;

use crate::ksy::{Identifier, Ksy};

pub fn codegen(ksys: HashMap<Identifier, Ksy>) -> Result<Scope, Error> {
    let mut scope = Scope::new();

    for (id, ksy) in ksys {
        let module = scope.new_module(id.as_ref());
        let _main_type = module.new_struct(id.as_ref());
    }

    Ok(scope)
}
