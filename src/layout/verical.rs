use ratatui::layout::{Constraint, Layout, Rect};

pub struct Vertical {
    field_height: u16,
}

pub struct VerticalAreas {
    pub intro: Rect,
    pub header: Rect,
    pub fields: Vec<Rect>,
    pub footer: Rect,
}

impl Vertical {
    pub fn new() -> Self {
        Self { field_height: 4 }
    }

    pub fn field_height(mut self, field_height: u16) -> Self {
        self.field_height = field_height;
        self
    }

    pub fn split(self, area: Rect, field_count: usize) -> VerticalAreas {
        let field_count = field_count.max(1);

        let mut constraints = vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ];
        constraints
            .extend(std::iter::repeat(Constraint::Length(self.field_height)).take(field_count));
        constraints.push(Constraint::Fill(3));
        constraints.push(Constraint::Length(1));

        let areas = Layout::vertical(constraints).split(area);

        let intro = areas[0];
        let header = areas[1];
        let fields = areas[3..3 + field_count].to_vec();
        let footer = areas[areas.len() - 1];

        VerticalAreas {
            intro,
            header,
            fields,
            footer,
        }
    }
}

impl Default for Vertical {
    fn default() -> Self {
        Self::new()
    }
}
