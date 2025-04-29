use readline::{Event, Readline};
use std::path::Path;
use tokio::fs;

mod fakestdin;
use fakestdin::FakeStdin;

#[tokio::test]
async fn test_simple() {
    let input = b"test command -r test\rthis is the second command -m 123\r\x1B\x01\x41\r\x03";
    let fake_stdin = FakeStdin::new(input);

    let rl = Readline::new(
        fake_stdin,
        "simple > ",
        Some(Path::new(".readline_test_history")),
    )
    .await;

    assert_eq!(
        rl.run().await.unwrap(),
        Event::Line("test command -r test".to_string())
    );

    assert_eq!(
        rl.run().await.unwrap(),
        Event::Line("this is the second command -m 123".to_string())
    );

    assert_eq!(
        rl.run().await.unwrap(),
        Event::Line("this is the second command -m 123".to_string())
    );

    assert_eq!(rl.run().await.unwrap(), Event::CTRLC);

    std::mem::drop(rl);

    let input = b"\x1B\x01\x41\r";
    let fake_stdin = FakeStdin::new(input);
    let rl = Readline::new(
        fake_stdin,
        "simple > ",
        Some(Path::new(".readline_test_history")),
    )
    .await;

    assert_eq!(
        rl.run().await.unwrap(),
        Event::Line("this is the second command -m 123".to_string())
    );

    fs::remove_file(".readline_test_history").await.unwrap();
}
