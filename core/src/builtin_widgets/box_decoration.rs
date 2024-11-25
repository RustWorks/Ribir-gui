use crate::{prelude::*, wrap_render::*};

/// The BoxDecoration provides configuration options to draw the background and
/// border of a box.
///
/// If a background color is specified, a derived foreground calculation from
/// the background will be applied to its children.
#[derive(Default, Clone)]
pub struct BoxDecoration {
  /// The background of the box.
  pub background: Option<Brush>,
  /// A border to draw above the background
  pub border: Option<Border>,
  /// The corners of this box are rounded by this `BorderRadius`. The round
  /// corner only work if the two borders beside it are same style.
  pub border_radius: Option<Radius>,
}

impl Declare for BoxDecoration {
  type Builder = FatObj<()>;
  #[inline]
  fn declarer() -> Self::Builder { FatObj::new(()) }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Border {
  pub left: BorderSide,
  pub right: BorderSide,
  pub top: BorderSide,
  pub bottom: BorderSide,
}

#[derive(Debug, Default, Clone, PartialEq, Lerp)]
pub struct BorderSide {
  pub color: Brush,
  pub width: f32,
}

impl BorderSide {
  #[inline]
  pub fn new(width: f32, color: Brush) -> Self { Self { width, color } }
}

impl_compose_child_for_wrap_render!(BoxDecoration);

impl WrapRender for BoxDecoration {
  #[inline]
  fn perform_layout(&self, clamp: BoxClamp, host: &dyn Render, ctx: &mut LayoutCtx) -> Size {
    host.perform_layout(clamp, ctx)
  }

  fn paint(&self, host: &dyn Render, ctx: &mut PaintingCtx) {
    let size = ctx.box_size().unwrap();
    if !size.is_empty() {
      let rect = Rect::from_size(size);
      let painter = ctx.painter();
      if let Some(ref background) = self.background {
        painter.set_fill_brush(background.clone());
        if let Some(radius) = &self.border_radius {
          painter.rect_round(&rect, radius);
        } else {
          painter.rect(&rect);
        }
        painter.fill();
      }
      self.paint_border(painter, &rect);

      if let Some(Brush::Color(ref background)) = self.background {
        let foreground = Palette::of(&ctx).on_container_of(background);
        ctx
          .painter()
          .set_fill_brush(foreground)
          .set_stroke_brush(foreground);
      }
      host.paint(ctx)
    }
  }
}

impl BoxDecoration {
  fn paint_border(&self, painter: &mut Painter, rect: &Rect) {
    if self.border.is_none() {
      return;
    }
    let border = self.border.as_ref().unwrap();
    if let Some(radius) = &self.border_radius {
      self.paint_round_border(painter, radius, border, rect);
    } else {
      self.paint_rect_border(painter, border, rect);
    }
  }

  fn is_border_uniform(&self) -> bool {
    self.border.as_ref().map_or(true, |border| {
      border.top == border.left && border.top == border.right && border.top == border.bottom
    })
  }

  fn paint_round_border(
    &self, painter: &mut Painter, radius: &Radius, border: &Border, content_rect: &Rect,
  ) {
    assert!(self.is_border_uniform(), "radius can't be setted with different border");
    let width_half = border.left.width / 2.;
    let min_x = content_rect.min_x() + width_half;
    let max_x = content_rect.max_x() - width_half;
    let min_y = content_rect.min_y() + width_half;
    let max_y = content_rect.max_y() - width_half;
    let radius = Radius::new(
      radius.top_left + width_half,
      radius.top_right + width_half,
      radius.bottom_left + width_half,
      radius.bottom_right + width_half,
    );

    painter
      .set_line_width(border.top.width)
      .set_stroke_brush(border.top.color.clone());
    painter.rect_round(
      &Rect::new(Point::new(min_x, min_y), Size::new(max_x - min_x, max_y - min_y)),
      &radius,
    );
    painter.stroke();
  }

  fn paint_rect_border(&self, painter: &mut Painter, border: &Border, content_rect: &Rect) {
    let min_x = content_rect.min_x() - border.left.width;
    let max_x = content_rect.max_x() + border.right.width;
    let min_y = content_rect.min_y() - border.top.width;
    let max_y = content_rect.max_y() + border.bottom.width;
    let vertexs = [
      Point::new(min_x, min_y), // lt
      Point::new(max_x, min_y), // rt
      Point::new(max_x, max_y), // rb
      Point::new(min_x, max_y), // lb
    ];
    let edges = [(0, 1), (1, 2), (2, 3), (3, 0)];
    let borders = [&border.top, &border.right, &border.bottom, &border.left];
    let borders_offset = [
      Size::new(0., border.top.width / 2.),
      Size::new(-border.right.width / 2., 0.),
      Size::new(0., -border.bottom.width / 2.),
      Size::new(border.left.width / 2., 0.),
    ];
    edges
      .iter()
      .zip(borders.iter())
      .zip(borders_offset.iter())
      .for_each(|((edge, border), offset)| {
        if border.is_visible() {
          painter
            .set_line_width(border.width)
            .set_stroke_brush(border.color.clone());
          painter.begin_path(vertexs[edge.0] + *offset);
          painter.line_to(vertexs[edge.1] + *offset);
          painter.end_path(false).stroke();
        }
      });
  }
}

impl BorderSide {
  fn is_visible(&self) -> bool { self.width > f32::EPSILON && self.color.is_visible() }
}

impl Border {
  #[inline]
  pub fn all(side: BorderSide) -> Self {
    Self { left: side.clone(), right: side.clone(), top: side.clone(), bottom: side }
  }

  #[inline]
  pub fn only_left(left: BorderSide) -> Self { Self { left, ..Default::default() } }

  #[inline]
  pub fn only_right(right: BorderSide) -> Self { Self { right, ..Default::default() } }

  #[inline]
  pub fn only_bottom(bottom: BorderSide) -> Self { Self { bottom, ..Default::default() } }

  #[inline]
  pub fn only_top(top: BorderSide) -> Self { Self { top, ..Default::default() } }

  #[inline]
  pub fn none() -> Self { Self { ..Default::default() } }
}
#[cfg(test)]
mod tests {
  use ribir_dev_helper::*;

  use super::*;
  use crate::test_helper::*;

  #[test]
  fn default_value_is_none() {
    let dummy = std::mem::MaybeUninit::uninit();
    // just for test, we know BoxDecoration not use `ctx` to build.
    let ctx: BuildCtx = unsafe { dummy.assume_init() };
    let mut w = BoxDecoration::declarer().finish();
    let w = w.get_box_decoration_widget();

    assert_eq!(w.read().border, None);
    assert_eq!(w.read().border_radius, None);
    assert_eq!(w.read().background, None);

    std::mem::forget(ctx);
  }

  const SIZE: Size = Size::new(100., 100.);

  widget_layout_test!(
    with_border,
    WidgetTester::new(fn_widget! {
      @MockBox {
        size: SIZE,
        border: Border {
          left: BorderSide::new(1., Color::BLACK.into()),
          right: BorderSide::new(2., Color::BLACK.into()),
          top: BorderSide::new(3., Color::BLACK.into()),
          bottom: BorderSide::new(4., Color::BLACK.into()),
        },
      }
    }),
    LayoutCase::default().with_size(Size::new(100., 100.)),
    LayoutCase::new(&[0]).with_rect(ribir_geom::rect(0., 0., 100., 100.))
  );
}
