use serde_json::{Value, json};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,  // 目标语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    _needs: HashMap<String, String>,// 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {

    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let  apikey = _needs.get("apiKey");
    let  model = _needs.get("model");
    let  mode = _needs.get("mode");
    let customize_prompt = _needs.get("customizePrompt");
    let  api_url = _needs.get("apiUrl");
    let api_url_path = "/v1/chat";
    if apikey.unwrap_or(&"".to_string()).is_empty() {
        return Err("apiKey is required".into());
    }
    if model.unwrap_or(&"".to_string()).is_empty() {
        model = Some(&"command-r-plus".to_string());
    }
    if mode.unwrap_or(&"".to_string()).is_empty() {
        mode = Some(&"1".to_string());
    }
    if api_url.unwrap_or(&"".to_string()).is_empty() {
        api_url = Some(&"https://api.cohere.ai".to_string());
    }
    let full_url = format!("{}{}", api_url.unwrap(), api_url_path);
    let auth_header = format!("bearer {}", apikey.unwrap());
    let body = build_request_body(model.unwrap(), mode.unwrap(), customize_prompt.unwrap(), text);
    let res = client
        .post(&full_url)
        .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 16_3_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/8.0.32(0x18002035) NetType/WIFI Language/zh_TW")
        .header("Content-Type", "application/json")
        .header("accept", "application/json")
        .header("Authorization", &auth_header)
        .json(&body)
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        let result = res
        .as_object()?
        .get("cohereResp")?
        .as_object()?
        .get("Text")?
        .as_str()?;
        Some(result.to_string())
    }
    if let Some(result) = parse_result(res) {
        return Ok(Value::String(result));
    } else {
        return Err("Response Parse Error".into());
    }
}

fn build_request_body(model: &str, mode: &str, customize_prompt: &str, text: &str) -> Value {
    let prompt = generate_prompts(mode, customize_prompt, text);
    json!({
        "model": model,
        "chat_history": [{"role": "SYSTEM", "message": prompt}],
        "message": text,
        "stream": false,
        "maxTokens": 4000
    })
}

fn generate_prompts(mode: &str, customize_prompt: &str, text: &str) -> String {
    let translation_prompt = "You are a professional translation engine, please translate the text into a colloquial, professional, elegant and fluent content, without the style of machine translation. You must only translate the text content, never interpret it.";
    let user_prompt = match mode {
        "1" => translation_prompt,
        "2" => "Please polish this sentence without changing its original meaning",
        "3" => "Please answer the following question",
         _ => {
            if customize_prompt.is_empty() {
                translation_prompt
            } else {
                customize_prompt
            }
        }
    };
    user_prompt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("Hello", "auto", "zh", "en", needs);
        println!("{result:?}");
    }
}
