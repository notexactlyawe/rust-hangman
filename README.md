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
