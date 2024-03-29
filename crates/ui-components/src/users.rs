use crate::layout::Layout;
use db::User;
use dioxus::prelude::{*, dioxus_elements::button};

struct Props {
    users: Vec<User>
}

// Take a Vec<User> and create an HTML table.
pub fn users(users: Vec<User>) -> String {
    // Inner function to create our rsx! component
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx!{
            Layout {
                title: "Users Table",
                table {
                    thead {
                        tr {
                            th {"ID"}
                            th {"Email"}
                        }
                    }
                    tbody {
                        cx.props.users.iter().map(|user| rsx!(
                            tr {
                                td {strong {"{user.id}"}}
                                td {"{user.email}"}
                            }
                        ))
                    }
                }
                form {
                    action: "/sign_up",
                    method: "POST",
                    label { r#for: "user_email", "Email:" }
                    input { id: "user_email", name: "email", r#type: "email", required: "true" }
                    button { "Submit" }
                }
            }
        })
    }

    // Construct our component and render it to a string.
    let mut app = VirtualDom::new_with_props(
        app,
        Props {
            users
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}