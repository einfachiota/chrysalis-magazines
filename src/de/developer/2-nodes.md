<!--
---article_info
title: Nodes
author: [huhn]
reviews: [CrashOverride, reviewer_2]
---
-->

# Nodes

IOTA-Netzwerke bestehen aus miteinander verbundenen Nodes auf denen dieselbe IOTA-Node-Software ausgeführt wird. Diese Software ermöglicht Lese- / Schreibzugriff auf das Tangle,  Validierung von Transaktionen und das Speichern von Transaktionen in ihren lokalen Ledgern.

Wir möchten dir hier eine Übersicht über die derzeitigen IOTA Nodes geben. Aktuell läuft das Chrysalis Netzwerk mit zwei verschiedenen Node implementionen im Mainnet- der Community-getriebenen Node `HORNET` und der von der IOTA Foundation implementierten Node `Bee`.

### IRI (außer Dienst)
Die erste **I**ota **R**eference **I**mplementation (IRI) wurde in der Programmiersprache Java geschrieben. Dieser war bis zum 11.12.2019 (Erscheinungsdatum von HORNET) der einzigste Standard Node-Software. Durch HORNET (als zweiter verbesserter Core-Client) gab es neben IRI eine weitere Node-Software für den IOTA Tangle. IRI ist mittlerweile veraltet und wurde mit dem Crysalis-Update außer Dienst gestellt.   

### HORNET
HORNET ist eine leistungsstarke, Community-getriebene IOTA-Nodesoftware. Die Basis ist eine komplett neue Implementierung des Core Clients in der Programmiersprache Go. Ziel war es, einen neuen Core Client zu entwickeln, welcher mit einer deutlich gesteigerten Leistungsfähigkeit massiv zur Skalierung des Netzwerks beitragen soll. HORNET wird die Architektur und das modulare Konzept von GoShimmer verwenden und ist auch für Low-End-Geräten wie einem Raspberry Pi vorgesehen.  Der bekannte "Koordinator" läuft ebenfalls nur noch als Plugin in einer HORNET-Node.

### Bee
Produktionsreife Implementierung des Core Clients ohne Koordinator in der Programmiersprache Rust. Bee wird nicht nur eine Node-Software, es wird ein Art Plattform für Rust Entwickler, welche auch für Clients oder Anwendungen genutzt werden kann.

### GoShimmer
Prototyp von Shimmer-Voting-System in der Programmiersprache Go. Mit dieser Node-Testsoftware wird bereits heute ohne den Koordinator ein Konsens erzielt. Die GoShimmer Implementierung ist somit vollkommen dezentral.  GoShimmer implementiert die verschiedenen Module von Coordicide, wie beispielsweise Autopeering, Node-Identitäten, Mana usw. GoShimmer dient als Testumgebung für die ersten Alpha-Version und das Testnetz. Alles was hier erprobt wird, soll nach und nach mit Hornet zusammengeführt und später in Bee implementiert werden.    

### WASP
Wasp, ein Node für Smart Contracts - Das IOTA Smart Contract Protocol (ISCP) ist ein 2-Layer-Protokoll, das auf dem Kernprotokoll aufbaut und auf eigenständigen (WASP)-Goshimmer-Nodes ausgeführt wird. Wasp wird sich mit GoShimmer-Nodes verbinden, um Zugriff auf das Tangle zu erhalten.

### Chronicle
Das ist die offizielle Permanode Lösung der IOTA Foundation. Sie ermöglicht, alle Transaktionen die einen Node erreichen in einer verteilten Datenbank zu speichern, welche Sicher ist gut auch bei großen Datenmengen gut Skaliert. Chronicle wird verwendet, um den unbegrenzten Datenfluss des Tangle zu speichern und abfragbar zu machen. Mit anderen Worten, eine Permanenz ermöglicht eine unbegrenzte Speicherung der gesamten Historie des Tangle und macht diese Daten leicht zugänglich. (Da die endgültige Spezifikation von Chronicle zum Redaktionszeitpunkt noch nicht erfolgt ist, können sich Einzelheiten hier noch ändern)
