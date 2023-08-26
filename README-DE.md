# Willkommen bei Veilid

-   [Aus der Umlaufbahn](#aus-der-umlaufbahn)
-   [Betreibe einen Node](#betreibe-einen-node)
-   [Entwicklung](#entwicklung)

## Aus der Umlaufbahn

Als Erstes wird die Frage behandelt "Was ist Veiled?". Die allgemeinste Beschreibung ist, dass Veilid ein Peer-to-Peer-Netzwerk zum Teilen von verschiedenen Arten von Daten ist.

Veilid wurde mit der Idee im Hinterkopf entworfen, dass jeder Benutzer seine eigenen Inhalte im Netzwerk speichern kann. Aber es ist auch möglich diese mit anderen ausgewählten Leuten zu teilen oder (wenn gewollt) auch mit gesamten Rest der Welt.

Der primäre Zweck des Veild Netzwerks ist es Infrastruktur für eine besondere Art von geteilten Daten zur Verfügung zu stellen: Social Medien in verschiedensten Arten. Dies umfasst leichtgewichtige Inhalte wie Twitters/Xs Tweets oder Mastodons Toots, mittleschwere Inhalte wie Bilder oder Lieder und schwergewichtige Inhalte wie Videos. Es ist eben so beabsichtigt Meta-Inhalte (wie persönliche Feeds, Antworten, private Nachrichten und so weiter) auf Basis von Veilid laufen zu lassen.

## Betreibe einen Node
Der einfachst Weg dem Veilid Netzwerk beim Wachsen zu helfen ist einen eigenen Node zu betreiben. Jeder Benutzer von Veilid ist ein Node, aber einige Nodes helfen dem Netzwerk mehr als Andere. Diese Nodes, die das Netzwerk unterstützen sind schwergewichtiger als Nodes, die Nutzer auf einem Smartphone in Form eine Chats oder eine Social Media Applikation starten würde.Droplets oder AWS EC2 mit hoher Bandbreite, Verarbeitungsressourcen und Verfügbarkeit sind wesentlich um das schneller, sicher und private Routing zu bauen, das Veilid zur Verfügung stellen soll.

Um einen solchen Node zu betreiben, setze einen Debian- oder Fedora-basierten Server auf und installiere den veilid-server Service. Um diese besonders leicht zumachen, stellen wie Paket Manager Repositories for .deb und .rpm Pakete bereit. Für weitergehenden Information schaue in den [Installation](./INSTALL.md) Leitfaden.

## Entwicklung
Falls Du dazu tendierst sich an Entwicklung von Code und Nicht-Code Entwicklung zu beteiligen, schau bitte in den [Beteiligung](./CONTRIBUTING.md) Leitfaden. Wir sind bestrebt diese Projekt offen zu entwickeln und zwar von Leute für Leute. Spezifische Bereiche in denen wir nach Hilfe suchen sind:

* Rust
* Flutter/Dart
* Python
* Gitlab DevOps und CI/CD
* Dokumentation
* Sicherheitsprüfungen
* Linux Pakete
