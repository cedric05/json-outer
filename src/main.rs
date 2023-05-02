use std::process::Stdio;

use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncRead};

enum StreamType {
    Stdout,
    Stderr,
}

impl StreamType {
    fn key(&self) -> &str {
        match self {
            StreamType::Stdout => "stdout",
            StreamType::Stderr => "stderr",
        }
    }
}

async fn clean_read_print<R>(r: R, stream: StreamType) -> Result<(), Box<dyn std::error::Error>>
where
    R: AsyncRead + Unpin,
{
    let mut r = tokio::io::BufReader::new(r);
    let stream = stream.key();
    loop {
        let mut s = String::new();
        r.read_line(&mut s).await?;
        let s = s.strip_suffix("\n").unwrap_or_default();
        if !s.is_empty() {
            let s = json!({
                "log": s,
                "stream": stream,
                "time": chrono::Local::now().to_rfc3339()
            });
            println!("{}", s.to_string());
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).expect("path of executable is not provided");
    let args: Vec<String> = args.collect();
    let command = tokio::process::Command::new(path)
        .args(&args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .stdin(Stdio::inherit())
        .spawn()
        .unwrap();
    let stderr = command.stderr.unwrap();
    let stdout = command.stdout.unwrap();

    let (_, _) = tokio::join!(
        tokio::spawn(async move {
            clean_read_print(stdout, StreamType::Stdout)
                .await
                .unwrap_or_default()
        }),
        tokio::spawn(async move {
            clean_read_print(stderr, StreamType::Stderr)
                .await
                .unwrap_or_default()
        })
    );
}
