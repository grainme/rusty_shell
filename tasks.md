TASK 1: PROPER ERROR HANDLING

Create ShellError enum with variants:

DirectoryNotFound
PermissionDenied
CommandNotFound
InvalidPath

Implement Error trait for it
Replace Box<dyn Error> with your error type
Fix cd() to return proper errors

TASK 2: FIX STATE MANAGEMENT

Make current_dir private
Add proper getter method
Make cd() update state safely
Ensure OS and struct state stay in sync

TASK 3: IMPROVE COMMAND STORAGE

Replace HashSet<&'a str> with HashSet<&'static str>
Consider using a simpler static collection
Add method to check if command exists
Separate builtin check from execution

TASK 4: SEPARATE DISPLAY LOGIC

Make pwd() return PathBuf instead of printing
Make type_s() return a result instead of printing
Move all println! to caller
Add proper Display implementation
