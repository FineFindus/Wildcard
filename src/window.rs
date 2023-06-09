// SPDX-License-Identifier: GPL-3.0-or-later

use regex::{Regex, RegexBuilder};

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use glib::clone;

use gettextrs::gettext;

use crate::i18n::ngettext_f;

use crate::application::Application;
use crate::config::{APP_ID, VERSION, PROFILE};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/fkinoshita/Patterns/ui/window.ui")]
    pub struct Window {
        pub settings: gio::Settings,

        #[template_child]
        pub regex_text_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub test_text_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub matches_label: TemplateChild<gtk::Label>,
    }

    impl Default for Window {
        fn default() -> Self {
            Self {
                settings: gio::Settings::new(APP_ID),

                regex_text_view: TemplateChild::default(),
                test_text_view: TemplateChild::default(),
                matches_label: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("win.about", None, move |obj, _, _| {
               obj.show_about_dialog();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            // Devel profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            obj.setup();
            obj.setup_signals();
            obj.load_window_size();
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {
        fn close_request(&self) -> gtk::Inhibit {
            let window = self.obj();

            if let Err(err) = window.save_window_size() {
                println!("Failed to save window state, {}", &err);
            }

            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

impl Window {
    pub fn new(application: &Application) -> Self {
        glib::Object::builder().property("application", application).build()
    }

    fn setup(&self) {
        let imp = self.imp();

        let regex_buffer = imp.regex_text_view.buffer();
        regex_buffer.set_text("[a-z]{6}");

        let test_buffer = imp.test_text_view.buffer();

        test_buffer.set_text("This is a test string");

        test_buffer.create_tag(Some("marked_first"), &[("background", &"#99c1f1")]);
        test_buffer.create_tag(Some("marked_second"), &[("background", &"#62a0ea")]);
        test_buffer.create_tag(Some("marked_highlight"), &[("background", &"#f9f06b")]);

        self.check_regex();
    }

    fn setup_signals(&self) {
        let imp = self.imp();

        imp.regex_text_view.buffer().connect_changed(
            clone!(@strong self as this => move |_| {
                this.check_regex();
            })
        );

        imp.test_text_view.buffer().connect_changed(
            clone!(@strong self as this => move |_| {
                this.check_regex();
            })
        );
    }

    fn check_regex(&self) {
        let imp = self.imp();

        let regex_buffer = imp.regex_text_view.buffer();
        let regex = regex_buffer.text(&regex_buffer.start_iter(), &regex_buffer.end_iter(), false);

        let test_buffer = self.imp().test_text_view.buffer();
        let test_string = test_buffer.text(&test_buffer.start_iter(), &test_buffer.end_iter(), false);

        let re: Regex = match RegexBuilder::new(regex.as_str()).multi_line(true).build() {
            Ok(r) => r,
            Err(_) => {
                Regex::new(r"").unwrap()
            },
        };

        test_buffer.remove_all_tags(&test_buffer.start_iter(), &test_buffer.end_iter());

        let mut captures = 0;

        for (index, caps) in re.captures_iter(test_string.as_str()).enumerate() {
            let m = caps.get(0).unwrap();

            let mut start_iter = test_buffer.start_iter().clone();
            start_iter.set_offset(m.start() as i32);

            let mut end_iter = test_buffer.start_iter().clone();
            end_iter.set_offset(m.end() as i32);

            if index % 2 == 0 {
                test_buffer.apply_tag_by_name(format!("marked_first").as_str(), &start_iter, &end_iter);
            } else {
                test_buffer.apply_tag_by_name(format!("marked_second").as_str(), &start_iter, &end_iter);
            }

            captures += 1;
        }

        imp.matches_label.set_label(
            ngettext_f(
                "{matches} match",
                "{matches} matches",
                captures,
                &[("matches", format!("{}", captures).as_str())]
            ).as_str()
        );
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();

        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");

        self.set_default_size(width, height);
    }

    fn show_about_dialog(&self) {
        let dialog = adw::AboutWindow::builder()
            .application_icon(APP_ID)
            .application_name(gettext("Patterns"))
            .license_type(gtk::License::Gpl30)
            .comments(gettext("Test your regular expressions"))
            .website("https://github.com/fkinoshita/Patterns")
            .issue_url("https://github.com/fkinoshita/Patterns/issues/new")
            .version(VERSION)
            .transient_for(self)
            .translator_credits(gettext("translator-credits"))
            .developer_name("Felipe Kinoshita")
            .developers(vec!["Felipe Kinoshita <fkinoshita@gnome.org>"])
            .build();

        dialog.present();
    }
}
