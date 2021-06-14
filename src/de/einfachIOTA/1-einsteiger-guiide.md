<!--
---article_info
title: IOTA Einsteiger Guide
author: [Schmucklos]
reviews: [ruegenlord, reviewer_2]
---
-->

# IOTA Einsteiger Guide

> Der IOTA Einsteiger Guide ist eine Website, die neueste Artikel sowie die Grundlagen rund um IOTA auf deutsch und für jeden kostenlos anbietet. Dieser Artikel ist nur ein kleiner Ausschnitt. Schau auf jeden Fall mal vorbei!
>
> **https://iota-einsteiger-guide.de**

## Was ist IOTA?  - Eine Zusammenfassung
IOTA ist keine Blockchain, IOTA ist ein skalierbares Open-Source-Kommunikationsprotokoll mit Token (Kryptowährung) für einen Wertetransfer. Es wird entwickelt und bereitgestellt von der gemeinnützigen, also nicht gewinnorientierten IOTA Foundation (Stiftung nach deutschem Recht) mit Hauptsitz in Berlin. 

Der Name IOTA stammt von dem altgriechischen Buchstaben Iota (altgriechisches Neutrum Ἰῶτα; „der kleinste Buchstabe") und bezeichnet "etwas Geringes". Das Iota ist der 9. Buchstabe des altgriechischen Alphabets und wurde in der Antike wie heute identisch, nämlich i ausgesprochen. Von ihm stammt bspw. auch der lateinische Buchstabe i ab. Im IOTA Kommunikationsprotokoll steht das i für 1 Iota und ist im Netzwerk die kleinste handelbare Werteinheit - hier schließt sich der Kreis zu "etwas Geringes".

Das Ziel der IOTA Foundation ist es, eine Vertrauensschicht (engl. Trust Layer) für das Internet of Everything (IoE) zu erschaffen, die es Geräten im IoE ermöglicht,  Daten und Werte untereinander unveränderlich sowie gebührenfrei auszutauschen. IOTA strebt in Zusammenarbeit mit der Industrie und der Object-Management-Group (einem Konsortium, das sich mit der Entwicklung von Standards für die herstellerunabhängige systemübergreifende objektorientierte Programmierung beschäftigt) eine Standardisierung ihres Kommunikationsprotokolls an. Mit einer hohen Interoperabilität wird IOTA das "Ledger Of Everything" sein, dessen Infrastruktur auch von externen Anwendungen ohne Erlaubnis genutzt werden kann.

 
![](https://iota-einsteiger-guide.de/media/images/2020-08-16-17_34_34-iota_ledger_of_everything.jpg-irfanview.png)

IOTA ermöglicht eine schnelle, manipulationssichere und dezentrale Übertragung von Werten und Daten über viele Nodes. Dabei werden Werte- und Daten-Transaktionen grundsätzlich unterschiedlich gehandhabt. Während Werte-Transaktionen von Full-Nodes validiert werden müssen, werden Daten-Transaktionen direkt bestätigt und sind notarisiert. An dieser Stelle werden sich die ein oder anderen Lesenden fragen: Warum benötige ich für eine reine Daten-Transaktion die IOTA Distributed Ledger Technologie? Ich kann die Daten doch einfach verschlüsseln, signieren und via TCP/IP versenden. 

Nun, abgesehen davon, dass „Man in the Middle“-Angriffe möglich wären, beweisen signierte Daten nur, dass die Daten von Person A kommen. Es erlaubt Person B weder zu beweisen, wann Person A sie gesendet hat, noch ob Person A die gleichen Daten an alle weiteren Personen gesendet hat. Person A könnte eine bestimmte Information an eine Person B und eine andere Information an eine andere Person C senden. Signaturen allein werden niemanden vor solchen Gefahren schützen.

Mithilfe der „Notarisierung“ kann bewiesen werden, dass ein elektronisches Dokument in einer bestimmten Form zu einem bestimmten Zeitpunkt existiert hat und seit der Erstellung nicht verändert wurde. Bei der Erstellung einer Notarisierung wird ein eindeutiger Hash (Fingerabdruck) eines Dokumentes berechnet und gemeinsam mit einem Zeitstempel im IOTA-Ledger (dem "Tangle") unveränderbar gespeichert. Falls zu einem späteren Zeitpunkt verifiziert werden soll, dass das betreffende Dokument zum behaupteten Zeitpunkt existiert hat und/oder nicht verändert wurde, werden die Daten aus dem Tangle abgerufen und mit den vorliegenden Informationen verglichen.

Wenn das IOTA Tangle als Transportmedium verwendet wird, dann sind diese Art von Angriffen nicht möglich. Aktuell gibt es viele Anwendungsfälle, bei denen es entscheidend ist, dass alle beteiligten Parteien sicher sein können, dass alle an oder mit den gleichen Informationen arbeiten (z.B. Lieferketten, Oracle, synchronisierte Fernsteuerungssysteme usw.). Es geht daher primär um Usecases, die eben nicht einfach via TCP/IP zu übertragen sind. Ansonsten könnte auch direkt BitTorrent dafür genutzt werden. Sender und Empfänger benötigen eine transparente Form der Übertragung zwischen mehreren Parteien oder Organisationseinheiten.

Unter dem Strich bedeutet dies Folgendes: Es geht nicht nur darum, dass niemand die Daten während der Übertragung manipuliert, sondern auch darum, dass der Empfänger diese Daten nicht manipuliert. Zur Verdeutlichung soll ein Beispiel dienen: Ein Sensor (mit IDoT Chip) hat einige Werte gemessen/erfasst und versendet diese Daten über den IOTA-Tangle, welcher den Hash dieser Daten speichert. Wenn diese Daten später verkauft werden sollen, kann dieser Hash als Nachweis vorgelegt werden und dem Käufer anhand des Tangle beweisen, dass die Daten vom Sensor im Nachhinein nicht verändert wurden. Die IOTA Technologie, also das Tangle, fungiert hierbei wie eine Art Fingerabdruck - mit ihr können alle gesendeten Daten verifiziert werden.
An dieser Stelle noch ein kurzer Hinweis: Um Missbrauchsszenarien zu verhindern und auf eine effiziente Art und Weise die Maschinenökonomie zu ermöglichen, werden zukünftig Nodes und Geräte eine eindeutige Kennung (ID) erhalten, siehe auch Identität der Dinge (IDoT).

Bei IOTA sind die Guthaben auf den Adressen allen teilnehmenden Full-Nodes jederzeit bekannt. Jedoch versteht sich IOTA nicht als Datenspeicher, sodass die Transaktionshistorie auf den Full-Nodes nicht über einen längeren Zeitraum gespeichert wird (analog zum TCP/IP-Traffic des Internets, der ebenfalls nicht gespeichert wird). Möchte ein Nutzer oder ein Unternehmen zusätzlich auch die Transaktionshistorie über mehrere Jahre hinweg abrufen bzw. speichern, müssen dafür zusätzliche Lösungen eingesetzt werden. Dies könnte ein selektiver Permanode, eine eigene Second-Layer Anwendung oder zukünftig auch ein Smart-Contract sein, welcher mehrere Nodes für diese Dienstleistung bezahlt.

Das Kommunikationsprotokoll ist modular aufgebaut, was zukünftig schnellere und einfachere Updates ermöglicht. Für die Redundanz im Netzwerk sind sehr viele Nodes nötig - in ein paar Jahren kann jedoch jedes Auto, jede Maschine, jeder Router usw. ein Node sein. Je mehr Nodes am Netzwerk teilnehmen, umso schneller und sicherer wird das Netzwerk.


## Technologie


Um die oben genannten Ziele zu ermöglichen, sind folgende Protokollmerkmale von grundlegender Bedeutung:

Skalierbar - Verarbeiten einer beträchtlichen Anzahl von Transaktionen/Messages pro Sekunde über ein großes Netzwerk von Nodes mit schnellen Bestätigungszeiten.  
Schlankes System  - Leistungsschwache Geräte sollten in der Lage sein, direkt am Netzwerk teilzunehmen.  
Gebührenfrei - Das Senden aller Transaktionen sollte ohne Zahlung von Netzwerkgebühren erfolgen: Wenn 50 Mi versendet werden, kommen auch 50 Mi beim Empfänger an. 
 


![](https://iota-einsteiger-guide.de/media/images/feeless-animation.gif)
 

### Datenstruktur

Neben den Gebühren weisen herkömmliche DLTs wie die Blockchain weitere begrenzende Faktoren auf und sind daher für die Erreichung des Ziels von IOTA ungeeignet. Ein Beispiel ist die inhärente Beschränkung der Geschwindigkeit von Blockchain-Netzwerken, welche allgemein als "Blockchain-Bottleneck" bezeichnet wird. In der Blockchain gibt es nur eine Seite, an der neue Transaktionen angefügt werden können - das Ende der Kette. Die daraus resultierenden negativen Auswirkungen auf den Netzwerkdurchsatz werden in dieser einfachen Grafik dargestellt:


![](https://iota-einsteiger-guide.de/media/images/blockchain_bottleneck.gif)


Im Gegensatz dazu, wird bei IOTA nicht Block an Block aneinandergereiht und es werden auch keine Miner zur Validierung von Transaktionen benötigt. Die Kerndatenstruktur bei IOTA ist von Natur aus hoch skalierbar. Dies wird mit einer einfachen Regel ermöglicht: Jede Transaktion referenziert und genehmigt bis zu acht weitere Transaktionen. Diese Regel definiert die zugrundeliegende Datenstruktur von IOTA - das Tangle - welches mathematisch als gerichteter azyklischer Graph (DAG) bezeichnet wird.

Bildlich lässt sich das wie folgt beschreiben: Eine Menge von Transaktionen sind durch Pfade verbunden - dies ist ein Graph.  
Jeder dieser Pfade besitzt eine eindeutig festgelegte Laufrichtung, womit es ein gerichteter Graph ist.  
Lässt sich niemals ein Pfad finden, der zu seinem Ausgangspunkt zurückkehrt, ist der Graph azyklisch (im Kreis laufende Pfade wären dagegen zyklisch).
 

### Transaktionsstatus

![](https://iota-einsteiger-guide.de/media/images/1234567.png)      

Grün = Bestätigt (Konsens)   /   Weiß = Unbestätigt   /   Grau = Neu angehängt (Tips)

 

Anstatt, wie bei der Blockchain, auf einer einzigen Seite für das Anhängen neuer Transaktionen begrenzt zu sein, bieten DAGs mehrere Punkte an, an denen Transaktionen angehängt werden können. Benutzer können weiterhin neue Transaktionen an verschiedene Teile des Tangle anhängen, ohne auf die Bestätigung durch andere Transaktionen warten zu müssen:

![](https://iota-einsteiger-guide.de/media/images/tangle_bottleneck.gif)

In dieser chaotischen Ordnung werden sämtliche Transaktionen parallel abgearbeitet und ermöglichen eine sehr hohe Skalierung, da mit steigender Anzahl der Transaktionen auch die Bestätigungsraten ansteigen. Dies ist, wie oben zu sehen war, genau konträr zu der Funktionsweise der derzeitigen "klassischen" Blockchains. 



### Konsensmechanismus

In der Blockchain teilt der Nakamoto-Konsens das Netzwerk in Miner und Nutzer auf. Miner verbrauchen große Mengen an Rechenleistung und erfüllen damit den Proof-of-Work (PoW), der für die Verkettung der Blöcke erforderlich ist. Miner werden durch die Gebühren, welche Benutzer bereit sind zu zahlen, damit ihre Transaktion in einen Block aufgenommen wird, incentiviert. Diese gebührenbasierte Anreizstruktur wäre eine erhebliche Barriere in einer Machine-to-Machine-Wirtschaft, in der die Mikrozahlungswerte zwischen den Maschinen niedriger sein können als die anfallenden Gebühren.

Bei IOTA gibt es keinen Unterschied zwischen Minern und Nutzern. Alle Nodes können am Konsens teilnehmen. Dies bedeutet, dass ein IOTA-Node eine wesentlich andere Rolle spielt als ein Bitcoin-Miner. IOTA-Nodes führen nur grundlegende Operationen durch, welche sie nicht viel Rechenleistung erfordern (z.B. Speichern des Ledgers oder Validieren von Transaktionen). Benutzer können mit minimalen Kosten einen Node einrichten und aktiv am Netzwerkkonsens teilnehmen und so die Sicherheit des Netzwerks erhöhen.

Der Konsensmechanismus legt fest, wie sich die Nodes darauf einigen, welche Transaktionen vertrauenswürdig sind und gewährleistet somit Übereinstimmung im Netzwerk. In der aktuellen IOTA-Implementierung vertrauen Nodes nur (Wert-)Transaktionen, die von Meilensteinen referenziert und genehmigt werden. Diese Meilensteine wiederum werden vom Koordinator ausgestellt. Ähnlich wie beim Bitcoin - hier gab es bis zum 16.06.2014 s.g. Checkpoints - ist der Einsatz dieser zentralisierten "Finalitätsvorrichtung" derzeit noch notwendig, um die Sicherheit des jungen Netzwerks zu gewährleisten. 


![](https://iota-einsteiger-guide.de/media/images/milestones.gif)
    
Hinweis: Der Open-Source "Koordinator" kann lediglich Transaktionen validieren. Konsensregeln kann er dagegen nicht umgehen, womit es ihm auch nicht möglich ist, IOTA-Token zu erschaffen, IOTA-Token einzufrieren oder einen anderen "Betrug" im Netzwerk zu begehen!

### IOTA ohne Koordinator

Der Koordinator wird in naher Zukunft abgeschaltet (Stichwort: Coordicide). Diesbezüglich wurden alle Forschungsarbeiten weitestgehend abgeschlossen und u.a. von mehreren Universitäten überprüft. Der Entwicklungsfahrplan - offiziell beschrieben als Roadmap - der die IOTA-Technologie zur Produktionsreife und Adoption führen soll, wurde definiert und wird derzeit umgesetzt. Vor dem Coordicide wurde das IOTA-Netzwerk Ende April 2021 mit einer Zwischenaktualisierung namens Chrysalis vollständig optimiert, um eine unternehmenstaugliche Lösung für das Ökosystem anzubieten.
Ein weiterer wichtiger Meilenstein ist das erst vor kurzem gestartete "IOTA 2.0 DevNet" (urspr. als "Nectar" bezeichnet). Es ist das erste vollständig dezentrale, also ohne Koordinator funktionierende, IOTA-Entwicklungsnetzwerk, welches auf dem Chrysalis Update basiert und stellt den Prototypen für das zukünftige, vollständig dezentrale IOTA-Hauptnetzwerk dar.
Nach ausführlichem Testen soll (voraussichtlich in 2021/2022) der Wechsel vom Entwicklungs- zum Hauptnetzwerk, und damit der eigentliche Coordicide, stattfinden.
 
Die finale Abschaltung des Koordinators stellt sicher, dass das Netzwerk ohne den Koordinator einen Konsens erzielt und gleichzeitig die folgenden Eigenschaften aufweist:

Skalierbar: Die Transaktionsrate im Netzwerk ist nicht durch das Protokoll begrenzt - eine beispiellose Skalierbarkeit wird ermöglicht.  
Sicher: Ein Angreifer kann den Konsens nicht beeinflussen.  
Dezentral: Alle ehrlichen Nodes können Teil des Konsensprozesses sein. 

![](https://iota-einsteiger-guide.de/media/images/trilemma.gif)

Aktuelle Blockchain-Lösungen können maximal zwei dieser drei Eigenschaften gleichzeitig lösen. Die Algorithmen wurden so entwickelt, dass sie die Mining-Schwierigkeit an einem bestimmten Punkt halten, sodass das Netzwerk keinen neuen Block produzieren kann, während der bestehende Block von allen Nodes geklatscht (im Netzwerk verteilt) und verifiziert wird. Das bedeutet: Je größer die Anzahl der Nodes im Netzwerk ist, desto länger dauert es, bis der Block bei allen Nodes ankommt und verifiziert wird. Wäre die Blockzeit gering, würden viele Blöcke gleichzeitig von verschiedenen Akteuren produziert werden und der Nakamoto-Konsens könnte sich nicht auf ein einziges Ergebnis einigen. Verschiedene Nodes würden verschiedene Chains als die längste Chain zur gleichen Zeit sehen.

Sie lösen dies, indem sie die Mining-Schwierigkeit hoch ansetzen, sodass die Blockzeit im Durchschnitt 10 Minuten beträgt. In diesen 10 Minuten versuchen verschiedene Akteure einen Block zu erzeugen, was jedoch normalerweise nur einem Akteur gelingt. Alle anderen Akteure lassen (sprichwörtlich) alles fallen, was sie gerade taten, akzeptieren den erfolgreichen Block und versuchen es erneut. Es gibt Zeiten, in denen mehrere Akteure (normalerweise 2) Glück haben und einen gültigen Block zur gleichen Zeit erstellen. Die Hälfte des Netzwerks akzeptiert Block 1 und die andere Hälfte akzeptiert Block 2. Dies wird gelöst, indem der nächste Miner den nächsten Block findet und ihn an eine der beiden Ketten anhängt. Diese Kette ist nunmehr die längste und "gewinnt" folglich. Alle Nodes akzeptieren sie als gültig. Nach diesem Prinzip wird in Proof-of-Work und (auch wenn es in Proof-of-Stake keine Miner gibt) nach einem ähnlichen Konzept in Proof-of-Stake verfahren. Die Nodes müssen sich auf einen Block einigen, bevor sie mit der Produktion des nächsten Blockes beginnen. Mit der Anzahl der Nodes steigt auch die Netzwerkverzögerung, also die Zeit, bis der Block an alle anderen Nodes geklatscht wird. Da der Nakamoto-Konsens ein synchroner Konsens ist, kann er bestenfalls 2 der 3 Punkte (Sicherheit, Dezentralisierung, Skalierbarkeit) lösen.

Dieses Problem wird als **Blockchain-Trilemma** bezeichnet. 

P.S.: Durch die begrenzte Blockgröße entsteht ein weiteres Problem: Da immer mehr Transaktionen über das Netzwerk gesendet werden, entscheiden sich die Miner dafür, die Transaktionen mit den höchsten Gebühren zu validieren, was zu steigenden Transaktionskosten und langen Wartezeiten führt. Der revolutionäre Charakter dieser Proof-of-Work-basierten Lösung sollte nicht unterschätzt werden - gleichzeitig dürfen jedoch die damit verbundenen Einschränkungen des Netzwerkdurchsatzes nicht vernachlässigt werden.



IOTA ist durch seine asynchrone Natur grundsätzlich kein Opfer des Blockchain-Trilemma. Jeder Node darf gleichzeitig Transaktionen produzieren, während er mit einer Teilmenge von Nodes für den Konsens interagiert und dabei eine nicht lineare Datenstruktur verwendet. Mit IOTA 3.0 (also inkl. Sharding) wird IOTA eine Lösung anbieten, welche das Trilemma umgeht und damit eigentlich auch löst (wobei dies Ansichtssache ist). Konkret lässt sich Sharding folgendermaßen beschreiben: IOTA-Nodes haben eine Obergrenze für Transaktionen pro Sekunde (TPS), die sie verarbeiten können. Durch eine Form der Datenbankpartitionierung - sprich: das Aufteilen einer sehr großen Datenbank in kleinere Datenbanken - in besser verwaltbare Segmente (s.g. Shards) würde jeder Shard einen einzigartigen Satz der Kontensalden enthalten und Nodes würden dann einzelnen Shards zugewiesen, um Transaktionen zu validieren. 

![](https://iota-einsteiger-guide.de/media/images/3_wcj-m9uqsociqfrx1bexrq.png)

Ziel ist es, dass durch die Aufteilung in besser verwaltbare Segmente der Transaktionsdurchsatz erhöht wird. Das Trilemma wird innerhalb einzelner Shards grundsätzlich weiterhin bestehen. Sobald jedoch in einem einzelnen Shard der Netzwerkdurchsatz die Verarbeitungskapazitäten der Nodes übersteigt, bildet sich dynamisch ein weiterer Shard (Fluid-Sharding). Durch das neuartige, sehr flexible Sharding werden bei IOTA theoretisch unendliche hohe Transaktions-Geschwindigkeiten ermöglicht.

Das Tangle ermöglicht es Benutzern, neue Transaktionen an jeden Teil des Tangle anzuhängen. Aufgrund der DAG-Struktur ist lediglich erforderlich, dass jede neue Transaktion bis zu acht weitere Transaktionen referenziert. Durch die Möglichkeit, mehrere Anhängepunkte zu verwenden und die Vermeidung der Notwendigkeit von Blöcken, ist das Tangle von Natur aus skalierbar. Es gibt im Grunde keine protokollbedingten Engpässe - die Skalierbarkeit ist nur durch die Hardware und die Gesetze der Physik beschränkt. 

Bei IOTA gibt es im Grunde keine protokollbedingten Engpässe - die Skalierbarkeit ist einzig durch die Hardware und die Gesetze der Physik beschränkt. 
Derzeit ist der Koordinator ein solcher Engpass, welcher das Protokoll an der Skalierung hindert. Zudem stellt der Koordinator einen "Single Point of Failure" dar und verhindert, dass IOTA ein vollständig dezentrales Netzwerk ist. Die Entfernung des Koordinators ist aus den genannten Gründen das nächste große Ziel der IOTA Foundation.

 

### Der verbesserte Tangle nach dem Coordicide: Dezentral, skalierbar und sicher

Die Entfernung des Koordinators allein reicht nicht aus, um das Skalierbarkeits-Trilemma sicher und dezentral zu lösen. Für ein solches Netzwerk müssen weitere Sicherheitsmodule implementiert werden, die einer hohen Transaktionsrate nicht im Wege stehen. Kern der Lösung ist eine zusätzliche Sicherheitsebene mit dem proaktiven Abstimmungsmechanismus namens Shimmer, über den Nodes die Meinungen anderer Nodes anfordern. Mittels derer soll entschieden werden, welche Transaktionen in das Tangle aufgenommen und welche abgewiesen (verwaist) werden sollen.

![](https://iota-einsteiger-guide.de/media/images/voting-01.gif)

Um den Koordinator zu entfernen, müssen eine Reihe von Herausforderungen gelöst werden. Aufgrund der Komplexität der Lösung wird der Coordicide in verschiedene Komponenten zerlegt. Dieser Ansatz macht den Coordicide-Vorschlag und das zukünftige Protokoll modular, sodass jedes Modul unabhängig voneinander ersetzt werden kann, wenn zukünftig neue Forschungsergebnisse weitere Verbesserungen ergeben.

 

### Schlüsselfaktoren der IOTA Distributed-Ledger-Technologie im produktionsfertigen Zustand (nach der Entfernung des Koordinators)

**Vollständige Dezentralisierung**: Als global verteiltes Netzwerk ist die IOTA widerstandsfähig und robust gegen Angriffe. Ohne den Koordinator wird keine Instanz eine besondere Rolle im Netzwerk spielen. Dies bedeutet auch: Sobald der Koordinator abgeschaltet ist, wird es der IOTA-Stiftung nicht mehr möglich sein, ihn neu zu starten.

**Permissionless/Erlaubnisloser Zugang zum Netzwerk**: Jeder - oder besser: Alles - kann dem Netzwerk jederzeit beitreten, um Transaktionen hinzuzufügen und zu  validieren. Wenn andere DLTs ihre Nodeanzahl begrenzen oder sogar einen zugelassenen Ansatz für die Skalierbarkeit einführen müssen, umfasst IOTAs Ansatz schlichtweg: Zusätzliche Nodes. Eine größere Anzahl von ehrlichen Nodes verbessert die Sicherheit des Netzwerks, indem sie den Anteil der ehrlichen Stimmen erhöht.  

**Partitionstolerant**: Ein produktionsfertiges Tangle ist partitionstolerant. Das bedeutet, ein Teil des Tangle kann für eine bestimmte Zeit vom Main-Tangle abgetrennt werden und ohne Internetverbindung weiterlaufen. Diese Teile können erneut mit dem Main-Tangle verbunden werden, wenn die Internetverbindung wiederhergestellt wurde.

**Endgültigkeit innerhalb von Sekunden**: Der Abstimmungsprozess ermöglicht es dem Netzwerk, Entscheidungen über Transaktionen sehr schnell zu treffen und abzuschließen, ohne auf mehrere zusätzliche Genehmigungsvorgänge warten zu müssen, um die Sicherheit zu erhöhen. Darüber hinaus kann das Netzwerk "wahre Endgültigkeit" (deterministisch statt probabilistisch) erreichen, da ein Angreifer mit unbegrenzter Hashing-Power den Ledger-Zustand nicht "umkehren" könnte.

**Zuverlässiger Zeitstempel**: Der Emittent (Node/Benutzer) der Transaktion, setzt zum Zeitpunkt der Ausgabe einen Zeitstempel. Die Einigung über die Glaubwürdigkeit von Zeitstempeln ermöglicht es IOTA, ein vollständig geordnetes Tangle zu etablieren - ein großer Schritt hin zu Smart Contracts.

**Skalierbarkeit**: Eine erhöhte Netzwerkaktivität verringert die Transaktionsabwicklungszeiten - es gibt somit keine protokollbedingten Engpässe. Die Skalierbarkeit ist nur durch die Hardware und die Gesetze der Physik eingeschränkt. Der Wegfall des Koordinators als eine einzige Einheit, die alle Transaktionen verarbeitet und verifiziert, bildet die Grundlage für einen dynamischen Sharding-Prozess, der eine "echte" unbegrenzte Skalierbarkeit ermöglicht. Ein adaptiver Ratenregelungsalgorithmus, der auf Nodebasis arbeitet, wird es Angreifern unmöglich machen, das Netzwerk mit ungesundem Spam zu überfordern, während ehrliche Nodes dennoch ungestört arbeiten können.

**Erhöhte Zuverlässigkeit**: Die Bestimmung des  bevorzugten Teils des Tangle durch Abstimmung ermöglicht die Umsetzung  unterschiedlicher Strategien zur Tip-Auswahl, durch die  die meisten (wenn nicht sogar alle) ehrlichen Transaktionen aufgegriffen  werden. Dies reduziert den Bedarf an Reattachments und Promotionen. 

**Sicherheit (Mana)**: Der neuartige Sybil-Schutzmechanismus namens Mana sichert zusammen mit weiteren  Sicherheitsmechanismen wie bspw. dem  Abstimmungsmechanismus (FPC) das Netzwerk auch bei einer  großen Anzahl von böswilligen Akteuren.  

**Intelligentes und stabiles Auto-Peering**:  Der Wegfall des manuellen Peering reduziert den Wartungsaufwand für die  Nodebetreiber und macht das Netzwerk stabiler.  

**UTXO-Modell**: Jeder  Token auf einer Adresse ist dann eindeutig identifizierbar und jede Ausgabe  benennt genau den Token, die sie bewegen möchten. Dies ermöglicht eine  schnellere und genauere Konfliktbehandlung und verbessert die  Belastbarkeit sowie die Sicherheit des Protokolls.  

**Gebührenfreie Transaktionen**: Das Fehlen von Minern macht IOTA-Transaktionen völlig gebührenfrei: Person A sendet einen Cent an Person B und Person B erhält exakt diesen einen Cent. Dies ermöglicht echte Mikrozahlungen für die  Machine-to-Machine-Wirtschaft, um damit auch die Bezahlung für Kleinstmengen  oder verbrauchsabhängige Bezahlungen sinnvoll umzusetzen.

**Weniger Dokumentation**: Unternehmen müssen gebührenfreie Transaktionen nicht für das Finanzamt dokumentieren oder für Jahre speichern.
Lokale Snapshots ermöglichen es Nodes, nur eine Teilmenge der Historie des Ledgers zu speichern, sodass auch Nodes mit begrenzten Hardware-Ressourcen am Netzwerk teilnehmen können.

**Fairness**: Alle Transaktionen werden gleich behandelt. Es gibt keine Möglichkeit, eine Prämie zu zahlen (z.B. durch erhöhte Gebühren) um eine höhere Priorität bei der Verarbeitung durch das Netzwerk zu erhalten.

**Schlankes System**: Konzipiert für Geräte, wie z.B. Sensoren, die an einem Niedrigenergie-Netzwerk teilnehmen.

**Modularität**: Ein modularer Aufbau ermöglicht es, die Bestandteile des Protokolls unabhängig voneinander zu entwickeln. Der mehrschichtige Ansatz ermöglicht zukünftig weitere Erweiterung des Basisprotokolls in ähnlicher Weise wie das Internetprotokoll selbst. Ein Protokoll welches sich nicht updaten lässt, ist kein Protokoll.

**Zuverlässige Führung**: Die IOTA Foundation als gemeinnützige Organisation nach deutschem Recht optimiert die Akzeptanz und zukünftige Entwicklung des IOTA-Netzwerks. Das Fehlen von Minern ermöglicht es, neue Funktionen ohne Interessenkonflikte umzusetzen.

**Open-Source**: Die Technologie ist kostenlos, Open Source und jede Person kann Lösungen darauf aufbauen.

**Unveränderlich**: Es wird sichergestellt, dass die Informationen vertrauenswürdig sind und nicht manipuliert werden können.

**Daten-Transaktionen**: Sind unabhängig zu Wert-Transaktionen. Die sogenannten "Messages" ermöglichen Anwendungsfälle der Technologie, die über finanzielle Belange hinausgehen. Die große Mehrheit aller Transaktionen werden reine "Messages" (also Transaktionen ohne Wert) sein. Das können Daten sein, die bspw. auf Marktplätzen gehandelt, von Sensoren erfasst oder auch von Apps ausgetauscht werden sowie noch etliche andere Anwendungsfälle. In dem unten stehenden Bild ist zu sehen, wie allein durch die IOTA-Architektur der schnelle Austausch von Daten und Werten gegenüber der Blockchain  begünstigt wird.
 


![](https://iota-einsteiger-guide.de/media/images/2020-07-06-16_56_24-architektur.pptx-powerpoint.png)



## Lösungen für unterschiedliche Anwendungsfälle
Die Sicherstellung der Wahrhaftigkeit ist genau das, was die Distributed Ledger Technologie ermöglicht. IOTA ist ein Protokoll, welches einen Konsens über den Stand der Dinge in einem Netzwerk erzielt, indem es eine einzige kryptographisch sichere Quelle mit einer einheitlichen Wahrheit zur Verfügung stellt. Dies wird in Zukunft eine große technologische Innovationsexplosion nach sich ziehen und völlig neue Anwendungsfälle bzw. Geschäftsfelder mit Begleitwirtschaft ermöglichen, die in der Vergangenheit, weil diese bspw. zentralisiert, nicht skalierbar, zu teuer, nicht sicher oder die Regeln des Datenschutzes verletzt haben, unmöglich waren.

 

IOTA wird für eine Vielzahl dieser neuen Anwendungsfälle Lösungen anbieten. Nachfolgend einige Teaser der zukünftig wichtigsten Aspekte: 

**Vielseitig einsetzbar**: Indem es die Privatsphäre und die Sicherheit in einem offenen Netzwerk verbessert, kann das IOTA-Protokoll auch als  Instrument zur Erhaltung eines freien und offenen Internets eingesetzt werden.

**Mikrotransaktionen**: Gebührenfreie Transaktionen ermöglichen erstmals echte Mikrozahlungen. Sowohl für den Menschen als auch für die Machine-to-Machine-Wirtschaft, um damit bspw. die Bezahlung für  Kleinstmengen oder verbrauchsabhängige Bezahlungen sinnvoll umzusetzen.  

**Maschinenwirtschaft**:  Die Machine-to-Machine-Wirtschaft ist eine Wirtschaft, in  der Maschinen selbstbestimmte Marktteilnehmer sind, die über eigene Bankkonten  verfügen. Die wachsende Verbreitung des IoT und die Fortschritte bei der  künstlichen Intelligenz ermöglichen es vernetzten, intelligenten  Maschinen autonom zu agieren. Das bedeutet, Maschinen werden direkt  miteinander kommunizieren, untereinander Daten auszutauschen und sich  gegenseitig für Dienstleistungen bezahlen, ohne dass eine menschliche  Interaktion erforderlich ist. 

**Digitale Identität**: Das Internet bildet die Grundlage für viele  unserer Interaktionen in der modernen Welt. Es hat neue  Geschäftsmöglichkeiten, bessere Kundenzufriedenheit und eine Verbesserung  unseres täglichen Lebens geschaffen. Es fehlt jedoch an wesentlichen  Eigenschaften von Vertrauen und Privatsphäre. Mit einem dezentralen digitalen  Identitätsprotokoll wird IOTA diese Eigenschaften zu den  Online-Interaktionen hinzufügen.  

**IOTA Stronghold**: Stronghold ist eine Sammlung von Mehrzweckbibliotheken zur sicheren Verwaltung von Passwörtern, persönlichen Daten und privaten Schlüsseln. Es ist eine sichere Software-Implementierung mit dem alleinigen Zweck, digitale Geheimnisse vor der Gefahr durch Hacker und versehentliche Lecks zu isolieren. Sie verwendet versionierte, dateibasierte Snapshots mit doppelter Verschlüsselung, die leicht gesichert und sicher zwischen Geräten ausgetauscht werden können. Die hochgradig entwicklerfreundlichen Bibliotheken integrieren das IOTA-Protokoll und dienen als Referenzimplementierung für jeden, der nach Inspiration oder den besten Tools seiner Klasse sucht. Die Low-Level-Bibliotheken haben keinerlei Bezug zur Kryptowährung oder dem Tangle. Diese Funktionen befinden sich auf höherer Ebene, so dass Stronghold auch für Software-Projekte eingesetzt werden kann, die nicht direkt etwas mit Distributed Ledger zu tun haben, sondern bspw. lediglich ihre wertvollen Daten lokal schützen wollen. Mit anderen Worten: Jede Person aus jeder Branche kann sie benutzen.

**IOTA Streams**: Streams verfügen über eine integrierte Methode zum  Senden von Nachrichten an IOTA-Nodes. Sie sind jedoch auch so flexibel, dass sie erweitert werden können, um Nachrichten auf andere Weise zu senden, z.B. in  HTTP-URLs. Es handelt sich um ein multifunktionales Second-Layer-Datenübertragungsprotokoll, welches für verschiedene Arten der  Datenübertragung (z.B. Streaming-Daten) verwendet werden kann. Bspw. ermöglicht es Sensoren und anderen Geräten, ganze Datenströme zu verschlüsseln  und im IOTA-Tangle zu verankern. Das Konsensprotokoll von IOTA fügt diesen  Nachrichtenströmen Integrität und Authentizität hinzu. Angesichts  dieser Eigenschaften erfüllt IOTA Streams ein wichtiges Bedürfnis in Branchen, in denen Integrität, Datenschutz und Unveränderlichkeit aufeinandertreffen.

**IOTA Access**: IOTA Access ist ein Open-Source-DLT-Framework für den Aufbau richtlinienbasierter Zugangskontrollsysteme und die Ermöglichung von Pay-per-Use-Funktionalitäten am Rande des Netzwerks (Endgeräte). Es wurde entwickelt, um eine fein abgestufte Zugangskontrolle für jede Maschine, jedes Gerät und jedes Gebäude zu ermöglichen, ohne auf ein zentralisiertes System angewiesen zu sein oder eine ständige Internetverbindung zu benötigen. Bei IOTA Access geht es um die Ermöglichung von Dienstleistungen und Sicherheit in großem Maßstab. Wenn es ein Gerät gibt, welches einen Dienst anbieten kann, dann kann Access in dieses Gerät integriert werden, um diesen Dienst durch eingebettete Zugangskontrollrichtlinien zu automatisieren. Auf diese Weise können Gerätebenutzer und -besitzer den Zugriff auf ihr Gerät oder ihren Datenstrom auf entfernte, unbefugte, kontaktlose und überprüfbare Weise gewähren oder anfordern.

**Digital Assets**: Dies sind zweckgebundene Token, die  Vermögenswerte aus der realen Welt (auf Basis von IOTA (on Top)) auf dem  Tangle manipulationssicher darstellen können. Sie sind nicht dafür  vorgesehen, Geldwerte zu handeln – mit ihnen könnte man bspw. etwas einlösen oder Eigentumsverhältnisse regeln. Konkret geht es hierbei um die Tokenisierung von Vermögenswerten wie z.B. Immobilien, Automobilen, Unternehmensanteilen oder Goldbarren.

**IOTA Smart Contracts**: Ein Smart Contract (dt. Intelligenter  Vertrag) ist eine programmierte Vereinbarung, die vollständig  deterministisch ist und automatisch durchgesetzt wird. Dazu  werden die vertraglichen Verpflichtungen zwischen Käufer und  Verkäufer verkapselt in der Software verankert. Dies macht es  unmöglich diese getroffene Vereinbarung zu bestreiten. 

![](https://iota-einsteiger-guide.de/media/images/sc-.gif)

Der Hauptunterschied von IOTA Smart Contracts zu  herkömmlichen Smart Contracts (wie bspw. im Ethereum-Netzwerk) ist die Multi-Chain-Umgebung, die durch den Tangle gesichert ist und viele reguläre Blockchains parallel ausführen kann. Die IOTA Smart-Contract-Chains laufen asynchron - daher hat die Aktivität eines Smart Contracts  keinen Einfluss auf die Geschwindigkeit und den Durchsatz der anderen Smart-Contract-Chains.

![](https://iota-einsteiger-guide.de/media/images/sc3.gif)

Volatile und unvorhersehbare Gebühren sind ein Hindernis für die Einführung von Smart Contracts. In IOTA Smart Contracts werden die Gebühren vom Eigentümer definiert, beginnen bei 0 und sind im Voraus bekannt. Es gibt keinen "Bieterkrieg" wie bei regulären Blockchains, weshalb mit vorhersehbaren Geschäftsmodellen und Einnahmequellen kalkuliert werden kann.

![](https://iota-einsteiger-guide.de/media/images/flexible-committees.gif)

IOTA Smart Contracts sind hochflexibel. Komitees können von einer Einzelperson, einem Unternehmenskonsortium oder durch einen offenen, erlaubnisfreien Marktplatz von Validator-Nodes gebildet werden.

![](https://iota-einsteiger-guide.de/media/images/virtualmachines-01.gif)

IOTA Smart Contracts sind skalierbar. Mehrere Smart-Contract-Chains, die von verschiedenen Virtualmachines betrieben werden und können gleichzeitig/parallel über mehrere Komitees laufen.

**IOTA Oracles**: Oracles wurden entwickelt, um eine sichere Brücke zwischen der digitalen und der physischen Welt auf eine dezentrale, erlaubnisfreie Weise zu bauen. Sie  bringen im IOTA-Netzwerk Off-Chain-Daten zu dezentralen Anwendungen und Smart Contracts. 
Generell leiden Oracles unter dem uralten "Garbage in, garbage out"-Problem (dt. Müll rein, Müll raus). Wenn die Daten, die in ein Oracle eingehen, manipuliert oder zensiert werden können, ist es möglich, dass "falsche" Daten zu "falschen" Ergebnissen führen. Einige Oracle-Lösungen versuchen dieses Problem zu lösen, indem sie sicherstellen, dass die Oracle Eingaben aus einer ausreichend großen Anzahl unabhängiger Quellen verwenden. Andere Oracles auf dem Markt haben eine Reihe von Standards vorgeschlagen, um Off-Chain-Daten auf die Chain zu bringen. Aber auch sie leiden unter inhärenten Engpässen, die den Einsatz in der realen Welt behindern können. Aus Sicht von IOTA müssen optimal vertrauenswürdige Daten direkt von der Ursprungsquelle kommen, an der die Daten erzeugt werden, während sie dezentral verarbeitet und gesichert werden.


![](https://iota-einsteiger-guide.de/media/images/evd78pwxaaegh3.gif)

Das Potential der Datenmanipulation wird stark reduziert, wenn die Quelle (z.B. ein Sensor) die Daten direkt, also ohne den Umweg über mehrere Intermediäre, an ein manipulationssicheres Distributed Ledger übergibt. Das einfachste IOTA-Oracle ist das s.g. First-Party-Oracle. IOTA-First-Party-Oracles nutzen weder externe Datenquellen noch Daten, die von einem Dritten verarbeitet und auf einem DLT zur Verfügung gestellt wurden, sondern verlassen sich auf Daten, die vom Datenherausgeber selbst an den IOTA-Tangle übermittelt wurden. Im Kontext von IoT-Netzwerken würde sich der "Datenausgeber" auf die Sensoren selbst beziehen, ohne dass diese von irgendjemandem oder irgendetwas manipuliert oder umformatiert wurden. Sobald IOTA Smart Contracts live sind, können First-Party-Oracles verwendet werden, um Smart Contracts direkt mit Daten zu füttern, ohne sich um einen Vermittler kümmern zu müssen, der die Daten abruft, verarbeitet, hostet oder pflegt. Im Gegensatz zu einigen anderen Lösungen auf dem Markt interagiert die IOTA Foundation niemals mit Daten, die von Datenanbietern bereitgestellt werden. Dazu kann/wird den Daten mittels Alvarium ein Confidence Score hinzugefügt.


**Flash Channels**: Ein Flash Channel ist ein offline Zahlungskanal für Ein- und  Auszahlungen, der sofortige Transaktionen mit hohem Durchsatz  ermöglicht. Dies ist möglich, da im IOTA-Tangle nur zwei Transaktionen  stattfinden, nämlich das Öffnen und Schließen eines Flash-Kanals. Im  Wesentlichen bieten sie den beiden Geschäftspartnern eine Möglichkeit, mit  hoher Frequenz zu handeln, ohne darauf zu warten, dass jede Transaktion im  öffentlichen IOTA-Netzwerk bestätigt wird. Dieses Feature wird zukünftig innerhalb von IOTA Streams umgesetzt.

**Digitale  Marktplätze**: Daten werden seit einiger Zeit als das neue  Öl der digitalen Welt bezeichnet. Ein Grundgedanke des IoT ist der  Austausch und Handel von solchen Datensätzen. Mit der „Maschine zu Maschine“-Kommunikation können Geräte die verschiedensten Daten autonom untereinander  handeln. Aber auch für den Menschen könnte es sehr interessant werden, direkt  vom sensorischen Endverbraucher Daten oder Güter zu erhalten, ohne den Weg  über eine Drittanbieter-Plattform gehen zu müssen. Das Erschaffen von  Datenmarktplätzen ist eine unvermeidliche Konsequenz der IoT-Revolution. Es  wird die Art und Weise, wie wir Daten verbinden, mit ihnen interagieren oder sie handeln grundlegend verändern.  

**Digitale Zwillinge**: Ein digitaler Zwilling ist eine  digitale Repräsentanz eines materiellen oder immateriellen Objekts oder  Prozesses aus der realen Welt in der digitalen Welt. Das Industrial  IOTA Lab Aachen, Pickert, ECLASS und das Digital Twin Konsortium treiben die Technologie für IOTA voran.

## Anwendungsgebiete

Grundsätzlich wird fast jede Branche digitalisiert, mit dem IoT verknüpft und dementsprechend seine Vorteile daraus ziehen. Die IOTA Foundation beschreibt einige Anwendungsfälle in einer veröffentlichten interaktiven Broschüre:  

 
![](https://iota-einsteiger-guide.de/media/images/2020-05-24-20_38_32-1st-meetup-introduction-to-iota.pdf-adobe-acrobat-reader-dc.png)
