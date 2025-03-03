use druid::widget::{Align, Flex, Label};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc, Env};

fn build_ui() -> impl Widget<()> {
    let label = Label::new(LocalizedString::new("Le Finder"));
    let ui = Flex::column()
        .with_child(label)
        .with_default_spacer()
        .align_horizontal(Align::Center);
    ui
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title(LocalizedString::new("Le Finder"));
    let data = ();
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Failed to launch application");
}
