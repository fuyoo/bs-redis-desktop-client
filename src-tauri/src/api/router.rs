use std::future::Future;
use std::pin::Pin;
use crate::api::resp::Response;
use tauri::{Result};
use crate::r_error;

pub trait RouteHandle<T:serde::Serialize> {
    
    handle: Pin<Box<dyn Future<Output = Result<Response<T>>>>>,

    fn route(&self) -> impl std::future::Future<Output = String> + Send {
        async {
           match self.handle.await {
               Ok(r) => r.to_string(),
               Err(e) => r_error!(e).to_string(),
           }
       }
   }
}

pub struct Route< T: serde::Serialize> {
   pub path: &'static str,
   handle: Pin<Box<dyn Future<Output = Result<Response<T>>>>>,
}

impl<T:  serde::Serialize> RouteHandle<T> for Route<T> {}

pub struct Router<T: serde::Serialize>(Vec<Route<T>>);

impl<T: serde::Serialize> Router<T> {
   pub fn add(&mut self, path: &'static str, f: impl Future<Output = Result<Response<T>>> + Send + 'static ) {
        self.0.push(
            Route{
                path,
                handle:Box::pin(f)
            }
        );
   }
}
