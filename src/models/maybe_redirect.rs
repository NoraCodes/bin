use rocket::{response::Redirect, Responder};
use rocket_dyn_templates::Template;

// Allowed LEV because Rocket doesn't let us box Responders like Redirect.
// See https://github.com/SergioBenitez/Rocket/issues/568
#[allow(clippy::large_enum_variant)]
#[derive(Responder)]
pub enum MaybeRedirect {
    Redirect(Redirect),
    Template(Template),
}

impl From<Redirect> for MaybeRedirect {
    fn from(other: Redirect) -> Self {
        Self::Redirect(other)
    }
}

impl From<Template> for MaybeRedirect {
    fn from(other: Template) -> Self {
        Self::Template(other)
    }
}
