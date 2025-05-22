use dioxus::prelude::*;

/// ターミナル/出力欄切り替えUI部品
pub fn terminal_or_output_ui(
    is_terminal: bool,
    terminal_content: &str,
    output_content: &str,
    on_toggle: Callback<Event<MouseData>>,
) -> Element {
    rsx! {
        div {
            button { onclick: on_toggle,
                if is_terminal {
                    "出力欄に切替"
                } else {
                    "ターミナルに切替"
                }
            }
            if is_terminal {
                div { class: "terminal-mock",
                    pre { "{terminal_content}" }
                }
            } else {
                div { class: "output-mock",
                    pre { "{output_content}" }
                }
            }
        }
    }
}

/// :コマンド形式入力＋補足説明UI部品
pub fn command_input_ui(
    commands: Vec<(String, String)>, // (コマンド, 補足説明)
    selected: usize,
    input: String,
    on_input: Callback<Event<FormData>>,
    on_select: Callback<usize>,
    on_submit: Callback<()>,
) -> Element {
    rsx! {
        div {
            input {
                r#type: "text",
                value: "{input}",
                oninput: on_input,
                onkeydown: {
                    let on_select = on_select.clone();
                    let on_submit = on_submit.clone();
                    move |e: KeyboardEvent| {
                        match e.key() {
                            Key::ArrowUp => {
                                if selected > 0 {
                                    on_select.call(selected - 1);
                                }
                            }
                            Key::ArrowDown => {
                                if selected + 1 < commands.len() {
                                    on_select.call(selected + 1);
                                }
                            }
                            Key::Enter => on_submit.call(()),
                            _ => {}
                        }
                    }
                },
                placeholder: ":コマンド...",
            }
            ul {
                for (i , (cmd , desc)) in commands.iter().enumerate() {
                    li { class: if i == selected { "selected" } else { "" },
                        "{cmd} - "
                        if i == selected {
                            span { class: "desc", "{desc}" }
                        } else {
                            span { class: "desc", "{desc.chars().next().unwrap_or(' ')}" }
                        }
                    }
                }
            }
        }
    }
}
