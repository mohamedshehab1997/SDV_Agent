use std::io::Write;
use serde_json::Value;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await{
        if let Ok(request) = serde_json::from_str::<Value>(&line) {
            // Process the JSON value
            handle_request(request).await;
        }
    }
}
fn send_response(response: Value) {
    let mut stdout = std::io::stdout();
    // let response_str = serde_json::to_string(&response).unwrap();
    writeln!(stdout,"{}", response).unwrap();
    stdout.flush().unwrap();
}

async fn handle_request(_request: Value) {
    match _request["method"].as_str().unwrap_or_default() {
        "initialize" => {
            let _responce = serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result":{
                    "protocolVersion": "2024-11-05",
                    "capabilities":{"tools":{}},
                    "serverInfo": {"name": "SDV-Tool-Server", "version": "0.1.0"}
                }
            });
            send_response(_responce);
        }
        "tools/list" => {
            let _responce = serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "tools": [
                        {
                            "name": "analyze_android_logger",
                            "description": "Analyzes Android log files and extracts relevant information.",
                            "inputScema": {
                                "type": "object",
                                "properties": {
                                    "logFilePath": {
                                        "type": "string",
                                        "description": "The file path of the Android log file to be analyzed."
                                    }
                                },
                                "required": ["logFilePath"]
                            },
                            "outputScema": {
                                "type": "object",
                                "properties": {
                                    "analysisResult": {
                                        "type": "string",
                                        "description": "The result of the log analysis."
                                    }
                                },
                                "required": ["analysisResult"]
                            }
                        }
                    ]
                }
            });
            send_response(_responce);
        }
        "tools/execute" => {
                // let params = &_request["params"];
                // let tool_name = params["toolName"].as_str().unwrap_or_default();
                // let tool_input = &params["input"];
                // // Here you would implement the logic to execute the specified tool with the given input
                // // For demonstration, we will just return a dummy response
                // let _responce = serde_json::json!({
                //     "jsonrpc": "2.0",
                //     "id": 1,
                //     "result": {
                //         "output": {
                //             "analysisResult": format!("Executed tool: {}, with input: {}", tool_name, tool_input)
                //         }
                //     }
                // });
                // send_response(_responce).await;
        }
        _ => {
            println!("Invalid request method");
        }
}
}
