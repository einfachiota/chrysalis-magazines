# Nodes

IOTA-Netzwerke bestehen aus miteinander verbundenen Nodes, auf denen dieselbe Node-Software ausgeführt wird. Diese Software ermöglicht Lese- / Schreibzugriff auf das Tangle,  Validierung von Transaktionen und das Speichern von Transaktionen in ihren lokalen Ledgern.

Wir möchten dir hier eine Übersicht über die derzeitigen IOTA Nodes geben. Aktuell läuft das Chrysalis Netzwerk mit zwei verschiedenen Node implementionen im Mainnet- der Community-getriebenen Node `HORNET` und der von der IOTA Foundation implementierten Node `Bee`.

### IRI (außer Dienst)
Die erste **I**ota **R**eference **I**mplementation (IRI) wurde in der Programmiersprache Java geschrieben. Dies war der erste Core-Client, am 11. Dez'19 kam HORNET als zweiter Core-Client hinzu. IRI ist mittlerweile veraltet und wurde mit dem Crysalis-Update außer Dienst gestellt.   

### HORNET
HORNET ist eine leistungsstarke, Community-getriebene IOTA-Nodesoftware. Die basis ist eine komplett neue Implementierung des Core Clients in der Programmiersprache Go. Ziel war es, einen neuen Core Client zu entwickeln, welcher mit einer deutlich gesteigerten Leistung massiv zur Skalierung des Netzwerks beitragen soll. HORNET wird die Architektur und das modulare Konzept von GoShimmer verwenden und ist für Low-End-Geräten wie einem Raspberry Pi vorgesehen, zudem läuft der Koordinator nur noch als Plugin in einer HORNET-Node.

### Bee
Produktionsreife Implementierung des Core Clients ohne Koordinator in der Programmiersprache Rust. Bee wird nicht nur eine Node-Software, es wird ein Art Plattform für Rust Entwickler, welche auch für Clients oder Anwendungen genutzt werden kann.

### GoShimmer
Prototype von Shimmer in der Programmiersprache Go. Mit dieser Node-Test-Software wird bereits ohne Koordinator ein Konsens erzielt.  GoShimmer implementiert die verschiedenen Module von Coordicide, wie  beispielsweise Autopeering, Node-Identitäten, Mana usw. GoShimmer dient als Testumgebung für die ersten Alpha-Version und das Testnetz, alles was hier erprobt wird, soll nach und nach mit Hornet zusammengeführt und später in Bee implementiert werden.    

### WASP
Wasp, ein Node für Smart Contracts - Das IOTA Smart Contract Protocol (ISCP) ist ein 2-Layer-Protokoll, das auf dem Kernprotokoll aufbaut und von Goshimmer-Nodes ausgeführt wird. Die Entwicklung dieses Protokolls wird vollständig in einen  separaten Nodes namens Wasp entkoppelt. Wasp wird sich mit GoShimmer-Nodes verbinden, um Zugriff auf das Tangle zu erhalten.

### Chronicle
Das ist die offizielle Permanode Lösung der IOTA Foundation, sie ermöglicht, alle Transaktionen, die einen Node erreichen, in einer verteilten Datenbank zu speichern, die sicher ist und gut skaliert. Chronicle wird verwendet, um den unbegrenzten Datenfluss des Tangle zu speichern und abfragbar zu machen. Mit anderen Worten, eine Permanenz ermöglicht eine unbegrenzte Speicherung der gesamten Historie des Tangle und macht diese Daten leicht zugänglich. (Hier ist vieles noch nicht veröffentlicht und unklar, daher müssen die endgültigen Spezifikationen abgewartet werden)