use lifec::plugins::{Plugin, ThunkContext};
use poem::{Route, Server, listener::TcpListener};
use tokio::select;

pub trait WebApp {
    /// update context and returns a new instance of self
    fn create(context: &mut ThunkContext) -> Self;

    /// update self an returns routes for the host
    fn routes(&mut self) -> Route;
}

pub struct AppHost<A>(fn(&mut ThunkContext) -> A)
where
    A: WebApp + Send + Sync;

impl<A> Default for AppHost<A> 
where
    A: WebApp + Send + Sync
{
    fn default() -> Self {
        Self(A::create)
    }
}

impl<A> Plugin<ThunkContext> for AppHost<A> 
where
    A: WebApp + Send + Sync
{
    fn symbol() -> &'static str {
        "app_host"
    }

    fn description() -> &'static str {
        "Creates an app host with `address`, w/ routes provided by some type `A` which implements WebApp"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<(tokio::task::JoinHandle<ThunkContext>, tokio::sync::oneshot::Sender<()>)> {
        context.clone().task(|cancel_source| {
            let mut tc = context.clone();
            async {
                let mut app = A::create(&mut tc);

                let app = app.routes();

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
                                        tc.error(|e| {
                                            e.with_text("err", format!("app host error: {}", err));
                                        });
                                    },
                                }
                        }
                        _ = cancel_source => {
                            tc.update_status_only("Cancelling server").await; 
                        }
                    }
                }

                Some(tc)
            }
        })
    }
}