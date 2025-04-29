use readline::{Event, Readline};

mod fakestdin;
use fakestdin::FakeStdin;

#[tokio::test]
async fn test_arrows_updown() {
    // \x7E is CANC
    // \x03 is CTRL+C
    // \r is ENTER
    // \x1B\x01\x41 is up arrow
    // \x1B\x01\x42 is down arrow
    // \x1B\x01\x43 is rigth arrow
    // \x1B\x01\x44 is left arrow

    let input = b"test command -r one\rnot the previous command\r\x1B\x01\x41\x1B\x01\x41\x1B\x01\x42\r\x03";
    let fake_stdin = FakeStdin::new(input);

    let rl = Readline::new(fake_stdin, "arrows updown > ", None).await;

    assert_eq!(
        rl.run().await.unwrap(),
        Event::Line("test command -r one".to_string())
    );

    assert_eq!(
        rl.run().await.unwrap(),
        Event::Line("not the previous command".to_string())
    );

    assert_eq!(
        rl.run().await.unwrap(),
        Event::Line("not the previous command".to_string())
    );

    assert_eq!(rl.run().await.unwrap(), Event::CTRLC);
}
