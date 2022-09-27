use yew::prelude::*;
use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use js_sys::JsString;

fn main() {
    yew::start_app::<Login>();
}

#[function_component(Login)]
fn login() -> Html{
    let input_id = NodeRef::default();
    let input_id_clone = input_id.clone();

    let input_pw = NodeRef::default();
    let input_pw_clone = input_pw.clone();

    let login_state = use_state(|| None);
    let login_state_clone = login_state.clone();

    let onclick = Callback::from(move |mouse_event:MouseEvent| {
        let id = input_id.cast::<HtmlInputElement>().unwrap().value();
        let pw = input_pw.cast::<HtmlInputElement>().unwrap().value();

        // let login_flag = use_state(|| None);
        // let login_flag_clone = login_flag.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let backend = format!("http://host.docker.internal:8081/login?id={id}&pw={pw}");
            let backend_response = Request::get(&backend).send().await.unwrap().text().await.unwrap();
            web_sys::console::log_1(&JsString::from(backend_response.clone()));
            if backend_response == String::from("welcome!") {
                // login_flag.set(Some(1));
                web_sys::console::log_1(&"login success!!!".into());
            }
            else {
                // login_flag.set(Some(0));
                web_sys::console::log_1(&"login fail...".into());            
            }
        });

        // match *login_flag_clone {
        //     Some(v) => {
        //         login_state.set(Some(v));
                
        //     }
        //     None => {
        //         login_state.set(Some(0));
        //     }
        // }
    });

    html!{
        <div>
            <p>{"id"}</p>
            <input ref = {input_id_clone} type="text"/>
            <p>{"password"}</p>
            <input ref = {input_pw_clone} type="text"/>
            <button {onclick}>{"login"}</button>
            <LoginResult login_state={*login_state_clone} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LoginResultProps {
    login_state : Option<i32>
}

#[function_component(LoginResult)]
fn login_result(props : &LoginResultProps) -> Html{
    let result = match &props.login_state {
        Some(v) => v,
        None => return html!{}
    };
    
    html!{
        <div>{result}</div>
    }
}
