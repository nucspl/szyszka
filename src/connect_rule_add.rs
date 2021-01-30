use crate::class_gui_data::GuiData;
use crate::help_function::populate_rules_tree_view;
use crate::notebook_enum::{to_notebook_enum, NotebookEnum};
use crate::rules::{RuleData, RulePlace, RuleType};
use gtk::prelude::*;
use gtk::{ButtonExt, WidgetExt};
use std::ops::DerefMut;

pub fn connect_rule_add(gui_data: &GuiData) {
    let button_rule_window_add = gui_data.window_rules.button_rule_window_add.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();
    let notebook_choose_rule = gui_data.window_rules.notebook_choose_rule.clone();
    let rules = gui_data.rules.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();

    let window_rules = gui_data.window_rules.clone();

    let radio_button_letters_type_uppercase = window_rules.size_letters.radio_button_letters_type_uppercase.clone();
    let radio_button_letters_type_lowercase = window_rules.size_letters.radio_button_letters_type_lowercase.clone();
    let radio_button_letters_usage_name = window_rules.size_letters.radio_button_letters_usage_name.clone();
    let radio_button_letters_usage_extension = window_rules.size_letters.radio_button_letters_usage_extension.clone();
    let radio_button_letters_usage_both = window_rules.size_letters.radio_button_letters_usage_both.clone();

    let radio_button_purge_name = window_rules.purge.radio_button_purge_name.clone();
    let radio_button_purge_extension = window_rules.purge.radio_button_purge_extension.clone();
    let radio_button_purge_both = window_rules.purge.radio_button_purge_both.clone();

    let radio_button_add_text_after_name = window_rules.add_text.radio_button_add_text_after_name.clone();
    let radio_button_add_text_before_name = window_rules.add_text.radio_button_add_text_before_name.clone();
    let entry_add_text_text_to_add = window_rules.add_text.entry_add_text_text_to_add.clone();

    let radio_button_trim_name_start = window_rules.trim.radio_button_trim_name_start.clone();
    let radio_button_trim_name_end = window_rules.trim.radio_button_trim_name_end.clone();
    let radio_button_trim_extension_start = window_rules.trim.radio_button_trim_extension_start.clone();
    let radio_button_trim_extension_end = window_rules.trim.radio_button_trim_extension_end.clone();
    let radio_button_trim_case_insensitive = window_rules.trim.radio_button_trim_case_insensitive.clone();
    let radio_button_trim_case_sensitive = window_rules.trim.radio_button_trim_case_sensitive;

    button_rule_window_add.connect_clicked(move |_e| {
        window_with_rules.hide();
        window_main.set_sensitive(true);
        let mut rule = rules.borrow_mut();
        let rule = rule.deref_mut();

        let rule_type: RuleType;
        let rule_place: RulePlace;
        let mut rule_data: RuleData = RuleData::new();

        match to_notebook_enum(notebook_choose_rule.get_current_page().unwrap()) {
            NotebookEnum::CaseSize => {
                rule_type = RuleType::CaseSize;

                rule_data.to_lowercase = true;
                if radio_button_letters_type_uppercase.get_active() {
                    rule_data.to_lowercase = false;
                } else if radio_button_letters_type_lowercase.get_active() {
                    rule_data.to_lowercase = true;
                } else {
                    panic!("Button not available");
                }
                if radio_button_letters_usage_extension.get_active() {
                    rule_place = RulePlace::Extension;
                } else if radio_button_letters_usage_both.get_active() {
                    rule_place = RulePlace::ExtensionAndName;
                } else if radio_button_letters_usage_name.get_active() {
                    rule_place = RulePlace::Name;
                } else {
                    panic!("Invalid Button Clicked");
                }
            }
            NotebookEnum::Purge => {
                rule_type = RuleType::Purge;
                if radio_button_purge_extension.get_active() {
                    rule_place = RulePlace::Extension;
                } else if radio_button_purge_both.get_active() {
                    rule_place = RulePlace::ExtensionAndName;
                } else if radio_button_purge_name.get_active() {
                    rule_place = RulePlace::Name;
                } else {
                    panic!("Invalid Button Clicked");
                }
            }
            NotebookEnum::AddText => {
                rule_type = RuleType::AddText;
                if radio_button_add_text_after_name.get_active() {
                    rule_place = RulePlace::BeforeName;
                } else if radio_button_add_text_before_name.get_active() {
                    rule_place = RulePlace::AfterName;
                } else {
                    panic!("Invalid Button Clicked");
                }
                rule_data.add_text_text = entry_add_text_text_to_add.get_text().to_string();
            }
            NotebookEnum::Trim => {
                rule_type = RuleType::Trim;

                if radio_button_trim_case_sensitive.get_active() {
                    rule_data.case_sensitive = true;
                } else if radio_button_trim_case_insensitive.get_active() {
                    rule_data.case_sensitive = false;
                } else {
                    panic!("Invalid Button Clicked");
                }

                if radio_button_trim_name_start.get_active() {
                    rule_place = RulePlace::FromNameStart;
                } else if radio_button_trim_name_end.get_active() {
                    rule_place = RulePlace::FromNameEndReverse;
                } else if radio_button_trim_extension_start.get_active() {
                    rule_place = RulePlace::FromExtensionStart;
                } else if radio_button_trim_extension_end.get_active() {
                    rule_place = RulePlace::FromExtensionEndReverse;
                } else {
                    panic!("Invalid Button Clicked");
                }
            }

            _ => {
                panic!("Invalid notebook name");
            }
        }
        rule.add_rule(rule_type, rule_place, rule_data);

        // Reset TreeView and populate it again
        populate_rules_tree_view(&tree_view_window_rules, &rule);
    });
}