mod video;
mod data;
pub use video::Video;
pub use data::get_data;
pub use video::VideosListProps;
use yew::{function_component, html};

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos.iter().map(|video| html! {
        <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect()
}