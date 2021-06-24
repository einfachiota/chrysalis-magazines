<!--
---article_info
title: Nodes
author: [huhn]
reviews: [CrashOverride, Doenermaker, DanieKrie]
---
-->

# Nodes

IOTA-Netzwerke bestehen aus miteinander verbundenen Nodes auf denen eine IOTA-Node Software ausgeführt wird. Diese Software ermöglicht Lese- und Schreibzugriff auf den Tangle,  Validierung von Transaktionen und die Speicherung von Transaktionen in ihren lokalen Ledgern.

Wir möchten dir hier eine Übersicht über die derzeitigen IOTA-Nodes geben. Aktuell läuft das Chrysalis Netzwerk mit zwei verschiedenen Node-Implementierungen im Mainnet - der Community-getriebenen Node `HORNET` und der von der IOTA Foundation implementierten Node `Bee`.

### IRI (außer Dienst)
Die erste **I**ota **R**eference **I**mplementation (IRI) wurde in der Programmiersprache Java geschrieben. Diese war bis zum 11.12.2019 die einzige Standard Node-Software. Mit der Erscheinung von HORNET, einem verbesserten Core-Client, gab es neben IRI eine weitere Node-Software für den IOTA Tangle. IRI ist mittlerweile veraltet und wurde mit dem Crysalis-Update außer Dienst gestellt.   

### HORNET
Eine leistungsstarke, von der Community vorangetriebene IOTA-Fullode-Software, die vollständig in der Programmiersprache Go geschrieben wurde. Ziel war es, einen neuen Core Client zu entwickeln, welcher mit einer deutlich gesteigerten Leistungsfähigkeit massiv zur Skalierung des Netzwerks beiträgt. HORNET verwendet die Architektur und das modulare Konzept von GoShimmer und ist auch für Low-End-Geräte wie dem Raspberry Pi geeignet. Der bekannte "Coordinator" läuft ebenfalls nur noch als Plugin auf einer HORNET-Node.

### Bee
Bee ist eine produktionsreife Implementierung des Core-Clients ohne "Coordinator" in der Programmiersprache Rust. Teile aus allen vorherigen und zukünftigen Entwicklungen werden standardisiert und zu dieser einheitlichen Plattformlösung zusammengeführt. Bee dient als Referenzimplementierung für Entwickler, welche für die Implmentierung von Clients oder Anwendungen genutzt werden kann. Durch die modulare, ereignisgesteuerte Softwarearchitektur kann Bee so angepasst werden, dass alle möglichen Arten von Node-Software ausführbar sind.

### GoShimmer
Ein in der Programmiersprache Go geschriebener Prototyp des Shimmer-Voting-Systems. Er dient als Testumgebung für erste Alpha-Versionen und das Testnetz selbst. Mit dieser Node-Software wird bereits heute ohne den "Coordinator" ein Konsens erzielt, die GoShimmer Implementierung ist somit vollkommen dezentral. In GoShimmer werden die verschiedenen Module von IOTA 2.0, wie beispielsweise Autopeering, Node-Identitäten, Mana usw. implementiert und getestet. Alles was hier erprobt wird, soll nach und nach mit Hornet zusammengeführt und später in Bee implementiert werden.

### WASP
Eine Node-Software für Smart Contracts. Das IOTA-Smart-Contract-Protocol (ISCP) ist ein 2nd-Layer-Protokoll, welches auf dem Kernprotokoll aufbaut und auf eigenständigen (WASP)-Goshimmer-Nodes ausgeführt wird. Wasp wird sich mit GoShimmer-Nodes verbinden, um Zugriff auf den Tangle zu erhalten.

### Chronicle
Die offizielle Permanode Lösung der IOTA Foundation. Sie ermöglicht es, alle Transaktionen, die einen Node erreichen, in einer verteilten Datenbank zu speichern, welche sicher ist und auch bei großen Datenmengen gut skaliert. Chronicle wird verwendet, um den unbegrenzten Datenfluss des Tangles zu speichern und abfragbar zu machen. Mit anderen Worten: eine Permanode ermöglicht eine unbegrenzte Speicherung der gesamten Historie des Tangles und macht diese Daten leicht zugänglich. (Da die endgültige Spezifikation von Chronicle zum Redaktionszeitpunkt noch nicht erfolgt ist, können sich Einzelheiten hier noch ändern.)
