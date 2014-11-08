# ury-faked

The URY implementation of BAPS3 takes the form of a *server stack*: a BAPS3 service is implemented by adding features on top of a simpler BAPS3 service where possible.  Testing these server stacks requires setting up an entire chain of working servers, and having access to all the dependencies thereof (music files, sound card, development environments, libraries, etc.)

**faked** is a *fake* BAPS3 service server.  It claims to implement various BAPS3 services, and responds to requests for those services as if it were actually a genuine service.  However, it only *pretends*: it doesn't load or play out music, it doesn't seek, etc.

(Implementation forthcoming)
