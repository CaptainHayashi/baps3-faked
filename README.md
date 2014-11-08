# ury-faked

The URY implementation of BAPS3 takes the form of a *server stack*: a BAPS3 service is implemented by adding features on top of a simpler BAPS3 service where possible.  Testing these server stacks requires setting up an entire chain of working servers, and having access to all the dependencies thereof (music files, sound card, development environments, libraries, etc.)

**faked** is a *fake* BAPS3 service server.  It claims to implement various BAPS3 services, and responds to requests for those services as if it were actually a genuine service.  However, it only *pretends*: it doesn't load or play out music, it doesn't seek, etc.

Uses include:

* Testing higher-level parts of BAPS3 ([listmaster], [rapid], etc)
* An example of how to use [baps3-protocol.rs]
* Proof of concept of BAPS3 modularity (being able to mix and match service implementations)
* Dummy output/using BAPS3 without a sound card
* Astound your friends!  Confound your enemies!

(Implementation forthcoming)

[listd]: https://github.com/UniversityRadioYork/listmaster
[ury-rapid]: https://github.com/UniversityRadioYork/ury-rapid
