These are the terms that will be used in the Rhotic Text Editor.

### Window
A window is the part of the screen that displays the rhotic program.

### Stage 
A stage is the emacs equivalent of a mode. A Stage can be anything from a text buffer, where you edit text, to a graphical interface.

### Package
A Package can be thought of as a mini program that can be installed within Rhotic. Just like emacs or any linux package manager.

### Store
A stage where packages can be downloaded, and thus installed.

# Minibuffer
The minibuffer is a little text bar at the bottom of the window, (not stage window) where the command being entered shows.
It also shows other status information.

# Function
This is a regular function that operates on a stage. The function is defined as a closure, so there is limided side-effects.
It can also not DIRECTLY operate on things outside of its scope, such as another stage, or the Rhotic global state. 
However, side effects can be implemented on rhotic globally, to achieve intended effects.
