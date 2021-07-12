use font_awesome_as_a_crate as fa;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

pub fn fa_helper(
    h: &Helper<'_, '_>,
    _r: &Handlebars<'_>,
    _ctx: &Context,
    _rc: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    trace!("fa_helper (handlebars helper)");

    let type_ = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .and_then(|v| fa::Type::from_str(v).ok())
        .ok_or_else(|| {
            RenderError::new("Param 0 with String type is required for fontawesome helper.")
        })?;

    let name = h.param(1).and_then(|v| v.value().as_str()).ok_or_else(|| {
        RenderError::new("Param 1 with String type is required for fontawesome helper.")
    })?;

    trace!("fa_helper: {} {}", type_, name);

    let name = if name.starts_with("fa-") {
        &name[3..]
    } else if name.starts_with("fab-") {
        &name[4..]
    } else if name.starts_with("fas-") {
        &name[4..]
    } else {
        &name[..]
    };

    if let Some(id) = h.param(2).and_then(|v| v.value().as_str()) {
        out.write(&format!("<span class=fa-svg id=\"{}\">", id))?;
    } else {
        out.write("<span class=fa-svg>")?;
    }
    out.write(
        fa::svg(type_, name).map_err(|_| RenderError::new(format!("Missing font {}", name)))?,
    )?;
    out.write("</span>")?;

    Ok(())
}
