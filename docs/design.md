This document is now outdated.

# Definition
## Mod
A __mod__ is a content that modify a __game__. It is composed of __entrys__ (vary based on the __game__), and __files__ (Are worth special treatment due to the size they take).
### Entry
An __entry__ is caracterized by three value: The table it effect, its ID (may either be a string or an number. The ID is unique, and may be used to indiquate a unique entry) and the __entry data__ by themselves.
### Entry data
An __entry data__ contain the data that associated to an __entry__. For exemple, if we want to modify/create a dialog, we will add, in the table "Dialogue", with the id "greeting_rust", the __entry data__ {"text": "Hello, I'm someone"}.
### Table
A __table__ is a list (or rather a dictionnary) of __entry__
### File
Explicit name. __File__ are stored in a special folder (maybe mod_dir/file... TODO:). They just replace file of a __game__. They may have two kind of __file__ added by a __mod__: __files__ that will be directely copied to __game__ file system, or __file__ that will need to be compiled.
## Load order
__Mods__ have a load order. Each __mod__ override the __entry__ of previous __mods__ if they override. They can also decide to delete an __entry__. The last __mod__ in the __load order__ are the __mod__ that is currently edited if in the editor.
## Game
A __game__ is what define: How to compile a __mod__, what are existing __table__, and the form the __entrys__ and __entrys data__ should take. I plan to add some sort of modularity to this (but I don't know how, will probably be something to worry in some later stage).
# Implementation
## ModList
contain the list of mods and their load order:
mods: _Vec_<_Rc_<_Mod_>>
## game
game_tables: _HashMap_<_Rc_<_EntryData_>>
