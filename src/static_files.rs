use lifec::{plugins::{Plugin, ThunkContext}, Component, DenseVecStorage};
use poem::{Route, Server, endpoint::StaticFilesEndpoint, listener::TcpListener};
use tokio::{sync::oneshot::Sender, task::JoinHandle, select};


/// Static files plugin that starts a server host on text_attribute `address`
/// and serving files from `work_dir`.
#[derive(Default, Clone, Component)]
#[storage(DenseVecStorage)]
pub struct StaticFiles;

impl Plugin<ThunkContext> for StaticFiles {
    fn symbol() -> &'static str {
        "static_files"
    }

    fn description() -> &'static str {
        "Starts a static file server host for file directory specified by `work_dir`"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<(JoinHandle<ThunkContext>, Sender<()>)> {
        context.clone().task(|cancel_source| {
            let tc = context.clone();
            async {
                if let Some(work_dir) = tc.as_ref().find_text("work_dir") {
                    tc.update_status_only(format!("Serving work_dir {}", work_dir)).await;
                    let app = Route::new().nest(
                        "/",
                        StaticFilesEndpoint::new(
                            work_dir
                        ),
                    );
                    
                    if let Some(address) = tc.as_ref().find_text("address") {
                        tc.update_status_only(format!("Starting {}", address)).await;
                        select! {
                            result = Server::new(
                                TcpListener::bind(address))
                                .run(app) => {
                                    match result {
                                        Ok(_) => {
                                            tc.update_status_only("Server is exiting").await; 
                                        },
                                        Err(err) => {
                                            tc.update_status_only(format!("Server error exit {}", err)).await;
                                        },
                                    }
                            }
                            _ = cancel_source => {
                                tc.update_status_only("Cancelling, server is exiting").await; 
                            }
                        }
                    }
                }

                Some(tc)
            }
        })
    }
}
