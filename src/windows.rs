pub mod stats_window;
pub mod typing_window;

use ratatui::{
  layout::Rect, Frame,
  style::{Color, Style, Stylize},
  text::{Line, Span, Text},
  widgets::Paragraph,
  crossterm::event::KeyEvent
};

use crate::{
  helper::ToDark,
  layout::{LayoutColorTypeEnum, Layout as TukaiLayout}
};

pub struct Instruction<'a> {
  title: &'a str,
  shortcut: &'a str,
  color_type: LayoutColorTypeEnum
}

impl<'a> Instruction<'a> {
  pub fn new(
    title: &'a str,
    shortcut: &'a str,
    color_type: LayoutColorTypeEnum
  ) -> Self {
    Self {
      title,
      shortcut,
      color_type
    }
  }
}

pub struct InstructionWidget<'a> {
  layout: &'a TukaiLayout,
  instructions: Vec<Instruction<'a>>
}

impl<'a> InstructionWidget<'a> {
  pub fn new(layout: &'a TukaiLayout) -> Self {
    Self {
      layout,
      instructions: Vec::new()
    }
  }

  fn get_instruction_color(&self, color_type: &LayoutColorTypeEnum) -> Color {
    match color_type {
      LayoutColorTypeEnum::Primary => self.layout.get_primary_color(),
      LayoutColorTypeEnum::Secondary => self.layout.get_primary_color(),
      LayoutColorTypeEnum::Text => self.layout.get_text_color(),
      LayoutColorTypeEnum::TextReverse => self.layout.get_text_current_bg_color(),
      LayoutColorTypeEnum::Error => self.layout.get_error_color(),
      LayoutColorTypeEnum::Background => self.layout.get_background_color()
    }
  }

  pub fn add_instruction(&mut self, instruction: Instruction<'a>) {
    self.instructions.push(instruction);
  }

  pub fn get_paragraph(&self) -> Paragraph {
    let instructions_spans = self.instructions.iter()
      .enumerate()
      .flat_map(|(index, instruction)| {
        let color = self.get_instruction_color(&instruction.color_type);

        vec![
          Span::from(format!(" {}", instruction.title)).style(Style::default().fg(color.to_dark())),
          Span::from(
            format!(" {}{}", instruction.shortcut, if index != self.instructions.len() - 1 { " |" } else { "" })
          ).style(Style::default().fg(color).bold()),
        ]
      }).collect::<Vec<Span>>();

    Paragraph::new(Text::from(Line::from(instructions_spans)))
  }
}



pub trait Window {
  //fn default(storage_handle: Option<&mut StorageHandler>) -> Self;
  fn default() -> Self;

  /// Handle events
  /// Returns `true` if event is consumed
  fn handle_events(&mut self, key: KeyEvent) -> bool;

  /// Window is currently active
  fn is_active(&self) -> bool;
  fn toggle_active(&mut self);

  /// After another window switched
  fn hide(&mut self) {
    self.toggle_active();
  }

  /// Render window instructions
  fn render_instructions(&self, frame: &mut Frame, layout: &TukaiLayout, area: Rect);

  /// Render window
  fn render(&self, frame: &mut Frame, layout: &TukaiLayout, version: &String, area: Rect);
}
