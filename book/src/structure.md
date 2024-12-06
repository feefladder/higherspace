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

### Technical part

_definition_ **Resilience**: "The word resilience has been originally originated from the Latin word “resiliere,” which means to “bounce back.” The common use of resilience word implies the ability of an entity or system to return to normal condition after the occurrence of an event that disrupts its state." (Hosseini et. al, 2016) However, this does not necessarily apply well to software systems, in which the ideal state is also continuously changing, as well as different in different social contexts. This calls for modular, extensible software and - until AI takes over - continuous human intervention with or development of the software. That is: For the larger system to be resilient in the face of disaster, the software needs to be adaptable, or multi-purpose.



The main research question is:
 **How can an Agroecological farming game as next iteration of the AHF be developed, so that it can be easily embedded in various resilient socio-technical ecosystems**
  
0. What is my personal motivation and metaphysical perspective, from which I will be writing?
1. What scoio-technical environment can serve as a fertile soil for building playful models, in particular an agroecological farming game?
2. Given this environment, which libraries can be created or improved upon?
3. Given these libraries, how do their communities/licensing encourage their use in accordance to FAIR and CARE principles?

 a

1. What scoio-technical environment can serve as a fertile ground for building playful models, in particular the next iteration of the AHF?
   - protection against obsolescense is captured in "fertile ground"
   - "playful models" is a specific version of AI
   - Possible environments and prior art that I playfully engaged with:
     - "Classical" R/Python modelling environment, built on top of high-performance code (FORTRAN/C/C++). Also there is Julia
       - need for a runtime
       - highlight xarray-simlab as a Python modelling environment for fine-grained control over variables
     - C++/GDScript in Godot: <media-tag src="https://cryptpad.disroot.org/blob/e2/e274aef6857ae289ea42712389aea70c7561ea665dd2fbcb" data-crypto-key="cryptpad:HfeyTFqnSTjfqoB2dkBfR6TrVXR7TZ0pHKuEEXnASH8="></media-tag>
     - Rust in Bevy
       - Bevy's scheduler as having very similar functionality to xarray-simlab
     - Cloud-native file formats (Leith, 2024)
3. Given these tools, what can they be surrounded by to encourage its use in accordance to CARE/SPOC principles
   - in particular: How can we _prevent_ it from being used for further ecological (in the broad sense) destruction?
   - reflect on the main sponsor of Bevy being a mining company -> Reach out to LeafWing dev for building accountable relationship

the third is answered by CARE source licensing: How can we ensure that  <- MIR tree for licenses.

<media-tag src="https://cryptpad.disroot.org/blob/e2/e2092075c99f6456424400417730f9c7e8e09d805c372918" data-crypto-key="cryptpad:H6/Akxl+1O21ocKXScLjIWDk/nfzHWSkmZq7ug0tE84="></media-tag>

<media-tag src="https://cryptpad.disroot.org/blob/54/544976d4950986210c8c31fae6663cc26c9432499e2546e3" data-crypto-key="cryptpad:QgjzGtGHJ0YhGPcolQSVD/Plvfi/JiQ6Hg0PzYb1izM="></media-tag>

### Deliverables

short:

- [ ] [tiff2](https://github.com/feefladder/tiff2)
- [ ] [geotiff](https://github.com/georust/geotiff)
- [ ] [rgis](https://github.com/frewsxcv/rgis/issues/124)/[bevy_terrain](https://github.com/kurtkuehnert/bevy_terrain/issues/19)
- [ ] erosion model
  - [ ] how is this different than whitebox tools?
    - Because we built the mycelium
- [ ] licensing
- [ ] The AHF Book aka thesis report

Minimum:
- raster support in rgis
- erosion model
easy extra:
- bevy_terrain?
where?


long:

- [ ] tiff2
  - [x] first iteration: async in tiff
  - [x] get rejected PR
  - [x] discuss about making tiff2
  - [x] make a plan
  - [x] entry
  - [x] ifd
  - [x] image
  - [ ] chunkdecoder
  - [ ] decoder
  - [ ] figure out licensing
- [ ] geotiff
  - [ ] wait for [PR](https://github.com/georust/geotiff/pull/21) #21 coordinate transform to land
  - [ ] build #21 on top of tiff2
- [ ] rgis/bevy_terrain
  - [ ] get to know rgis code
    - [ ] transformation code screen space/map space/raster space
    - [ ] streaming in rasters in Bevy (tile-based)
      - [ ] make data structure (quadtree?)
    - [ ] display (shaders)
- [ ] licensing
  - [ ] [reference doc](https://cryptpad.disroot.org/code/#/2/code/view/YX4fyHFIHoaAdReeq8oay6cmqk2R0xGAc6zsDE0tJLA/present/)
  - [ ] write summary of:
    - [ ] 
- [ ] The AHF Book aka thesis report



week:
- [x] Mailtje/update gestuurd naar Geo luitjes (waar je stage deed) en gevraagd of ze je thesis mede kunnen beoordelen;
- [ ] Aanmelden weer voor Osiris dat je met de thesis bezig bent.
  - [x] deel 1
  - [ ] deel 2: prerequisites
- [ ] RQs nog een keer fris bekeken en scherper/beter geformuleerd;
- [ ] Tiff2 deliverable afgemaakt en een stukje geschreven daarover dat in je thesis zou passen;



vandaag:

- [x] OGH mailen
- [x] chunkdecoder
- [ ] farm story
- [ ] a

## The Digital Ecosystem also known as AI: Rust Roest

I hereby invite the reader on a journey into the Digital Realm. Very much like our Internal Worlds[^1], the digital realm is a crystallization of our imagination. As a human being, I can transform my world, which becomes our world if we create a shared vision of change. Now, I would very much like the physical world to change, as well as the digital world. At a time where my internal world was ¬black and ¬white and existed mainly in geometry, a good friend shared that their internal world was an agroecological farm. I've been growing that image since, because I think if we can picture something as an agroecological farm, it can probably be sustainable. Therefore, please follow along.

Imagine a farm. This can be any farm, or not even a farm at all. But it should be a place where you work with your fellow human and non-human beings for your own (collective) sustenance. Please feel free to create your own picture. For example, if I mention a toolshed, there is no need for there to be a shed necessarily, but at least it should be a place for tools. Now, the physics of this world work quite differently from how they would function in our shared physical world. Most importantly, the fabric of existence is not constructed out of molecules and atoms, but - quite literally - made of memory. As a result, any tool that is made for a specific place - such as a shovel or a spade - only has to be made once. Then, it can be instantenously infinitely copied, so that everyone on the farm can use it at the same time.

Additionally, if we were to be exact in our analogy, the surrounding world doesn't have the ability to do anything they haven't learned and they're "unable" to learn for themselves. That is: they need a teacher which is you. I do not know to which extent this is in the nature of the machine


1: I first heard about this concept from the following dialogue: I asked 

## Global overview of the thesis argument

First, a minute history of the AHF will be presented, along with its death and the process of its re-creation. Within that section, I will include some of my positionality, evolution of the internet and digital realm, as well as a vision of change  I hold where the AHF could play a positive role. I will sway between technical and social language, since there is a strong coupling between technical (im)possibilities and the digital world we are creating. Finally, a more technical deep-dive into digital innovations and Open Science (FAIR-TRUST-CARE) principles will be explored that form the basis of the AHF 2.0. Based on that, the architectural design of the AHF will be explained, followed by recommendations for further research and development. 

Small sidenote on the top paragraph that misplacedly certain vocabulary is used.


Present the workflow in the Introduction:
- Establish a reciprocal relationship with the community that holds me accoutable for the data I use.
- qa
```
Conway's law => reflective Conway's law => sustainable community structure <=> Digital Fourth Way

How I used the 16 guiding principles in my life for a harmonious Self and relationships. Invite the reader to do what mathematicians do and verify them for themselves. How this is very scientific, but untestable.

AI4Indigenet -> reflect on this -> AI = any artificial intelligence, such as also this document holds its own intelligence.
  |\-> Things mentioned as AI are acutally human-made models (Human-made Intelligence)
  |\-> This paper that described fuzzified data, that puts it on the blockchain and criticize it for holding its own ethereum account, thus giving the owner of that wallet a lot of power. (and how it uses petrol as their main example, together with "bidding on data", whereas it is all about sovereignity in the rest of the paper).
   \-> Present the AHF as AI, but more importantly the ecosystem

Digital Fourth way
  |\-> Highlight "the Cloud" as a physical place with a power hierarchy, unless they mean self-hosted cloud
  |\-> compare with CARE principles and how CARE principles are better as a howto white scientist with indigenous data.
   \-> this is the main argument (actually the first version I got from scribd) that links to 


model-based games/playified models -> Distritbuted explorative modeling of farming =>
FAIR-TRUST-CARE principles
  |\-> Software is Data => principles apply to software
  |\-> less focus on the community-building aspect than Digital Fourth Way, more a guide/list of requirements on how a model-based game should be agnostic to data handling so as to allow for CAREful data sharing.
  |     \-> how to reciprocate game/development efforts back to the developer
  |\-> make the design of the game modular and open source; work in-browser
  |\-> Bevy ECS design, which should give easily dumpable data structures into e.g. a GeoPackage
```
PIP-16Principles [comparison](https://cryptpad.disroot.org/code/#/2/code/view/cRb4HMab8p9ciOuEoBtu7xQjhkMGmjufGJF88EMRc0o/present/) 

### Structure of the thesis

this will need some shifting, but for now:

#### Starting from within/prologue

- describe my personal process

### Introduction

- Kop: Nic/Jaapie Goedhart story of Wageningen, ancestral farming, leading to the Wageningen Uni and bla
- romp: Indigenous wisdo
- staart: 



- Death of Flash
- Birth of the Digital Fourth Way and Indigenet "Inoculated seed bombs of development.
- much of the above argumentation

knowledge gap is: The analyzed models (LISEM/FARMDESIGN/) or games (SeGaE/...) are not linked. What digital ecosystem would be needed to place the AFG in that niche?
Additionally, western science on system change lacks the experiential knowledge of peacebuilding, but now I realize that may actually be off-topic. This is actually a very real problem and also 



### Methods

1. Creating a _positive relationship_, based on mutual understanding and trust, where I am accountable and reciprocate with the Digital Fourth Way people
2. Engaging with Farmz2U and developing the game in Unity, then Godot bc DGP (also present the game with screenshots)

1. Creating a _positive relationship_, based on mutual understanding and trust, where I am accountable and reciprocate with the Digital Fourth Way people
3. Above RPC list, analysis of digital landscape (in Soil Ecology terms)
4. Comparison of Bevy as an ECS with xarray-simlab in terms of parallelized process execution (modeling framework).
5. Implementation of the actual tooling
   - ?Schiermonnikoog? - I went to Schiermonnikoog during the Farm Experience Internship and actually discussed modeling that farm as a game, or getting this excel sheet that is shared between agroecological farmers for their innovative pay-as-you-can Community-Supported-Agriculture system.
   - FAO56 excel
6. Engaging in the GeoRust community, asking for license change

### Results

The things were made and not playtested. Possibly show some graphs from example gameplay

### Discussion... and beyond

1. 

- reflexive Conway's Law: We build software as a mirror image of ourselves. Therefore, we should be rooted/embedded in the environment that we are modeling. aka the AHF cannot be made well by me without deeply engaging with the Nigerian context, which I didn't do. Also, finishing the game in that context would have had political implications that I did not want to go forwards on.

- overview of the digital landscape
- design and partial implementation of the AHF in Godot
  - no dependency management in Godot

- CARE principles, digital commons and software licensing
- further development in Bevy/Rust as a data-driven playified modelling environment.


#### The death of Flash, obsolesence and proprietarity in Software

Flash died, this is more worked out in the thesis, 

## Introduction/starting from within

Knowledge gaps: 

## How does
The Farmer Game should be embedded in a fertile social and digital structure.




#### Methodology/Working in a Circle

#### Positionality/In a Sacred Manner <- over-fitting the principles? This should be the starting from within part

</details>


- Adesemowo, A. K., Abayomi-Alli, A., Olabanjo, O. O., Odusami, M. O., Arogundade, O. T., & Abioye, T. E. (2020). Harnessing the Potentials of Mobile Phone for Adoption and Promotion of Organic Farming Practices in Nigeria. In S. K. Sharma, Y. K. Dwivedi, B. Metri, & N. P. Rana (Eds.), Re-imagining Diffusion and Adoption of Information Technology and Systems: A Continuing Conversation (pp. 170–181). Springer International Publishing. https://doi.org/10.1007/978-3-030-64861-9_16
- Clark, A. (2012). Embodied, embedded, and extended cognition. In K. Frankish & W. Ramsey (Eds.), The Cambridge Handbook of Cognitive Science. Cambridge University Press. https://doi.org/10.1017/CBO9781139033916.018
- Deterding, S., Dixon, D., Khaled, R., & Nacke, L. (2011). From game design elements to gamefulness: Defining “gamification.” Proceedings of the 15th International Academic MindTrek Conference: Envisioning Future Media Environments, 9–15. https://doi.org/10.1145/2181037.2181040
- De Domenico, M., & Baronchelli, A. (2019). The fragility of decentralised trustless socio-technical systems. EPJ Data Science, 8(1), 2. https://doi.org/10.1140/epjds/s13688-018-0180-6
- Dulong de Rosnay, M., & Stalder, F. (2020). Digital commons. Internet Policy Review, 9(4), 15 p. https://doi.org/10.14763/2020.4.1530
- Edler, D., Keil, J., & Dickmann, F. (2020). From Na Pali to Earth—An ‘Unreal’ Engine for Modern Geodata? In D. Edler, C. Jenal, & O. Kühne (Eds.), Modern Approaches to the Visualization of Landscapes (pp. 279–291). Springer Fachmedien. https://doi.org/10.1007/978-3-658-30956-5_15
- Hosseini, S., Barker, K., & Ramirez-Marquez, J. E. (2016). A review of definitions and measures of system resilience. Reliability Engineering & System Safety, 145, 47–61. https://doi.org/10.1016/j.ress.2015.08.006
- Johnson, N. W. (1966). Convex Polyhedra with Regular Faces. Canadian Journal of Mathematics, 18, 169–200. https://doi.org/10.4153/CJM-1966-021-8
- Kyriakou, K.-I. D., & Tselikas, N. D. (2022). Complementing JavaScript in High-Performance Node.js and Web Applications with Rust and WebAssembly. Electronics, 11(19), Article 19. https://doi.org/10.3390/electronics11193217
- Haid, H. I. (2023). Exposed Nerves and Archival Impulses: Digital Ruination and the Death of Adobe Flash.
- Kvarnström, A. (n.d.). EVERYTHING, BY EVERYONE.
- Leith, A., Adams, C., Borges, D., & Sundwall, J. (2024). Cloud Native Geospatial: Realizing the Digital Earth Vision. IGARSS 2024 - 2024 IEEE International Geoscience and Remote Sensing Symposium, 6886–6889. https://doi.org/10.1109/IGARSS53475.2024.10642711
- Lewis, J. E., Abdilla, A., Arista, N., Baker, K., Benesiinaabandan, S., Brown, M., Cheung, M., Coleman, M., Cordes, A., Davison, J., Duncan, K., Garzon, S., Harrell, D. F., Jones, P.-L., Kealiikanakaoleohaililani, K., Kelleher, M., Kite, S., Lagon, O., Leigh, J., … Whaanga, H. (2020). Indigenous Protocol and Artificial Intelligence Position Paper [Monograph]. Indigenous Protocol and Artificial Intelligence Working Group and the Canadian Institute for Advanced Research. https://doi.org/10.11573/spectrum.library.concordia.ca.00986506
- Lewis, J. E., Arista, N., Pechawis, A., & Kite, S. (2018). Making Kin with the Machines. Journal of Design and Science. https://doi.org/10.21428/bfafd97b
- Messer, P. W. (2002). Closed-Form Expressions for Uniform Polyhedra and Their Duals. Discrete & Computational Geometry, 27(3), 353–375. https://doi.org/10.1007/s00454-001-0078-2
- Nicholson, B., Nielsen, P., Sæbø, J. I., & Tavares, A. P. (2022). Digital Public Goods for Development: A Conspectus and Research Agenda. In Y. Zheng, P. Abbott, & J. A. Robles-Flores (Eds.), Freedom and Social Inclusion in a Connected World (pp. 455–470). Springer International Publishing. https://doi.org/10.1007/978-3-031-19429-0_27
- Rajpoot, H. C. (2023). Mathematical analysis of regular pentagonal right antiprism. https://doi.org/10.13140/RG.2.2.22879.33444
- Ranaweera, M., & Mahmoud, Q. H. (2024). Deep Reinforcement Learning with Godot Game Engine. Electronics, 13(5), Article 5. https://doi.org/10.3390/electronics13050985
10. Sæbø, J. I., Nicholson, B., Nielsen, P., & Sahay, S. (2021). Digital Global Public Goods (No. arXiv:2108.09718). arXiv. https://doi.org/10.48550/arXiv.2108.09718
- Sharma, P., Martin, M., & Swanlund, D. (2023). MapSafe: A complete tool for achieving geospatial data sovereignty. Transactions in GIS, 27(6), 1680–1698. https://doi.org/10.1111/tgis.13094
- Sharma, P., Martin, M., Swanlund, D., Latham, C., Anderson, D., & Wood, W. (2024). A cloud-based solution for trustless indigenous data sovereignty: Protecting Māori biodiversity management data in Aotearoa New Zealand. Transactions in GIS, 28(4), 836–857. https://doi.org/10.1111/tgis.13153
