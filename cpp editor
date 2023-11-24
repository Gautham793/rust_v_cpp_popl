#include "editor.hpp"
#include <fstream>
#include <iostream>

Editor::Editor() {
  selectedNoteIndex = -1;
}

void Editor::createNewNote(const std::string &title) {
  notes.emplace_back(title);
  selectedNoteIndex = notes.size() - 1;
}

bool Editor::selectNoteByTitle(const std::string &title) {
  int index = findNoteIndexByTitle(title);
  if (index != -1) {
    selectedNoteIndex = index;
    return true;
  }
  return false;
}

bool Editor::insertTextToSelected(const std::string &text) {
  if (selectedNoteIndex != -1) {
    notes[selectedNoteIndex].appendContent(text);
    return true;
  }
  return false;
}

void Editor::displaySelectedNote() const {
  if (selectedNoteIndex != -1) {
    std::cout << "Title: " << notes[selectedNoteIndex].getTitle() << std::endl;
    std::cout << notes[selectedNoteIndex].getContent() << std::endl;
  }
}

bool Editor::saveSelectedNoteToFile() const {
  if (selectedNoteIndex != -1) {
    const Note &note = notes[selectedNoteIndex];
    std::string filename = "notes/" + note.getTitle() + ".txt";
    std::ofstream file(filename);
    if (file.is_open()) {
      file << note.getContent();
      file.close();
      return true;
    }
  }
  return false;
}

bool Editor::loadNoteFromFile(const std::string &filename) {
  std::ifstream file(filename);
  if (file.is_open()) {
    std::string title = filename.substr(filename.find_last_of("/\\") + 1);
    title = title.substr(0, title.find_last_of("."));
    notes.emplace_back(title);
    Note &note = notes.back();
    note.appendContent(std::string(std::istreambuf_iterator<char>(file),
                                   std::istreambuf_iterator<char>()));
    selectedNoteIndex = notes.size() - 1;
    file.close();
    return true;
  }
  return false;
}

int Editor::findNoteIndexByTitle(const std::string &title) {
  for (int i = 0; i < notes.size(); i++) {
    if (notes[i].getTitle() == title) {
      return i;
    }
  }
  return -1;
}
