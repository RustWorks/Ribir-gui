use std::ops::Range;

use ribir_core::prelude::*;

use super::caret_state::CaretPosition;

#[derive(Default)]
pub(crate) struct GlyphsHelper {
  pub(crate) glyphs: Option<VisualGlyphs>,
}

impl GlyphsHelper {
  pub(crate) fn caret_position_from_pos(&self, x: f32, y: f32) -> CaretPosition {
    let glyphs: &VisualGlyphs = self.glyphs.as_ref().unwrap();
    let (para, mut offset) = glyphs.nearest_glyph(x, y);
    let rc = glyphs.glyph_rect(para, offset);
    if (rc.min_x() - x).abs() > (rc.max_x() - x).abs() {
      offset += 1;
    }
    let cluster = glyphs.position_to_cluster(para, offset);
    CaretPosition {
      cluster,
      position: Some((para, offset)),
    }
  }

  pub(crate) fn line_end(&self, caret: CaretPosition) -> CaretPosition {
    let glyphs: &VisualGlyphs = self.glyphs.as_ref().unwrap();

    let row = caret_position(glyphs, caret).0;
    let col = glyphs.glyph_count(row, true);
    let cluster = self.cluster_from_glyph_position(row, col);
    CaretPosition { cluster, position: Some((row, col)) }
  }

  pub(crate) fn line_begin(&self, caret: CaretPosition) -> CaretPosition {
    let glyphs: &VisualGlyphs = self.glyphs.as_ref().unwrap();
    let row = caret_position(glyphs, caret).0;
    let cluster: usize = self.cluster_from_glyph_position(row, 0);
    CaretPosition { cluster, position: Some((row, 0)) }
  }

  pub(crate) fn cluster_from_glyph_position(&self, row: usize, col: usize) -> usize {
    let glyphs: &VisualGlyphs = self.glyphs.as_ref().unwrap();
    glyphs.position_to_cluster(row, col)
  }

  pub(crate) fn prev_cluster(&self, caret: CaretPosition) -> CaretPosition {
    let glyphs: &VisualGlyphs = self.glyphs.as_ref().unwrap();
    let (mut row, mut col) = caret_position(glyphs, caret);

    (row, col) = match (row > 0, col > 0) {
      (_, true) => (row, col - 1),
      (true, false) => (row - 1, glyphs.glyph_count(row - 1, true)),
      (false, false) => (0, 0),
    };

    let cluster = glyphs.position_to_cluster(row, col);
    CaretPosition { cluster, position: Some((row, col)) }
  }

  pub(crate) fn next_cluster(&self, caret: CaretPosition) -> CaretPosition {
    let glyphs: &VisualGlyphs = self.glyphs.as_ref().unwrap();
    let (mut row, mut col) = caret_position(glyphs, caret);
    if col < glyphs.glyph_count(row, true) {
      col += 1;
    } else {
      row += 1;
      col = 0;
    }

    let cluster = glyphs.position_to_cluster(row, col);
    CaretPosition { cluster, position: Some((row, col)) }
  }

  pub(crate) fn up_cluster(&self, caret: CaretPosition) -> CaretPosition {
    let glyphs: &VisualGlyphs = self.glyphs.as_ref().unwrap();
    let (mut row, mut col) = caret_position(glyphs, caret);
    if row == 0 {
      return caret;
    } else {
      row -= 1;
      col = col.min(glyphs.glyph_count(row, true));
      let cluster = glyphs.position_to_cluster(row, col);
      CaretPosition { cluster, position: Some((row, col)) }
    }
  }

  pub(crate) fn down_cluster(&self, caret: CaretPosition) -> CaretPosition {
    let glyphs: &VisualGlyphs = self.glyphs.as_ref().unwrap();
    let (mut row, mut col) = caret_position(glyphs, caret);
    if row == glyphs.glyph_row_count() - 1 {
      return caret;
    }
    row += 1;
    col = col.min(glyphs.glyph_count(row, true));
    let cluster = glyphs.position_to_cluster(row, col);
    CaretPosition { cluster, position: Some((row, col)) }
  }

  pub(crate) fn cursor(&self, caret: CaretPosition) -> (Point, f32) {
    if let Some(glyphs) = self.glyphs.as_ref() {
      let (row, col) = caret_position(glyphs, caret);

      let line_height = glyphs.line_height(row);
      if col == 0 {
        let glphy = glyphs.glyph_rect(row, col);
        (Point::new(glphy.min_x(), glphy.min_y()), line_height)
      } else {
        let glphy = glyphs.glyph_rect(row, col - 1);
        (Point::new(glphy.max_x(), glphy.min_y()), line_height)
      }
    } else {
      (Point::zero(), 0.)
    }
  }

  pub(crate) fn selection(&self, rg: &Range<usize>) -> Vec<Rect> {
    if rg.is_empty() {
      return vec![];
    }
    self
      .glyphs
      .as_ref()
      .map_or(vec![], |glyphs| glyphs.select_range(rg))
  }
}

fn caret_position(glyphs: &VisualGlyphs, caret: CaretPosition) -> (usize, usize) {
  caret
    .position
    .clone()
    .unwrap_or_else(|| glyphs.position_by_cluster(caret.cluster))
}
