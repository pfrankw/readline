use readline::{Event, Readline};

mod fakestdin;
use fakestdin::FakeStdin;

#[tokio::test]
async fn test_arrows_leftright() {
    // \x7E is CANC
    // \x03 is CTRL+C
    // \r is ENTER
    // \x1B\x01\x41 is up arrow
    // \x1B\x01\x42 is down arrow
    // \x1B\x01\x43 is rigth arrow
    // \x1B\x01\x44 is left arrow

    // The test writes a wrong command only to go backward, then forward, CANC the letter and replace it with the correct one.

    let input =
        b"ls -la Deskrop\x1B\x01\x44\x1B\x01\x44\x1B\x01\x44\x1B\x01\x44\x1B\x01\x43\x7Et\r\x03";
    let fake_stdin = FakeStdin::new(input);

    let rl = Readline::new(fake_stdin, "arrows leftright > ", None).await;

    assert_eq!(rl.run().await.unwrap(), Event::Line("ls -la Desktop".to_string()));

    assert_eq!(rl.run().await.unwrap(), Event::CTRLC);
}

