# Glossar

> Ein Glossar ist eine Liste von Wörtern mit beigefügten Bedeutungserklärungen oder Übersetzungen.


## A
**Atomic Transactions (Atomare Transaktionen)**: Ersetzt das bisherige Bundle-Konstrukt; verringert den Wartungsaufwand durch Reduzierung der Netzwerkaufwände und verbessert den Spam-Schutz.

## B
**Bech32-Adressformat**: Spezifisches Segwit-Adressformat, welches auch als „bc1-Adresse“ bezeichnet wird

**Blockchain**:

## C
**Chrysalis**:

**Chrysalis Netzwerk**: 

**Coordicide**: Abschaffung des zentralisierten Konsensmechanismus


## D
**DLT (Distributed Ledger Technology; Technik verteilter Kassenbücher)**: Konsens über gemeinsam genutzte und synchronisierte digitale Daten, die über mehrere geografische Standorte oder Institutionen verteilt sind. Anders, als bei einer klassischen Datenbank, gibt es keinen zentralen Administrator.

**Dust Protection**: Bezeichnung des Mechanismus, der das IOTA Netzwerk vor einer sogenannten „Dust Attack“ schützt. Dabei versucht ein Angreifer über das massenhafte Versenden von Kleinstbeträgen die Speicherkapazität des Knotens (node) zu überlasten. 


## E

**EdDSA (Edwards-curve Digital Signature Algorithm)**: Standardkryptografie, die das alte ternäre WOTS-Schema ersetzt. Es reduziert die Transaktionsgröße und erhöht dadurch den Durchsatz. Ermöglicht auch die Wiederverwendung von bereits genutzten  Adressen. IOTA nutzt das Signaturschema Ed25519.

**Enterprise-Ready/Production-Ready**: Beschreibt die Fähigkeit des aktuellen Chrysalis-Netzwerkes (IOTA 1.5) bereits implementierte Anwendungen nach Durchführung des Coordicides auf das neue dezentrale Netzwerk migrieren zu können.


## F


## G
**GDPR (General Data Protection Regulation)**: Datenschutz-Grundverordnung, welche festlegt, wie personenbezogene Daten von EU-Bürgern gesammelt und verarbeitet werden dürfen.

**Git-Repository**: freie Software zur verteilten Versionsverwaltung von Dateien bzw. Software-Projekten

## H


## I
**Impersonification (auch Impersonation)**: Beschreibt einen Identitätswechsel, der oft in betrügerischer Absicht verwendet wird, um sich Zugang zu einem System zu verschaffen, in dem man sich als berechtigte Instanz ausgibt.

**Inter-Shard-Transaktionen**: Bezeichnet Transaktionen die zwischen unterschiedlichen Teilmengen (Shards) eines DLT durchgeführt werden müssen. -> siehe auch **Sharding**


## J


## K
**Node (Knoten)**: IOTA-Netzwerke bestehen aus miteinander verbundenen Nodes, auf denen dieselbe Node-Software ausgeführt wird. Diese Software ermöglicht Lese- / Schreibzugriff auf das Tangle,  Validierung von Transaktionen und das Speichern von Transaktionen in ihren lokalen Ledgern.


## L
**Legacy-Netzwerk**: Bezeichnung für das alte IOTA-Netzwerk (IOTA 1.0)

**Local Snapshot**: Wird von einem IOTA-Knoten (node) durchgeführt, um die Datenbank zu bereinigen bzw. die Datenbankgröße zu reduzieren (siehe auch -> Snapshot). Dadurch ist eine schnellere Synchronisierung möglich.

## M
**Man in the Middle-Angriff**: Ermöglicht dem Angreifer Einsicht oder sogar Kontrolle des Datenverkehrs zwischen zwei Kommunikationspartnern.

**Meilensteine**: Signierte Transaktion ohne Wert, die vom Coordinator in regelmässigen Abständen gesendet werden, um das aktuelle Netzwerk zu sichern.

**Mnemonik**: Merkhilfe

**MQTT (Message Queuing Telemetry Transport)**: Offenes Netzwerkprotokoll für Machine-to-Machine-Kommunikation (M2M), das die Übertragung von Telemetriedaten in Form von Nachrichten zwischen Geräten und in Netzwerken mit hoher Latenz und geringer Bandbreite ermöglicht.

## N


## O


## P


## Q


## R
**RFC (Requests for Comments; Bitte um Kommentare)**: ein nummeriertes Dokument, in dem Protokolle, Konzepte, Methoden und Programme behandelt, beschrieben und definiert werden. 

**Reattachments**: Wiederanhängen einer Transaktion an den Tangle. Dieser Prozess besteht lediglich darin, den Arbeitsnachweis (Proof of Work) und den Tippauswahlprozess durchzuführen, um die Transaktion wieder an einen anderen Teil des Tangle anzuhängen.

## S
**Sharding**: Als Sharing bezeichnet man eigentlich die Aufteilung einer großen Datenbank auf mehrere Datenbankinstanzen. Im DLT-Kontext wird damit ein Mechanismus zur besseren Skalierbarkeit der Knoten beschrieben, der einen höheren Durchsatz von Transaktionen ermöglicht. Dies erreicht man z.B. dadurch, dass ein Knoten jeweils nur eine Teilmenge von Transaktionen verarbeitet. Um Begriffsverwirrung zu vermeiden, wird im IOTA-Kontext auch der Begriff „Slicing“ verwendet.

**Snapshot**: Im IOTA-Tangle werden durch einen Snapshot alle Transaktionen mit einem leeren Saldo sowie Metadaten gelöscht. Übrig bleibt eine Liste mit Adressen und Salden, die einen neuen Anfangspunkt (Genesis) für den Tangle darstellt. 

## T
**Tangle**: Bezeichnung für den IOTA-Ledger.

**Token**: Wert einer Blockchain oder DLT, bei IOTA heißt dieser "IOTA-Token".

**Tip**: Eine noch nicht genehmigte Transaktion.

**TPS (Transactions per Second)**: Durchsatz von validierten Transaktionen in einer Sekunde.

## U
**URTS (Uniformly Random Tip Selection; gleichmäßig-zufällige Tip-Auswahl)**: Strategie zur Auswahl von bis zu acht Transaktionen, die durch eine neue Transaktion genehmigt werden sollen.

**UTXO (Unspent Transaction Output)**: beschreibt den Kern des Bitcoin-Protokolls. Die Verwendung des auf IOTA angepassten UTXO-Modells ermöglicht eine Validierung von Transaktionen in einer konstanten Zeit. Es erhöht die Aussagekraft des Ledger-Zustands, um Anwendungsfälle wie „colored coins“ und „Layer1 smart contracts“ zu realisieren. 

## V


## W
**Wallet**: Ab IOTA 1.5 wurde die bisherige Wallet „Trinity“ durch „Firefly“ ersetzt. Sie dient zur Speicherung der Public-Keys und ermöglicht dem Nutzer mittels Private-Key Zugriff auf seine Tokens. 

**Wallet-Bibliothek**:

**White-Flag**: Ansatz zur Berechnung der Guthaben im Chrysalis-Netzwerk (IOTA 1.5), der die Geschwindigkeit und Effizienz der Tip-Auswahl verbessert, bestimmte Angriffe (z. B. Konflikt-Spam) eliminiert und die Notwendigkeit von reattachments (neu anhängen Transaktionen) deutlich reduziert. 

**WOTS (Winternitz One-Time Signatur)**: Hash-basierten Signaturschema des Legacy-Netzwerkes, welches die ternäre 243-Trit-Hash-Funktion Kerl verwendet und damit quantenresistent ist.

## X


## Y


## Z
