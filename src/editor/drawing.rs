use macroquad::prelude::*;
use macroquad::miniquad::CursorIcon;

impl super::Editor {
	pub fn draw(&self, drawing_rect: Rect) {
		self.update_cursor_icon();

		let line_nums_bar_width = self.style.measure_text(&format!(" {} ", self.lines.len())).width;

		// Draw background
		draw_rectangle(
			drawing_rect.x + line_nums_bar_width,
			drawing_rect.y,
			drawing_rect.w - line_nums_bar_width,
			drawing_rect.h,
			self.style.background,
		);

		// Draw line numbers bar
		draw_rectangle(
			drawing_rect.x,
			drawing_rect.y,
			line_nums_bar_width,
			drawing_rect.h,
			self.style.line_nums_background,
		);

		let text = self.content_as_text().replace("\t", &" ".repeat(self.style.tab_size));

		// Draw editor content
		for (i, line) in text.lines().enumerate() {
			let x = drawing_rect.x + self.style.text_padding + line_nums_bar_width;
			let y = drawing_rect.y + self.style.text_padding
				  + self.style.dimensions.height * (i + 1) as f32
				  + self.style.line_spacing * i as f32;

			// Draw line number
			draw_text_ex(
				// I am too lazy to make here real padding. just spaces.
				&format!(" {} ", i + 1),
				drawing_rect.x,
				y,
				self.style.line_nums_params,
			);

			// Draw line
			draw_text_ex(
				line,
				x,
				y,
				self.style.text_params,
			);

			if self.caret_row() == i {
				let x = x + self.style.dimensions.width * self.caret_col() as f32;
				let c = self.content[self.caret_pos];

				// Draw caret
				draw_rectangle(
					x,
					y - self.style.dimensions.height,
					self.style.dimensions.width,
					self.style.dimensions.height,
					self.style.text,
				);

				if !c.is_whitespace() {
					let mut params = self.style.text_params;
					params.color = self.style.background;

					// Draw char over the caret
					draw_text_ex(
						&c.to_string(),
						x,
						y,
						params,
					);
				}
			}
		}
	}
	
	fn content_as_text(&self) -> String {
		String::from_iter(self.content.iter())
	}

	fn update_cursor_icon(&self) {
		let mouse_pos = mouse_position();
		let context = unsafe { get_internal_gl().quad_context };

		// Cursor is over the editor
		if mouse_pos.0 > 0.1 && mouse_pos.0 < screen_width()
		&& mouse_pos.1 > 0.1 && mouse_pos.1 < screen_height() {
			context.set_mouse_cursor(CursorIcon::Text);
		}
		else {
			context.set_mouse_cursor(CursorIcon::Default);
		}
	}
}