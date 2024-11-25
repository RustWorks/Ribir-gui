use ribir_core::prelude::*;

use crate::layout::SizedBox;

/// Widget that let child paint as a icon with special size.
///
/// Unlike icon in classic frameworks, it's not draw anything and not require
/// you to provide image or font fot it to draw, it just center align and fit
/// size of its child. So you can declare any widget as its child to display as
/// a icon.
#[derive(Declare, Default, Clone, Copy)]
pub struct Icon {
  #[declare(default = IconSize::of(BuildCtx::get()).small)]
  pub size: Size,
}

impl<'c> ComposeChild<'c> for Icon {
  type Child = Widget<'c>;
  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> Widget<'c> {
    fn_widget! {
      let child = FatObj::new(child);
      @SizedBox {
        size: pipe!($this.size),
        @ $child {
          box_fit: BoxFit::Contain,
          h_align: HAlign::Center,
          v_align: VAlign::Center,
        }
      }
    }
    .into_widget()
  }
}

macro_rules! define_fixed_size_icon {
  ($($name: ident, $field: ident),*) => {
    $(
      #[derive(Declare, Default, Clone, Copy)]
      pub struct $name;

      impl<'c> ComposeChild<'c> for $name {
        type Child = Widget<'c>;
        fn compose_child(_: impl StateWriter<Value = Self>, child: Self::Child)
          -> Widget<'c>
        {
          fn_widget! {
            let icon = @Icon { size: IconSize::of(BuildCtx::get()).$field };
            @ $icon { @ { child } }
          }.into_widget()
        }
      }
    )*
  };
}

define_fixed_size_icon!(TinyIcon, tiny);
define_fixed_size_icon!(SmallIcon, small);
define_fixed_size_icon!(MediumIcon, medium);
define_fixed_size_icon!(LargeIcon, large);
define_fixed_size_icon!(HugeIcon, huge);
