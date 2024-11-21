# Thesis structure


#### Exploring the emerging Open Source digital landscape after the deprecation of adobe flash: Implementing an efficient WASM-compatible tiff reader as a stepping stone for playful modelling using cloud-native data.

## Why did we not rewrite the AHF?

1. Covid, geen reis, niet inheems, modelspel zou voornamelijk inzicht moeten geven en gegrond zijn in het vocabulair wat daadwerkelijk gebruikt wordt, en daarvoor was ik er niet genoeg in thuis.
   1. miste context en cosmologie die niet door papers of slechte zoomverbinding kan worden gemaakt (ook ontwijkgedrag door 3). 
   2. programmeren/modelleren bouwt op vocabulair (denk aan uitfasering van master-slave in software)
   3. toch wel vraagtekens bij of ik bij wil(de) dragen aan Farmz2U, omdat ik zeker niet wil bijdragen aan uitbreiding van industriele landbouw positionality
2. te veel op zoek geweest naar een goede architectuur en de hele tijd herschrijven (ook onderdeel van leerproces)
   1. eerder reflectiepunt dan in thesis

text

## What did we do?

I've stayed true to the main focal points of the research question _or_ methodology. That is, I've taken a participative approach to revitalizing the AHF in a new context, as an educational tool.
```
participation => taking wider context/re-evaluating the research questions
```
In this approach, a big search has been for a transdisciplinary (⚧️disciplinary) framework, on the interface of Art, Spirituality and Science. by using a reflective-holistic approach to software design. The first part, titled starting from within will be an autoethnographic overview of discovering my version of the [Great Turning](https://www.activehope.info/book-extracts/three-stories-of-our-times), as I came in contact with Four Worlds Europe. 
- I don't really know yet where to put the argument that this socio-spiritual dimension of software development is actually relevant for my thesis, I think I'll just say that in the intro and then the reader will have to sit through chapter 1 think
- [Phil's statement on AI](https://oneworld.earth/posts/bridging-ancient-traditions-and-technology-ai-guided-by-indigenous-wisdom), [International Treaty to Protect and REstore Mother Earth](https://www.fwii.net/m/blogpost?id=2429082%3ABlogPost%3A164700), [Digital Fourth Way](https://docs.google.com/document/d/1tcp0hSSnMnSV99H6g8kqmGOQCOGjsodN_2reJQIWnuY/edit?pli=1#heading=h.uj4tv9taa4xx) 16 guiding principles.
Then, working in a circle on the technical aspect, I will do the following:
- how I tried to make the AHF (erosion) model-based, but either was not good enough at modeling or didn't choose the right tool for the job.
  - Initial implementation of the AHF as a true-to-original game
  - Secondary attempts at creating a model in Godot, but COWs/GdScript-C++ shenanigans ruined the partey
  - Tertiary implementation of a playified erosion model in Bevy/Rust - Rust all the way, epic dependency management, but young ecosystem

(retroactively made) longer term list of requirements for the AFG (Agroecological Farming Game)

- long-term RPC list (Requirements Preferences Constraints) of the AHF:
  - R: Works using tried-and-tested models (FARMDESING/FAO56/RUSLE-MMF-Lisem)
  - R: Works using real-world, local data, that is readily available and easily adjustable to a farm
  - R: Built using purely Open Source/CARE source software - more on Care Source later
  - R: Uses stylized graphics to stress the artificiality and uncertainty in the simulation
  - R: Internationalization: Allows for easy localization (translation)
  - R: Is embedded in a broader artificial ecosystem where it can benefit from libraries other people made
  - P: Extensibility: Easy to be modified at different levels, allowing for multiple modes of interaction, allowing prosuerism by participitation at multiple levels of programmer-ness
  - P: The modeling equations and words used within the model reflect the language of farming.
  - C: Fits within the timeframe of a thesis
  - C: Should be able to run on the web, in the browser

- Spear-heads for selecting the environment
  - Open Source/Free (CARE source) Software 
  - Extensibility: The ability to build on top of the AHF
  - Ease of setting up the required tooling for building on any system
  - Ease of integration with existing GeoData formats (GeoPackage, GeoTiff, GeoArrow, etc)

- Short term Minimum Valuable Products of the AHF that was actually achieved (or is going to be atm):
  - Bevy/Rust as the environment that fulfills the needs of enabling collaboration and digital-infrastructure-as-epistemology building knowledge on knowledge.
    - One needs to like to learn how computers work to appreciate Rust and by extension Bevy.
  - Walk around in a dynamically loaded terrain
    - Actually refactored large parts of image/tiff repository to allow for streaming in geological data.
  - Export the current state of the game to an sql database/geopackage/series of geotiffs
    sh, proprietarity of Software, Digital Public Goods leading to Open Source
  - zasf
- The initial overview was kind of nice Did make a good plan for reading in Excel data, but dependency management in Godot
HF, since there were many lessons learned there. Maybe make a tool for 

  asdf
  
  
- 1.1: transdisciplinair framework "To Mother Earth, we are All Indigenous" voor onderzoek
  - bidirectional [Conway's Law](https://en.wikipedia.org/wiki/Conway%27s_law) aka "holistic programming"
  - Dat de CARE principles een doel willen aan data, wat poen source mensen niet snapten.
  - International Treaty to Protect and Restore Mother Earth
  - 16 Indigneous guiding principles
  - Digital Fourth Way (draft) `<-` meer achtergrond vanuit 1 hoek over CARE principles
    - Artificial Ecosystems: Het duurt vet lang om een programmeertaal+tools te maken, terwijl daar wel heel veel ontwerpkeuzes aan zitten die beïnvloeden hoe ermee gewerkt kan worden.
    - focus on extensibility .... dat woord voor "inviting software,"

## Chapter layout

- meteorite story or cloud story as introduction (similar to the structure of "The Wetiko Legal Principles")

```
\Ancestral farming through green revolution/ \_ Autoethnography 
 \ Awakening to Climate Change, 3 stories /  /of our time
     \ Meeting Four Worlds through XR  /     Mystical diplomacy
      \WP, Indigenet, CARE principles /      over-arching ⚧️disciplinary
         \     Digital 4th Way,    / 16 principles/PIP: Conway's bla
          \    AHF Flash story    /  Obsolescence, sustainability of Open Source
             |   AHF in Godot   |
             |  Workflow design |
             |    Bevy/Rust     |
             |   Indigelicense  |
           /                      \
        /Let the organizing principle\
       /      Of the Universe         \
      /       Do it's work             \
     /Respect the uncertainty principle \
```

1. Motivation
2. Background/Soil/Introduction
   1. divide further
3. Technical part/methods
   1. environment? (as soil ecosystem of a farm)
   2. object (what do we make?)
   3. inoculation (what to surround it by?)
4. results
5. conclusion
