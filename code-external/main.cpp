#include "editor.hpp"
#include <iostream> 
#include <chrono>

using namespace std::chrono;

int main() {
  // start timer
  auto start = high_resolution_clock::now();
  
  Editor editor;


  editor.createNewNote("Note 1");
  editor.insertTextToSelected("Hello, world!");
  editor.displaySelectedNote();
  editor.saveSelectedNoteToFile();

  editor.createNewNote("Note 2");
  editor.createNewNote("Note 3");


  editor.selectNoteByTitle("Note 1");


  editor.insertTextToSelected("This is the first note. ");
  editor.insertTextToSelected("It's a beautiful day!");
  editor.displaySelectedNote();


  editor.createNewNote("Note 2");
  editor.insertTextToSelected("This is a saved note.");
  editor.saveSelectedNoteToFile();


  editor.createNewNote("Note 3");
  editor.loadNoteFromFile("notes/Note 2.txt");


  if (!editor.selectNoteByTitle("Non-existent Note")) {
    std::cout << "Note with title \"Non-existent Note\" not found." << std::endl;
  }


  if (!editor.loadNoteFromFile("Non-existentNote.txt")) {
    std::cout << "File \"Non-existentNote.txt\" not found." << std::endl;
  }

  editor.createNewNote("Large Text");
  std::string loremIpsum =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";

  for (int i = 0; i < 500; i++) {
    editor.insertTextToSelected(loremIpsum);
  }


  editor.createNewNote("Special Characters");
  editor.insertTextToSelected("Note with special characters: #@$%!");


  editor.displaySelectedNote();

  // end timer
  auto stop = high_resolution_clock::now();

  auto duration = duration_cast<microseconds>(stop - start);

  std::cout << "----------------------------------------\n";
  std::cout << "Time taken by function: " << duration.count() << " microseconds\n";

  return 0;

  return 0;
}
