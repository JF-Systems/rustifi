use futures::StreamExt;
use rustifi::protect::ProtectClient;

#[tokio::main]
async fn main() -> rustifi::Result<()> {
    let base_url = std::env::var("PROTECT_URL").expect("PROTECT_URL not set");
    let api_key = std::env::var("PROTECT_API_KEY").expect("PROTECT_API_KEY not set");

    // Local consoles usually have self-signed certificates; only use the
    // insecure constructor on trusted networks.
    let client = ProtectClient::new_insecure(base_url, api_key)?;

    let info = client.meta_info().await?;
    println!("Protect version: {}", info.application_version);

    let cameras = client.cameras().await?;
    for camera in &cameras {
        println!("Camera: {} [{}] ({:?})", camera.name, camera.id, camera.state);
    }

    if let Some(camera) = cameras.first() {
        let jpeg = client.snapshot(&camera.id, false).await?;
        let path = format!("{}.jpg", camera.id);
        std::fs::write(&path, jpeg).expect("failed to write snapshot");
        println!("Saved snapshot of {} to {}", camera.name, path);
    }

    println!("Tailing live events (Ctrl-C to quit)...");
    loop {
        let mut stream = match client.subscribe_events().await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("subscription failed: {e}; retrying in 5s");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        };
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(event) => println!(
                    "[{:?}] {:?} on {} (smart: {:?})",
                    event.action, event.item.event_type, event.item.device,
                    event.item.smart_detect_types
                ),
                Err(e) => eprintln!("stream error: {e}"),
            }
        }
        eprintln!("disconnected; reconnecting in 5s (events in between are lost)");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
