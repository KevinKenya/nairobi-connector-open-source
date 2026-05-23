[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Connector

## Überblick
**Nairobi Connector** ist die AT-SPI2-Semantikbrücke und der Model Context Protocol (MCP) Server für Nairobi OS. Es ermöglicht **"Computer Use without pixels"**, indem es den Accessibility-Tree des Linux-Desktops für LLMs und KI-Agenten in einem hyperdichten, token-optimierten Format namens TOON (Token-Oriented Object Notation) zugänglich macht. Durch die rein semantische Interaktion auf UI-Knoten-Ebene anstelle von Pixeln erreicht es eine nahezu verzögerungsfreie Ausführung von Aktionen und reduziert den Token-Overhead drastisch.

## Hauptfunktionen
- **Computernutzung ohne Pixel**: Interagiert direkt über AT-SPI2 mit dem Linux-Desktop und umgeht die Notwendigkeit von Screenshots, OCR oder visueller Verarbeitung.
- **TOON-Komprimierungsalgorithmus**: Übersetzt D-Bus-Accessibility-Trees in eine stark komprimierte Markdown-Darstellung. Es filtert nicht-interaktive "Rauschen"-Knoten heraus und weist aktionsfähigen Elementen sequenzielle IDs zu.
- **MCP-Server-Integration**: Implementiert einen robusten, auf `rmcp` basierenden Server, der semantische Tools nativ bereitstellt.
- **Sicherer Sitzungslebenszyklus**: Verfügt über einen Heartbeat-Watcher, um eine Lähmung des Betriebssystems zu verhindern.

## Architektur
Der Connector fungiert als bidirektionale Brücke zwischen LLMs (über MCP) und dem Linux-Desktop (über AT-SPI2/D-Bus).

### Bereitgestellte MCP-Tools
- `nairobi_find_window`: Sucht und zielt auf ein Fenster anhand eines Titel-Teilstrings ab.
- `nairobi_get_ui_map`: Gibt den aktuellen UI-Accessibility-Tree als TOON-komprimierte Karte zurück. Erzeugt eine dichte Auflistung interaktiver Elemente mit sequenziellen `[ID: N]`-Tags.
- `nairobi_interact`: Führt semantische Aktionen (`click`, `activate`, `focus`) auf einem UI-Element aus.
- `nairobi_type_text`: Fügt Text atomar in ein bearbeitbares Feld ein.

## Nutzung
Agenten, die den Nairobi Connector verwenden, sollten dieser Schleife folgen:
1. Zielen Sie auf ein Fenster mit `nairobi_find_window`.
2. Beobachten Sie den aktuellen Status über `nairobi_get_ui_map`.
3. Lesen Sie die `[ID: N]` des gewünschten interaktiven Elements ab.
4. Führen Sie eine Aktion über `nairobi_interact` oder `nairobi_type_text` aus.
5. Wiederholen Sie ab Schritt 2, um vor der nächsten Interaktion aktuelle IDs zu erhalten.

## Unterstützung
Wenn Sie Nairobi OS nützlich finden, ziehen Sie bitte in Erwägung, das Projekt zu unterstützen:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Lizenz
Dieses Projekt ist unter der **Apache License 2.0** lizenziert.
*(Hinweis: Teile des TOON-Formats und der Implementierung gehen auf die TOON-Autoren zurück).*
