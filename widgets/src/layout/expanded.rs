use ribir_core::prelude::*;

/// A widget that expanded a child of `Flex`, so that the child fills the
/// available space. If multiple children are expanded, the available space is
/// divided among them according to the flex factor.
#[derive(Clone, PartialEq)]
// `Expand` should not support `FatObj`, as this may cause the `Expanded` to be invisible to its
// parent. `@Expanded { margin: EdgeInsets::all(10.) }` actually expands as `@Margin { @Expanded {
// .. } }`.
#[simple_declare]
pub struct Expanded {
  #[declare(default = 1.)]
  pub flex: f32,
}

impl<'c> ComposeChild<'c> for Expanded {
  type Child = Widget<'c>;
  #[inline]
  fn compose_child(this: impl StateWriter<Value = Self>, mut child: Self::Child) -> Widget<'c> {
    let data: Box<dyn Query> = match this.try_into_value() {
      Ok(this) => Box::new(Queryable(this)),
      Err(this) => {
        let modifies = this.raw_modifies();
        child = child.on_build(|id| id.dirty_on(modifies));
        Box::new(this)
      }
    };

    child.attach_data(data)
  }
}

#[cfg(test)]
mod tests {
  use ribir_core::{reset_test_env, test_helper::*};
  use ribir_dev_helper::*;

  use super::*;
  use crate::prelude::*;

  widget_layout_test!(
    one_line_expanded,
    WidgetTester::new(fn_widget! {
      let size = Size::new(100., 50.);
      @Row {
        @Expanded {
          flex: 1.,
          @SizedBox { size }
        }
        @SizedBox { size }
        @SizedBox { size }
        @Expanded {
          flex: 2.,
          @SizedBox { size }
        }
      }
    })
    .with_wnd_size(Size::new(500., 500.)),
    LayoutCase::default().with_size(Size::new(500., 50.)),
    LayoutCase::new(&[0, 0]).with_size(Size::new(100., 50.)),
    LayoutCase::new(&[0, 1]).with_rect(ribir_geom::rect(100., 0., 100., 50.)),
    LayoutCase::new(&[0, 2]).with_rect(ribir_geom::rect(200., 0., 100., 50.)),
    LayoutCase::new(&[0, 3]).with_rect(ribir_geom::rect(300., 0., 200., 50.))
  );

  widget_layout_test!(
    wrap_expanded,
    WidgetTester::new(fn_widget! {
      let size = Size::new(100., 50.);
      @Row {
        wrap: true,
        @Expanded {
          flex: 1. ,
          @SizedBox { size }
        }
        @SizedBox { size }
        @SizedBox { size }
        @SizedBox { size }
        @SizedBox { size }
        @Expanded {
          flex: 1. ,
          @SizedBox { size, }
        }
        @Expanded {
          flex: 4.,
          @SizedBox { size, }
        }
      }
    })
    .with_wnd_size(Size::new(350., 500.)),
    LayoutCase::default().with_rect(ribir_geom::rect(0., 0., 350., 100.)),
    LayoutCase::new(&[0, 0]).with_rect(ribir_geom::rect(0., 0., 50., 50.)),
    LayoutCase::new(&[0, 1]).with_rect(ribir_geom::rect(50., 0., 100., 50.)),
    LayoutCase::new(&[0, 2]).with_rect(ribir_geom::rect(150., 0., 100., 50.)),
    LayoutCase::new(&[0, 3]).with_rect(ribir_geom::rect(250., 0., 100., 50.)),
    LayoutCase::new(&[0, 4]).with_rect(ribir_geom::rect(0., 50., 100., 50.)),
    LayoutCase::new(&[0, 5]).with_rect(ribir_geom::rect(100., 50., 50., 50.)),
    LayoutCase::new(&[0, 6]).with_rect(ribir_geom::rect(150., 50., 200., 50.))
  );

  #[test]
  fn modifies_flex() {
    reset_test_env!();

    let (flex, w_flex) = split_value(1f32);
    let widget = fn_widget! {
      let expanded = @Expanded { flex: 1. };
      watch!(*$flex).subscribe(move |val| $expanded.write().flex = val);

      @Row {
        @ $expanded { @ { Void } }
        @Expanded {
          flex: 1.,
          @ { Void }
        }
        @SizedBox { size: Size::new(100., 100.) }
      }
    };

    let mut wnd = TestWindow::new_with_size(widget, Size::new(400., 100.));
    wnd.draw_frame();
    LayoutCase::expect_size(&wnd, &[0, 0], Size::new(150., 0.));
    *w_flex.write() = 2.;
    wnd.draw_frame();
    LayoutCase::expect_size(&wnd, &[0, 0], Size::new(200., 0.));
  }
}
