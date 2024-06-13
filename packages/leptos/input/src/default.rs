use leptos::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
)]
pub struct InputClass {}

#[component]
pub fn Input(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| InputClass {}.with_class(class.get()));

    view! {
        <input {..attributes} class=class />
    }
}
