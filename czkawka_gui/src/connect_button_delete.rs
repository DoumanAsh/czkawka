extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use gtk::prelude::*;
use gtk::Align;
use std::collections::BTreeMap;
use std::fs;
use std::fs::Metadata;

// TODO add support for checking if really symlink doesn't point to correct directory/file

pub fn connect_button_delete(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let buttons_delete = gui_data.bottom_buttons.buttons_delete.clone();
    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let window_main = gui_data.window_main.clone();
    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    let tree_view_big_files_finder = gui_data.main_notebook.tree_view_big_files_finder.clone();
    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    let tree_view_temporary_files_finder = gui_data.main_notebook.tree_view_temporary_files_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_zeroed_files_finder = gui_data.main_notebook.tree_view_zeroed_files_finder.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let tree_view_invalid_symlinks = gui_data.main_notebook.tree_view_invalid_symlinks.clone();
    let tree_view_broken_files = gui_data.main_notebook.tree_view_broken_files.clone();
    let check_button_settings_confirm_deletion = gui_data.settings.check_button_settings_confirm_deletion.clone();
    let check_button_settings_confirm_group_deletion = gui_data.settings.check_button_settings_confirm_group_deletion.clone();
    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();

    buttons_delete.connect_clicked(move |_| {
        if !check_if_can_delete_files(&check_button_settings_confirm_deletion, &window_main) {
            return;
        }

        match to_notebook_main_enum(notebook_main.get_current_page().unwrap()) {
            NotebookMainEnum::Duplicate => {
                if !check_button_settings_confirm_group_deletion.get_active() || !check_if_deleting_all_files_in_group(&tree_view_duplicate_finder.clone(), ColumnsDuplicates::Color as i32, &window_main, &check_button_settings_confirm_group_deletion)
                {
                    tree_remove(&tree_view_duplicate_finder.clone(), ColumnsDuplicates::Name as i32, ColumnsDuplicates::Path as i32, ColumnsDuplicates::Color as i32, &gui_data);
                }
            }
            NotebookMainEnum::EmptyDirectories => {
                empty_folder_remover(&tree_view_empty_folder_finder.clone(), ColumnsEmptyFolders::Name as i32, ColumnsEmptyFolders::Path as i32, &gui_data);
            }
            NotebookMainEnum::EmptyFiles => {
                basic_remove(&tree_view_empty_files_finder.clone(), ColumnsEmptyFiles::Name as i32, ColumnsEmptyFiles::Path as i32, &gui_data);
            }
            NotebookMainEnum::Temporary => {
                basic_remove(&tree_view_temporary_files_finder.clone(), ColumnsTemporaryFiles::Name as i32, ColumnsTemporaryFiles::Path as i32, &gui_data);
            }
            NotebookMainEnum::BigFiles => {
                basic_remove(&tree_view_big_files_finder.clone(), ColumnsBigFiles::Name as i32, ColumnsBigFiles::Path as i32, &gui_data);
            }
            NotebookMainEnum::SimilarImages => {
                if !check_button_settings_confirm_group_deletion.get_active()
                    || !check_if_deleting_all_files_in_group(&tree_view_similar_images_finder.clone(), ColumnsSimilarImages::Color as i32, &window_main, &check_button_settings_confirm_group_deletion)
                {
                    tree_remove(
                        &tree_view_similar_images_finder.clone(),
                        ColumnsSimilarImages::Name as i32,
                        ColumnsSimilarImages::Path as i32,
                        ColumnsSimilarImages::Color as i32,
                        &gui_data,
                    );
                    image_preview_similar_images.hide();
                }
            }
            NotebookMainEnum::Zeroed => {
                basic_remove(&tree_view_zeroed_files_finder.clone(), ColumnsZeroedFiles::Name as i32, ColumnsZeroedFiles::Path as i32, &gui_data);
            }
            NotebookMainEnum::SameMusic => {
                if !check_button_settings_confirm_group_deletion.get_active() || !check_if_deleting_all_files_in_group(&tree_view_same_music_finder.clone(), ColumnsSameMusic::Color as i32, &window_main, &check_button_settings_confirm_group_deletion)
                {
                    tree_remove(&tree_view_same_music_finder.clone(), ColumnsSameMusic::Name as i32, ColumnsSameMusic::Path as i32, ColumnsSameMusic::Color as i32, &gui_data);
                }
            }
            NotebookMainEnum::Symlinks => {
                basic_remove(&tree_view_invalid_symlinks.clone(), ColumnsInvalidSymlinks::Name as i32, ColumnsInvalidSymlinks::Path as i32, &gui_data);
            }
            NotebookMainEnum::BrokenFiles => {
                basic_remove(&tree_view_broken_files.clone(), ColumnsBrokenFiles::Name as i32, ColumnsBrokenFiles::Path as i32, &gui_data);
            }
        }
    });
}

pub fn check_if_can_delete_files(check_button_settings_confirm_deletion: &gtk::CheckButton, window_main: &gtk::Window) -> bool {
    if check_button_settings_confirm_deletion.get_active() {
        let confirmation_dialog_delete = gtk::Dialog::with_buttons(
            Some("Delete confirmation"),
            Some(window_main),
            gtk::DialogFlags::DESTROY_WITH_PARENT,
            &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
        );
        let label: gtk::Label = gtk::Label::new(Some("Are you sure that you want to delete files?"));
        let check_button: gtk::CheckButton = gtk::CheckButton::with_label("Ask next time");
        check_button.set_active(true);
        check_button.set_halign(Align::Center);

        let button_box = confirmation_dialog_delete.get_children()[0].clone().downcast::<gtk::Box>().unwrap().get_children()[0]
            .clone()
            .downcast::<gtk::Box>()
            .unwrap()
            .get_children()[0]
            .clone()
            .downcast::<gtk::ButtonBox>()
            .unwrap();

        let button_ok = button_box.get_children()[0].clone();
        button_ok.grab_focus();

        let internal_box = confirmation_dialog_delete.get_children()[0].clone().downcast::<gtk::Box>().unwrap();
        internal_box.add(&label);
        internal_box.add(&check_button);

        confirmation_dialog_delete.show_all();

        let response_type = confirmation_dialog_delete.run();
        if response_type == gtk::ResponseType::Ok {
            if !check_button.get_active() {
                check_button_settings_confirm_deletion.set_active(false);
            }
            confirmation_dialog_delete.hide();
            confirmation_dialog_delete.close();
        } else {
            confirmation_dialog_delete.hide();
            confirmation_dialog_delete.close();
            return false;
        };
    }
    true
}

pub fn check_if_deleting_all_files_in_group(tree_view: &gtk::TreeView, column_color: i32, window_main: &gtk::Window, check_button_settings_confirm_group_deletion: &gtk::CheckButton) -> bool {
    let selection = tree_view.get_selection();
    let (selection_rows, tree_model) = selection.get_selected_rows();
    if selection_rows.is_empty() {
        return false;
    }

    let mut current_selected_row = 0;

    let mut selected_all_records: bool = true;

    if let Some(first_iter) = tree_model.get_iter_first() {
        let current_iter = first_iter;
        if tree_model.get_value(&current_iter, column_color).get::<String>().unwrap().unwrap() != HEADER_ROW_COLOR {
            panic!("First element, should be a header"); // First element should be header
        };

        loop {
            if !tree_model.iter_next(&current_iter) {
                if selected_all_records {
                    break;
                }
                break;
            }

            if tree_model.get_value(&current_iter, column_color).get::<String>().unwrap().unwrap() == HEADER_ROW_COLOR {
                if selected_all_records {
                    break;
                }
            } else if current_selected_row != selection_rows.len() && selection_rows[current_selected_row] == tree_model.get_path(&current_iter).unwrap() {
                current_selected_row += 1;
            } else {
                selected_all_records = false;
            }
        }
    } else {
        return false;
    }

    if !selected_all_records {
        return false;
    } else {
        let confirmation_dialog_group_delete = gtk::Dialog::with_buttons(
            Some("Confirmation of deleting all files in group"),
            Some(window_main),
            gtk::DialogFlags::MODAL,
            &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
        );
        let label: gtk::Label = gtk::Label::new(Some("In some groups there are selected all records."));
        let label2: gtk::Label = gtk::Label::new(Some("Are you sure that you want to delete them?"));
        let check_button: gtk::CheckButton = gtk::CheckButton::with_label("Ask next time");
        check_button.set_active(true);
        check_button.set_halign(Align::Center);

        let button_box = confirmation_dialog_group_delete.get_children()[0].clone().downcast::<gtk::Box>().unwrap().get_children()[0]
            .clone()
            .downcast::<gtk::Box>()
            .unwrap()
            .get_children()[0]
            .clone()
            .downcast::<gtk::ButtonBox>()
            .unwrap();

        let button_ok = button_box.get_children()[0].clone();
        button_ok.grab_focus();

        let internal_box = confirmation_dialog_group_delete.get_children()[0].clone().downcast::<gtk::Box>().unwrap();
        internal_box.add(&label);
        internal_box.add(&label2);
        internal_box.add(&check_button);

        confirmation_dialog_group_delete.show_all();

        let response_type = confirmation_dialog_group_delete.run();
        if response_type == gtk::ResponseType::Ok {
            if !check_button.get_active() {
                check_button_settings_confirm_group_deletion.set_active(false);
            }
        } else {
            confirmation_dialog_group_delete.hide();
            confirmation_dialog_group_delete.close();
            return true;
        }
        confirmation_dialog_group_delete.hide();
        confirmation_dialog_group_delete.close();
    }

    false
}

pub fn empty_folder_remover(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();

    let selection = tree_view.get_selection();

    let (selection_rows, tree_model) = selection.get_selected_rows();
    if selection_rows.is_empty() {
        return;
    }
    let list_store = get_list_store(&tree_view);

    let mut messages: String = "".to_string();

    // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
    for tree_path in selection_rows.iter().rev() {
        let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), column_file_name).get::<String>().unwrap().unwrap();
        let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), column_path).get::<String>().unwrap().unwrap();

        // We must check if folder is really empty or contains only other empty folders
        let mut error_happened = false;
        let mut folders_to_check: Vec<String> = vec![format!("{}/{}", path, name)];
        let mut current_folder: String;
        let mut next_folder: String;
        'dir: while !folders_to_check.is_empty() {
            current_folder = folders_to_check.pop().unwrap();
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(_) => {
                    error_happened = true;
                    break 'dir;
                }
            };

            for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(_) => {
                        error_happened = true;
                        break 'dir;
                    }
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(_) => {
                        error_happened = true;
                        break 'dir;
                    }
                };
                if metadata.is_dir() {
                    next_folder = "".to_owned()
                        + &current_folder
                        + "/"
                        + match &entry_data.file_name().into_string() {
                            Ok(t) => t,
                            Err(_) => {
                                error_happened = true;
                                break 'dir;
                            }
                        };
                    folders_to_check.push(next_folder.clone());
                } else {
                    error_happened = true;
                }
            }
        }

        if !error_happened {
            match fs::remove_dir_all(format!("{}/{}", path, name)) {
                Ok(_) => {
                    list_store.remove(&list_store.get_iter(tree_path).unwrap());
                }
                Err(_) => error_happened = true,
            }
        }
        if error_happened {
            messages += format!("Failed to remove folder {}/{} because folder doesn't exists, you don't have permissions or isn't empty.\n", path, name).as_str()
        }
    }

    text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
    selection.unselect_all();
}

pub fn basic_remove(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();

    let selection = tree_view.get_selection();

    let (selection_rows, tree_model) = selection.get_selected_rows();
    if selection_rows.is_empty() {
        return;
    }
    let list_store = get_list_store(&tree_view);

    let mut messages: String = "".to_string();

    // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
    for tree_path in selection_rows.iter().rev() {
        let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), column_file_name).get::<String>().unwrap().unwrap();
        let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), column_path).get::<String>().unwrap().unwrap();

        match fs::remove_file(format!("{}/{}", path, name)) {
            Ok(_) => {
                list_store.remove(&list_store.get_iter(tree_path).unwrap());
            }
            Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
        }
    }

    text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
    selection.unselect_all();
}

// Remove all occurrences - remove every element which have same path and name as even non selected ones
pub fn tree_remove(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();

    let selection = tree_view.get_selection();

    let (selection_rows, tree_model) = selection.get_selected_rows();
    if selection_rows.is_empty() {
        return;
    }
    let list_store = get_list_store(&tree_view);

    let mut messages: String = "".to_string();

    let mut vec_path_to_delete: Vec<(String, String)> = Vec::new();
    let mut map_with_path_to_delete: BTreeMap<String, Vec<String>> = Default::default(); // BTreeMap<Path,Vec<FileName>>

    // Save to variable paths of files, and remove it when not removing all occurrences.
    for tree_path in selection_rows.iter().rev() {
        let file_name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), column_file_name).get::<String>().unwrap().unwrap();
        let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), column_path).get::<String>().unwrap().unwrap();

        list_store.remove(&list_store.get_iter(tree_path).unwrap());

        map_with_path_to_delete.entry(path.clone()).or_insert_with(Vec::new);
        map_with_path_to_delete.get_mut(path.as_str()).unwrap().push(file_name);
    }

    // Delete duplicated entries, and remove real files
    for (path, mut vec_file_name) in map_with_path_to_delete {
        vec_file_name.sort();
        vec_file_name.dedup();
        for file_name in vec_file_name {
            if fs::remove_file(format!("{}/{}", path.clone(), file_name.clone())).is_err() {
                messages += format!(
                    "Failed to remove file {}/{}. It is possible that you already deleted it, because similar images shows all possible file doesn't exists or you don't have permissions.\n",
                    path, file_name
                )
                .as_str()
            }
            vec_path_to_delete.push((path.clone(), file_name.clone()));
        }
    }

    // Remove only child from header
    if let Some(first_iter) = list_store.get_iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        if tree_model.get_value(&current_iter, column_color).get::<String>().unwrap().unwrap() != HEADER_ROW_COLOR {
            panic!("First deleted element, should be a header"); // First element should be header
        };

        let mut next_iter;
        let mut next_next_iter;
        'main: loop {
            if tree_model.get_value(&current_iter, column_color).get::<String>().unwrap().unwrap() != HEADER_ROW_COLOR {
                panic!("First deleted element, should be a header"); // First element should be header
            };

            next_iter = current_iter.clone();
            if !list_store.iter_next(&next_iter) {
                // There is only single header left (H1 -> END) -> (NOTHING)
                vec_tree_path_to_delete.push(list_store.get_path(&current_iter).unwrap());
                break 'main;
            }

            if tree_model.get_value(&next_iter, column_color).get::<String>().unwrap().unwrap() == HEADER_ROW_COLOR {
                // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                vec_tree_path_to_delete.push(list_store.get_path(&current_iter).unwrap());
                current_iter = next_iter.clone();
                continue 'main;
            }

            next_next_iter = next_iter.clone();
            if !list_store.iter_next(&next_next_iter) {
                // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                vec_tree_path_to_delete.push(list_store.get_path(&current_iter).unwrap());
                vec_tree_path_to_delete.push(list_store.get_path(&next_iter).unwrap());
                break 'main;
            }

            if tree_model.get_value(&next_next_iter, column_color).get::<String>().unwrap().unwrap() == HEADER_ROW_COLOR {
                // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                vec_tree_path_to_delete.push(list_store.get_path(&current_iter).unwrap());
                vec_tree_path_to_delete.push(list_store.get_path(&next_iter).unwrap());
                current_iter = next_next_iter.clone();
                continue 'main;
            }

            loop {
                // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                if !list_store.iter_next(&next_next_iter) {
                    break 'main;
                }
                // Move to next header
                if tree_model.get_value(&next_next_iter, column_color).get::<String>().unwrap().unwrap() == HEADER_ROW_COLOR {
                    current_iter = next_next_iter.clone();
                    continue 'main;
                }
            }
        }
        for tree_path in vec_tree_path_to_delete.iter().rev() {
            list_store.remove(&list_store.get_iter(&tree_path).unwrap());
        }
    }

    // Last step, remove orphan header if exists
    if let Some(iter) = list_store.get_iter_first() {
        if !list_store.iter_next(&iter) {
            list_store.clear();
        }
    }

    text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
    selection.unselect_all();
}
