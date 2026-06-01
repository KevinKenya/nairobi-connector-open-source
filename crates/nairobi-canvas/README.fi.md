[English](README.md) | [简体中文](README.zh-CN.md) | [廣東話](README.yue.md) | [Français](README.fr.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Suomi](README.fi.md) | [Nederlands](README.nl.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Canvas: Välittömän tilan solmukaaviokääntäjä (Immediate-Mode Node Graph Visual Compiler)

Nairobi Canvas on laitteistokiihdytetty visuaalinen kääntäjä tietojen käsittelyputkien rakentamiseen. Se tarjoaa `egui`/`egui-snarl`-pohjaisen välittömän tilan solmukaaviokäyttöliittymän, joka kääntää visuaaliset työnkulut GVariant DAG (suunnattu syklitön kaavio) -muotoon Nairobi Hubin suoritettavaksi.

## Ominaisuudet

- **Visuaalinen putkenrakentaja**: Vedä ja pudota -solmukaavioliittymä tietotyönkuluille
- **Natiivi tiedostonvalitsin**: Klikkaa 📂-painiketta Ingest-solmuissa selataksesi CSV-tiedostoja
- **SQL-kyselyasetukset**: Esikonfiguroidut kyselymallit (kaikki sarakkeet, yksi sarake, Where-lause, monisarake)
- **GVariant-serialisointi**: Kääntää kaaviot GVariant-muotoon nollakopio-IPC:tä varten
- **Topologinen lajittelu**: Automaattinen syklien tunnistus ja suoritusjärjestys

## Solmutyypit

| Solmu | Syötteet | Tuotokset | Kuvaus |
|-------|----------|-----------|--------|
| **Ingest** | 0 | 1 | Lataa CSV-tietoaineistoja natiivin tiedostonvalitsimen kautta |
| **SqlQuery** | 1 | 1 | Suorittaa Polars SQL -kyselyitä syötetiedolle |
| **AxiomCrunch** | 1 | 1 | Laskee tilastoja (keskiarvo, keskihajonta, huipukkuus) |
| **LagosPlot** | 1 | 0 | Renderöi visualisointeja (sparkline, hajontakaavio, PNG, JPG) |

## Asennus

```bash
pip install nairobi-os
```

Tai rakenna lähdekoodista:
```bash
cargo build --release
# Canvas-demo on Rust-binääri - katso examples/canvas_compile_demo.rs
```

## Käyttö

### Rust (Natiivi)

Aja demosovellus:
```bash
cargo run --example canvas_compile_demo
```

### Python

Käyttäen asennettua pakettia:
```python
import nairobi_os as nb

# Avaa visuaalinen kangas DAG-kääntämistä varten
dag_bytes = nb.canvas.open()

# Suorita käännetty putki
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

Tai aja koko testiskripti:
```bash
python test_canvas.py
```

Tämä skripti tekee seuraavaa:
1. `nairobi_os.ignite()` - Käynnistää Axum Refinery- ja Nairobi Hub -taustaprosessit
2. `nb.canvas.open()` - Käynnistää visuaalisen solmukaavioeditorin
3. `nb.canvas.execute(dag_bytes)` - Suorittaa käännetyt putket ajoitusmetriikoilla

Kangas vie GVariant-koodatun DAG:n, joka voidaan:
- Suorittaa `nb.canvas.execute()`-kautta
- Tallentaa levylle myöhempää käyttöä varten
- Lähettää D-Busin tai jaetun muistin välityksellä

## Kaavioiden rakentaminen

1. **Oikeaklikkaa** kankaan ruudukkoa avataksesi solmuvalikon
2. Valitse solmutyyppi (Ingest, SQL Query, Axiom Crunch tai Lagos Plot)
3. **Yhdistä** solmut vetämällä ulostulopinneistä (sininen) sisääntulopinneihin (vihreä)
4. Klikkaa **Compile Graph** serialisoidaksesi työnkulun

## Suoritusvirta

```
Canvas-kaavio → GVariant DAG → Nairobi Hub → Axum Refinery / Lagos Vision
```

Käännetty DAG lähetetään IPC:n kautta Hubille, joka reitittää solmut:
- **Axum Refinery**: Tiedon haku ja tilastollinen käsittely
- **Lagos Vision**: Laitteistokiihdytetty visualisointien renderöinti

Arkkitehtuurin yksityiskohdat ja järjestelmän yleiskatsaus löytyvät [pääarkiston README-tiedostosta](../README.md).

## Tuki
Jos koet Nairobi OS:n hyödylliseksi, harkitse projektin tukemista:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Lisenssi
Tämä projekti on lisensoitu **Apache License 2.0** -lisenssillä.

© 2026 Kevin Chege. Kaikki oikeudet pidätetään.
