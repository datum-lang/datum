#[cfg(test)]
mod test {
    #[test]
    fn should_run_hello_world_file() {
        use assert_cmd::Command;

        let mut cmd = Command::cargo_bin("cjc").unwrap();
        cmd.arg("docs/examples/hello-world.cj").unwrap();

        cmd.assert().success().stdout("hello,world\n");
    }

    #[test]
    fn should_run_call_hello_world_file() {
        use assert_cmd::Command;

        let mut cmd = Command::cargo_bin("cjc").unwrap();
        cmd.arg("docs/examples/func-call.cj").unwrap();

        cmd.assert().success().stdout("你好，世界！\n");
    }
}
