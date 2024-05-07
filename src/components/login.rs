
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };
    
    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="w-full h-full flex justify-center items-center">
            <div class="bg-gray-800 bg-opacity-75 rounded-lg p-8 max-w-md">
                <div class="container mx-auto flex flex-col justify-center items-center">
                    <h1 class="text-4xl font-bold text-white mb-6 text-center">{"Welcome Back"}</h1>
                    <form class="w-full flex flex-col">
                        <input {oninput} class="rounded p-4 mb-4 border border-gray-200 text-gray-800 bg-white" placeholder="Username"/>
                        <Link<Route> to={Route::Chat}>
                            <button {onclick} disabled={username.len()<1} class="px-8 rounded bg-violet-600 text-white font-bold p-4 uppercase border border-violet-600 hover:bg-violet-700 transition duration-300">{"Go Chatting!"}</button>
                        </Link<Route>>
                    </form>
                </div>
            </div>
        </div>
    }
}