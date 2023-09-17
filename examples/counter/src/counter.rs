use ribir::prelude::{color::RadialGradient, GradientStop, *};

pub fn counter() -> Widget {
  // let svg = Svg::open("E:/projects/Ribir/themes/material/src/icons/debug.svg").
  // unwrap();
  fn_widget! {
    // Column {
      //   Text { text: "😂❤️😍🤣😊🥺🙏💕😭😘👍😅\n🩷💀🫱🏿‍🫲🏻🌴🐢🐐🍄⚽🫧👑📸🪼👀\n🚨🏡🕊️🏆😻🌟🧿🍀🫶🏾🍜" }
      // }
      // @Column {
      //   @Text { text: "😀O🎷🐛\n😀O🎷🐛🐛🐛🔋👻"}
      //   @Text { text: "🚥🚦🛴🦽\n🦼🩼🚲🛵🏍️🚙🚗🛻🚐🚚🚛🚜🏎️🚒\n🚑🚓🚕🛺🚌🚈🚝🚅🚄🚂🚃\n🚋🚎🚞🚊🚉🚍🚔🚘🚖\n🚆🚢🛳️🛥️🚤⛴️⛵🛶🚟🚠\n🚡🚁🛸🚀✈️🛫🛬🛩️🛝🎢🎡🎠\n🎪🗼🗽🗿🗻🏛️💈⛲⛩️🕍🕌🕋🛕\n⛪💒🏩🏯🏰🏗️🏢🏭\n🏬🏪🏟️🏦🏫🏨🏣🏤🏥🏚️🏠\n🏡🏘️🛖⛺🏕️⛱️🏙️🌆🌇\n🌃🌉🌁🛤️🛣️🗾🗺️🌐💺" }
      //   @Text { text: "😂😂a😂" }
      // }

    @Container {
      size: Size::new(100., 100.),
      background: Brush::RadialGradient(RadialGradient {
        start_center: Point::new(60., 50.),
        start_radius: 10.,
        end_center: Point::new(80., 50.),
        end_radius: 50.,
        stops: vec![
          GradientStop {
            offset: 0.,
            color: Color::RED,
          },

          // GradientStop {
          //   offset: 0.5,
          //   color: Color::GREEN,
          // },
          GradientStop {
            offset: 1.,
            color: Color::GREEN,
          },
        ],
        ..Default::default()
      })
    }
  }
  .into()
}
