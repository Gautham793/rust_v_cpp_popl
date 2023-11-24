pub mod note;
pub mod editor;

use crate::editor::Editor;

pub fn main() {
    let mut editor = Editor::new();

    // Run the tests

    // Test Case 1: Create a New Note and Insert Text
    editor.create_new_note("Note 1");
    editor.insert_text_to_selected("Hello, world!");
    editor.display_selected_note();
    editor.save_selected_note_to_file();

    // Test Case 2: Create Multiple Notes
    editor.create_new_note("Note 2");
    editor.create_new_note("Note 3");

    // Test Case 3: Select a Note by Title
    editor.select_note_by_title("Note 1");

    // Test Case 4: Insert and Append Text to Selected Note
    editor.insert_text_to_selected("This is the first note. ");
    editor.insert_text_to_selected("It's a beautiful day!");
    editor.display_selected_note();

    // Test Case 5: Save a Note to a File
    editor.create_new_note("Note 2");
    editor.insert_text_to_selected("This is a saved note.");
    editor.save_selected_note_to_file();

    // Test Case 6: Undo the Last Edit
    editor.undo_last_edit();

    // Test Case 7: Handle Note Not Found
    if !editor.select_note_by_title("Non-existent Note") {
        println!("Note with title \"Non-existent Note\" not found.");
    }

    // Test Case 8: Load Non-existent File
    if !editor.load_note_from_file("Non-existentNote.txt") {
        println!("File \"Non-existentNote.txt\" not found.");
    }

    // Test Case 9: Create and Manage Multiple Notes (no specific output)

    // Test Case 10: Test Large Amount of Text
    editor.create_new_note("Large Text");
    let lorem_ipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
    // Insert a large amount of text (multiple times to test large text handling)
    for _ in 0..500 {
        editor.insert_text_to_selected(lorem_ipsum);
    }

    // Test Case 11: Load a Note with Special Characters
    editor.create_new_note("Special Characters");
    editor.insert_text_to_selected("Note with special characters: #@$%!");

    // Display the contents of the special characters note
    editor.display_selected_note();
}



