use crate::{
    components::ui_kit::{
        activity_indicator::ActivityIndicator,
        icon_button::IconButton,
        skeletons::{inline::InlineSkeleton, pfp::PFPSkeleton},
    },
    utils::config::Config,
    Account, STATE,
};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use warp::multipass::identity::Identity;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    on_call: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn TopBar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let state = use_atom_ref(&cx, STATE);
    let config = Config::load_config_or_default();

    // Read their values from locks
    let mp = cx.props.account.clone();

    // todo: move this into the `impl Conversations` by creating an accessor method
    // use the uuid of the current chat to extract the ConversationInfo from the list
    let opt = &state
        .read()
        .current_chat
        .and_then(|conversation_id| state.read().all_chats.get(&conversation_id).cloned());

    match opt {
        Some(conversation_info) => {
            let display_did = conversation_info
                .conversation
                .recipients()
                .last()
                .cloned()
                .unwrap_or_default();

            let display_user = mp
                .read()
                .get_identity(display_did.clone().into())
                .unwrap_or_default();

            let display_username = display_user
                .first()
                .map(Identity::username)
                .unwrap_or_else(String::new);
            // TODO-END

            let id = conversation_info.conversation.id();

            let identity = mp.read().get_own_identity().unwrap();
            let profile_picture = identity.graphics().profile_picture();


            cx.render(rsx! {
                div {
                    class: "topbar",
                    if profile_picture.is_empty() {
                        rsx! (
                            div {
                                class: "pfp"
                            }  
                        )   
                        } else {
                            rsx!(
                                img {
                                    src: "{profile_picture}",
                                    height: "50",
                                    width: "50",
        
                                }
                            )
                        },
                    div {
                        class: "who",
                        div {
                            class: "top-row",
                            h3 {
                                "{display_username}"
                            }
                        },
                        div {
                            class: "user-info-inline",
                            ActivityIndicator {
                                inline: true,
                                remote_did: display_did.clone(),
                                account: cx.props.account.clone(),
                            },
                            p {
                                class: "did",
                                config.developer.developer_mode.then(|| rsx!(
                                    span {
                                        "({id})"
                                    }
                                ))
                            }
                        }
                    },
                    div {
                        class: "controls",
                        IconButton {
                            icon: Shape::Phone,
                            on_pressed: move |_| {
                                cx.props.on_call.call(());
                            },
                        }
                    }
                },
            })
        }
        None => cx.render(rsx! {
            div {
                class: "topbar",
                PFPSkeleton {},
                div {
                    class: "who",
                    div {
                        class: "top-row",
                        InlineSkeleton {}
                    },
                    InlineSkeleton {}
                },
                div {
                    class: "controls",
                    IconButton {
                        icon: Shape::Phone,
                        on_pressed: move |_| {
                            cx.props.on_call.call(());
                        },
                    }
                }
            },
        }),
    }
}
