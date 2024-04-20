use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,   // 目标语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let api_url = match needs.get("apiUrl") {
        Some(raw_url) => {
            if !raw_url.starts_with("http") {
                format!("https://{raw_url}")
            } else {
                raw_url.to_string()
            }
        }
        None => {
            String::from("https://api.cohere.ai")
        }
    };

    let model = match needs.get("model") {
        Some(raw_model) => {
            raw_model.to_string()
        }
        None => {
            String::from("command-r-plus")
        }
    };

    let mode = match needs.get("model") {
        Some(raw_mode) => {
            raw_mode.to_string()
        }
        None => {
            String::from("1")
        }
    };
    let customize_prompt = match needs.get("customizePrompt") {
        Some(raw_prompt) => {
            raw_prompt.to_string()
        }
        None => {
            String::from("")
        }
    };
    let apikey = needs.get("apiKey");
    let api_url_path = "/v1/chat";
    if apikey.unwrap_or(&&"".to_string()).is_empty() {
        return Err("apiKey is required".into());
    }
    let full_url = format!("{}{}", api_url, api_url_path);
    let auth_header = format!("bearer {}", apikey.unwrap());
    let prompt = generate_prompts(&mode, &customize_prompt);
    let user_prompt = generate_user_prompts(&mode, &text, &from, &to, detect);
    let body = build_request_body(&model, &user_prompt, &prompt);
    let res = client
        .post(&full_url)
        .header("Content-Type", "application/json")
        .header("accept", "application/json")
        .header("Authorization", &auth_header)
        .json(&body)
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        let result = res
        .get("text")?
        .as_str()?
        .to_string();
        Some(result)
    }
    if let Some(result) = parse_result(res) {
        return Ok(Value::String(result));
    } else {
        return Err("Response Parse Error".into());
    }
}

fn build_request_body(model: &str,  text: &str, prompt: &str) -> Value {
    json!({
        "model": model,
        "chat_history": [{"role": "SYSTEM", "message": prompt}],
        "message": text,
        "stream": false,
        "maxTokens": 4000
    })
}

fn generate_prompts(mode: &str, customize_prompt: &str) -> String {
    let translation_prompt = "You are a professional translation engine, please translate the text into a colloquial, professional, elegant and fluent content, without the style of machine translation. You must only translate the text content, never interpret it.";
    let user_prompt = match mode {
        "1" => {
            format!("{}", translation_prompt)
        },
        "2" => {
            format!("{}", "You are a text embellisher, you can only embellish the text, never interpret it.")
        },
        "3" => {
            format!("{}", "You are a text summarizer, you can only summarize the text, never interpret it.")
        },
        _ => {
            format!("{}", customize_prompt)
        }
    };
    user_prompt
}

fn generate_user_prompts(mode: &str, text: &str, from: &str, to: &str, detect: &str) -> String {
    let user_prompt = match mode {
        "1" => {
            format!("from {} Translate into {}: \n {}", from, to, text)
        },
        "2" => {
            format!("from {} Embellish in {}: \n {}", from, detect, text)
        },
        "3" => {
            format!("from {} Summarize in {}: \n {}", from, to, text)
        },
        _ => {
            format!("from {} Translate into {}: \n {}", from, to, text)
        }
    };
    user_prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("Hello", "auto", "zh", "en", needs).unwrap();
        println!("{result:?}");
    }
}
