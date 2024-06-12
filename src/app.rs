use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/meter-checker.css"/>

        // sets the document title
        <Title text="MeterChecker"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the front page of the application, including the form to look up smart meter details.
#[component]
fn HomePage() -> impl IntoView {
    use crate::octopus::GetConsumption;

    let get_consumption = create_server_action::<GetConsumption>();

    view! {
        <ActionForm action=get_consumption class="get-consumption-form">
            <label for="mpan">MPAN:</label>
            <input type="text" name="mpan" required/>

            <label for="serial">Serial Number:</label>
            <input type="text" name="serial" required/>

            <button type="submit">Get Consumption</button>
        </ActionForm>
    }
}
