<!--
---article_info
title: Nodes
author: [huhn]
reviews: [CrashOverride, Doenermaker]
---
-->

# Nodes

IOTA-Netzwerke bestehen aus miteinander verbundenen Nodes auf denen dieselbe IOTA-Node-Software ausgeführt wird. Diese Software ermöglicht Lese- / Schreibzugriff auf den Tangle,  Validierung von Transaktionen und die Speicherung von Transaktionen in ihren lokalen Ledgern.

Wir möchten dir hier eine Übersicht über die derzeitigen IOTA Nodes geben. Aktuell läuft das Chrysalis Netzwerk mit zwei verschiedenen Node-Implementierungen im Mainnet - der Community-getriebenen Node `HORNET` und der von der IOTA Foundation implementierten Node `Bee`.

### IRI (außer Dienst)
Die erste **I**ota **R**eference **I**mplementation (IRI) wurde in der Programmiersprache Java geschrieben. Diese war bis zum 11.12.2019 (Erscheinungsdatum von HORNET) die einzige Standard Node-Software. Durch HORNET (als zweiter verbesserter Core-Client) gab es neben IRI eine weitere Node-Software für den IOTA Tangle. IRI ist mittlerweile veraltet und wurde mit dem Crysalis-Update außer Dienst gestellt.   

### HORNET
Eine leistungsstarke, Community-getriebene IOTA-Node-Software. Die Basis ist eine komplett neue Implementierung des Core Clients in der Programmiersprache Go. Ziel war es, einen neuen Core Client zu entwickeln, welcher mit einer deutlich gesteigerten Leistungsfähigkeit massiv zur Skalierung des Netzwerks beitragen soll. HORNET wird die Architektur und das modulare Konzept von GoShimmer verwenden und ist auch für Low-End-Geräte wie einem Raspberry Pi vorgesehen.  Der bekannte "Koordinator" läuft ebenfalls nur noch als Plugin in einer HORNET-Node.

### Bee
Produktionsreife Implementierung des Core Clients ohne Koordinator in der Programmiersprache Rust. Bee wird nicht nur eine Node-Software, es wird ein Art Plattform für Rust Entwickler, welche auch für Clients oder Anwendungen genutzt werden kann.

### GoShimmer
Prototyp des Shimmer-Voting-Systems in der Programmiersprache Go. Mit dieser Node-Testsoftware wird bereits heute ohne den Koordinator ein Konsens erzielt, die GoShimmer Implementierung ist somit vollkommen dezentral.  GoShimmer implementiert die verschiedenen Module von IOTA 2.0, wie beispielsweise Autopeering, Node-Identitäten, Mana usw. Dabei dient GoShimmer als Testumgebung für die ersten Alpha-Version und das Testnetz. Alles was hier erprobt wird, soll nach und nach mit Hornet zusammengeführt und später in Bee implementiert werden.    

### WASP
Eine Node-Software für Smart Contracts. Das IOTA Smart Contract Protocol (ISCP) ist ein 2nd-Layer-Protokoll, welches auf dem Kernprotokoll aufbaut und auf eigenständigen (WASP)-Goshimmer-Nodes ausgeführt wird. Wasp wird sich mit GoShimmer-Nodes verbinden, um Zugriff auf den Tangle zu erhalten.

### Chronicle
Die offizielle Permanode Lösung der IOTA Foundation. Sie ermöglicht, alle Transaktionen, die einen Node erreichen, in einer verteilten Datenbank zu speichern, welche trotzdem sicher ist und auch bei großen Datenmengen gut skaliert. Chronicle wird verwendet, um den unbegrenzten Datenfluss des Tangles zu speichern und abfragbar zu machen. Mit anderen Worten, eine Permanode ermöglicht eine unbegrenzte Speicherung der gesamten Historie des Tangles und macht diese Daten leicht zugänglich. (Da die endgültige Spezifikation von Chronicle zum Redaktionszeitpunkt noch nicht erfolgt ist, können sich Einzelheiten hier noch ändern)
