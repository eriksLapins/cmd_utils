pub mod functions;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        for arg in args {
            if arg.contains("--echo") {
                functions::echo(&arg);
            }

            if arg.contains("--ls") {
                functions::ls();
            }
        }
    }
}
