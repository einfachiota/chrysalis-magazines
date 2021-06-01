<!--
---article_info
title: Was ist Chrysalis?
author: [author_1]
reviews: [reviewer_1, reviewer_2]
---
-->

# ❌ Stronghold

## Was ist Stronghold?
> Stronghold ist eine Sammlung von Mehrzweckbibliotheken zur sicheren Verwaltung von Passwörtern, persönlichen Daten und privaten Schlüsseln.

## Stronghold, kurz und knackig erklärt

Stronghold ist eine Software-Implementierung, entwicket mit der Programmiersprache Rust, mit dem alleinigen Zweck, digitale Geheimnisse, wie Seeds, Schlüssel oder Passwörter vor der Gefährdung durch Hacker und versehentliche Lecks zu schützen. 

## Wie funktioniert das genau?

Stronghold verwendet versionierte und dateibasierte Snapshots mit doppelter Verschlüsselung, die leicht gesichert und sicher zwischen Geräten ausgetauscht werden können. Entwicklet in Rust, bietet Stronghold starke Garantien für Speichersicherheit und Prozessintegrität. 

Die entwicklerfreundlichen Bibliotheken von Stronghold integrieren das IOTA-Protokoll und dienen als Referenzimplementierung für jeden, der nach Inspiration oder den besten Tools seiner Klasse sucht. Auf der unteresten Ebene der Blibliotheken ist keinerlei zu Kryptowährung oder dem Tangle enthalten. Diese Funktionen befinden sich auf hörher ebene, sodass Stronghold auch für Software Projekte eingesetzt werden kann, die nich direkt etwas mit Distributed Ledger zu tun haben, und nur ihre Wertvollen Daten lokal schützen wollen.

Intern setzt die IOTA Foundation Stronghold für die neue Wallet Software Firefly ein, um die neue Wallet zu sichern. Ausserdem ist Stronghold die Grundlage für die neue Wallet Library wallet.rs - die in Rust geschrieben wurde, aber durch bindings auch mit anderen Programmiersprachen wie NodeJS, Java oder Python genutzt werden kann.


## Wo kann man Stronghold einsetzen?

Aufgrund der hohen Flexibilität, gibt es viele spannende Anwendungen, die mit Stronghold erstellt werden können - nicht nur Kryptowährung-Wallets. Die Low-Level-Engine ist völlig nutzfallunabhängig und so flexibel, dass die Verschlüsselungsalgorithmen nach Belieben ausgetauscht, auf neue Weise zusammengestellt und mit anderen Teilen praktisch jedes Stacks erweitert werden können. Die High-Level-Bibliotheken werden so solide sein, dass sie von Haus aus eine enormee Sicherheiheit mitbringen.

Hier sind einige mögliche Anwendungsbeispiele für Stronghold:

### Wallet
![](https://iota-einsteiger-guide.de/media/images/2_qmh-muqw-ehjshl5rg_jlg.png)

Alices IOTA-Wallet wird durch Stronghold geschützt, welche sie so konfigurieren kann (wie im Bild oben zu sehen), dass sie die Aktivitäten in ihrer Wallet überwacht und gefährliche Ereignisse verhindert.


### Exchanges
Alice die Daytraderin und ihre Börsen können gemeinsam die verteilte Schlüsselerzeugung von Stronghold und BLS-Schwellwertsignaturen nutzen, um die Überprüfbarkeit von IOTA-Token-Transfers mit hohem Volumen zu verbessern.

### Tools zur Passwortverwaltung
Das sichere Säubern des Speichers nach der Verwendung eines Passworts ist eine häufige Schwachstelle in Passwort-Managern. Stronghold ist so implementiert, das auch eine solche Schachstelle nicht von einem Angreifer ausgenutzt werden kann.


### Multimedia-Center

Alice leiht einen Film aus, den sie mit ihrem Handy auf dem Smart TV abspielt. Der Film wird als verschlüsselter Stream an den Fernseher gesendet, und ein Entschlüsselungsschlüssel wird mit dem Stronghold ihres Geräts synchronisiert. Nach 48 Stunden wird der Schlüssel durch den Dienst aus ihrem Stronghold gelöscht, und das Video kann nicht mehr abgespielt werden.

### GDPR-Datenprozessoren und -Controller
Anstatt persönlich identifizierbare Informationen in einer zentralen Datenbank zu speichern, die auf einen Schlag gestohlen werden können, kann Alice sich dafür entscheiden, den Zugriff auf ihre Daten direkt aus ihrer Stronghold-basierten Anwendung heraus freizugeben und zu widerrufen.


### Software-Entwickler
Die Verwendung der Stronghold-Befehlszeilenschnittstelle oder des Systemdämons als lokales Geheim- und Abrufsystem erhöht die Betriebssicherheit des Programmierers Alice und trägt dazu bei, eine versehentliche Offenlegung zu verhindern.


## Nützliche Links:
- Das offizielle Repository: https://github.com/iotaledger/stronghold.rs
- Dokumentation: https://stronghold.docs.iota.org/
- Stronghold X-Team: https://github.com/iota-community/X-Team_IOTA_Stronghold
