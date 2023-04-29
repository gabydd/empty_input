use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{de, Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = AddTest::register();
            _ = AddTestString::register();
        }

    } else {
    }
}

use std::{fmt::Display, str::FromStr};
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FromStrOption<T: FromStr>(Option<T>);

impl<T: FromStr> FromStr for FromStrOption<T> {
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            Ok(FromStrOption(None))
        } else {
            T::from_str(value).map(|val| FromStrOption(Some(val)))
        }
    }

    type Err = T::Err;
}

impl<'de, T: FromStr> Deserialize<'de> for FromStrOption<T>
where
    T::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

#[server(AddTest, "/api")]
pub async fn add_test(title: String, option: FromStrOption<u16>) -> Result<(), ServerFnError> {
    Ok(())
}
#[server(AddTestString, "/api")]
pub async fn add_test_string(title: String, option: String) -> Result<(), ServerFnError> {
    Ok(())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Title text="Test"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <Test/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Test(cx: Scope) -> impl IntoView {
    let add_test_mutli = create_server_multi_action::<AddTest>(cx);
    let add_test = create_server_action::<AddTest>(cx);
    let add_test_string_mutli = create_server_multi_action::<AddTestString>(cx);
    let add_test_string = create_server_action::<AddTestString>(cx);
    view! { cx,
        <div>
            <h2>"Optional number"</h2>
            <MultiActionForm action=add_test_mutli>
                <label>
                    "Name"
                    <input type="text" name="title" />
                </label>
                <label>
                    "Optional Number"
                    <input type="number" name="option" />
                </label>
                <button type="submit">"Submit MultiAction"</button>
            </MultiActionForm>
            <ActionForm action=add_test>
                <label>
                    "Name"
                    <input type="text" name="title" />
                </label>
                <label>
                    "Optional Number"
                    <input type="number" name="option" />
                </label>
                <button type="submit">"Submit Action"</button>
            </ActionForm>
        </div>
        <div>
            <h2>"Just strings"</h2>
            <MultiActionForm action=add_test_string_mutli>
                <label>
                    "Name"
                    <input type="text" name="title" />
                </label>
                <label>
                    "Try empty string"
                    <input type="text" name="option" />
                </label>
                <button type="submit">"Submit MultiAction"</button>
            </MultiActionForm>
            <ActionForm action=add_test_string>
                <label>
                    "Name"
                    <input type="text" name="title" />
                </label>
                <label>
                    "Try empty string"
                    <input type="text" name="option" />
                </label>
                <button type="submit">"Submit Action"</button>
            </ActionForm>
        </div>
    }
}
