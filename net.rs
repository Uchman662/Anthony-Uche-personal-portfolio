fn main () {
    let evil = make_static(&vec![0; 1 << 20]);
    println!("{:?}", evil);
}

fn helper<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T {
    v
}

fn make_static<'a, T>(imput: &'a T) -> &'a static T {
    let f: fn(_, &'a T) -> &'static T = helper;
    f(&&(), input)
}


fn main () {
    let vec =   vec![0; 1 << 20]
    let evil = make_static(&vec);
    drop(vec);
    println!("{:?}", evil);
}
    use std::process::Command;

    fn main() {
        // Execute a simple command (e.g., 'ls' on Linux/macOS, 'dir' on Windows)
        let output = Command::new("ls") // Or "dir" for Windows
            .arg("-l") // Add arguments to the command
            .output()
            .expect("Failed to execute command");

        println!("Status: {}", output.status);
        println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

        // You can also chain commands or pipe output
        let piped_output = Command::new("sh") // Or "cmd" for Windows
            .arg("-c")
            .arg("echo 'Hello from Rust' | grep 'Hello'")
            .output()
            .expect("Failed to execute piped command");

        println!("Piped output: {}", String::from_utf8_lossy(&piped_output.stdout));


    }