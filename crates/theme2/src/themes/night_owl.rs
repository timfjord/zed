// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn night_owl() -> UserThemeFamily {
    UserThemeFamily {
        name: "Night Owl".into(),
        author: "Sarah Drasner (sdras)".into(),
        themes: vec![
            UserTheme {
                name: "Night Owl".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x5f7e97ff).into()),
                        border_variant: Some(rgba(0x5f7e97ff).into()),
                        border_focused: Some(rgba(0x5f7e97ff).into()),
                        border_selected: Some(rgba(0x5f7e97ff).into()),
                        border_transparent: Some(rgba(0x5f7e97ff).into()),
                        border_disabled: Some(rgba(0x5f7e97ff).into()),
                        elevated_surface_background: Some(rgba(0x011526ff).into()),
                        surface_background: Some(rgba(0x011526ff).into()),
                        background: Some(rgba(0x011526ff).into()),
                        element_background: Some(rgba(0x7d56c1cc).into()),
                        text: Some(rgba(0xd6deebff).into()),
                        tab_inactive_background: Some(rgba(0x01101cff).into()),
                        tab_active_background: Some(rgba(0x0a2842ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x575656ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xef524fff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x21da6eff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffeb95ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x7fdbcaff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xffffffff).into()),
                        terminal_ansi_black: Some(rgba(0x011526ff).into()),
                        terminal_ansi_red: Some(rgba(0xef524fff).into()),
                        terminal_ansi_green: Some(rgba(0x21da6eff).into()),
                        terminal_ansi_yellow: Some(rgba(0xc5e478ff).into()),
                        terminal_ansi_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_cyan: Some(rgba(0x20c7a7ff).into()),
                        terminal_ansi_white: Some(rgba(0xffffffff).into()),
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Night Owl Light".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0xd9d9d9ff).into()),
                        border_variant: Some(rgba(0xd9d9d9ff).into()),
                        border_focused: Some(rgba(0xd9d9d9ff).into()),
                        border_selected: Some(rgba(0xd9d9d9ff).into()),
                        border_transparent: Some(rgba(0xd9d9d9ff).into()),
                        border_disabled: Some(rgba(0xd9d9d9ff).into()),
                        elevated_surface_background: Some(rgba(0xf0f0f0ff).into()),
                        surface_background: Some(rgba(0xf0f0f0ff).into()),
                        background: Some(rgba(0xfbfbfbff).into()),
                        element_background: Some(rgba(0x29a298ff).into()),
                        text: Some(rgba(0x403f53ff).into()),
                        tab_inactive_background: Some(rgba(0xf0f0f0ff).into()),
                        tab_active_background: Some(rgba(0xf6f6f6ff).into()),
                        terminal_background: Some(rgba(0xf6f6f6ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x403f53ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xde3c3aff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x07916aff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xdaa900ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x278dd7ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xd64289ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x29a298ff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xf0f0f0ff).into()),
                        terminal_ansi_black: Some(rgba(0x403f53ff).into()),
                        terminal_ansi_red: Some(rgba(0xde3c3aff).into()),
                        terminal_ansi_green: Some(rgba(0x07916aff).into()),
                        terminal_ansi_yellow: Some(rgba(0xe0ae01ff).into()),
                        terminal_ansi_blue: Some(rgba(0x278dd7ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xd64289ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x29a298ff).into()),
                        terminal_ansi_white: Some(rgba(0xf0f0f0ff).into()),
                        ..Default::default()
                    },
                },
            },
        ],
    }
}