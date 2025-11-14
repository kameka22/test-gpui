use gpui::{
    App, Application, Bounds, Context, Hsla, Render, SharedString, Window, WindowBounds,
    WindowOptions, div, hsla, prelude::*, px, size,
};
use rand::Rng;

// Color scheme for dark trading theme
fn bg_primary() -> Hsla { hsla(0.63, 0.42, 0.08, 1.0) }
fn bg_secondary() -> Hsla { hsla(0.62, 0.33, 0.12, 1.0) }
fn bg_tertiary() -> Hsla { hsla(0.62, 0.25, 0.16, 1.0) }
fn text_primary() -> Hsla { hsla(0.61, 0.13, 0.91, 1.0) }
fn text_secondary() -> Hsla { hsla(0.58, 0.10, 0.65, 1.0) }
fn accent_blue() -> Hsla { hsla(0.60, 0.91, 0.59, 1.0) }
fn green_positive() -> Hsla { hsla(0.41, 0.80, 0.52, 1.0) }
fn red_negative() -> Hsla { hsla(0.00, 0.84, 0.60, 1.0) }
fn border_color() -> Hsla { hsla(0.60, 0.18, 0.24, 1.0) }

#[derive(Clone, Debug)]
struct Stock {
    symbol: SharedString,
    name: SharedString,
    price: f64,
    change: f64,
    change_percent: f64,
    volume: u64,
    market_cap: String,
    pe_ratio: f64,
}

impl Stock {
    fn new(symbol: &str, name: &str, price: f64) -> Self {
        let mut rng = rand::thread_rng();
        let change = rng.gen_range(-10.0..10.0);
        let change_percent = (change / price) * 100.0;

        Self {
            symbol: SharedString::from(symbol.to_string()),
            name: SharedString::from(name.to_string()),
            price,
            change,
            change_percent,
            volume: rng.gen_range(1_000_000..100_000_000),
            market_cap: format!("{}B", rng.gen_range(10..500)),
            pe_ratio: rng.gen_range(10.0..50.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum NavigationTab {
    Markets,
    Watchlist,
    Portfolio,
    Orders,
}

struct TradingApp {
    current_tab: NavigationTab,
    stocks: Vec<Stock>,
    watchlist: Vec<Stock>,
    selected_stock: Option<usize>,
}

impl TradingApp {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let stocks = vec![
            Stock::new("AAPL", "Apple Inc.", 178.32),
            Stock::new("MSFT", "Microsoft Corporation", 412.67),
            Stock::new("GOOGL", "Alphabet Inc.", 142.89),
            Stock::new("AMZN", "Amazon.com Inc.", 178.25),
            Stock::new("TSLA", "Tesla Inc.", 242.84),
            Stock::new("META", "Meta Platforms Inc.", 485.72),
            Stock::new("NVDA", "NVIDIA Corporation", 875.28),
            Stock::new("AMD", "Advanced Micro Devices", 165.43),
            Stock::new("NFLX", "Netflix Inc.", 598.34),
            Stock::new("BABA", "Alibaba Group", 87.92),
        ];

        let watchlist = vec![
            Stock::new("AAPL", "Apple Inc.", 178.32),
            Stock::new("TSLA", "Tesla Inc.", 242.84),
            Stock::new("NVDA", "NVIDIA Corporation", 875.28),
        ];

        Self {
            current_tab: NavigationTab::Markets,
            stocks,
            watchlist,
            selected_stock: None,
        }
    }

    fn render_sidebar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(220.0))
            .h_full()
            .bg(bg_secondary())
            .border_r_1()
            .border_color(border_color())
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(60.0))
                    .px_6()
                    .border_b_1()
                    .border_color(border_color())
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(text_primary())
                            .child("LongBridge")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .p_4()
                    .child(self.nav_item("Markets", NavigationTab::Markets))
                    .child(self.nav_item("Watchlist", NavigationTab::Watchlist))
                    .child(self.nav_item("Portfolio", NavigationTab::Portfolio))
                    .child(self.nav_item("Orders", NavigationTab::Orders))
            )
    }

    fn nav_item(&self, label: &str, tab: NavigationTab) -> impl IntoElement {
        let is_active = self.current_tab == tab;
        let label_str = label.to_string();

        div()
            .flex()
            .items_center()
            .h(px(40.0))
            .px_4()
            .rounded_md()
            .cursor_pointer()
            .when(is_active, |style| {
                style.bg(accent_blue()).text_color(gpui::white())
            })
            .when(!is_active, |style| {
                style.text_color(text_secondary()).hover(|style| {
                    style.bg(bg_tertiary()).text_color(text_primary())
                })
            })
            .child(label_str)
    }

    fn render_market_data(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .bg(bg_primary())
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(60.0))
                    .px_6()
                    .border_b_1()
                    .border_color(border_color())
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(text_primary())
                            .child("Market Overview")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary())
                            .child("Updated: Real-time")
                    )
            )
            .child(self.render_stock_table())
    }

    fn render_stock_table(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .overflow_y_hidden()
            .child(
                div()
                    .flex()
                    .h(px(40.0))
                    .bg(bg_secondary())
                    .border_b_1()
                    .border_color(border_color())
                    .px_6()
                    .items_center()
                    .child(self.table_header("Symbol", px(120.0)))
                    .child(self.table_header("Name", px(200.0)))
                    .child(self.table_header("Price", px(120.0)))
                    .child(self.table_header("Change", px(120.0)))
                    .child(self.table_header("Volume", px(120.0)))
                    .child(self.table_header("Market Cap", px(120.0)))
                    .child(self.table_header("P/E", px(100.0)))
            )
            .children(self.stocks.iter().enumerate().map(|(idx, stock)| {
                self.render_stock_row(stock, idx)
            }))
    }

    fn table_header(&self, label: &str, width: gpui::Pixels) -> impl IntoElement {
        let label_str = label.to_string();
        div()
            .w(width)
            .text_xs()
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_color(text_secondary())
            .child(label_str)
    }

    fn render_stock_row(&self, stock: &Stock, idx: usize) -> impl IntoElement {
        let is_positive = stock.change >= 0.0;
        let change_color = if is_positive { green_positive() } else { red_negative() };
        let is_selected = self.selected_stock == Some(idx);

        div()
            .flex()
            .h(px(50.0))
            .px_6()
            .items_center()
            .border_b_1()
            .border_color(border_color())
            .when(is_selected, |style| style.bg(bg_tertiary()))
            .hover(|style| style.bg(bg_tertiary()).cursor_pointer())
            .child(
                div()
                    .w(px(120.0))
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(accent_blue())
                    .child(stock.symbol.clone())
            )
            .child(
                div()
                    .w(px(200.0))
                    .text_sm()
                    .text_color(text_secondary())
                    .child(stock.name.clone())
            )
            .child(
                div()
                    .w(px(120.0))
                    .text_sm()
                    .text_color(text_primary())
                    .child(format!("${:.2}", stock.price))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(120.0))
                    .child(
                        div()
                            .text_sm()
                            .text_color(change_color)
                            .child(format!("{}{:.2}", if is_positive { "+" } else { "" }, stock.change))
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(change_color)
                            .child(format!("({}{:.2}%)", if is_positive { "+" } else { "" }, stock.change_percent))
                    )
            )
            .child(
                div()
                    .w(px(120.0))
                    .text_sm()
                    .text_color(text_secondary())
                    .child(format!("{:.1}M", stock.volume as f64 / 1_000_000.0))
            )
            .child(
                div()
                    .w(px(120.0))
                    .text_sm()
                    .text_color(text_secondary())
                    .child(stock.market_cap.clone())
            )
            .child(
                div()
                    .w(px(100.0))
                    .text_sm()
                    .text_color(text_secondary())
                    .child(format!("{:.2}", stock.pe_ratio))
            )
    }

    fn render_watchlist(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .h_full()
            .bg(bg_secondary())
            .border_l_1()
            .border_color(border_color())
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(60.0))
                    .px_4()
                    .border_b_1()
                    .border_color(border_color())
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(text_primary())
                            .child("Watchlist")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_4()
                    .children(self.watchlist.iter().map(|stock| {
                        self.render_watchlist_item(stock)
                    }))
            )
    }

    fn render_watchlist_item(&self, stock: &Stock) -> impl IntoElement {
        let is_positive = stock.change >= 0.0;
        let change_color = if is_positive { green_positive() } else { red_negative() };

        div()
            .flex()
            .flex_col()
            .p_3()
            .rounded_md()
            .bg(bg_tertiary())
            .border_1()
            .border_color(border_color())
            .hover(|style| style.border_color(accent_blue()).cursor_pointer())
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(text_primary())
                            .child(stock.symbol.clone())
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(text_primary())
                            .child(format!("${:.2}", stock.price))
                    )
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_secondary())
                            .child(stock.name.clone())
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(change_color)
                            .child(format!("{}{:.2}%", if is_positive { "+" } else { "" }, stock.change_percent))
                    )
            )
    }

    fn render_heatmap(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_wrap()
            .gap_2()
            .p_6()
            .children(self.stocks.iter().map(|stock| {
                let is_positive = stock.change >= 0.0;
                let intensity = (stock.change_percent.abs() / 5.0).min(1.0) as f32;
                let bg_color = if is_positive {
                    green_positive().opacity(0.2 + intensity * 0.6)
                } else {
                    red_negative().opacity(0.2 + intensity * 0.6)
                };

                div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .w(px(120.0))
                    .h(px(100.0))
                    .rounded_md()
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color())
                    .hover(|style| style.border_color(accent_blue()).cursor_pointer())
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(gpui::white())
                            .child(stock.symbol.clone())
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(gpui::white().opacity(0.8))
                            .child(format!("${:.2}", stock.price))
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(gpui::white())
                            .child(format!("{}{:.2}%", if is_positive { "+" } else { "" }, stock.change_percent))
                    )
            }))
    }
}

impl Render for TradingApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let main_content = match self.current_tab {
            NavigationTab::Markets => self.render_market_data(cx).into_any_element(),
            NavigationTab::Watchlist => {
                div()
                    .flex_1()
                    .bg(bg_primary())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .h(px(60.0))
                            .px_6()
                            .border_b_1()
                            .border_color(border_color())
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .text_color(text_primary())
                                    .child("My Watchlist")
                            )
                    )
                    .child(self.render_heatmap(cx))
                    .into_any_element()
            }
            NavigationTab::Portfolio | NavigationTab::Orders => {
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .text_color(text_secondary())
                    .child("Coming soon...")
                    .into_any_element()
            }
        };

        div()
            .flex()
            .size_full()
            .bg(bg_primary())
            .text_color(text_primary())
            .child(self.render_sidebar(cx))
            .child(main_content)
            .child(self.render_watchlist(cx))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1400.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("LongBridge Pro - Trading Desktop".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| TradingApp::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
