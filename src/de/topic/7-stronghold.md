<!--
---article_info
title: Stronghold
author: [author_1]
reviews: [ruegenlord, DanieKrie]
---
-->

# Stronghold

## Was ist Stronghold?
> Stronghold ist eine Sammlung von Mehrzweckbibliotheken zur sicheren Verwaltung von Passwörtern, persönlichen Daten und privaten Schlüsseln.

## Stronghold: Kurz und knackig erklärt

Bei Stronghold handelt es sich um eine mittels der Programmiersprache Rust entwickelte Software-Implementierung. Ihr alleiniger Zweck ist es, digitale Geheimnisse, wie Seeds, Schlüssel oder Passwörter vor der Gefahr durch Hacker und versehentliche Lecks zu schützen. 

## Wie genau funktioniert das?

Stronghold verwendet versionierte und dateibasierte Snapshots mit doppelter Verschlüsselung, die leicht gesichert und sicher zwischen Geräten ausgetauscht werden können. Entwickelt in Rust, bietet Stronghold starke Garantien für Speichersicherheit und Prozessintegrität. 

Die entwicklerfreundlichen Bibliotheken von Stronghold integrieren das IOTA-Protokoll und dienen als Referenzimplementierung für jeden, der nach Inspiration oder den besten Tools seiner Klasse sucht. Die Low-Level-Bibliotheken haben keinerlei Bezug zur Kryptowährung oder dem Tangle. Diese Funktionen befinden sich auf höherer Ebene, so dass Stronghold auch für Software-Projekte eingesetzt werden kann, die nicht direkt etwas mit Distributed Ledger zu tun haben, sondern bspw. lediglich ihre wertvollen Daten lokal schützen wollen.

Intern setzt die IOTA Foundation Stronghold zur Sicherung ihrer neuen Wallet Software "Firefly" ein. Außerdem ist Stronghold die Grundlage für die neue Wallet Library wallet.rs. Diese wurde ebenfalls in Rust geschrieben, kann jedoch durch bindings auch mit anderen Programmiersprachen wie NodeJS, Java oder Python genutzt werden.


## Wo kann man Stronghold einsetzen?

Aufgrund der hohen Flexibilität gibt es neben der Sicherung von Kryptowährungs-Wallets noch viele weitere spannende Anwendungen, die mit Stronghold erstellt werden können. Die Low-Level-Engine ist völlig Anwendungsfall unabhängig und so flexibel, dass die Verschlüsselungsalgorithmen nach Belieben ausgetauscht, auf neue Weise zusammengestellt und mit anderen Teilen praktisch jedes Stacks erweitert werden können. Die High-Level-Bibliotheken werden so solide sein, dass sie von Haus aus eine enorme Sicherheit mitbringen.

Hier sind einige mögliche Anwendungsbeispiele für Stronghold:

### Wallet
![](https://iota-einsteiger-guide.de/media/images/2_qmh-muqw-ehjshl5rg_jlg.png)

Alice' IOTA-Wallet wird durch Stronghold geschützt, welche sie so konfigurieren kann (siehe obige Grafik), dass sie die Aktivitäten in ihrer Wallet überwacht und gefährliche Ereignisse verhindert.


### Exchanges
Die Daytraderin Alice und ihre Börsen können gemeinsam die verteilte Schlüsselerzeugung von Stronghold und BLS-Schwellwertsignaturen nutzen, um die Überprüfbarkeit von IOTA-Token-Transfers mit hohem Volumen zu verbessern.

### Tools zur Passwortverwaltung
Das sichere Säubern des Speichers nach der Verwendung eines Passwortes ist eine häufige Schwachstelle in Passwort-Managern. Stronghold ist so implementiert, dass auch eine solche Schwachstelle nicht von einem Angreifer ausgenutzt werden kann.


### Multimedia-Center
Alice leiht einen Film aus, den sie mit ihrem Handy auf dem Smart TV abspielt. Der Film wird als verschlüsselter Stream an das Smart TV gesendet und ein Entschlüsselungsschlüssel wird mit dem Stronghold ihres Gerätes synchronisiert. Nach 48 Stunden wird der Schlüssel durch den Dienst aus ihrem Stronghold gelöscht und der Film kann nicht mehr abgespielt werden.

### GDPR-Datenprozessoren und -Controller
Anstatt persönlich identifizierbare Informationen in einer zentralen Datenbank zu speichern, die auf einen Schlag gestohlen werden können, kann Alice sich dafür entscheiden, den Zugriff auf ihre Daten direkt aus ihrer Stronghold-basierten Anwendung heraus freizugeben und zu widerrufen.


### Software-Entwickler
Die Verwendung der Stronghold-Befehlszeilenschnittstelle oder des Systemdämons als lokales Geheim- und Abrufsystem erhöht die Betriebssicherheit der Programmiererin Alice und trägt dazu bei, eine versehentliche Offenlegung zu verhindern.


## Nützliche Links:
- Das offizielle Repository: https://github.com/iotaledger/stronghold.rs
- Dokumentation: https://stronghold.docs.iota.org/
- Stronghold X-Team: https://github.com/iota-community/X-Team_IOTA_Stronghold
