use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = AddTest::register();
        }
    }
}

#[server(AddTest, "/api", "Cbor")]
pub async fn add_test(string: String) -> Result<(), ServerFnError> {
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
    view! { cx,
        <h2>"Cbor encoding test"</h2>
        <MultiActionForm action=add_test_mutli>
            <label>
                "String"
                <input type="text" name="string" />
            </label>
            <button type="submit">"Submit MultiAction"</button>
        </MultiActionForm>
        <ActionForm action=add_test>
            <label>
                "String"
                <input type="text" name="string" />
            </label>
            <button type="submit">"Submit Action"</button>
        </ActionForm>
    }
}
