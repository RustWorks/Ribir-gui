use ribir::prelude::{color::RadialGradient, GradientStop, *};

pub fn counter() -> Widget {
  let svg = Svg::open("C:/Users/ribir/Desktop/Output/svg.svg").unwrap();

  fn_widget! {
    // Column {
      //
      // }
      @Column {
          scrollable: Scrollable::Both,
         @Text { text: "🟡🟡🟡🟡" }   
        @Text { text: "😀O🎷🐛\n😀O🎷🐛🐛🐛🔋👻"}
        @Text { text: "🚥🚦🛴🦽\n🦼🩼🚲🛵🏍️🚙🚗🛻🚐🚚🚛🚜🏎️🚒\n🚑🚓🚕🛺🚌🚈🚝🚅🚄🚂🚃\n🚋🚎🚞🚊🚉🚍🚔🚘🚖\n🚆🚢🛳️🛥️🚤⛴️⛵🛶🚟🚠\n🚡🚁🛸🚀✈️🛫🛬🛩️🛝🎢🎡🎠\n🎪🗼🗽🗿🗻🏛️💈⛲⛩️🕍🕌🕋🛕\n⛪💒🏩🏯🏰🏗️🏢🏭\n🏬🏪🏟️🏦🏫🏨🏣🏤🏥🏚️🏠\n🏡🏘️🛖⛺🏕️⛱️🏙️🌆🌇\n🌃🌉🌁🛤️🛣️🗾🗺️🌐💺" }
        @Text { text: "😂😂a😂" }
      }


  }
  .into()
}
