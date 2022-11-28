mod data_videos;
use data_videos::{VideosList};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let videos = data_videos::get_data();

    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <VideosList videos={videos} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
