use fennel_engine::runtime::RuntimeBuilder;

#[tokio::main]
async fn main() {
    let runtime = RuntimeBuilder::new()
        .name("game")
        .dimensions((500, 500))
        .build();
    println!("{}", runtime.unwrap().window.name);
}
