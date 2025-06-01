use kalosm::language::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let model = Llama::builder().with_source(LlamaSource::tiny_llama_1_1b_chat()).build().await?;
    let model = Llama::builder().with_source(LlamaSource::deepseek_r1_distill_qwen_1_5b()).build().await?;
    /*let model = Llama::builder().with_source(LlamaSource::new(FileSource::HuggingFace {
        model_id: String::from("Qwen/Qwen3-0.6B"),
        revision: String::from("main"),
        file: String::from("qwen3-0.6b"),
    })).build().await?;*/
    

    // New code
    let mut chat = model
        .chat();
        //.with_system_prompt("The assistant will act like a pirate");

    loop {
        chat(&prompt_input("\n> ")?).to_std_out().await?;
    }
}