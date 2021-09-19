### Bee

#### Aktuelles Ziel
- **Lokale Snapshots**
Eine Implementierung lokaler Snapshots, mit der der Benutzer die Größe der Knotendatenbank steuern kann.

#### Nächste Ziele
- **Lokale Snapshots**
Eine Implementierung lokaler Snapshots, mit der der Benutzer die Größe der Knotendatenbank steuern kann.

- **MQTT-Unterstützung**
Unterstützung für das MQTT-Pubsub-Messaging-Protokoll hinzufügen.

- **Erneuerung des Plugin-Systems**
Ein neueres und flexibleres Plugin-System wird entwickelt, damit unter anderem Drittparteien mit eigenen Plugins die Funktionalitäten von Bee einfach und effizient erweitern können.

- **Erneuerung der Node Architektur**
Basierend auf dem ASB (Actor System Backstage) Prinzip in Zusammenarbeit mit dem Chronicle Team.

- **Initial-Implementierung der Coordicide Netzwerkschicht**
Implementierung der aktuellen Version von GoShimmer ohne die libp2p Voraussetzungen um innerhalb des IOTA 2.0 Devnet betrieben zu werden.

- **Initial-Implementierung des "Coordicide Data-Flow Modells**
Implementierung des Data-Flow Models wie in den Forschungsspezifikationen dokumentiert: https://github.com/iotaledger/IOTA-2.0-Research-Specifications/blob/main/2.4%20Data%20Flow.md

#### Zuletzt erreichte Ziele
- **Beta-Version**
Eine Beta-Version von Bee.

- **Alpha Release**
Alpha-Veröffentlichung von Bee.

- **Dashboard-Integration**
Unterstützung für die neue Benutzeroberfläche des Node-Dashboards hinzufügen.

- **API**
Dies ist die Schnittstelle zwischen Clients und Datenbanken - eine API, mit der Informationen und Daten aus der Ferne abgefragt werden können. Modular aufgebaut, ermöglicht es die einfache Implementierung neuer Architekturen (REST, gRPC, …).

- **Messages**
Implementiert das neue atomare Nachrichtenlayout von Chrysalis Part 2.

- **UTXO-Ledger**
Ermöglicht eine schnellere und genauere Konfliktbehandlung und behandelt mögliche Szenarien für das erneute Anhängen ungültiger Transaktionen. Es wird dem Protokoll ermöglichen, in Zukunft farbige Münzen (sog. Colored Coins) zu unterstützen.

- **Proof of Work**
Ermöglicht dem Node das Durchführen von Proof of Work.

- **Tangle**
Implementiert die API für die typsichere Interaktion mit dem Tangle und dessen Bearbeitung.

- **Speicherebene**
Die Speicherschicht des Bee-Nodes. Dies ist die Schnittstelle zwischen dem Tangle und den Datenbanken. Modular aufgebaut, ermöglicht es die einfache Implementierung neuer Backends (SQL, KV,…).

- **Netzwerkschicht**
Bietet eine einfache und bequeme Möglichkeit zum Austausch von Nachrichten zwischen benachbarten Nodes.


- **Gossip Protokoll**
Ermöglicht es dem Node, Transaktionen über das Netzwerk zu verbreiten.

- **White Flag**
Ein einfacherer, konfliktunabhängiger Ansatz, der eine schnellere und effizientere Auswahl von Spitzen ermöglicht, bestimmte Angriffe eliminiert und den Bedarf an erneuten Anhängen erheblich reduziert.

- **Binär codiertes Ternäres (Vereinheitlichung)**
Schnittstelle zur Vereinheitlichung von binär-ternären Codierungen.

- **Signaturschema**
Implementiert dieselben Signaturschemata, die die Authentizität im aktuellen IOTA-Netzwerk gewährleisten, wie IRI v1.8.1.

- **Kryptografische Grundelemente**
Implementiert die kryptografischen Hash-Funktionen CurlP und Kerl. CurlP81 (ein neuer Typ für CurlP mit 81 Runden) und Kerl sind die beiden von IRI v1.8.1 verwendeten kryptografischen Hash-Funktionen.

#### Abgebrochene Ziele
Findet und aktualisiert Peers im Netzwerk automatisch.
