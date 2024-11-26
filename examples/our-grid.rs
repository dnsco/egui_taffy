use egui_taffy::{tui, TuiBuilderLogic};

use taffy::{
    prelude::{fr, length, line, percent, span},
    AlignContent, Line, Style,
};

fn main() -> eframe::Result {
    eframe::run_simple_native("demo", Default::default(), move |ctx, _frame| {
        ctx.options_mut(|options| {
            options.max_passes = std::num::NonZeroUsize::new(3).unwrap();
        });

        ctx.style_mut(|style| {
            style.wrap_mode = Some(egui::TextWrapMode::Extend);
        });

        egui::Window::new("Grid demo").show(ctx, |ui| render(ui));
    })
}

fn render(ui: &mut egui::Ui) {
    // Initialize Tui layout (Taffy ui layout)
    tui(ui, "grid")
        .reserve_available_space()
        .style(Style {
            display: taffy::Display::Grid,
            grid_template_columns: vec![length(80.0), fr(1.), length(80.0)],
            grid_template_rows: vec![length(50.0), fr(1.), length(100.0), length(50.0)],
            gap: length(15.0),
            size: percent(1.),

            // Stretch grid cells by default to fill space
            align_items: Some(taffy::AlignItems::Stretch),
            justify_items: Some(taffy::AlignItems::Stretch),

            ..Default::default()
        })
        .show(|tui| {
            tui.style(Style {
                grid_column: span(2),
                grid_row: line(1),
                ..Default::default()
            })
            .add_with_border(|ui| ui.label("I span two columns"));

            tui.ui(|_| {});
            tui.label("Justified\nBetween:");
            tui.style(Style {
                grid_column: line(2),
                grid_row: line(2),
                justify_content: Some(AlignContent::SpaceBetween),
                ..Default::default()
            })
            .add_with_border(|tui| {
                tui.label("JB First");
                tui.label("JB Last")
            });

            tui.style(Style {
                grid_column: Line {
                    start: line(2),
                    end: span(2),
                },
                grid_row: line(3),
                ..Default::default()
            })
            .add_with_border(|tui| tui.label("cols: start 2, span 2"));

            tui.style(Style {
                grid_column: span(3),
                ..Default::default()
            })
            .add_with_border(|tui| tui.label("Span 3"));
        })
}
