#ifndef NOTE_H
#define NOTE_H

#include <string>

class Note {
public:
    Note();
    Note(const std::string& title);

    const std::string& getTitle() const;
    const std::string& getContent() const;

    void appendContent(const std::string& text);

private:
    std::string title;
    std::string content;
};

#endif
