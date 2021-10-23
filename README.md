# keylogger-rs -- Simple Linux + Windows Keylogger
## Spent so long over-engineering this but it doesn't log to anywhere

I was planning to have the typed words be sent to a remote server, but you can implement that yourself if you'd like!

### Detected on Windows, not sure about Linux
Is also detected on Windows after it was 'cloud-scanned', so _you will have to put in some extra work_ if you want this FUD.

__Please leave an issue if you found this and want me to fix it/make it better__

## Waffle
Someone please fork this because I overhauled this like 5 times trying to make it work with backspaces and cursor positions.

One thing I would like to implement is ikeeping track of moving back and forth words as well as deleting words. The current word is stored in the `key_buf` so you should be able to get that working.

## Features
- Keeps track of cursor position
- Will wait until the enter key is pressed to 'send' the words.
- Works on windows but you will have add/ask me to get it working lol 

I'll add some comments for readability etc...

All in all a fun rust project everyone should try
