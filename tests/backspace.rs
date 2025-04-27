use readline::Readline;

mod fakestdin;
use fakestdin::FakeStdin;

#[tokio::test]
async fn test_backspace() {
    // \x7E is CANC
    // \x7F is Backspace
    // \x03 is CTRL+C
    // \r is ENTER
    // \x1B\x01\x41 is up arrow
    // \x1B\x01\x42 is down arrow
    // \x1B\x01\x43 is rigth arrow
    // \x1B\x01\x44 is left arrow

    // The test writes a wrong command only to delete a part of it and replace with the correct string.

    let input = b"ls -la Deskrop\x7F\x7F\x7Ftop\x7F\x7F\x7F\x7F\x7F\x7F\x7FDownloads\r\x03";
    let fake_stdin = FakeStdin::new(input);

    let rl = Readline::new(Some(Box::new(fake_stdin)), "arrows > ", None).await;

    assert_eq!(rl.run().await.unwrap(), "ls -la Downloads");

    assert!(rl.run().await.is_err());
}

