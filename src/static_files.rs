use std::path::PathBuf;

use lifec::{plugins::{Plugin, ThunkContext, AsyncContext}, Component, DenseVecStorage};
use poem::{Route, endpoint::{StaticFilesEndpoint, StaticFileEndpoint}};
use crate::{WebApp, AppHost};


/// Static files plugin that starts a server host on text_attribute `address`
/// and serving files from `work_dir`. URL will be formatted as {address}/{block_name}/index.html. 
/// If index_html is set, then {address}/{block_name} will direct to that file.
#[derive(Default, Clone, Component)]
#[storage(DenseVecStorage)]
pub struct StaticFiles(
    /// work_dir
    String,
    /// block_name
    String,
    // index_html
    Option<String>, 
);

impl WebApp for StaticFiles {
    fn create(context: &mut ThunkContext) -> Self {
        if let Some(work_dir) = context.as_ref().find_text("work_dir") {
            if let Some(index_html) = context.as_ref().find_text("index_html") {
                Self(work_dir,  context.block.block_name.to_string(), Some(index_html))
            } else {
                Self(work_dir, context.block.block_name.to_string(),  None)
            }
        } else {
            Self("".to_string(), context.block.block_name.to_string(), None)
        }
    }

    fn routes(&mut self) -> Route {
        let Self(work_dir, block_name, index_html) = self; 

        let path_prefix = format!("/{block_name}");
        eprintln!("{}", path_prefix);

        if let Some(index_html) = index_html {
            eprintln!("setting index - {}", index_html);

            let file_src = PathBuf::from(&work_dir).join(index_html);

            Route::new().nest(path_prefix, StaticFileEndpoint::new(file_src))
        } else {
            Route::new().nest(
                path_prefix,

                StaticFilesEndpoint::new(
                    work_dir.to_string()
                )
            )
        }
    }
}

impl Plugin<ThunkContext> for StaticFiles {
    fn symbol() -> &'static str {
        "static_files"
    }

    fn description() -> &'static str {
        "Starts a static file server host for file directory specified by `work_dir`"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<AsyncContext> {
        AppHost::<StaticFiles>::call_with_context(context)
    }
}
