use ftui_render::cell::PackedRgba;
use ftui_style::Style;
use ftui_widgets::block::Block;
use ftui_widgets::borders::{BorderSet, BorderType, Borders};

// ── ボーダー ──
pub const BORDER_FOCUSED: PackedRgba = PackedRgba::rgb(0, 210, 210);
pub const BORDER_UNFOCUSED: PackedRgba = PackedRgba::rgb(130, 130, 140);

// ── セッションステータス (ステータスバー用) ──
pub const STATUS_RUNNING: PackedRgba = PackedRgba::rgb(80, 200, 120);
pub const STATUS_QUEUED: PackedRgba = PackedRgba::rgb(220, 200, 80);
pub const STATUS_DONE: PackedRgba = PackedRgba::rgb(100, 160, 255);
pub const STATUS_FAILED: PackedRgba = PackedRgba::rgb(255, 90, 90);
pub const STATUS_NEEDS_INPUT: PackedRgba = PackedRgba::rgb(200, 130, 255);

// ── ステータスバー ──
pub const BAR_BG: PackedRgba = PackedRgba::rgb(30, 30, 46);
pub const HINT_FG: PackedRgba = PackedRgba::rgb(140, 140, 160);

// ── リスト/ハイライト ──
pub const HIGHLIGHT_BG: PackedRgba = PackedRgba::rgb(0, 180, 180);
pub const HIGHLIGHT_FG: PackedRgba = PackedRgba::rgb(0, 0, 0);

// ── プレースホルダー ──
pub const PLACEHOLDER: PackedRgba = PackedRgba::rgb(100, 100, 110);

// ── ダイアログ ──
pub const DIALOG_BORDER: PackedRgba = PackedRgba::rgb(255, 200, 60);

/// フォーカス状態に応じた角丸ボーダー付き Block を返す
pub fn panel_block(title: &str, focused: bool) -> Block<'_> {
    panel_block_with(title, focused, Borders::ALL)
}

/// 指定したボーダーフラグで角丸ボーダー付き Block を返す
pub fn panel_block_with(title: &str, focused: bool, borders: Borders) -> Block<'_> {
    let border_color = if focused {
        BORDER_FOCUSED
    } else {
        BORDER_UNFOCUSED
    };
    Block::new()
        .borders(borders)
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(Style::new().fg(border_color))
}

/// サイドバー内の下パネル用ブロック（上角をT字接合 ├┤ にして縦線を連続させる）
pub fn junction_panel_block(title: &str, focused: bool) -> Block<'_> {
    let border_color = if focused {
        BORDER_FOCUSED
    } else {
        BORDER_UNFOCUSED
    };
    let junction_set = BorderSet {
        top_left: '├',
        top_right: '┤',
        ..BorderSet::ROUNDED
    };
    Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Custom(junction_set))
        .title(title)
        .border_style(Style::new().fg(border_color))
}

/// プレースホルダーテキスト用スタイル
pub fn placeholder_style() -> Style {
    Style::new().fg(PLACEHOLDER).dim()
}
