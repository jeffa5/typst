use crate::doc::{Frame, FrameItem};

#[tracing::instrument(skip_all)]
pub fn txt(frame: &Frame) -> String {
    render_frame(frame)
}

fn render_frame(frame: &Frame) -> String {
    frame
        .items()
        .map(|(_, item)| render_frame_item(item))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_frame_item(item: &FrameItem) -> String {
    match item {
        FrameItem::Group(g) => return render_frame(&g.frame),
        FrameItem::Text(t) => return t.text.to_string(),
        FrameItem::Shape(_, _) => {}
        FrameItem::Image(_, _, _) => {}
        FrameItem::Meta(_, _) => {}
    }
    String::new()
}
