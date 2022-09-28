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
    let input_id = NodeRef::default();
    let _input_id = input_id.clone();

    let input_pw = NodeRef::default();
    let _input_pw = input_pw.clone();

    // https://yew.rs/docs/concepts/function-components/pre-defined-hooks#use_state
    let account = use_state(|| Account{id: String::from(""), pw: String::from("")});
    let _account = account.clone();

    // 현재 scope 내 모든 변수들은 아래 클로져로 소유권이 move 됨
    let onclick = Callback::from(move |mouse_event:MouseEvent| {
        account.set(Account{id: input_id.cast::<HtmlInputElement>().unwrap().value(), pw: input_pw.cast::<HtmlInputElement>().unwrap().value()});
    });

    // 따라서 아래에서 현재 scope 내 변수들을 사용하려면 clone한 것을 사용해야 함
    // clone한 변수들에는 앞에 언더바를 붙였음
    html!{
        <div>
            <p>{"id"}</p>
            <input ref = {_input_id} type="text"/>
            <p>{"password"}</p>
            <input ref = {_input_pw} type="text"/>
            <button {onclick}>{"login"}</button>
            // UseStateHandle<> type 변수들에 담겨있는 실제 state 값을 참조하려면 역참조 연산자 *를 사용해야 함
            // UseStateHandle<>은 일종의 smart pointer인 듯 함
            // 그리고 state 값의 소유권은 계속 가지고 있어야 하기 때문에 clone해서 넘긴다
            // 그렇지 않으면 dangling pointer가 될 듯?
            <LoginResult account={(*_account).clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LoginResultProps {
    account: Account
}

#[function_component(LoginResult)]
fn login_result(props : &LoginResultProps) -> Html{
    let account = (&props.account).clone();

    let login_result = use_state(|| false);
    let _login_result = login_result.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let id = account.id;
        let pw = account.pw;
        let backend_url = format!("http://127.0.0.1:8081/login?id={id}&pw={pw}");
        let backend_msg = Request::get(&backend_url).send().await.unwrap().text().await.unwrap();
        
        web_sys::console::log_1(&JsString::from(backend_msg.clone()));
        if backend_msg == String::from("welcome!") {
            login_result.set(true);
        }
        else {
            login_result.set(false);
        }
    });

    let login_result_msg = match *_login_result {
        true => "login success!!!",
        false => "login fail..."
    };

    html!{
        <div>{login_result_msg}</div>
    }
}
