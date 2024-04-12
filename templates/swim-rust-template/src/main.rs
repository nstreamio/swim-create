mod utils;

use std::error::Error;

use crate::utils::manage_handle;
use swimos::server::{Server, ServerBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Starting server...");
    let server = ServerBuilder::with_plane_name("MainPlane")
        .set_bind_addr("0.0.0.0:{{port}}".parse()?)
        .build()
        .await?;
    let (task, handle) = server.run();
    println!("Running server...");

    let shutdown = manage_handle(handle);
    let (_, result) = tokio::join!(shutdown, task);
    println!("Stopping server...");
    result?;
    println!("Server stopped successfully.");
    Ok(())
}
