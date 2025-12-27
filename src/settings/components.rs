use dioxus::prelude::*;

#[component]
fn Settings() -> Element {
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

    rsx! {

    }
}


#[component]
pub fn SettingsButton() -> Element {
    rsx! {
        div {
            class : "settings",

            button {
                class : "settings__button settings__button--open",
                onclick: move|_| {

                },
                "settings⚙️"
            }
        }
    }
}