use lifec::prelude::{Plugin, ThunkContext};
use poem::{Response, Route};

/// Trait that extends the plugin trait so that it can be turned into a route,
/// 
pub trait RoutePlugin : Plugin {
    /// Returns a route for this plugin,
    /// 
    /// Generally, plugins are stateless, but this trait will likely be used in conjunction with the WebApp trait. This means that, there will
    /// be a start-up phase of the app host that gives implementations, the chance to initialize/customize a route.
    /// 
    fn route(context: &ThunkContext) -> Route;

    /// Returns a response from the context,
    /// 
    fn response(context: &mut ThunkContext) -> Response {
        context.take_response().expect("should have a response").into()
    }
}
