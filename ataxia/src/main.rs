use ataxia::App;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title("Ataxia")
        .theme(App::theme)
        .run()
}
