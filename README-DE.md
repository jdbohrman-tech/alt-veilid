# Willkommen bei Veilid

-   [Aus der Umlaufbahn](#aus-der-umlaufbahn)
-   [Betreibe einen Node](#betreibe-einen-node)
-   [Entwicklung](#entwicklung)

## Aus der Umlaufbahn

Als Erstes wird die Frage beantwortet "Was ist Veilid?". Die beste Beschreibung ist, dass Veilid ein Peer-to-Peer-Netzwerk zum Teilen von verschiedenen Arten von Daten ist.

Veilid wurde mit der Idee im Hinterkopf entworfen, dass jeder Benutzer seine eigenen Inhalte im Netzwerk speichern kann. Aber es ist auch möglich diese mit anderen ausgewählten Leuten zu teilen oder (wenn gewollt) auch mit gesamten Rest der Welt.

Der primäre Zweck des Veilid Netzwerks ist es eine Infrastruktur für eine besondere Art von geteilten Daten zur Verfügung zu stellen: Soziale Medien in verschiedensten Arten. Dies umfasst sowohl leichtgewichtige Inhalte wie Twitters/Xs Tweets oder Mastodons Toots, mittleschwere Inhalte wie Bilder oder Musik aber auch schwergewichtige Inhalte wie Videos. Es ist eben so beabsichtigt Meta-Inhalte (wie persönliche Feeds, Antworten, private Nachrichten und so weiter) auf Basis von Veilid laufen zu lassen.

## Betreibe einen Node
Der einfachste Weg dem Veilid Netzwerk beim Wachsen zu helfen ist einen eigenen Node zu betreiben. Jeder Benutzer von Veilid ist automatisch ein Node, aber einige Nodes helfen dem Netzwerk mehr als Andere. Diese Nodes, die das Netzwerk unterstützen sind schwergewichtiger als Nodes, die Nutzer auf einem Smartphone in Form eine Chats oder eine Social Media Applikation starten würde.Droplets oder AWS EC2 mit hoher Bandbreite, Verarbeitungsressourcen und Verfügbarkeit sind wesentlich um das schnellere, sichere und private Routing zu bauen, das Veilid zur Verfügung stellen soll.

Um einen solchen Node zu betreiben, setze einen Debian- oder Fedora-basierten Server auf und installiere den veilid-server Service. Um dies besonders einfach zu machen, stellen wir Paketmanager Repositories als .deb und .rpm Pakete bereit. Für weitergehendene Information schaue in den [Installation](./INSTALL.md) Leitfaden.

## Entwicklung
Falls Du Lust hast, dich an der Entwicklung von Code oder auch auf andere Weise zu beteiligen, schau bitte in den [Mitmachen](./CONTRIBUTING.md) Leitfaden. Wir sind bestrebt dieses Projekt offen zu entwickeln und zwar von Menschen für Menschen. Spezifische Bereiche in denen wir nach Hilfe suchen sind:

* Rust
* Flutter/Dart
* Python
* Gitlab DevOps und CI/CD
* Dokumentation
* Sicherheitsprüfungen
* Linux Pakete
