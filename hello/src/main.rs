use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

enum AppMsg {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        self.counter = match msg {
            AppMsg::Increment => self.counter.wrapping_add(1),
            AppMsg::Decrement => self.counter.wrapping_sub(1),
        };
        true
    }
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

struct AppWidgets {
    window: gtk::ApplicationWindow,
    vbox: gtk::Box,
    inc_button: gtk::Button,
    dec_button: gtk::Button,
    label: gtk::Label,
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    fn init_view(model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
        let window = gtk::ApplicationWindow::builder()
            .title("Simple app")
            .default_width(300)
            .default_height(100)
            .build();
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        vbox.set_margin_all(5);

        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        // Connect the widgets
        window.set_child(Some(&vbox));
        vbox.append(&inc_button);
        vbox.append(&dec_button);
        vbox.append(&label);

        // Connect events
        let btn_sender = sender.clone();
        inc_button.connect_clicked(move |_| {
            send!(btn_sender, AppMsg::Increment);
        });

        dec_button.connect_clicked(move |_| {
            send!(sender, AppMsg::Decrement);
        });

        Self {
            window,
            vbox,
            inc_button,
            dec_button,
            label,
        }
    }

    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(&mut self, model: &AppModel, _sender: Sender<<AppModel as Model>::Msg>) {
        self.label.set_text(&format!("Counter: {}", model.counter));
    }
}

fn main() {
    let model = AppModel { counter: 0 };
    let app = RelmApp::new(model);
    app.run();
}

