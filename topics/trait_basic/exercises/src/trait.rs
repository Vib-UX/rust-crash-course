// Unlocking Polymorphism in Rust with Traits
#[derive(Debug,PartialEq)]
struct Solidity{
    version: String,
}

struct Vyper{
    version: String,
}

trait Compiler{
    fn compile(&self, file_path: &str) -> String;
}

impl Compiler for Solidity{
    fn compile(&self, file_path: &str) -> String{
        format!("solc {}", file_path)
    }
}

impl Compiler for Vyper{
    fn compile(&self, file_path: &str)-> String{
        format!("vyper {}", file_path)
    }
}

fn compile_contract(lang: &impl Compiler, file_path: &str)-> String{
    return lang.compile(file_path);
}

fn main() {
    // Create instances of our language structs
    let sol = Solidity { version: "0.8.20".to_string() };
    let vy = Vyper { version: "0.3.7".to_string() };

    // Method 1: Calling trait methods directly on instances
    println!("Direct call - Solidity: {}", sol.compile("example.sol"));
    println!("Direct call - Vyper:    {}", vy.compile("example.vy"));

    // Method 2: Passing instances to our generic compile_contract function
    println!("Generic fn - Solidity: {}", compile_contract(&sol, "example.sol"));
    println!("Generic fn - Vyper:    {}", compile_contract(&vy, "example.vy"));
}