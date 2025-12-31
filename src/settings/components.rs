use dioxus::prelude::*;


#[component]
pub fn SettingsControls() -> Element {

    rsx! {
        /* 
            SetTimer에서 사용자가 시간을 설정후 submit을 누르면 minutes가 전달된
            
         */
        SettingsButton {}
    }
}


#[component]
fn SetTimer(on_close: EventHandler<MouseEvent>) -> Element {
    /*
        설정 화면을 어떻게 구성할 것인가
        시간 설정외에도 추가적인 부분을 고려해야 한다
        그래도 지금은 시간만 생각하자
        디자인은?
        시간 입력
            5분 단위 업다운 and 60분 이상부터는 10분 단위 업다운 120분부터는 30분단위...
        그럼 화면 디자인은 단순해진다
        Settings()는 화면만 실제 로직은 use_setting?
        상태signal를 전달해야 하기 때문에 use_setting을 붙이면 되는건가?
    */

    /* 
        전달할 데이터
        decrease_time
        minutes
        increase_time    
     */
   let mut minutes = use_signal(|| 25);

    // 버튼 클릭 시 로직 (나중에 use_settings로 옮길 수 있습니다)
    let increase_time = move |_| {
        let current = minutes();
        if current < 60 {
            minutes.set(current + 5);
        } else if current < 120 {
            minutes.set(current + 10);
        } else {
            minutes.set(current + 30);
        }
    };

    let decrease_time = move |_| {
        let current = minutes();
        if current <= 5 { return; } // 0분 이하 방지
        
        if current <= 60 {
            minutes.set(current - 5);
        } else if current <= 120 {
            minutes.set(current - 10);
        } else {
            minutes.set(current - 30);
        }
    };

    rsx! {
        div {
            class: "settings-card",
            
            div { 
                class: "time-setter",

                button {
                    class: "step-btn",
                    onclick: decrease_time,
                    "−"
                }
                input {
                    class: "time-input",
                    r#type: "number",
                    value: "{minutes}",
                    readonly: true, // 버튼으로만 조작하게 설정
                }
                span { 
                    class: "unit", "min" 
                }
                
                button {
                    class: "step-btn",
                    onclick: increase_time,
                    "+"
                }
            }

            div {
                class : "submit-exit",

                button {
                    class : "submit-btn",
                    // onclick: minutes,
                    /* 
                        사용자가 submit을 누르면
                        설정된 시간(minutes)가 lib.rs의 pub fn DioxusTimer() -> Element {
                        initial_duration으로 전달되어야 한다.
                     */
                    "submit",
                }

                button {
                    class : "exit-btn",
                    // onclick:
                    "exit",
                }


            }
        }
    }
}

#[component]
pub fn SettingsButton() -> Element {

    let mut is_open = use_signal(|| false);

    rsx! {
        div {
            class : "settings",

            button {
                class : "settings__button settings__button--open",
                onclick: move|_| {
                 is_open.set(true);
                },
                "settings⚙️"
            }
        }

        if is_open() {
            SetTimer { 
                // 닫기 기능을 위해 상태를 다시 넘겨줄 수도 있습니다.
                on_close: move |_| is_open.set(false) 
            }
        }
    }
}
