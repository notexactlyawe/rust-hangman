Rust Hangman
============

Classic hangman game written to learn Rust.

Rules of Hangman
----------------

 - Game randomly selects a word
 - User inputs a character
 - If the character is in the word, the user is shown where and offered the chance to guess again
 - This continues until either the user guesses every character in the word or they run out of guesses

Word Selection
--------------

The word to guess will be selected randomly from a modified version of [this gist](https://gist.github.com/deekayen/4148741).

TODO
----

 - Stop user from guessing same character again
 - Show guessed characters
 - Change "guesses" to "guess" for 1 guess
 - If a user inputs a word, it shouldn't treat the first character of that word as a guess
