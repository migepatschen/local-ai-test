use kalosm::language::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = Llama::builder().with_source(LlamaSource::tiny_llama_1_1b_chat()).build().await?;

    // New code
    let mut chat = model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");

    loop {
        chat(&prompt_input("\n> ")?).to_std_out().await?;
    }
}