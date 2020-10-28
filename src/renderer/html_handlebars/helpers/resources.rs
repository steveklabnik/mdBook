use std::collections::HashMap;

use crate::utils;

use handlebars::{Context, Handlebars, Helper, HelperDef, Output, RenderContext, RenderError};

// Handlebars helper to find filenames with hashes in them
#[derive(Clone)]
pub struct ResourceHelper {
    pub hash_map: HashMap<String, String>,
}

impl HelperDef for ResourceHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'_>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        let param = h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
            RenderError::new("Param 0 with String type is required for theme_option helper.")
        })?;

        let base_path = rc
            .evaluate(ctx, "@root/path")?
            .as_json()
            .as_str()
            .ok_or_else(|| RenderError::new("Type error for `path`, string expected"))?
            .replace("\"", "");

        let path_to_root = utils::fs::path_to_root(&base_path);

        out.write(&path_to_root)?;
        out.write(
            self.hash_map
                .get(&param[..])
                .map(|p| &p[..])
                .unwrap_or(&param),
        )?;
        Ok(())
    }
}
