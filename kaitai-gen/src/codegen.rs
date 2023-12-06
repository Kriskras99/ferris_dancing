use std::collections::HashMap;

use anyhow::Error;
use codegen::Scope;

use crate::ksy::{Identifier, Ksy, Import};

pub fn codegen(ksys: HashMap<Identifier, Ksy>) -> Result<Scope, Error> {
    let mut scope = Scope::new();

    for (id, ksy) in ksys {
        // Root metadata needs to be available!
        let meta = ksy.meta.expect("Invalid .ksy: has no MetaSpec!");
        let module = scope.new_module(id.as_ref());
        for import in meta.imports.unwrap_or_default() {
            let Import::Identifier(import_id) = import else { unreachable!("This should be a Identifier at this point!")};
            module.import(import_id.as_ref(), &import_id.to_pascal_case());
        }
        let main_type = module.new_struct(id.as_ref());
        // main_type.
        for attribute in ksy.seq.unwrap_or_default() {
            // let id = 
        }

    }

    Ok(scope)
}
