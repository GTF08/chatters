use uuid::Uuid;
use yew::{function_component, html, use_callback, use_state_eq, Callback, Component, Html, MouseEvent, Properties};
use crate::common::atoms::{
    buttons::logout_button::LogoutButton, sidebar::{
        body::Body, footer::Footer, header::Header
    }, users::current_user_display::CurrentUserDisplay
};
use crate::common::organisms::sidebar::Sidebar;
use crate::common::atoms::chats::chats_choice_sidebar::ChatChoiceSidebar;
// use crate::common::molecules::chats::old_messages_box::OldMessagesBox;
// use crate::common::molecules::chats::new_messages_box::NewMessagesBox;
//use crate::common::organisms::chat_body::ChatBody;
use crate::common::organisms::chat::Chat;

#[derive(Properties, PartialEq)]
struct PageState {
    current_chat: Option<Uuid>
}

#[function_component(UserMainPage)]
pub fn user_main_page() -> Html {
    let state = use_state_eq(|| PageState{current_chat : None});
    let callback = Callback::from({
            let state = state.clone();
            move |current_chat: Uuid| {
                state.set(PageState{current_chat: Some(current_chat)});
                gloo::console::log!(current_chat.to_string());
            }
        }
    );

    html!{
        <>
            <Sidebar>
                <Sidebar class="sidebar-chats">
                    <ChatChoiceSidebar chat_choose_callback={callback.clone()}/>
                </Sidebar>
                <Sidebar class = "sidebar-content">
                    <Header></Header>
                    <Body>
                    </Body>
                    <Footer>
                        <CurrentUserDisplay />
                        <LogoutButton />
                    </Footer>
                </Sidebar>
            </Sidebar>
            <div class="main">
                {
                    match state.current_chat {
                        Some(value) => {
                            html!{
                                <>
                                //<Chat current_chat_id={value}/>
                                </>
                            }
                        },
                        None => {
                            html!{}
                        }
                    }
                }
            </div>
        </>
    }
}