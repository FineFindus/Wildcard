// SPDX-License-Identifier: GPL-3.0-or-later

use regex::{Regex, RegexBuilder};

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

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
        pub regex_buffer: TemplateChild<gtk::TextBuffer>,
        #[template_child]
        pub test_buffer: TemplateChild<gtk::TextBuffer>,
        #[template_child]
        pub matches_label: TemplateChild<gtk::Label>,
    }

    impl Default for Window {
        fn default() -> Self {
            Self {
                settings: gio::Settings::new(APP_ID),

                regex_text_view: TemplateChild::default(),
                regex_buffer: TemplateChild::default(),
                test_buffer: TemplateChild::default(),
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
            klass.bind_template_callbacks();

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

            obj.setup_text_views();
            obj.load_window_size();
        }
    }

    #[gtk::template_callbacks]
    impl Window {
        #[template_callback]
        fn on_buffer_changed(&self, _text_buffer: &gtk::TextBuffer) {
            let regex = self.regex_buffer.text(&self.regex_buffer.start_iter(), &self.regex_buffer.end_iter(), false);
            let test_string = self.test_buffer.text(&self.test_buffer.start_iter(), &self.test_buffer.end_iter(), false);

            let re: Regex = match RegexBuilder::new(regex.as_str()).multi_line(true).build() {
                Ok(r) => r,
                Err(_) => {
                    Regex::new(r"").unwrap()
                },
            };

            self.test_buffer.remove_all_tags(&self.test_buffer.start_iter(), &self.test_buffer.end_iter());

            let mut captures = 0;

            for (index, caps) in re.captures_iter(test_string.as_str()).enumerate() {
                let m = caps.get(0).unwrap();

                let mut start_iter = self.test_buffer.start_iter().clone();
                start_iter.set_offset(m.start() as i32);

                let mut end_iter = self.test_buffer.start_iter().clone();
                end_iter.set_offset(m.end() as i32);

                if index % 2 == 0 {
                    self.test_buffer.apply_tag_by_name(format!("marked_first").as_str(), &start_iter, &end_iter);
                } else {
                    self.test_buffer.apply_tag_by_name(format!("marked_second").as_str(), &start_iter, &end_iter);
                }

                captures += 1;
            }

            if regex.len() < 1 || captures == 0 {
                self.matches_label.set_label(&gettext("no matches"));
                return;
            }

            self.matches_label.set_label(
                ngettext_f(
                    "{matches} match",
                    "{matches} matches",
                    captures,
                    &[("matches", format!("{}", captures).as_str())]
                ).as_str()
            );

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

    fn setup_text_views(&self) {
        let imp = self.imp();

        imp.regex_text_view.grab_focus();

        imp.test_buffer.create_tag(Some("marked_first"), &[("background", &"#99c1f1")]);
        imp.test_buffer.create_tag(Some("marked_second"), &[("background", &"#62a0ea")]);
        imp.test_buffer.create_tag(Some("marked_highlight"), &[("background", &"#f9f06b")]);
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
