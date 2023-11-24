#include "note.hpp"

Note::Note() {
  title = "";
  content = "";
}

Note::Note(const std::string &title) {
  this->title = title;
  content = "";
}

const std::string &Note::getTitle() const { return title; }

const std::string &Note::getContent() const { return content; }

void Note::appendContent(const std::string &text) { content += text; }
