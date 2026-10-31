use super::*;

pub(super) struct SplitViewFixture {
    right: bool,
}

#[derive(Clone)]
pub(super) enum Message {
    Toggle,
}


impl Component for SplitViewFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { right: false }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Toggle => self.right = !self.right,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let placement = if self.right {
            SplitViewPanePlacement::Right
        } else {
            SplitViewPanePlacement::Left
        };
        context.window_title("Reactor UI fixture: split-view");
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                SplitView::new()
                    .display_mode(SplitViewDisplayMode::Inline)
                    .is_pane_open(true)
                    .open_pane_length(160.0)
                    .pane_placement(placement)
                    .pane(TextBlock::new().automation_id("pane").text("Pane"))
                    .content(TextBlock::new().automation_id("content").text("Content")),
                Button::new()
                    .automation_id("toggle")
                    .content("Toggle")
                    .on_click(context.message(Message::Toggle)),
            ))
            .into()
    }
}
