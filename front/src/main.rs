use yew::prelude::*;
use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use js_sys::JsString;

fn main() {
    yew::start_app::<Login>();
}

#[derive(PartialEq, Debug, Clone)]
struct Account {
    id : String,
    pw : String    
}

#[function_component(Login)]
fn login() -> Html{
    let input_id_ref = NodeRef::default();
    let input_pw_ref = NodeRef::default();

    // document node reference들은 반드시 clone해서 써야 한다
    let input_id = input_id_ref.clone();
    let input_pw = input_pw_ref.clone();

    // https://yew.rs/docs/concepts/function-components/pre-defined-hooks#use_state
    let login_result = Box::new(use_state(|| None));
    let _login_result = login_result.clone();
    // 현재 scope 내 모든 변수들은 아래 클로져로 소유권이 move 됨
    let onclick = {
        let login_result = login_result.clone();
        Callback::from(move |_| {
            let id = input_id.cast::<HtmlInputElement>().unwrap().value();
            let pw = input_pw.cast::<HtmlInputElement>().unwrap().value();
            let login_result = login_result.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let backend_url = format!("http://host.docker.internal:8081/login?id={id}&pw={pw}");
                let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
                
                web_sys::console::log_1(&JsString::from(backend_msg.clone()));
                if backend_msg == String::from("welcome!") {
                    login_result.set(Some(true));
                }
                else {
                    login_result.set(Some(false));
                }
            });    
        })
    };

    // 따라서 아래에서 현재 scope 내 변수들을 사용하려면 clone한 것을 사용해야 함
    html!{
        <div>
            <p>{"id"}</p>
            <input ref = {input_id_ref} type="text"/>
            <p>{"password"}</p>
            <input ref = {input_pw_ref} type="text"/>
            <button onclick={onclick}>{"login"}</button>
            // UseStateHandle<> type 변수들에 담겨있는 실제 state 값을 참조하려면 역참조 연산자 *를 사용해야 함
            // UseStateHandle<>은 일종의 smart pointer인 듯 함
            // 그리고 state 값의 소유권은 계속 가지고 있어야 하기 때문에 clone해서 넘긴다
            // 그렇지 않으면 dangling pointer가 될 듯?
            <LoginResult login_result={*(_login_result.as_ref().clone())} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LoginResultProps {
    login_result: Option<bool>
}

#[function_component(LoginResult)]
fn login_result(props : &LoginResultProps) -> Html{
    let login_result_msg = match &props.login_result {
        Some(true) => "login success!!!",
        Some(false) => "login fail...",
        None => ""
    };

    html!{
        <div>{login_result_msg}</div>
    }
}
