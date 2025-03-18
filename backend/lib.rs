use ic_cdk::update;
use ic_llm::{ChatMessage, Model};

#[update]
async fn prompt(prompt_str: String) -> String {
    ic_llm::prompt(Model::Llama3_1_8B, prompt_str).await
}

#[update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    ic_llm::chat(Model::Llama3_1_8B, messages).await
}

#[update]
async fn analyze_code(code: String) -> String {
    let prompt_str = format!(
        "Analyze the following Rust code and return one of these responses:
        
        - ✅ **If the code is correct:** Return exactly this: `The code is correct.`
        - ❌ **If there are errors:** Clearly state the error and provide the corrected version.
        - 🔍 **If improvements can be made:** Suggest improvements but keep the original functionality.

        **Rust Code to Analyze:**
        ```rust
        {}
        ```
        
        **Your response must be one of these three formats ONLY:**
        1️⃣ `The code is correct.`
        2️⃣ `Syntax error: <explanation>. Corrected code:\n<corrected code>`
        3️⃣ `Improvement suggestion: <explanation>. Improved code:\n<improved code>`",
        code
    );

    let response = ic_llm::prompt(Model::Llama3_1_8B, prompt_str)
        .await
        .chars()
        .take(1000) // Batasi output maksimal 1000 karakter
        .collect();

    response
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
