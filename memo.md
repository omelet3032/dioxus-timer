### 포모도로 30분
**logic**
1. 사용자가 settings를 누르면 Settings{} 컴포넌트(page)로 넘어가게 하기
2. Settings{}에서 사용자가 시간을 선택할 수 있도록 하기 (60,90,120)
3. submit를 누르면 타이머 화면으로 복귀(Timer{})하면서 사용자가 선택한 시간으로 설정된다.(initial_duration)


**todo**

1. 터미널창에는 로그찍으면서 dioxus 어플 화면으로 동시에 확인하는 법
    - 로그 찍는 연습
2. 종료 로직은 별도로 만든다(Quit) 
    이벤트 가로채기
        WindowCloseBehaviour
