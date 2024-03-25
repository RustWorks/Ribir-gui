mod messages;
use messages::messages;
use ribir::prelude::*;

fn main() {
  unsafe {
    AppCtx::set_app_theme(material::purple::light());
  }

  App::new_window(messages(), None).set_title("Messages");
  App::exec();
}

#[cfg(test)]
use ribir::core::test_helper::*;
#[cfg(test)]
use ribir::material as ribir_material;
#[cfg(test)]
use ribir_dev_helper::*;
#[cfg(test)]
widget_image_test!(messages, wnd_size = Size::new(400., 600.),);
