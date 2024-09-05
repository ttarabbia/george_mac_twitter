# George MacDonald Twitter Bot

This repo reads in the entirety of George Macdonald's catalogue, splits out each book, and finds interesting quotes to tweet out.

Currently it has a hardcoded file name, hardcoded delimiter between each book, and requires authentication every time you run it.

Future possible features:

- Set a schedule to tweet and keep the process running with refreshing oauth tokens.
- Randomly choose a book or go through books N tweets at a time
- Swap out the model for a local Ollama model with long context window.
- Break up the book into pieces based on the chosen model's context window.
- Check each excerpt to make sure it exists in the book exactly.
- Make better prompts
