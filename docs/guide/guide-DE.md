# Frühe A Dokumente

# Bitte nicht öffentlich teilen

# Veilid Architektur Leitfaden

-   [Aus der Umlaufbahn](#aus-der-umlaufbahn)
-   [Aus der Vogelperspektive](#vogelperspektive)
    -   [Peer Netzwerk zum Datenspeichern](#peer_netzwerk_zur_datenspeicherung)
    -   [Block Speicher](#block-speicher)
    -   [Key-Value Speicher](#key-value-speicher)
    -   [Datenstrukturierung](#datenstrukturierung)
    -   [Peer- und Benutzeridentät](#peer-und-benutzeridentität)
-   [Am Boden](#am_boden)
    -   [Peer Netzwerk im Detail](#peer-netzwerk-im-detail)
    -   [Benutzerprivatsphäre](#benutzerprivatsphäre)
    -   [Block Speicher im Detaail](#block-store-im-detail)
    -   [Key-Value Speicher im Detail](#key-value-speicher-im-detail)

## Aus der Umlaufbahn

Als Erstes wird die Frage behandelt "Was ist Veiled?". Die allgemeinste Beschreibung ist, dass Veilid ein Peer-to-Peer-Netzwerk zum Teilen von verschiedenen Arten von Daten ist.

Veilid wurde mit der Idee im Hinterkopf entworfen, dass jeder Benutzer seine eigenen Inhalte im Netzwerk speichern kann. Aber es ist auch möglich diese mit anderen ausgewählten Leuten zu teilen oder (wenn gewollt) auch mit gesamten Rest der Welt.

Der primäre Zweck des Veild Netzwerks ist es Infrastruktur für eine besondere Art von geteilten Daten zur Verfügung zu stellen: Social Medien in verschiedensten Arten. Dies umfasst leichtgewichtige Inhalte wie Twitters/Xs Tweets oder Mastodons Toots, mittleschwere Inhalte wie Bilder oder Lieder und schwergewichtige Inhalte wie Videos. Es ist eben so beabsichtigt Meta-Inhalte (wie persönliche Feeds, Antworten, private Nachrichten und so weiter) auf Basis von Veilid laufen zu lassen.

* * *

## Vogelperspektive

Nachdem wir nun wissen was Veilid ist und was in Veilid abgelegt werden sollte, ist es nun an der Zeit die Teile zu behandeln, die erklären wie Veilid dies ermöglicht. Natürlich nicht super detailiert (das kommt später), sondern her auf eine mittleren Detailgrad so dass alles auf einmal zur gleichen Zeit vom Leser verstanden werden kann.

### Peer Netzwerk zur Datenspeicherung

Auf unterster Ebene ist Veilid ein Netzwerk aus "Peers", die miteinander über das Internet kommunizieren. Peers senden sich gegenseitig Nachrichten (sogenannte Remote Procedure Calls) bezüglich der im Netzwerk ge- bzw. zu speichernden Daten und auch Nachrichten über das Netzwerk selbst. Zum Beispiel, kann eine Peer einen Anderen nach einer Datei fragen oder nach Informationen fragen, welche anderen Peers in Netzwerk bekannt sind/existieren.

Die Daten, die im Netzwerk gespeichert sind, werden in zwei Arten von Daten unterteilt: Datei-artige Daten, die üblicherweise groß sind und textartige Daten, die üblicherweise klein sind. Jede Art wird in einem eigenen Untersystem gespeichert, dass das so gestaltet wurde, dass es für diese Art von Daten optimal ist.

### Block Speicher

Datei-artige Inhalte werden in einem inhaltsaddressierbaren Block Speicher abgelegt. Jeder Block ist einfach ein Haufen (Blob) aus irgendwelchen Daten (z.B. ein JPEG oder ein MP4) beliebiger Größe. Die Prüfsumme (Hash) eines Block dient als eindeutige ID für diesen Block und kann von anderen Peers benutzt werden, um diesen Block abzufragen. Technisch gesehen können auch textuelle Daten als Block gespeichert werden und das sollte genau dann passieren, wenn die textuelle Daten als eine Art Dokument oder Datei eines bestimmten Typs verstanden werden.

### Key-Value Speicher

Kleinere und kurzlebigere textuelle Inhalte werden in einem Key-Value Speicher gespeichert (KV Speicher).Sachen wie z.B. Status Updates, Blogeinträge oder Benutzerbeschreibungen usw. sind alle dafür gedachte in diesem Teil des Datenspeichers gespeichert zu werden. KV Speicher Daten sind nicht einfach "im Veild Netzwerk", sondern gehören Benutzern und (werden auch von diesen gesteuert/kontrolliert). Sie werden über einen beliebigen vom Eigentümer der Daten gewählten Namen identifiziert. Jede Gruppe von Benutzern kann Daten hinzufügen, aber man kann nur die Daten ändern, die man selbst hinzugefügt hat.

Nehmen wir 2 beispielhafte Benutzer Boone und Marquette: Boones Benutzerbeschreibung und ihren Blogpost mit dem Titel "Hi, ich bin Bonne!" sind 2 Sachen, die dem selben Benutzer gehören, aber unterschiedliche ID haben. Boones Benutzerbeschreibung und Marquettes Benutzerbeschreibung sind 2 Sachen, die zwei unterschiedlichen Benutzer gehören, aber dieselbe ID haben.

KV Speicher Daten sind zustandsbehaftet, so dass Änderungen an ihnen vorgenommen werden könen. Boones Benutzerbeschreibung z.B. wird sicher nicht unverändert bleiben, sondern sich eher im Laufe der Zeit ändern, wenn er z.B. den Job wechselt oder sich neue Hobbies aneignet usw.. Die Zustandsbehaftung zusammen mit den beliebigen nutzer-definierten Identifiern (anstatt Prüfsummen auf Inhalte (Content Hashes)) führt dazu,dass man Boones Benutzerbeschreibung als abstrakte Sache betrachten kann und sich für Update auf diese registrieren kann.

### Datenstrukturierung

Mit der Verbindung aus Block Speicher und Key_Value Speicher ist es mögliche auch komplexe Konzepte zu kombinieren. Ein Song könnte z.B. an zwei Orten in Veilid verteilt abgelegt sein. Der Block Speicher würde dabei die Rohdaten speichern und der Key-Value Speicher würde eine Beschreibung zur Idee des Song abspeichern. Diese wäre vielleicht in Form eines JSON Objekts mit Metadaten über den Song wie dem Titel, den Komponisten, das Datum, die Codierungsinformationen usw. ebenso wie die ID der Daten im Block Speicher. Wir können dann auch verschiedene Versionen der JSON Daten abspeichern, wenn das Stück aktualisiert, verbessert, überarbeitet oder was auch immer wird. Jede Version würde auf einen anderen Block im Block Speicher verweisen. Es ist immer noch "derselbe Song" aus konzeptueller Sicht und somit hat er auch dieselbe ID im KV Speicher, aber die Rohbits, die damit verbunden sind, sind für jede Version unterschiedlich.

Ein anderes Beispiel (wenn auch mit einer noch etwas schwächeren Verbindung zum Block Speicher) wäre die Beschreibung eines Profilbildes. "Marquettes Profilbild" ist eine sehr abstrakte Beschreibung und genau genommen sind können sich die Bits, die damit zusammenhängen, im Laufe der Zeit stark variieren. So könnte es nicht nur verschiedene Versionen des Bildes, sondern auch komplett andere Bilder geben. Vielleicht ist es an einem Tage eine Foto von Marquette und am nächsten Tag ist es das Foto einer Blume.

In Soziale Medien finden sich viele Beispiel für solche Konzepte: Freundeslisten, Blocklisten, Indizes von Postings und Favoriten-Listen. Die sind alle zustandsbehaftete Beschreibungen einer bestimmten Art: Eine stabile Referenz auf eine Sache, aber der genaue Inhalt der Sache verändert sich im Laufe der Zeit. Das ist genau das, was wir im KV Speicher ablegen wollen und sich somit vom Block Store abgrenzt, auch wenn diese Daten auf die Inhalte des Block Store referenzieren.

### Peer- und Benutzeridentität

Es gibt zwei Darstellungen von Identitäten im Netzwerk: Die Peer- und die Benutzeridentität. Die Peeridentität ist einfach genug: Jeder Peer hat ein kryptografisches Schlüsselpaar, das er benutzt, um mit anderen Peers sicher zu kommunizieren und zwar für beides: Traditionelle verschlüsselte Kommuniktion und auch durch verschiedene verschlüsselte Routen. Die Peeridentität ist nur die ID einer bestimmten Instanz der Veilid Software, die auf einem Computer läuft.

Die Benutzeridentität wird deutlich umfassender genutzt. Benutzer (also Leute) wollen auf das Veilid Netzwerk zugreifen, so dass sie eine konsistente Identität über Geräte und Apps/Programme hinweg haben. Da aber Veilid keine Server in traditionellen Sinne hat, können wir nicht auf das normale Konzept von Benutzer-"Konten" zurückgreifen. Würde man das tun, würden Zentralisierungsspunkte im Netzwerk eingeführt werden, die in klassischen System oft die Quelle von Problemen gewesen sind. Viele Mastodon Benutzer habe sich schon in schwierigen Situationen befunden, wenn Ihre Instanzen Schwierigkeiten mit Ihren Sysadmin hatten und diese dann plötzlich die Instanzen abschalteten ohne vorher genug zu warnen.

Um diese Re-Zentralisierung von Identitäten zu vermeiden, nutzen wir kryptografische Identitäten auch für alle Benutzer. Das Schlüsselpaar den Benutzers wird zum Signieren und Verschlüsseln ihrer Inhalte benutzt, wie es notwendig ist, wenn man Inhalte auf den Datenspeicher veröffentlichen will. Ein Benutzer wird als "in seine Client App/Anwendung eingeloggt" bezeichnet, wenn die App/Anwendung eine Kopie seines privaten Schlüssels hat. Wenn man in eine Client App eingeloggt, dann verhält sie sich wie jede andere der Client Apps des Benutzers und ermöglicht es ihm Inhalte zu ent- und verschlüsseln,Nachrichten zu signieren und so weiter. Schlüssel können in neue Apps eingefügt werden, um sich in diese einzuloggen. Dies erlaubt dem Benutzer so viele Client Apps (auf jeder beliebigen Anzahl an Geräten zu haben) wie sie haben wollen.

* * *

## Am Boden

Die Vogelperspektive macht es möglich alles auf einmal im Kopf zu behalten, dafür lässt sie aber viele Implementierungsdetails weg. Deswegen ist es jetzt an der Zeit auf den "Boden" zurückzukehren und sich die Hände schmutzig zu machen. Grundsätzlich sollten es genug Informationen sein, um eine System ähnliche wie Veilid zu implementieren (mit der Ausnahmen von spezifischen Details der APIs und Datenformate). Dieser Anschnitt enthält keinen Code. Er ist keine Dokumentation des Codes selber, sondern der Kern eine Whitepapers.

### Peer Netwerk im Detail

Lasst uns als Erstes das Peer Netzwerk ansehen, weil seine Struktur die Basis für den Rest des Datenspeicherungsansatz bildet. Veilids Peer Netzwerk ist in der Art ähnlich zu anderen Peer to Peer Systemen, dass es sich von oben auf auf andere Protokolle legt und diese überlagert. Veilid versucht auf seine Art protokoll-agnostisch zu sein und ist aktuell entworfen um TCP, UDP, WebSocket und WebRTC sowie verschiedene Methoden zu Überwindung von NATs zu nutzen, so dass Veilid Peers auch Smartphones oder Computer bei unvertrauenswürdigen Internetanbietern und Ähnlichem seien können. Um das sicher zu stellen werden Peers nicht über eine Netzwerk Identität wie IP Adressen identifiziert, sondern über eine kryptografische Schlüsselpaar, das durch den Peer festgelegt wird. Jeder Peer veröffentlicht/bewirbt eine Menge von Optionen wie man mit Ihm kommunizieren kann. Diese nennt sich Dial Info und wenn ein Peer mit einem Anderen sprechen will, dann besorgt er sich die Dial Info von dem Peer aus dem Netzwerk und nutzt diese um zu kommunizieren.

Wenn sich ein Peer erstmalig mit Veilid verbindet, dann macht er dass in dem er die "Bootstrap Nodes" kontaktiert, die einfache IP Adressen Dial Infos haben, für die durch die Netzwerk Maintainer garantiert wird, dass diese (zeit-) stabil sind. Diese Bootstrap-Peers sind die ersten Einträge in der Routing Tabelle des Peers. Die Routing Tabelle ist ein sortiertes Adressbuch mit dem man feststellen kann, wie man mit einem Peer sprechen kann.

Die Routing Tabelle besteht aus einer Zuordnung öffentlicher Schlüssel der Peers zu einer nach Priorität sortierten Auswahl von Dial Infos. Um die Routing Tabelle zu befüllen, fragt der Peer andere Peers was seine Nachbarn im Netzwerk sind. Die Bezeichnung "Nachbar" ist hier über ein Ähnlichkeitsmaß bezüglich der Peer IDs definiert (genauer gesagt dem XOR Maß das viele Verteilte Hash Tables (DHTs) nutzen). Im Verlauf der Interaktionen mit dem dem Netzwerk wird der Peer die Dail Infos aktualisieren, wenn er Veränderungen feststellt. Ebenso kann er auch Dail Info für Peers abhängig von Ihrer Peer ID hinzufügen, die er im Verlauf entdeckt.

Um mit einen bestimmten Peer zu sprechen wird seine Dail Info in der Routing Tabelle nachgeschlagen. Wenn es eine Dail Info gibt, dann werden die Optionen in der durch die Routing Tabelle festgelegten Priorisierungsreihenfolge durchprobiert. Falls es die Dail Info nicht gibt, dann muss der Peer die Dail Info aus den Netzwerk anfragen. Dabei schaut er in seine Routing Tabelle, um den Peer zu finden der entsprechend der XOR Maß der nächstgelegene zum Zielpeer ist und schickt ihm einen RPC Aufruf mit dem Namen "find-node". Für jede gegebene Peer ID antwortet der Empfänger des "find-node" auf Rufs mit den Dial Infos der Peers in seiner Routing Tabelle, die der ID am nächsten sind. Die bringt den Peer näher an sein Ziel und zumindest in die Richtung des Peers nach dem er fragt. Wenn die Info des gewünschten Peers in der Antwort des Aufrufs enthalten war, dann ist der Vorgang beendet, sonst sendet er weiter "find-node" Aufrufe um dem gewünschten Zielpeer näher zu kommen. So bahnt er sich seinen Weg und versucht verschiedene alternative Peers (falls notwendig), in dem er den Nächsten als Erstes fragt, bis er entweder die gewünschte Dail Info findet oder das gesamte Netzwerk durchsucht hat oder den Vorgang abbricht.

### Benutzerprivatsphäre
Um sicherzustellen, dass Benutzer mit ein gewissen Maß an Privatsphäre in Veilid teilnehmen können, muss man sich um die Herausforderung kümmern, dass wenn man sich mit Veilid verbindet folglich auch mit anderen Nodes kommuniziert und somit IP Adressen teilt. Der Peer eines Benutzers wird deshalb regelmäßig RPC-Aufrufe erstellen, die die Identifikationsinformationen des Benutzer mit den IDs seiner Peers in Zusammenhang bringt. Veilid ermöglicht Privatsphäre durch die Nutzung einer RPC-Weiterleitungsmechanismus, der Kryptographie vergleichbar mit so genanntem "Onion Routing" nutzt. Hierbei wird der Kommunikationspfad zwischen dem tatsächlich ursprünglichen sendenden Peer und des schlussendlich final empfangenden Peer versteckt, in dem man über mehrere dazwischen liegenden Peers springt.

Der spezifisches Ansatz den Veilid bezüglich Privatsphäre hat ist zweiseitig: Privatsphäre des Senders und Privatsphäre des Empfängers. Jeweils einer oder beide könnten sich Privatsphäre wünschen oder für sich ausschließen. Um die Privatsphäre des Sender sicherzustellen nutzt Veilid etwas das sich „Safety Route" nennt: Eine Sequenz Peers beliebiger Größer, die durch den Sender ausgewählt wird, der die Nachricht sendet. Die Sequenz von Adressen werden (wie bei einer Matryoshka Puppe) verschlüsselt ineinander geschachtelt, so dass jeder Hop den Vorherigen und den Nächsten sehen kann, aber kein Hop die gesamte Route sehen kann. Dies ist ähnliche zu einer TOR (The Onion Router) Route, mit dem Unterschied, dass nur die Adressen für jeden Hop verschlüsselt sind. Die Route kann für jede Nachricht, die geschickt wird, zufällig gewählt werden.

Die Privatsphäre des Empfängers ist sofern ähnlich als das es dort auch eine Schachtelung von verschlüsselten Adressen gibt, aber mit dem Unterschied, dass die verschiedenen Adressen vorab geteilt worden sein müssen, weil es um eingehende Nachrichten geht. Diese werden „Private Routen“ genannt und sie werden als Teil der öffentlichen Daten eine Benutzers im KV Speicher veröffentlicht. Um volle Privatsphäre an beiden Enden sicherzustellen, wird eine Private Route als endgültiges Ziel einer Safety Route genutzt und die gesamt Route ist die Zusammensetzung aus beiden, so dass weder der Sender noch der Empfänger die IP adressiere des Anderen wissen.

Jeder Peer im Hop (einschließlich des initialen Peers) sendet einen "route" RPC Aufruf an den nächsten Peer im Hop, mit dem Rest der Gesamtroute (safety + private), die mit den Daten in der Kette weitergereicht wird. Der letzte Peer entschlüsselt den Rest der Route, die dann leer ist und kann sich dann den weitergeleiteten RPC ansehen und dann entsprechend diesem handeln. Der RPC selber muss nicht verschlüsselt sein, aber es ist gute Praxis diesen für den finalen Peer zu verschlüsseln, so dass die Peers dazwischen den User nicht durch Analyse des Datenverkehrs de-anonymisieren können.

Nimm bitte zur Kenntnis, dass die Routen benutzerorientiert sind. Sie sollten so verstanden werden, dass sie einen Möglichkeit darstellen mit einem bestimmten Benutzer-Peer zu sprechen, wo auch immer dieser ist. Jeder Peer in dieser Abfolge muss die tatsächlichen IP Adressen der Peers wissen, sonst können sie nicht kommunizieren. Aber die Savety und die Private Routes machen es schwer die Identität des Benutzer mit der Identität seiner Peers zusammenzubringen. Du weißt nur, dass der Nutzer irgendwo im Netzwerk ist, aber Du weißt nicht welche seine Adresse ist, auch wenn Du die Dail Infos seiner Peers in der Routing Tabelle gespeichert hast.

### Block Speicher im Detail

Wie bereits in der Vogelperspektive erwähnt ist es das Ziel des Blockspeichers inhaltsadressierbare Blocks von Daten zu speichern. Wie viele andere Peer to Peer Systeme zum Speichern von Daten nutzt auch Veilid verteilte Hash Tabellen (DHTs) als Kern des Block Speichers. Die Blockspeicher DHT hat als Schlüssel BLAKE3 Prüfsummen der Block Inhalte. Für jeden Schlüssel hält die DHT eine Liste von Peer IDs vor, die im Netzwerk deklariert wurden, dass sie den Block bereitstellen können.

Wenn ein Peer den Block bereitstellen möchte, dann macht einer einen "supply_block" RPC Aufruf mit der ID des Blocks in das Netzwerk. Der Empfänger des Calls kann dann die Informationen speichern, die der Peer bezüglich des vorgesehenen Block bereitstellt (wenn er will). Er kann auch andere Peers zurückgeben, die näher an der Block ID dran sind, die auch die Information speichern sollte. Die Peers stellen abhängig davon wie nah sie an der Block ID sind fest, ob sie die Information speichern oder nicht. Es kann auch entscheiden werden den Block zwischenzuspeichern, um sich selbst als Anbieter zu deklarieren.

So bereitgestellte Datensätze sind möglicherweise vergänglich, weil Peers das Netzwerk verlassen und somit ihre Informationen nicht mehr verfügbar sind. Deswegen wird jeder Peer, der einen Block bereitstellen will regelmäßig "supply_block" Nachrichten senden, um den Datensatz aktuell zu halten. Peers, die den Block zwischenspeichern, entscheiden an Hand der Popularität, des Speicherplatzes, der Bandbreite usw., die er übrig hat, wann das Zwischenspeichern endet.

Um einen Block zu empfangen, der im Block Speicher gespeichert wurde, sendet ein Peer den "find_block" RPC Aufruf. Der Empfänger wird dann entweder den Block zurücksenden oder möglicherweise auch eine Liste von Anbietern für den Block zurücksenden, die er kennt oder er liefert eine Liste von Peers zurück, die näher an dem Block liegen als er selbst.

Anders als bei BitTorrent sind Blocks nicht zwangsläufig Teil einer größeren Datei. Ein Block kann einfach eine einzelne Datei sein und dies wird oft der Fall für kleine Dateien sein. Größere Dateien können in kleinere Blocks aufgeteilt werden. In diesem Fall wird eine zusätzlicher Block mit einer Liste aus Block-Komponenten im Block Speicher gespeichert. Veilid selbst wird diese Block wie alle anderen Blocks behandeln und es gibt keinen eingebauten Mechanismus der festlegt, welchen Block man als erstes heruntergeladen oder teilen muss usw. (wie es sie bei BitTorrent gibt). Solche Features wäre dann abhängig von der Peer Software zu implementieren und können variieren. Verschiedene Clients werden auch die Möglichkeit haben zu entscheiden wie sie solche Block-Komponenten herunterladen wollen (z.B. automatisch, auf Eingabe des User oder etwas Anderes).

Der Mechanismus (Blocks zu haben die auf andere Blocks verweisen) ermöglicht es auch die Nutzung von IPFS-artige DAGs mit hierarchischen Daten als einen möglichen Modus. Somit können ganze Verzeichnisstrukturen gespeichert werden (und nicht nur Dateien). Allerdings ist das (genau so wie die Subfile Blocks) kein eingebauter Teil von Veilid, sondern eher ein möglicher Verwendungsmodus. Wie sie dem Benutzer heruntergeladen und dargestellt werden, obliegt dem Client Programm.

### Key-Value Speicher im Detail

Der Key-Value Speicher ist eine DHT (ähnlich wie der Block Store). Allerdings statt inhaltsbezogener Hashes als Schlüssel, nutzt der KV Speicher Benutzer-IDs als Schlüssel (Achtung: _NICHT_ Peer IDs). Für einen gegebenen Key hat der KV Speicher eine hierarchische Key-Value Map, die im Prinzip beliebige Strings mit Werten verknüpft, die selber wiederum Nummern, Zeichenketten, ein Datum, Uhrzeiten oder andere Key-Value Maps sein können. Der spezifische Wert, der an einer ID des Users gespeichert wird, ist versioniert, so dass man bestimmte Schemata von Unterschlüsseln und Werten definieren kann, die dann entsprechend der unterschiedlichen Versionen vom Client anders behandelt werden.

Wenn ein Nutzer wünscht Daten im Bezug auf ihren Schlüssel zu speichern, dann senden sie einen "set_value" RPC Aufruf an die Peers deren IDs am nächsten gemäß XOR Metric zu Ihrer eigenen ID liegen. Der Wert der dem RPC Aufruf mitgegeben wird ist ein Einzelwert, so dass das Netzwerk sicherstellen kann, dass nur der vorgesehene Nutzer Daten in seinem Key speichert. Die Peers, die den RPC Aufruf empfangen, können andere Peer IDs näher am Key zurückliefern (und so weiter), ähnlich wie auch beim den "supply_block" calls des Block Speichers. Am Ende werden einige Peers die Daten speichern. Der Peer des Users sollte regelmäßig die gespeicherten Daten aktualisieren, um sicher zu gehen, dass sie dauerhaft gespeichert bleiben. Es ist auch gute Praxis für den eigenen Peer des Benutzers, dass er seine eigene Daten in einem Cache (Zwischenspeicher) vorhält, so dass Client Programme den Peer des eigenen Benutzer als autorisierte Quelle der aktuellsten Daten verwenden kann. Hierfür ist es aber notwendig eine Route zu veröffentlichen, die es erlaubt das andere Peers dem eigenen Peer des Benutzers Nachrichten schicken. Eine private Route reicht hierfür aus.

Der Abruf ist ähnlich wie der Abruf beim Block Store. Der gewünschte Key wird über eine "get_value" Call bereit gestellt, der einen Wert zurückliefern kann (oder eine Liste von Peers die näher am Key liegen). Am Ende werden die signieren Daten zurückgeliefert und der Empfänger kann verifizieren, dass diese tatsächlich dem spezifizierten Benutzer gehören in dem man die Signatur prüft.

Beim Speichern und Abrufen von Daten ist es nicht notwendig, dass der im RPC Aufruf bereitgestellte Key ausschließlich die Benutzer ID sein muss. Er kann eine Liste von Zeichenketten beinhalten, die als Pfad zu gespeicherten Daten beim Benutzer Key diene, um damit spezifische Updates oder Abrufe durchzuführen. Dadurch wird der Datenverkehr im Netzwerk minimiert, weil nur die jeweils relevante Information umherbewegt wird.

Der spezifische Inhalt des Benutzers Keys wird über das Protokoll bestimmt und im Besonderen von der Client Software. Frühe Versionen des Protokolls nutzen eine DHT-Schema-Version, die ein sehr einfaches Social-Network-orientiertes Schema definiert. Später Versionen werden mehr ein generischeres Schema ermöglichen, so dass Client Plug-ins reichhaltiger Informationen speichern und darstellen können.

Die zustandsbehaftete Natur des Key Value Speichers bedeutet, dass sich Werte mit der Zeit ändern werden und es müssen Maßnahmen ergriffen werden, um auch auf diese Veränderungen zu reagieren. Ein Abfragemechanismus könnte verwendet werden, um regelmäßig abzufragen, ob neue Werte vorliegen, aber das wird zu einer Menge unnötigem Datenverkehrs im Netzwerk führen. Um das zu vermeiden erlaubt es Veilid, dass Peer einen "watch_value" RPC Aufruf senden, der einen DHT Key (mit Unterkeys) als Argument enthält. Der Empfänger würde dann einen Datensatz speichern, dass der Sender des RPC benachrichtigt werden möchte, wenn der Empfänger nachfolgend einen "set_value" RPC Aufruf erhält. In diesem Fall sendet der Empfänger dem sendenden Peer einen "value_changed" RPC Aufruf um ihm den neuen Wert zu übermitteln. Wie auch bei andere RPC Calls, muss auch "watch_value" regelmäßig neu gesendet werden, um das "Abonnement" für den Wert zu verlängern. Zusätzlich kann er Peers näher am Key zurückgegeben oder andere Peers die erfolgreich abonniert haben und so mit auch als Quelle dienen können.

TODO: Wie vermeidet man das Replay Updates? Vielleicht über eine Sequenznummer einem signierten Patch?


## Anhang 1: Dial Info und Signaling

## Anhang 2: RPC Listing
