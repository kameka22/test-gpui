use gpui::{
    App, Application, Bounds, Context, Hsla, Render, SharedString, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};
use rand::Rng;

// Color scheme for dark trading theme
const BG_PRIMARY: Hsla = rgb(0x0a0e1a);
const BG_SECONDARY: Hsla = rgb(0x141824);
const BG_TERTIARY: Hsla = rgb(0x1e2330);
const TEXT_PRIMARY: Hsla = rgb(0xe4e6eb);
const TEXT_SECONDARY: Hsla = rgb(0x9ca3af);
const ACCENT_BLUE: Hsla = rgb(0x3b82f6);
const GREEN_POSITIVE: Hsla = rgb(0x10b981);
const RED_NEGATIVE: Hsla = rgb(0xef4444);
const BORDER_COLOR: Hsla = rgb(0x2d3748);

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
            symbol: symbol.into(),
            name: name.into(),
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
            .bg(BG_SECONDARY)
            .border_r_1()
            .border_color(BORDER_COLOR)
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(60.0))
                    .px_6()
                    .border_b_1()
                    .border_color(BORDER_COLOR)
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(TEXT_PRIMARY)
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

        div()
            .flex()
            .items_center()
            .h(px(40.0))
            .px_4()
            .rounded_md()
            .cursor_pointer()
            .when(is_active, |style| {
                style.bg(ACCENT_BLUE).text_color(gpui::white())
            })
            .when(!is_active, |style| {
                style.text_color(TEXT_SECONDARY).hover(|style| {
                    style.bg(BG_TERTIARY).text_color(TEXT_PRIMARY)
                })
            })
            .child(label)
    }

    fn render_market_data(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .bg(BG_PRIMARY)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(60.0))
                    .px_6()
                    .border_b_1()
                    .border_color(BORDER_COLOR)
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(TEXT_PRIMARY)
                            .child("Market Overview")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(TEXT_SECONDARY)
                            .child("Updated: Real-time")
                    )
            )
            .child(self.render_stock_table())
    }

    fn render_stock_table(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .h(px(40.0))
                    .bg(BG_SECONDARY)
                    .border_b_1()
                    .border_color(BORDER_COLOR)
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
        div()
            .w(width)
            .text_xs()
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_color(TEXT_SECONDARY)
            .child(label)
    }

    fn render_stock_row(&self, stock: &Stock, idx: usize) -> impl IntoElement {
        let is_positive = stock.change >= 0.0;
        let change_color = if is_positive { GREEN_POSITIVE } else { RED_NEGATIVE };
        let is_selected = self.selected_stock == Some(idx);

        div()
            .flex()
            .h(px(50.0))
            .px_6()
            .items_center()
            .border_b_1()
            .border_color(BORDER_COLOR)
            .when(is_selected, |style| style.bg(BG_TERTIARY))
            .hover(|style| style.bg(BG_TERTIARY).cursor_pointer())
            .child(
                div()
                    .w(px(120.0))
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(ACCENT_BLUE)
                    .child(stock.symbol.clone())
            )
            .child(
                div()
                    .w(px(200.0))
                    .text_sm()
                    .text_color(TEXT_SECONDARY)
                    .child(stock.name.clone())
            )
            .child(
                div()
                    .w(px(120.0))
                    .text_sm()
                    .text_color(TEXT_PRIMARY)
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
                    .text_color(TEXT_SECONDARY)
                    .child(format!("{:.1}M", stock.volume as f64 / 1_000_000.0))
            )
            .child(
                div()
                    .w(px(120.0))
                    .text_sm()
                    .text_color(TEXT_SECONDARY)
                    .child(stock.market_cap.clone())
            )
            .child(
                div()
                    .w(px(100.0))
                    .text_sm()
                    .text_color(TEXT_SECONDARY)
                    .child(format!("{:.2}", stock.pe_ratio))
            )
    }

    fn render_watchlist(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .h_full()
            .bg(BG_SECONDARY)
            .border_l_1()
            .border_color(BORDER_COLOR)
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(60.0))
                    .px_4()
                    .border_b_1()
                    .border_color(BORDER_COLOR)
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(TEXT_PRIMARY)
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
        let change_color = if is_positive { GREEN_POSITIVE } else { RED_NEGATIVE };

        div()
            .flex()
            .flex_col()
            .p_3()
            .rounded_md()
            .bg(BG_TERTIARY)
            .border_1()
            .border_color(BORDER_COLOR)
            .hover(|style| style.border_color(ACCENT_BLUE).cursor_pointer())
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
                            .text_color(TEXT_PRIMARY)
                            .child(stock.symbol.clone())
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(TEXT_PRIMARY)
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
                            .text_color(TEXT_SECONDARY)
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
                let intensity = (stock.change_percent.abs() / 5.0).min(1.0);
                let bg_color = if is_positive {
                    rgb(0x10b981).opacity(0.2 + intensity * 0.6)
                } else {
                    rgb(0xef4444).opacity(0.2 + intensity * 0.6)
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
                    .border_color(BORDER_COLOR)
                    .hover(|style| style.border_color(ACCENT_BLUE).cursor_pointer())
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
        div()
            .flex()
            .size_full()
            .bg(BG_PRIMARY)
            .text_color(TEXT_PRIMARY)
            .child(self.render_sidebar(cx))
            .child(
                match self.current_tab {
                    NavigationTab::Markets => self.render_market_data(cx),
                    NavigationTab::Watchlist => {
                        div()
                            .flex_1()
                            .bg(BG_PRIMARY)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .h(px(60.0))
                                    .px_6()
                                    .border_b_1()
                                    .border_color(BORDER_COLOR)
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(TEXT_PRIMARY)
                                            .child("My Watchlist")
                                    )
                            )
                            .child(self.render_heatmap(cx))
                    }
                    NavigationTab::Portfolio | NavigationTab::Orders => {
                        div()
                            .flex()
                            .flex_1()
                            .items_center()
                            .justify_center()
                            .text_color(TEXT_SECONDARY)
                            .child("Coming soon...")
                    }
                }
            )
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
