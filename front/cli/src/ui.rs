use chippy::emu::gpu::{self, Gpu};
use eyre::Result;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Frame,
};

const PIXEL_WIDTH: u16 = 1;
const PIXEL_HIGHT: u16 = 1;
const GRID_WIDTH: u16 = gpu::SCREEN_WIDTH as u16 * PIXEL_WIDTH;
const GRID_HEIGHT: u16 = gpu::SCREEN_HEIGHT as u16 * PIXEL_HIGHT;

pub struct Ui<'a> {
    gpu: &'a Gpu,
    block: Option<Block<'a>>,
}

impl<'a> Ui<'a> {
    pub fn new(gpu: &'a Gpu) -> Self {
        Self { gpu, block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> Ui<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for Ui<'a> {
    fn render(mut self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let final_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        for y in 0..gpu::SCREEN_HEIGHT {
            for x in 0..gpu::SCREEN_WIDTH {
                let pixel = self.gpu.get(x, y);
                let text = match pixel {
                    true => "█",
                    false => " ",
                    // false => "·",
                };
                let xx = final_area.x + x as u16;
                let yy = final_area.y + y as u16;
                buf.set_string(xx, yy, text, Style::default().fg(Color::White));
            }
        }
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, gpu: &Gpu) {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::LightYellow))
        .title("Chippy");
    f.render_widget(main_block, f.size());

    let vertical_padding_block_height =
        f.size().height.checked_sub(GRID_HEIGHT).unwrap_or_default() / 2;

    let horizontal_padding_block_width =
        f.size().width.checked_sub(GRID_WIDTH).unwrap_or_default() / 2;

    let v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(vertical_padding_block_height),
            Constraint::Length(GRID_HEIGHT + 2),
            Constraint::Min(vertical_padding_block_height),
        ])
        .split(f.size());

    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(horizontal_padding_block_width),
            Constraint::Length(GRID_WIDTH + 2),
            Constraint::Min(horizontal_padding_block_width),
        ])
        .split(v_layout[1]);

    let ui = Ui::new(gpu).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White)),
    );
    f.render_widget(ui, h_layout[1]);
}
