use yew::{function_component, html, Callback, Html};
use crate::common::media_test::MediaTest;
use crate::common::molecules::forms::login_form::LoginForm;
use crate::common::audio_recorder::AudioRecorder;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    html!{
        <div>
            <div class="main">
                <LoginForm />
                //<AudioRecorder on_audio_recorded={on_audio_recorded}/>
            </div>
        </div>
    }
}