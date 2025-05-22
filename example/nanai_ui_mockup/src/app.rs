#![allow(non_snake_case)]

use dioxus::prelude::*;
use nanai_web_ui_set::{command_input_ui, terminal_or_output_ui};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = init_xterm)]
    fn init_xterm(id: &str);
}

pub fn xterm_terminal() -> Element {
    use_effect(|| {
        unsafe {
            init_xterm("terminal");
        }
        ()
    });
    rsx! {
        div { id: "terminal", style: "width:100%;height:300px;" }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn App() -> Element {
    // ターミナル/出力欄切り替え用の状態
    let mut is_terminal = use_signal(|| true);
    let mut terminal_content = use_signal(|| String::from("$ echo hello\nhello"));
    let mut output_content = use_signal(|| String::from("コマンド出力例\n結果: OK"));

    // :コマンドUI用の状態
    let mut command_input = use_signal(|| String::new());
    let mut selected_command = use_signal(|| 0usize);
    let commands = vec![
        ("say".to_string(), "テキストを表示".to_string()),
        ("exit".to_string(), "終了する".to_string()),
        ("help".to_string(), "ヘルプを表示".to_string()),
    ];

    rsx! {
        link { rel: "stylesheet", href: "styles.css" }
        main { class: "container",
            h1 { "UI Mockup: Terminal/Output & :Command" }
            // xterm.jsターミナル
            {xterm_terminal()}
            // ターミナル/出力欄切り替え
            {
                terminal_or_output_ui(
                    *is_terminal.read(),
                    &terminal_content.read(),
                    &output_content.read(),
                    Callback::new(move |_| {
                        let current = *is_terminal.read();
                        is_terminal.set(!current)
                    }),
                )
            }
            // :コマンドUI
            {
                command_input_ui(
                    commands.clone(),
                    *selected_command.read(),
                    command_input.read().clone(),
                    Callback::new(move |evt: Event<FormData>| command_input.set(evt.value())),
                    Callback::new(move |idx| selected_command.set(idx)),
                    Callback::new({
                        let commands = commands.clone();
                        let selected_command = selected_command.clone();
                        move |_| {
                            let idx = *selected_command.read();
                            let cmd = &commands[idx].0;
                            output_content.set(format!("コマンド実行: {}", cmd));
                        }
                    }),
                )
            }
        }
    }
}
