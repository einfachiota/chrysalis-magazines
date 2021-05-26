<!--
---article_info
title: IOTA Einsteiger Guide
author: [Schmucklos]
reviews: [reviewer_1, reviewer_2]
---
-->

# IOTA Einsteiger Guide

> Der IOTA Einsteiger Guide ist eine Website, die neuste Artikel und die Grundlagen auf deutsch für jeden kostenlos anbietet. Schau auf jeden Fall mal vorbei! 
>
> **https://iota-einsteiger-guide.de**

## Was ist IOTA?  - eine Zusammenfassung
IOTA ist keine Blockchain, IOTA ist ein skalierbares Open-Source Kommunikationsprotokoll mit Token (Kryptowährung) für einen Wertetransfer, es wird entwickelt und bereitgestellt von der nicht gewinnorientierten IOTA Foundation (Stiftung nach deutschem Recht) mit Hauptsitz in Berlin. 

Der Name IOTA stammt aus dem altgriechischem Alphabet (Ἰῶτα „der kleinste Buchstabe") und bezeichnet "etwas Geringes". Das Iota ist der 9. Buchstabe des altgriechischen Alphabets und wurde in der Antike wie heute identisch, nämlich i ausgesprochen; von ihm stammt auch der lateinische Buchstabe i ab. Im IOTA Kommunikationsprotokoll steht das i für 1 Iota und ist im Netzwerk die kleinste handelbare Werteinheit, hier schließt sich der Kreis zu "etwas Geringes".

Das Ziel der IOTA Foundation ist es, eine Vertrauensschicht (engl. Trust Layer) für das Internet of Everything (IoE) zu erschaffen, die es Geräten im IoE ermöglicht unveränderlich Daten und Werte untereinander gebührenfrei auszutauschen. IOTA strebt in Zusammenarbeit mit der Industrie und der Object-Management-Group, eine Standardisierung ihres Kommunikationsprotokolls an. Mit einer hohen Interoperabilität wird IOTA das "Ledger Of Everything" sein, dessen Infrastruktur auch von externen Anwendungen ohne Erlaubnis genutzt werden kann.

 
![](https://iota-einsteiger-guide.de/media/images/2020-08-16-17_34_34-iota_ledger_of_everything.jpg-irfanview.png)

IOTA ermöglicht eine schnelle, manipulationssichere und dezentrale Übertragung von Werten und Daten über viele Nodes, dabei werden Werte- und Daten-Transaktionen grundsätzlich unterschiedlich gehandhabt, während Werte-Transaktionen von Full-Nodes validiert werden müssen, werden Daten-Transaktionen direkt bestätigt und sind notarisiert. Jetzt wird sich der ein oder andere Fragen, warum benötige ich für eine reine Daten-Transaktion die IOTA Distributed Ledger Technologie, ich kann die Daten doch einfach verschlüsseln, signieren und via TCP/IP versenden. 

Nun, abgesehen davon, dass „Man in the Middle“ Angriffe möglich wären, beweisen signierte Daten nur, dass die Daten von Ihnen kommen. Es erlaubt mir weder zu beweisen, wann Sie sie gesendet haben, noch ob Sie die gleichen Daten an alle gesendet haben. Sie könnten eine bestimmte Information an eine Person und eine andere Information an eine andere Person senden. Signaturen allein werden niemanden vor solchen Dingen schützen.

Mithilfe der „Notarisierung“ kann bewiesen werden, dass ein elektronisches Dokument in einer bestimmten Form zu einem bestimmten Zeitpunkt existiert hat und seit der Erstellung nicht verändert wurde. Bei der Erstellung einer Notarisierung wird ein eindeutiger Hash (Fingerabdruck) eines Dokumentes berechnet und gemeinsam mit einem Zeitstempel im IOTA-Ledger (Tangle) unveränderbar gespeichert. Falls zu einem späteren Zeitpunkt verifiziert werden soll, dass das betreffende Dokument zum behaupteten Zeitpunkt existiert hat und/oder nicht verändert wurde, werden die Daten aus dem Tangle abgerufen und mit den vorliegenden Informationen verglichen.

Wenn das IOTA Tangle als Transportmedium verwendet wird, dann sind diese Art von Angriffen nicht möglich und es gibt viele Anwendungsfälle, bei denen es entscheidend ist, dass alle beteiligten Parteien sicher sein können, dass alle an oder mit den gleichen Informationen arbeiten (z.B. Lieferketten, Oracle, synchronisierte Fernsteuerungssysteme und so weiter). Es geht daher primär um Usecases die eben nicht einfach via TCP/IP zu übertragen sind, sonst könnte man auch gleich BitTorrent dafür nutzen. Sender und Empfänger benötigen eine transparente Form der Übertragung zwischen mehreren Parteien oder Organisationseinheiten.

Unterm Strich bedeutet dies, dass es nicht nur darum geht, dass niemand die Daten während der Übertragung manipuliert, sondern auch darum, dass der Empfänger diese Daten nicht manipuliert. Beispiel: Ein Sensor (mit IDoT Chip) hat einige Werte gemessen/erfasst und versendet diese Daten über den IOTA-Tangle, welcher den Hash dieser Daten speichert. Wenn diese Daten später verkauft werden sollen, kann dieser Hash als Nachweis vorgelegt werden und dem Käufer anhand des Tangle beweisen, dass die Daten vom Sensor im Nachhinein nicht verändert wurden. Die IOTA Technologie (Tangle) fungiert also wie eine Art Fingerabdruck, mit ihm können alle gesendeten Daten verifiziert werden. Hinweis: Um Missbrauchsszenarien zu verhindern und auf eine effiziente Art und Weise, die Maschinenökonomie zu ermöglichen, werden zukünftig Nodes und Geräte eine eindeutige Kennung (ID) erhalten, siehe auch Identität der Dinge (IDoT).

Die Guthaben auf den Adressen sind allen teilnehmenden Full-Nodes jederzeit bekannt, darüber hinaus versteht sich IOTA jedoch nicht als Datenspeicher, so dass die Transaktionshistorie auf den Full-Nodes nicht über einen längeren Zeitraum gespeichert wird - analog zum TCP/IP-Traffic des Internets, der ebenfalls nicht gespeichert wird. Möchte ein Nutzer oder ein Unternehmen zusätzlich auch die Transaktionshistorie über mehrere Jahre hinweg abrufen bzw. speichern, müssen dafür zusätzliche Lösungen eingesetzt werden, dies könnte ein selektiver Permanode, eine eigene Second-Layer Anwendung oder zukünftig auch ein Smart-Contract sein, welcher mehrere Nodes für diese Dienstleistung bezahlt.

Das Kommunikationsprotokoll ist modular aufgebaut, dies ermöglicht zukünftig schnellere und einfachere Updates. Für die Redundanz im Netzwerk sind sehr viele Nodes nötig, jedes Auto, Maschine, Router, usw. kann in ein paar Jahren eine Node sein. Je mehr Nodes am Netzwerk teilnehmen, umso schneller und sicherer wird das Netzwerk.


## Technologie


Um die oben genannten Ziele zu ermöglichen, sind folgenden Merkmale von grundlegender Bedeutung:

Skalierbar - Verarbeiten einer beträchtliche Anzahl von Transaktionen pro Sekunde über ein großes Netzwerk von Nodes mit schnellen Bestätigungszeiten.  
Schlankes System  - Leistungsschwache Geräte sollten in  der Lage sein, direkt am Netzwerk teilzunehmen.  
Gebührenfrei - Das Senden von Transaktionen sollte ohne Zahlung von Netzwerkgebühren erfolgen, wenn 50Mi versendet werden, kommen auch  50Mi an. 
 


![](https://iota-einsteiger-guide.de/media/images/feeless-animation.gif)
 

### Datenstruktur

Neben den Gebühren weisen herkömmliche DLTs wie die Blockchain weitere begrenzende Faktoren auf und sind daher für die Erreichung des IOTA-Ziels ungeeignet. Beispielsweise die inhärente Beschränkung der Geschwindigkeit von Blockchain-Netzwerken, diese wird allgemein als "Blockchain-Bottleneck" bezeichnet. In der Blockchain gibt es nur eine Seite, an dem neue Transaktionen angefügt werden können - das Ende der Kette. Die daraus resultierenden negativen Auswirkungen auf den Netzwerkdurchsatz werden in dieser einfachen Grafik dargestellt:


![](https://iota-einsteiger-guide.de/media/images/blockchain_bottleneck.gif)


Im Gegensatz dazu, wird bei IOTA nicht Block an Block aneinandergereiht und es werden auch keine Miner zur Validierung von Transaktionen benötigt. Die Kerndatenstruktur bei IOTA ist von Natur aus hoch skalierbar, dies wird mit einer einfachen Regel ermöglicht: Jede Transaktion referenziert und genehmigt bis zu acht weitere Transaktionen. Diese Regel definiert die zugrundeliegende Datenstruktur von IOTA - das Tangle - das mathematisch als gerichteter azyklischer Graph (DAG) bezeichnet wird.

eine Menge von Transaktionen ist durch Pfade verbunden, dies ist  ein Graph  
jeder dieser Pfade besitzt eine eindeutig festgelegte Laufrichtung, damit  ist es ein gerichteter Graph  
lässt sich niemals ein Pfad finden, der zu seinem Ausgangspunkt  zurückkehrt, ist der Graph azyklisch (im Kreis laufenden  Pfade wären dagegen zyklisch)
 

Transaktionsstatus

![](https://iota-einsteiger-guide.de/media/images/1234567.png)      

grün = bestätigt (Konsens)   /   weiß = unbestätigt   /   grau = neu angehängt (Tips)

 

Anstatt wie bei der Blockchain auf einer einzigen Seite für das Anhängen neuer Transaktionen begrenzt zu sein, bieten DAGs mehrere Punkte an, an denen Transaktionen angehängt werden können. Benutzer können weiterhin neue Transaktionen an verschiedene Teile des Tangle anhängen, ohne auf die Bestätigung durch andere Transaktionen zu warten:

![](https://iota-einsteiger-guide.de/media/images/tangle_bottleneck.gif)

In dieser chaotischen Ordnung werden sämtliche Transaktionen parallel abgearbeitet und ermöglichen eine sehr hohe Skalierung, da mit steigender Anzahl der Transaktionen auch die Bestätigungsraten ansteigen, statt sich genau gegenläufig zu entwickeln, wie es derzeit bei den klassischen Blockchains der Fall ist. 



### Konsensmechanismus

In der Blockchain teilt der Nakamoto-Konsens das Netzwerk in Miner und Nutzer auf. Miner verbrauchen große Mengen an Rechenleistung und erfüllen damit den Proof-of-Work (PoW), der für die Verkettung der Blöcke erforderlich ist. Miner werden durch die Gebühren, die Benutzer bereit sind zu zahlen, motiviert, damit ihre Transaktion in einen Block aufgenommen wird. Diese gebührenbasierte Anreizstruktur wäre eine erhebliche Barriere in einer Machine-to-Machine-Wirtschaft, in der die Mikrozahlungswerte zwischen den Maschinen niedriger sein können als die anfallenden Gebühren.

Bei IOTA gibt es keinen Unterschied zwischen Minern und Nutzern. Alle Nodes können am Konsens teilnehmen. Das bedeutet, dass ein IOTA-Node eine ganz andere Rolle spielt als ein Bitcoin-Miner. IOTA-Nodes führen nur grundlegende Operationen durch, die sie nicht viel Rechenleistung erfordern (z.B. Speichern des Ledgers, Validieren von Transaktionen). Benutzer können mit minimalen Kosten einen Node einrichten und aktiv am Netzwerkkonsens teilnehmen und so die Sicherheit des Netzwerks erhöhen.

Der Konsensmechanismus legt fest wie sich die Nodes darauf einigen, welche Transaktionen vertrauenswürdig sind, somit gewährleistet er Übereinstimmung im Netzwerk. In der aktuellen IOTA-Implementierung (Stand Juni'20) vertrauen Nodes nur (Wert-)Transaktionen, die von Meilensteinen referenziert und genehmigt werden, die vom Koordinator ausgestellt wurden. Ähnlich wie beim Bitcoin (Checkpoints bis zum 16.06.2014) ist der Einsatz dieses zentralisierten "Finalitätsvorrichtung" derzeit noch notwendig, um die Sicherheit des in den Kinderschuhen steckenden Netzwerks zu gewährleisten. 

Hinweis: Der Open-Source Koordinator kann nur Transaktionen bestätigen, Konsensregeln umgehen kann er nicht, somit ist es ihm auch nicht möglich Token zu erschaffen, einzufrieren oder zu entwenden. Zur Überwachung ist diese feste Regel und die Adresse des Koordinators in jedem Node fest programmiert, der Einfluss des Koordinators auf den Tangle ist daher sehr stark eingeschränkt, da dieser zusätzlich ständig von allen anderen Nodes überwacht wird.


![](https://iota-einsteiger-guide.de/media/images/milestones.gif)
    

### IOTA ohne Koordinator

Der Koordinator wird in naher Zukunft abgeschaltet (Coordicide), diesbezüglich wurden alle Forschungsarbeiten weitestgehend abgeschlossen und von mehreren Universitäten überprüft. Der Entwicklungsfahrplan (Roadmap) der IOTA zur Produktionsreife und Adoption führt wurde definiert und wird derzeit umgesetzt. Vor dem Coordicide wird das IOTA-Netzwerk mit einer Zwischenaktualisierung namens Chrysalis vollständig optimiert, um eine Unternehmenstaugliche Lösung für das Ökosystem anzubieten, auch hier wurde bereits eine Release-Strategie veröffentlicht.

 

Die anstehende Abschaltung (voraussichtlich 2021) des Koordinators stellt sicher, dass das Netzwerk ohne den Koordinator einen Konsens erzielt und gleichzeitig die folgenden Eigenschaften aufweist:

Skalierbar: Die Transaktionsrate im Netzwerk ist  nicht durch das Protokoll begrenzt, eine beispiellose Skalierbarkeit wird  ermöglicht.  
Sicher: Ein Angreifer kann den Konsens nicht  beeinflussen.  
Dezentral: Alle ehrlichen Nodes können Teil des Konsensprozesses sein. 

![](https://iota-einsteiger-guide.de/media/images/trilemma.gif)

Aktuelle Blockchain-Lösungen können maximal zwei dieser drei Eigenschaften gleichzeitig lösen. Die Algorithmen wurden so entwickelt, dass sie die Mining-Schwierigkeit an einem bestimmten Punkt halten, sodass das Netzwerk keinen neuen Block produzieren kann, während der bestehende Block von allen Nodes geklatscht (im Netzwerk verteilt) und verifiziert wird. Das bedeutet: Je größer die Anzahl der Nodes im Netzwerk ist, desto länger dauert es, bis der Block bei allen Nodes ankommt und verifiziert wird. Wäre die Blockzeit gering, würden viele Blöcke gleichzeitig von verschiedenen Akteuren produziert werden und der Nakamoto-Konsens könnte sich nicht auf ein einziges Ergebnis einigen. Verschiedene Nodes würden verschiedene Chains als die längste Chain zur gleichen Zeit sehen.

Sie lösen dies, indem sie die Mining-Schwierigkeit hoch ansetzen, sodass die Blockzeit im Durchschnitt 10 Minuten beträgt. In diesen 10 Minuten versuchen verschiedene Akteure, einen Block zu erzeugen, aber normalerweise gelingt es nur einem. Alle anderen lassen alles fallen, was sie gerade taten, akzeptieren den erfolgreichen Block und versuchen es erneut. Es gibt Zeiten, in denen mehrere Akteure (normalerweise 2) Glück haben und einen gültigen Block zur gleichen Zeit erstellen. Die Hälfte des Netzwerks akzeptiert 1 Block und die andere Hälfte akzeptiert den zweiten. Dies wird gelöst, indem der nächste Miner den nächsten Block findet und ihn an eine Kette anhängt. Diese Kette ist die längste und sie gewinnt. Alle Nodes akzeptieren sie als gültig. So wird es in PoW gemacht und auch wenn es in PoS keine Miner gibt, ist das Konzept ähnlich. Die Nodes müssen sich auf einen Block einigen, bevor sie mit der Produktion des nächsten Blocks beginnen. Mit der Anzahl der Nodes steigt auch die Netzwerkverzögerung (Zeit, bis der Block an alle anderen Nodes geklatscht wird). Da der Nakamoto-Konsens ein synchroner Konsens ist, kann er bestenfalls 2 der 3 Punkte (Sicherheit, Dezentralisierung, Skalierbarkeit) lösen.

Dieses Problem wird als "Blockchain-Trilemma" bezeichnet. 

PS: Durch die begrenzte Blockgröße entsteht ein weiteres Problem, da immer mehr Transaktionen über das Netzwerk gesendet werden, entscheiden sich die Miner dafür, die Transaktionen mit den höchsten Gebühren zu validieren, was zu steigenden Transaktionskosten und langen Wartezeiten führt. Der revolutionäre Charakter dieser PoW-basierten Lösung sollte nicht unterschätzt werden - aber gleichzeitig darf man die damit verbundenen Einschränkungen des Netzwerkdurchsatzes nicht vernachlässigen.



IOTA ist durch seine asynchrone Natur grundsätzlich kein Opfer des Blockchain-Trilemma, jeder Node darf gleichzeitig Transaktionen produzieren, während er mit einer Teilmenge von Nodes für den Konsens interagiert und dabei eine nicht lineare Datenstruktur verwendet. Mit IOTA 3.0 (also inkl. Sharding) wird IOTA eine Lösung anbieten, welche das Trilemma umgeht und damit eigentlich auch löst (Ansichtssache). Was ist Sharding: IOTA-Nodes haben eine Obergrenze für Transaktionen pro Sekunde (TPS), die sie verarbeiten können, durch eine Form der Datenbankpartitionierung (das Aufteilen einer sehr großen Datenbank in kleinere) in besser verwaltbare Segmente (Shards) würde jeder Shard einen einzigartigen Satz der Kontensalden enthalten und Nodes würden dann einzelnen Shards zugewiesen, um Transaktionen zu validieren. 

![](https://iota-einsteiger-guide.de/media/images/3_wcj-m9uqsociqfrx1bexrq.png)

Ziel ist es, dass durch die Aufteilung in besser verwaltbare Segmente der Transaktionsdurchsatz erhöht wird. Das Trilemma wird innerhalb einzelner Shards weiterhin bestehen, aber sobald in einem einzelnem Shard der Netzwerkdurchsatz die Verarbeitungskapazitäten der Nodes übersteigt, bilden sich dynamisch ein weiterer Shard (Fluid-Sharding). Durch das neuartige sehr flexible Sharding werden bei IOTA theoretisch unendliche hohe Transaktions-Geschwindigkeiten ermöglicht.

Das Tangle ermöglicht es Benutzern, neue Transaktionen an jeden Teil des Tangle anzuhängen. Aufgrund der DAG-Struktur ist lediglich erforderlich, dass jede neue Transaktion bis zu acht  weitere Transaktionen referenziert. Durch die Möglichkeit, mehrere Anhängepunkte zu verwenden und die Vermeidung der Notwendigkeit von Blöcken, ist das Tangle von Natur aus skalierbar. Es gibt eigentlich keine protokollbedingten Engpässe, die Skalierbarkeit ist nur durch die Hardware und die Gesetze der Physik eingeschränkt. 

Derzeit ist der Koordinator ein solcher Engpass, welcher das Protokoll an der Skalierung hindert. Zudem gewährleistet der Koordinator zwar die Netzwerksicherheit, stellt aber auch einen Single Point of Failure (einzige Fehlerquelle) dar und verhindert somit auch, dass IOTA ein vollständig dezentrales Netzwerk ist. Daher ist die Entfernung des Koordinators aus dem Konsens-Mechanismus das Hauptziel der IOTA Foundation.

 

### Der verbesserte Tangle nach dem Coordicide: Dezentral, skalierbar und sicher

Die Entfernung des Koordinators (Projekt: Coordicide) allein reicht nicht aus, um das Skalierbarkeits-Trilemma sicher und dezentral zu lösen. Für ein solches Netzwerk müssen weitere Sicherheitsmodule implementiert werden die einer hohen Transaktionsrate nicht im Wege stehen. Kern der Lösung ist eine zusätzliche Sicherheitsebene mit dem proaktiven Abstimmungsmechanismus namens Shimmer, über den Nodes die Meinungen anderer Nodes anfordern, um zu entscheiden, welche Transaktionen in das Tangle aufgenommen und welche abgewiesen (verwaist) werden sollen.

![](https://iota-einsteiger-guide.de/media/images/voting-01.gif)

Um den Koordinator zu entfernen, müssen eine Reihe von Herausforderungen gelöst werden. Aufgrund der Komplexität der Lösung wird der Coordicide in verschiedene Komponenten zerlegt. Dieser Ansatz macht den Coordicide-Vorschlag und das zukünftige Protokoll modular, so dass jedes Modul unabhängig voneinander ersetzt werden kann, wenn zukünftig neue Forschungsergebnisse weitere Verbesserungen ergeben.

Hinweis: Wer direkt tiefere Einblicke zum Post Koordinator Konsensmechanismus wünscht, der erfährt in dem Themenbereich Coordicide mehr (nicht mehr ganz aktuell), bitte unbedingt auch alle aktuelleren weiterführenden Erklärungen lesen.

 

 

Schlüsselfaktoren der IOTA Distributed Ledger Technologie im produktionsfertigen Zustand (nach der Entfernung des Koordinators)

Vollständige Dezentralisierung: Als global  verteiltes Netzwerk ist die IOTA widerstandsfähig und robust gegen Angriffe,  ohne den Koordinator wird keine Instanz eine besondere Rolle im Netzwerk  spielen, um es klarzustellen: Sobald der Koordinator abgeschaltet ist, wird es  der IOTA-Stiftung nicht mehr möglich sein, ihn neu zu starten.  
Permissionless / Erlaubnisloser Zugang zum  Netzwerk: Jeder oder besser alles kann jederzeit dem  Netzwerk beitreten, um Transaktionen hinzuzufügen und zu  validieren. Wenn andere DLTs ihre Nodeanzahl begrenzen  oder sogar einen zugelassenen Ansatz für die Skalierbarkeit einführen müssen,  umfasst IOTAs Ansatz zusätzliche Nodes. Eine größere Anzahl von ehrlichen  Nodes verbessert die Sicherheit des Netzwerks, indem sie den Anteil der  ehrlichen Stimmen erhöht.  
Partitionstolerant: Ein produktionsfertiges Tangle ist partitionstolerant, dass bedeutet, ein Teil des Tangle kann für eine bestimmte Zeit vom Main-Tangle abgetrennt werden und ohne Internet-Verbindung weiterlaufen. Diese Teile können wieder mit dem Main-Tangle verbunden werden, wenn die Internetverbindung wieder hergestellt wurde.
Endgültigkeit innerhalb von Sekunden: Der Abstimmungsprozess ermöglicht es dem Netzwerk, Entscheidungen über Transaktionen sehr schnell zu treffen und abzuschließen, ohne auf mehrere zusätzliche Genehmigungsvorgänge warten zu müssen, um die Sicherheit zu erhöhen. Darüber hinaus kann das Netzwerk "wahre Endgültigkeit" (deterministisch statt probabilistisch) erreichen, da ein Angreifer mit unbegrenzter Hashing-Power den Ledger-Zustand nicht "umkehren" könnte.
Zuverlässiger Zeitstempel: Der Emittent (Node/Benutzer) der Transaktion, setzt zum Zeitpunkt der Ausgabe einen Zeitstempel. Die Einigung über die Glaubwürdigkeit von Zeitstempeln ermöglicht es IOTA, ein vollständig geordnetes Tangle zu etablieren - ein großer Schritt hin zu Smart Contracts.
Skalierbarkeit: Eine erhöhte Netzwerkaktivität verringert die Transaktionsabwicklungszeiten, es gibt keine protokollbedingten Engpässe. Die Skalierbarkeit ist nur durch die Hardware und die Gesetze der Physik eingeschränkt. Der Wegfall des Koordinators als eine einzige Einheit, die alle Transaktionen verarbeitet und verifiziert, bildet die Grundlage für einen dynamischen Sharding-Prozess, der eine "echte" unbegrenzte Skalierbarkeit ermöglicht.
Ein adaptiver Ratenregelungsalgorithmus, der auf Nodebasis arbeitet, wird es Angreifern unmöglich machen, das Netzwerk mit ungesundem Spam zu überfordern, während ehrliche Nodes dennoch normal arbeiten können.
Erhöhte Zuverlässigkeit: Die Bestimmung des  bevorzugten Teils des Tangle durch Abstimmung ermöglicht die Umsetzung  unterschiedlicher Strategien zur Tip-Auswahl, durch die  die meisten (wenn nicht sogar alle) ehrlichen Transaktionen aufgegriffen  werden. Dies reduziert den Bedarf an Reattachments und Promotionen.  
Sicherheit  / Mana: Der neuartige Sybil-Schutzmechanismus namens Mana sichert zusammen mit weiteren  Sicherheitsmechanismen wie beispielsweise dem  Abstimmungsmechanismus (FPC) das Netzwerk auch bei einer  großen Anzahl von böswilligen Akteuren.  
Intelligentes und stabiles Auto-Peering:  Der Wegfall des manuellen Peering reduziert den Wartungsaufwand für die  Nodebetreiber und macht das Netzwerk stabiler.  
UTXO-Modell: Jeder  Token auf einer Adresse ist dann eindeutig identifizierbar und jede Ausgabe  benennt genau den Token, die sie bewegen möchten. Dies ermöglicht eine  schnellere und genauere Konfliktbehandlung und verbessert die  Belastbarkeit sowie die Sicherheit des Protokolls.  
Gebührenfreie Transaktionen: Das Fehlen von Minern macht  IOTA-Transaktionen völlig gebührenfrei, 1 Cent senden, 1 Cent  erhalten. Dies ermöglicht echte Mikrozahlungen für die  Machine-to-Machine-Wirtschaft, um damit auch die Bezahlung für Kleinstmengen  oder verbrauchsabhängige Bezahlungen sinnvoll umzusetzen.
Weniger Dokumentation: Unternehmen müssen gebührenfreie Transaktionen nicht für das Finanzamt dokumentieren oder für Jahre speichern.
Lokale Snapshots ermöglichen es Nodes, nur eine Teilmenge der Historie des Ledgers zu speichern, so dass Nodes mit begrenzten Hardware-Ressourcen am Netzwerk teilnehmen können.
Fairness: Alle Transaktionen werden gleich behandelt. Es gibt keine Möglichkeit, eine Prämie zu zahlen (z.B. durch erhöhte Gebühren), um eine höhere Priorität bei der Verarbeitung durch das Netzwerk zu erhalten.
Schlankes System: Konzipiert für Geräte, wie z.B. Sensoren, die an einem Niedrigenergie-Netzwerk teilnehmen.
Modularität: Ein modularer Aufbau ermöglicht es, die Bestandteile des Protokolls unabhängig voneinander zu entwickeln. Der mehrschichtige Ansatz ermöglicht zukünftig weitere Erweiterung des Basisprotokolls in ähnlicher Weise wie das Internetprotokoll selbst. Ein Protokoll welches sich nicht updaten lässt, ist kein Protokoll.
Zuverlässige Führung: Die gemeinnützige Organisation nach deutschem Recht hinter IOTA optimiert die Akzeptanz und zukünftige Entwicklung des Netzwerks. Das Fehlen von Minern ermöglicht es, neue Funktionen ohne Interessenkonflikte umzusetzen.
Open-Source: Die Technologie ist kostenlos, Open Source, und jeder kann Lösungen darauf aufbauen.
Unveränderlich: Stellt sicher, dass die Informationen vertrauenswürdig sind und nicht manipuliert werden können.
Daten-Transaktionen: Unabhängig zu der Wert-Transaktionen. Die sogenannten "Messages" ermöglichen Anwendungsfälle der Technologie, die über das Finanzielle hinausgehen. Die große Mehrheit alle Transaktionen werden reine "Messages" (ohne Wert) sein, dass können Daten sein die beispielsweise auf Marktplätzen gehandelt, von Sensoren erfasst oder auch von Apps ausgetauscht werden und noch etliche andere Anwendungsfälle. In dem unten stehenden Bild ist zu sehen, wie allein durch die IOTA Architektur der schnelle Austausch von Daten und Werten gegenüber der Blockchain  begünstigt wird.
 


![](https://iota-einsteiger-guide.de/media/images/2020-07-06-16_56_24-architektur.pptx-powerpoint.png)



## Lösungen für unterschiedliche Anwendungsfälle
Die Sicherstellung der Wahrhaftigkeit ist genau das, was die Distributed Ledger Technologie ermöglicht. IOTA ist ein Protokoll, welches einen Konsens über den Stand der Dinge in einem Netzwerk erzielt, indem es eine einzige kryptographisch sichere Quelle mit einer einheitlichen Wahrheit zur Verfügung stellt. Dies wird in Zukunft eine große technologische Innovationsexplosion nach sich ziehen und völlig neue Anwendungsfälle bzw. Geschäftsfelder mit Begleitwirtschaft ermöglichen, die in der Vergangenheit unmöglich waren, weil diese beispielsweise zentralisiert, nicht skalierbar, zu teuer, nicht sicher oder die Regeln des Datenschutzes verletzt haben.

 

IOTA wird für eine Vielzahl dieser neuen Anwendungsfälle Lösungen anbieten, nachfolgend teaser ich die wichtigsten Aspekte an: 

Vielseitig einsetzbar: Das IOTA Protokoll kann auch als  Instrument zur Erhaltung eines freien und offenen Internets eingesetzt werden,  indem es die Privatsphäre und die Sicherheit in einem offenen Netzwerk  verbessert.  
Mikrotransaktionen: Gebührenfreie Transaktionen,  ermöglichen erstmals echte Mikrozahlungen sowohl für den Menschen als  auch für die Machine-to-Machine-Wirtschaft, um damit auch die Bezahlung für  Kleinstmengen oder Verbrauchsabhängige Bezahlungen sinnvoll umzusetzen.  
Maschinenwirtschaft:  Eine  Maschinenwirtschaft (Maschine-zu-Maschine, abk. M2M) ist eine Wirtschaft, in  der Maschinen selbstbestimmte Marktteilnehmer sind, die über eigene Bankkonten  verfügen. Die wachsende Verbreitung des IoT und die Fortschritte bei der  künstlichen Intelligenz (KI) ermöglichen es vernetzten intelligenten  Maschinen, autonom zu agieren. Das bedeutet, Maschinen werden direkt  miteinander kommunizieren, untereinander Daten auszutauschen und sich  gegenseitig für Dienstleistungen bezahlen, ohne dass eine menschliche  Interaktion erforderlich ist.  
Digitale Identität: Das Internet bildet die Grundlage für viele  unserer Interaktionen in der modernen Welt. Es hat neue  Geschäftsmöglichkeiten, bessere Kundenzufriedenheit und eine Verbesserung  unseres täglichen Lebens geschaffen. Es fehlt jedoch an wesentlichen  Eigenschaften von Vertrauen und Privatsphäre. Mit einem dezentralen digitalen  Identitätsprotokoll wird IOTA diese Eigenschaften zu den  Online-Interaktionen hinzufügen.  
IOTA Stronghold: Stronghold ist eine Sammlung von Mehrzweckbibliotheken zur sicheren Verwaltung von Passwörtern, persönlichen Daten und privaten Schlüsseln. Es ist eine sichere Software-Implementierung mit dem alleinigen Zweck, digitale Geheimnisse vor der Gefährdung durch Hacker und versehentliche Lecks zu isolieren. Sie verwendet versionierte, dateibasierte Snapshots mit doppelter Verschlüsselung, die leicht gesichert und sicher zwischen Geräten ausgetauscht werden können. Die hochgradig entwicklerfreundlichen Bibliotheken integrieren das IOTA-Protokoll und dienen als Referenzimplementierung für jeden, der nach Inspiration oder den besten Tools seiner Klasse sucht. In den Bibliotheken auf niedriger Ebene ist keine Kryptowährung enthalten, und sie können vollständig ohne die Bibliotheken auf hoher Ebene verwendet werden. Mit anderen Worten, jeder aus jeder Branche kann sie benutzen.
IOTA Streams: Streams verfügen über eine integrierte Methode zum  Senden von Nachrichten an IOTA-Nodes. Sie sind jedoch auch so flexibel, dass  Sie sie erweitern können, um Nachrichten auf andere Weise zu senden, z. B. in  HTTP-URLs. Es handelt sich um ein multifunktionales Second Layer  Datenübertragungsprotokoll, welches für verschiedene Arten der  Datenübertragung (z.B. Streaming-Daten) verwendet werden kann. Beispielsweise  ermöglicht es Sensoren und anderen Geräten, ganze Datenströme zu verschlüsseln  und im IOTA-Tangle zu verankern. Das Konsensprotokoll von IOTA fügt diesen  Nachrichtenströmen Integrität und Authentizität hinzu. Angesichts  dieser Eigenschaften erfüllt IOTA Streams ein wichtiges Bedürfnis in Branchen,  in denen Integrität, Datenschutz und Unveränderlichkeit aufeinandertreffen.
IOTA Access: Das ist Open-Source-DLT-Framework für den Aufbau richtlinienbasierter Zugangskontrollsysteme und die Ermöglichung von Pay-per-Use-Funktionalitäten am Rande des Netzwerks (Endgeräte). IOTA Access wurde entwickelt, um eine fein abgestufte Zugangskontrolle für jede Maschine, jedes Gerät und jedes Gebäude zu ermöglichen, ohne auf ein zentralisiertes System angewiesen zu sein oder eine ständige Internetverbindung zu benötigen. Bei Access geht es um die Ermöglichung von Dienstleistungen und Sicherheit in großem Maßstab. Wenn es ein Gerät gibt, das einen Dienst anbieten kann, dann kann Access in dieses Gerät integriert werden, um diesen Dienst durch eingebettete Zugangskontrollrichtlinien zu automatisieren. Auf diese Weise können Gerätebenutzer und -besitzer den Zugriff auf ihr Gerät oder ihren Datenstrom auf entfernte, unbefugte, kontaktlose und überprüfbare Weise gewähren oder anfordern.
Digital Assets: Das sind zweckgebundene Token, die  Vermögenswerte aus der realen Welt, auf Basis von IOTA (on Top), auf dem  Tangle manipulationssicher darstellen können. Sie sind nicht dafür  vorgesehen Geldwerte zu handeln – mit ihnen könnte man  beispielsweise etwas einlösen oder Eigentumsverhältnisse regeln.  Schlagwort: Tokenisierung von Vermögenswerten  
IOTA Smart-Contracts: Ein Smart-Contract (dt. Intelligenter  Vertrag) ist eine programmierte Vereinbarung, die vollständig  deterministisch ist und automatisch durchgesetzt wird, dazu  werden die vertraglichen Verpflichtungen zwischen Käufer und  Verkäufer verkapselt in der Software verankert. Dies macht es  unmöglich diese getroffene Vereinbarung zu bestreiten. 

![](https://iota-einsteiger-guide.de/media/images/sc-.gif)

Der Hauptunterschied von IOTA Smart Contracts zu  herkömmlichen Smart Contracts (Bsp. ETH) ist die Multi-Chain-Umgebung, die durch den Tangle gesichert ist und viele reguläre Blockchains parallel ausführen kann. Die IOTA Smart-Contract-Chains laufen asynchron, daher hat die Aktivität eines Smart Contracts  keinen Einfluss auf die Geschwindigkeit und den Durchsatz der anderen.

![](https://iota-einsteiger-guide.de/media/images/sc3.gif)
Volatile und unvorhersehbare Gebühren sind ein Hindernis für die Einführung von Smart Contracts. In IOTA Smart Contracts werden die Gebühren vom Eigentümer definiert, beginnen bei 0 und sind im voraus bekannt. Es gibt keinen "Bieterkrieg" wie bei regulären Blockchains, es kann mit vorhersehbaren Geschäftsmodellen und Einnahmequellen kalkuliert werden.

![](https://iota-einsteiger-guide.de/media/images/flexible-committees.gif)
IOTA Smart Contracts sind hochflexibel. Komitees können von einer Einzelperson, einem Unternehmenskonsortium oder durch einen offenen, erlaubnisfreien Marktplatz von Validator-Nodes gebildet werden.

![](https://iota-einsteiger-guide.de/media/images/virtualmachines-01.gif)
IOTA Smart Contracts sind skalierbar. Mehrere Smart-Contract-Chains, die von verschiedenen Virtualmachines betrieben werden und können gleichzeitig / parallel über mehrere Komitees laufen.

IOTA Oracles: Oracles wurden entwickelt, um eine sichere Brücke zwischen der digitalen und der physischen Welt auf eine dezentrale, erlaubnisfreie Weise zu bauen, sie  bringen im IOTA-Netzwerk Off-Chain-Daten zu dezentralen Anwendungen und Smart Contracts. 
Generell leiden Oracles unter dem uralten "Garbage in, garbage out"-Problem (dt. Müll rein, Müll raus). Wenn die Daten, die in ein Oracle eingehen, manipuliert oder zensiert werden können, ist es möglich, dass "falsche" Daten zu "falschen" Ergebnissen führen. Einige Oracle-Lösungen versuchen, dieses Problem zu lösen, indem sie sicherstellen, dass die Oracle Eingaben aus einer ausreichend großen Anzahl unabhängiger Quellen verwenden. Andere Oracles auf dem Markt haben eine Reihe von Standards vorgeschlagen, um Off-Chain-Daten auf die Chain zu bringen, aber auch sie leiden unter inhärenten Engpässen, die den Einsatz in der realen Welt behindern können. Aus Sicht von IOTA müssen optimal vertrauenswürdige Daten direkt von der Ursprungsquelle kommen, an der die Daten erzeugt werden, während sie dezentral verarbeitet und gesichert werden.


![](https://iota-einsteiger-guide.de/media/images/evd78pwxaaegh3.gif)
Das Potential der Datenmanipulation wird stark reduziert, wenn die Quelle (z.B. ein Sensor) die Daten direkt an ein manipulationssicheres Distributed Ledger übergibt, ohne den Umweg über mehrere Intermediäre. Das einfachste IOTA-Oracle ist das sogenannte First Party Oracle. IOTA-First-Party-Oracles nutzen weder externe Datenquellen noch Daten, die von einem Dritten verarbeitet und auf einem DLT zur Verfügung gestellt wurden, sondern verlassen sich auf Daten, die vom Datenherausgeber selbst an den IOTA-Tangle übermittelt wurden. Im Kontext von IoT-Netzwerken würde sich der "Datenausgeber" auf die Sensoren selbst beziehen, ohne dass diese von irgendjemandem oder irgendetwas manipuliert oder umformatiert wurden. Sobald IOTA Smart Contracts live sind, können First Party Oracles verwendet werden, um Smart Contracts direkt mit Daten zu füttern, ohne sich um einen Vermittler kümmern zu müssen, der die Daten abruft, verarbeitet, hostet oder pflegt. Im Gegensatz zu einigen anderen Lösungen auf dem Markt interagiert die IOTA Foundation niemals mit Daten, die von Datenanbietern bereitgestellt werden. Dazu kann/wird den Daten mittels Alvarium ein Confidence Score hinzugefügt.


 



Flash Channels: Das ist ein offline Zahlungskanal für Ein- und  Auszahlungen, der sofortigen Transaktionen mit hohem Durchsatz  ermöglicht. Dies ist möglich, da im IOTA-Tangle nur zwei Transaktionen  stattfinden, nämlich das Öffnen und Schließen eines Flash-Kanals. Im  Wesentlichen bieten sie den beiden Geschäftspartnern eine Möglichkeit, mit  hoher Frequenz zu handeln, ohne darauf zu warten, dass jede Transaktion im  öffentlichen IOTA-Netzwerk bestätigt wird. Dieses Feature wird zukünftig innerhalb von IOTA-Streams umgesetzt.
Digitale  Marktplätze: Daten werden seit einiger Zeit als das neue  Öl der digitalen Welt bezeichnet. Ein Grundgedanke des IoT, ist der  Austausch und Handel von solchen Datensätzen. Mit der „Maschine zu Maschine“  Kommunikation können Geräte die verschiedensten Daten autonom untereinander  handeln, aber auch für den Menschen könnte es sehr interessant werden, direkt  vom sensorischen Endverbraucher Daten oder Güter zu erhalten, ohne den Weg  über eine Drittanbieter Plattform gehen zu müssen. Das Erschaffen von  Datenmarktplätzen ist eine unvermeidliche Konsequenz der IoT Revolution, es  wird die Art und Weise, wie wir Daten verbinden, mit ihnen interagieren oder  handeln grundlegend verändern.  
Digitale Zwillinge: Ein digitaler Zwilling ist eine  digitale Repräsentanz eines materiellen oder immateriellen Objekts oder  Prozesses aus der realen Welt in der digitalen Welt. Das Industrial  IOTA Lab Aachen, Pickert, ECLASS und das Digital Twin Konsortium treiben die Technologie für IOTA voran.

## Anwendungsgebiete

Grundsätzlich wird fast jede Branche digitalisiert, mit dem IoT verknüpft und dementsprechend daraus seine Vorteile ziehen. Die IOTA Foundation beschreibt einige Anwendungsfälle in einer veröffentlichten interaktiven Broschüre. Des Weiteren beschreibe ich sehr viele PoC von aktuellen Projekten im Guide.  

 
![](https://iota-einsteiger-guide.de/media/images/2020-05-24-20_38_32-1st-meetup-introduction-to-iota.pdf-adobe-acrobat-reader-dc.png)
