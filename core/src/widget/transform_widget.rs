use crate::{prelude::*, impl_query_self_only};

#[derive(SingleChild, Declare, Clone)]
pub struct TransformWidget {
  #[declare(builtin)]
  pub transform: Transform,
}

impl Render for TransformWidget {
  fn perform_layout(&self, clamp: BoxClamp, ctx: &mut LayoutCtx) -> Size {
    ctx.single_child().map_or_else(Size::zero, |c| {
      ctx.perform_child_layout(c, clamp)
    })
  }

  #[inline]
  fn paint(&self, ctx: &mut PaintingCtx) {
    ctx.painter().set_transform(self.transform);
  }
}

impl Query for TransformWidget {
  impl_query_self_only!();
}

impl TransformWidget {
  #[inline]
  pub fn new(transform: Transform) -> Self { Self { transform } }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::widget_and_its_children_box_rect;
  use painter::{ Transform };

  #[test]
  fn smoke() {
    let widget = widget! {
      TransformWidget {
        transform: Transform::new(2., 0., 0., 2., 0., 0.),
        SizedBox {
          size: Size::new(100., 100.)
        }
      }
    };

    let (rect, _) =
      widget_and_its_children_box_rect(widget.into_widget(), Size::new(800., 800.));

    assert_eq!(rect, Rect::from_size(Size::new(100., 100.)));
  }
}