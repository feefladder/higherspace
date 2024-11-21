1. What scoio-technical environment can serve as a fertile soil for building playful models, in particular an agroecological farming game?

**The focus is actually on being able to load in raster data to run an erosion model.**

For answering this question, four game engines: Unreal, Unity, Godot, and Bevy were compared based on the following:


|Engine|Progrgamming Language(s)|Wasm support|Dependency management|stability|documentation|open source
|:-----|:-----------------------|:-----------|:--------------------|:--------|:------------|--|
|Unreal|C++                     |✗           |C++ style            | stable  | |✗|
|Unity |C#                      |✓           |Asset store, plugins | stable  | |✗|
|Godot |GDScript, C++; Python/Rust[^1]|✓     |C++ style, plugin system | stable, fast development|✓|
|Bevy  |Rust                    |✓           |Cargo                | unstable| |✓|
|Fyrox |Rust                    |✓           |Cargo                | unstable| |✓|

[^1]: Python, Rust and other languages are community-maintained plugins

Unreal doesn't target the web, so is not applicable for our use-case. Unreal and Unity are both not open source, so they are both not fit for our use-case. For choosing between Bevy and Godot, a closer look at dependency management is required.

> A low-level language is one whose programs require attention to the irrelevant.
> -- Alan perlis

In our farm analogy, we are able to farm and sustain ourselves because the surrounding (soil) ecosystem functions well. Since programs have different architectural layers as well as soil, we'll say that the game engine, embedded in its programming language and related tooling is the soil. That is: below the soil are millions of organisms (programmers) that make the ecosystem work. Any programmer has soil under them: A game developer walks on game engines; a game engine developer walks on programming language and the standard library; programming languague developers (compiler-builders) use hardware and often the presence of an operating system and OS developers stand both on the shoulders of giants, as well as on hardware which has both a physical (mineral) nature as well as that it has also been completely designed. When choosing where to plant our seed, it becomes important to also consider where the roots can grow.

### Godot

#### Programming Languages

Godot is developed in _C++_, and has made its own scripting language _GDScript_, which works more like C++ and looks like Python. This allows for faster prototyping, since C++ is an unsafe, more low-level language. Since Godot 4.0, the way C++ code can be added has changed, from [GDNative]() to [GDExtension](), where [C++]() [modules]() has remained the same. For our use-case of being able to read Cloud-Optimized GeoTIFFs directly into the game, we would need to link against [libtiff]() or [tinytiff](). When I proposed adding tiff support to the core engine, I received the following answer:

> Not enough interest and tiff is a rather large specification



#### Dependency management

Godot has a built-in asset library, which allows for easily adding dependencies _one level down_. That is: If I need a `shovel` and a `rake`, and both the `shovel` and `rake` require `stick`, I need to  additionally install `stick`. For a `rake` and `shovel`, this may be simple, but e.g. a `tractor` has many more moving parts. What is then often done, is to fully implement the `tractor` in a single plugin.

### Bevy


### Fyrox
