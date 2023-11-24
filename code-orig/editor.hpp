#ifndef EDITOR_H
#define EDITOR_H

#include "note.hpp"
#include <vector>

class Editor {
public:
    Editor();

    void createNewNote(const std::string& title);
    bool selectNoteByTitle(const std::string& title);
    bool insertTextToSelected(const std::string& text);
    void displaySelectedNote() const;

    bool saveSelectedNoteToFile() const;
    bool loadNoteFromFile(const std::string& filename);

private:
    std::vector<Note> notes;
    int selectedNoteIndex;

    int findNoteIndexByTitle(const std::string& title);
};

#endif
