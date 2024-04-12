use std::net::SocketAddr;
use swimos::server::ServerHandle;
use tokio::select;
use tokio::sync::oneshot;

pub async fn manage_handle(handle: ServerHandle) {
    manage_handle_report(handle, None).await
}

async fn manage_handle_report(
    mut handle: ServerHandle,
    bound: Option<oneshot::Sender<SocketAddr>>,
) {
    let mut shutdown_hook = Box::pin(async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to register interrupt handler.");
    });
    let print_addr = handle.bound_addr();

    let maybe_addr = select! {
        _ = &mut shutdown_hook => None,
        maybe_addr = print_addr => maybe_addr,
    };

    if let Some(addr) = maybe_addr {
        if let Some(tx) = bound {
            let _ = tx.send(addr);
        }
        println!("Bound to: {}", addr);
        shutdown_hook.await;
    }

    println!("Stopping server.");
    handle.stop();
}
