use crate::config::LanguageConfig;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use std::path::Path;

pub fn language_option(
    h: &Helper<'_, '_>,
    _r: &Handlebars<'_>,
    ctx: &Context,
    rc: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    trace!("language_option (handlebars helper)");

    let param = h.param(0).and_then(|v| v.value().as_str()).ok_or_else(|| {
        RenderError::new("Param 0 with String type is required for language_option helper.")
    })?;

    let languages = rc.evaluate(ctx, "@root/language_config").and_then(|c| {
        serde_json::value::from_value::<LanguageConfig>(c.as_json().clone())
            .map_err(|_| RenderError::new("Could not decode the JSON data"))
    })?;

    let current_path = rc
        .evaluate(ctx, "@root/path")?
        .as_json()
        .as_str()
        .ok_or_else(|| RenderError::new("Type error for `path`, string expected"))?
        .replace("\"", "");

    let rendered_path = Path::new(&current_path)
        .with_extension("html")
        .to_str()
        .ok_or_else(|| RenderError::new("Path could not be converted to str"))?
        .to_string();

    let path_to_root = rc
        .evaluate(ctx, "@root/path_to_root")?
        .as_json()
        .as_str()
        .ok_or_else(|| RenderError::new("Type error for `path_to_root`, string expected"))?
        .to_string();

    let language = languages
        .0
        .get(param)
        .ok_or_else(|| RenderError::new(format!("Unknown language identifier '{}'", param)))?;

    let mut href = String::new();
    href.push_str(&path_to_root);
    href.push_str("../");
    href.push_str(param);
    href.push_str("/");
    href.push_str(&rendered_path);

    out.write(&format!(
        "<a href=\"{}\"><button role=\"menuitem\" class=\"language\" id=\"light\">",
        href
    ))?;
    out.write(&language.name)?;
    out.write("</button></a>")?;

    Ok(())
}
