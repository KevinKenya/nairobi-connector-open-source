[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md) | [Finnish](README.fi.md) | [Cantonese](README.yue.md)

# Nairobi OS: Korkean suorituskyvyn, Zero-Copy AI- ja tietotiede-infrastruktuuri

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## Alkuperä: Upokkaasta metalliin

Nairobi OS ei ole mukavan yrityshautomon tai pääomasijoittajien rahoittaman tutkimuslaboratorion tuote. Se on ehdottoman välttämättömyyden tulos, joka on syntynyt syvien henkilökohtaisten kriisien sarjasta ja säälimättömästä halusta toimia siellä, missä alan vakiotyökalut epäonnistuvat.

Olen Kevin Chege, Sovereign Systems Labin (Nairobi, Kenia) perustaja. Vuosina 2009–2022 elämäni kului vakavan alkoholismin parissa. Se maksoi minulle ammatillisen asemani, mahdollisuuteni ja melkein elämäni. Riippuvuuteni huipulla työskentelin analyytikkona Open Universityn strategiatoimistossa Milton Keynesissä, Isossa-Britanniassa, toimittuani sitä ennen AIESECin perustajana ja puheenjohtajana Ruandassa (2006–2010). Tänään olen neljättä vuotta jatkuvasti raittiina.

```
                     LEGIO XIII GEMINA
              "13. legioona — 13. kesäkuuta"
     Kolmetoista kadotettua vuotta. Kolmetoista vuotta takaisin vaadittavaksi.
```

Ohjelmointimatkani juuret ovat matalan tason järjestelmäarkkitehtuurissa ja äärimmäisessä optimoinnissa. Vuonna 2015 esitin vision hajautetun, erittäin teknisen kapasiteetin rakentamisesta Afrikan mantereelle [tässä tutkielmassa Kenian Piilaaksosta](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/). Kun LLM-kultaryntäys alkoi vuonna 2023, olin varhaisessa vaiheessa mukana. Rakensin ja otin käyttöön LLM-kääreitä (wrappers), mutta tunnistin nopeasti niiden rajoitukset, kuten on dokumentoitu tässä varhaisessa [2023 LLM-kääreen esittelyssä](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/).

Tajusin, että korkean tason kääreiden rakentaminen epävakaiden API-rajapintojen päälle oli arkkitehtoninen umpikuja. Todellinen sota käydään paikallisten laiterajoitusten ja resurssien allokoinnin risteyksessä.

Koko vuoden 2025 ajan elin Lenovo X13 ThinkPadilla, jossa oli erittäin rajoitettu laiteprofiili:

```
Prosessori: AMD Ryzen 5 PRO 4650U (6 ydintä, 12 säiettä)
Grafiikka: AMD Radeon RX Vega 6 iGPU
Muisti: 32 Gt RAM (korkealla järjestelmäkäytöllä)
Tallennustila: 256 Gt NVMe (99 % täynnä)
```

Tällä täsmälleen samalla koneella rakensin vuonna 2025 **Tumzin** ([Sarafakai](http://www.sarafakai.com)), ilmarakoisen (air-gapped), nollaviiveisen kliinisen päätöksentuen tekoälyn. Se suoritti suoraa, reaaliaikaista puheen litterointia ja kliinistä päättelyä samanaikaisesti integroidulla GPU:lla (iGPU), pitäen koko Unified Medical Language System (UMLS) -järjestelmän RAM-muistissa. Teemme parhaillaan yhteistyötä kenialaisen sairaalan kanssa Tumzin pilotoimiseksi vuoden mittaisessa kliinisessä tutkimuksessa – koska ihmisten terveys vaatii tiukkaa, empiiristä validointia, ei kehittäjien oletuksia.

Tumzin kehityksen aikana kohtasin nykyaikaisen tietotiedepinon massiiviset, järjestelmälliset tehottomuudet:
1. **Python-vero**: Päästä päähän suoritettava muistin kopiointi, GIL-pullonkaulat ja massiivinen ajonaikainen yleiskustannus.
2. **Selainvero**: Manifest V3 -komplikaatiot, renderöinnin viive ja korkeataajuiset viestintävirheet pitkäkestoisissa agenttikeskusteluissa.
3. **Käyttöjärjestelmän ytimen pullonkaula**: Tehoton prosessien ajoitus, CPU-säikeiden nääntyminen ja näyttöpalvelimen yleiskustannukset (Wayland vs. X11 -kontekstin vaihto).

Joten vuoden 2025 lopussa aloin rakentaa infrastruktuuripinoa, joka ohittaa nämä rajat kokonaan – Agentic Operating System, joka on suunniteltu nollakopio-tietoputkille ja laitenatiiville tekoälyn suorittamiselle. Tämä arkisto on kyseisen moottorin avoimen lähdekoodin ydin.

---

## Globaali suosio ja telemetria

6. toukokuuta 2026 julkaistu Nairobi OS on saavuttanut nopeasti suosiota järjestelmäohjelmoijien, kvantitatiivisten tutkijoiden ja reunalaskenta-arkkitehtien keskuudessa maailmanlaajuisesti. Nämä lataustilastot on saatu suorasta [ClickPy Nairobi OS Dashboardista](https://clickpy.clickhouse.com/dashboard/nairobi-os), jossa voit itse hakea ja tutkia mittareita.

### Kumulatiivinen maailmanlaajuinen jakelu (6. toukokuuta 2026 – 23. toukokuuta 2026)

| Mittari | Mittaustulos | Konteksti |
| :--- | :--- | :--- |
| **Globaali sijoitus** | **#75,293** | PyPI:n 797 894 aktiivisesta paketista |
| **Persentiili** | **9.43%** | Huipputason sijoitus järjestelmätason Python-laajennuksille |
| **Lataukset yhteensä** | **1,525** | Puhtaita, orgaanisia kehittäjälatauksia |

### Latausmäärä versioittain

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### Top 10 itsenäistä käyttöönottualuetta

| Sija | Alue | Maakoodi | Latausmäärä |
| :--- | :--- | :--- | :--- |
| 1 | Yhdysvallat | US | 661 |
| 2 | Hongkong | HK | 103 |
| 3 | Kiina | CN | 84 |
| 4 | Saksa | DE | 74 |
| 5 | Japani | JP | 65 |
| 6 | Singapore | SG | 56 |
| 7 | Yhdistynyt kuningaskunta | GB | 51 |
| 8 | Ranska | FR | 51 |
| 9 | Venäjä | RU | 42 |
| 10 | Etelä-Korea | KR | 30 |

---

## Tuki ja suvereniteetti

Jos Nairobi OS optimoi tietoputkiasi, alentaa pilvilaskujasi tai ohjaa paikallisia agenttiarkkitehtuuriasi, harkitse riippumattoman järjestelmätutkimuksemme tukemista. Jokainen lahjoitus käytetään suoraan laitetason kääntäjäoptimointeihin ja reunalaskennan testaukseen Nairobissa.

[![Tue Nairobi OS:n kehitystä](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

---

## Keskeiset ominaisuudet

* **Tietokoneen käyttö ilman pikseleitä**: Ohittaa hitaat ja kalliit näköpohjaiset agenttiputket. Vuorovaikuttaa natiivisti Linux-työpöydän kanssa AT-SPI2:n ja TOON (Token-Oriented Object Notation) -pakkausalgoritmin kautta, syöttäen raa'at hierarkkiset puut suoraan LLM-malleille.
* **Nollakopio-sisäänotto**: Laitteistokiihdytetty, ytimen ohittava tiedon lataus käyttäen `io_uring`-rajapintaa ja 1 Gt:n Huge Page -sivuja.
* **Laitteistokiihdytetty visualisointi**: Matalaviiveinen, interaktiivinen Jupyter-piirtäminen käyttäen `lagos-lite`-renderöintitaustaprosessia, joka on rakennettu `wgpu`- ja `egui`-kirjastojen päälle.
* **Vektorisoitu analyyttinen suoritus**: Äärimmäinen CPU-saturaatio hyödyntäen Polars-kyselysuoritusta ja Rayon-monisäikeisiä tietoputkia.
* **Suvereeni rajapinta**: Sujuva Python-API (`SovereignFrame`), joka kapseloi muistiin kuvatut tiedostokahvat ja IPC-viestinnän.

---

## Avoin lähdekoodi vs. yritysarkkitehtuuri

Nairobi OS on rakenteellisesti kaksihaarainen. Avoimen lähdekoodin arkisto tarjoaa perustavanlaatuiset korkean suorituskyvyn tietojenkäsittelyn ja yhden solmun visualisointiprimitiivit. Suljetun lähdekoodin kaupallinen ekosysteemi sisältää edistyneet moniagentti-, korkean käytettävyyden ja toimialakohtaiset toteutukset.

```
                                  +---------------------------------------+
                                  |         Nairobi Python API            |
                                  +---------------------------------------+
                                                      |
                                     [ GVariant D-Busin yli / jaettu muisti ]
                                                      |
                                                      v
                                  +---------------------------------------+
                                  |           Nairobi Hub                 |
                                  +---------------------------------------+
                                                      |
                    +---------------------------------+---------------------------------+
                    |                                                                   |
                    v                                                                   v
     +------------------------------+                                    +------------------------------+
     |     Axum Refinery (Data)     | <===[ Zero-Copy IPC / iceoryx2 ]===> |     Lagos Vision (Visual)    |
     +------------------------------+                                    +------------------------------+
```

### Avoimen lähdekoodin Crate-työalue (`crates/`)

1. **`nairobi-axum-refinery`**: Korkean suorituskyvyn Rust-taustaprosessi, joka hallitsee raakadatan sisäänottoa, Rayon-parallisoituja tilastoja ja Polars-vektorisoitua kyselysuoritusta.
2. **`nairobi-hub`**: Keskitetty IPC-orkestraattori. Hallitsee ja reitittää tiedostokahvoja ja signaaleja asiakkaiden ja refinery-taustaprosessin välillä.
3. **`lagos-lite`**: Visuaalinen aivokuori. Päätön, tapahtumaohjattu renderöintimoottori, joka kuvaa muistiin kuvatut tiedostot suoraan GPU-putkeen.
4. **`nairobi-protocol`**: Jaettu protokollakerros. Määrittelee standardit GVariant-serialisointiskeemat, virhetyypit ja jaetun muistin asettelut.
5. **`nairobi-python`**: Python-laajennusmoduuli, joka on käännetty `PyO3`-kirjastolla ja pakattu `Maturin`-työkalulla.

### Yksityinen yritysekosysteemi (`modules/`)

Yritystason komponenttimme pidetään yksityisessä arkistossa (`Sovereign-Systems-Lab`) ja ne on lisensoitu teolliseen, taloudelliseen ja valtiolliseen infrastruktuuriin.

1. **`sovereign-ui`**: Yritystason AT-SPI2-moottori. Toteuttaa Aegis-protokollan suojauksen, laitesidonnan ja tuotantotason työpöydän hallinnan.
2. **`nairobi-connector`**: Kehittynyt Model Context Protocol (MCP) -palvelin, joka hallitsee raakoja, matalaviiveisiä D-Bus-signaaleja yritystason LLM-malleille.
3. **`tactical-rtos-node`**: Erittäin matalaviiveinen reaaliaikakäyttöjärjestelmän ajoitusohjelma turvallisuuskriittiseen teolliseen reunalaskenta-automaatioon.
4. **`industrial-guardian-rust` / `industrial-guardian-python`**: Autonominen sivuston luotettavuuden hallinnan (SRE) kerros, jossa on ennakoiva OOM-muistinhallinta, muistivuotojen ja järjestelmän kaatumisen esto.
5. **`fintech-bridge-rust`**: Reaaliaikainen korkeataajuinen transaktioparseri ja perintöjärjestelmien pääkone-silta (EBCDIC/SBA-terminaaliparsinta).
6. **`aviation-audio-rust`**: Alle millisekunnin, lukoton äänivirran käsittely, akustinen telemetria-analyysi ja raaka-aalto-DSP.
7. **`drawbridge_api`**: Suojattu, todennettu, monen käyttäjän gRPC-nostosilta, joka eristää paikallisen ytimen luottamattomista pilviagenttipuheluista.

### Suorituskykyvertailumatriisi

| Kyvykkyys / Ominaisuus | Avoin lähdekoodi Core (`crates/`) | Yrityspaketti (`modules/`) |
| :--- | :---: | :---: |
| **Sisäänotto-moottori** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1 Gt:n Huge Pages |
| **Tilastollinen analyysi** | Perustason kuvailevat tilastot | Vektorisoitu, monivaiheinen vinous/huipukkuus, korrelaatio |
| **Kyselymoottori** | Prosessin sisäinen Polars SQL | Hajautettu Apache Arrow / DataFusion -klusteri |
| **IPC-mekanismi** | POSIX-jaettu muisti / D-Bus | Zero-Copy `iceoryx2` -jaetun muistin areenat |
| **Visualisointi** | Paikallinen Jupyter `anywidget` | WebRTC GStreamer / läpinäkyvät Wayland Layer-Shell -peitteet |
| **Turvallisuus ja vaatimustenmukaisuus** | Standardit POSIX-rajat | Aegis-protokolla, SHA-256-ketjutettu oikeuslääketieteellinen pääkirja |
| **Todennus** | Ei mitään (paikallinen luotettu käyttäjä) | Laitesidonta (TPM 2.0 / CPU ID), yksityinen PKI |
| **Alustakohde** | Yhden solmun Linux | Hajautettu pilvi / reunasolmu / korkeataajuinen kaupankäynti |

---

## Asennus ja käyttöönotto

### Vaatimukset
- **Käyttöjärjestelmä**: Linux (Ubuntu 22.04+ suositeltu) tai Windows Subsystem for Linux (WSL2).
- **GPU**: Vulkan-, Metal- tai OpenGL-yhteensopiva ohjain.
- **Python**: 3.10 tai uudempi.
- **Rust**: Stabiili työkaluketju (jos rakennat lähteestä).

### Pika-asennus (PyPI)
```bash
pip install nairobi-os
```

### Rakenna lähteestä
Koko työalueen kääntämiseksi, mukaan lukien natiivit taustaprosessit ja Python-laajennus:

1. **Kloonaa arkisto**:
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **Määritä virtuaaliympäristö**:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **Suorita työalueen rakennus**:
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   Tämä kääntää natiivit taustaprosessit, kopioi ne pakettihakemistoon ja rakentaa wheel-paketin hakemistoon `crates/nairobi-python/target/wheels/`.

---

## Käyttöopas

### 1. Data-analytiikka (In-Memory-tietoputki)

Nairobi OS tarjoaa `SovereignFrame`-rajapinnan. Se käsittelee raakaa muistikuvausta taustalla, mahdollistaen nopean tiedon käsittelyn.

```python
import nairobi_os as nb

# Käynnistä taustalla oleva refinery-taustaprosessi
nb.connect()

# Lue tietoaineisto käyttäen nollakopio-muistiputkea
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Suorita vektorisoituja laskelmia Rust refineryn kautta
profile = frame.crunch("value")
print(f"Keskiarvo: {profile['mean']:.4f}")
print(f"Keskihajonta: {profile['std_dev']:.4f}")

# Suorita mielivaltaisia SQL-kyselyitä suoraan muistiin kuvatulle kehykselle
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Käynnistä Lagos-kiihdytetty interaktiivinen piirto-widget
subset.plot(column="value")
```

### 2. Tietokoneen käyttö ilman pikseleitä (MCP)

Käyttääksesi AT-SPI2-semanttista rajapintaa, tekoälyagenttisi tulisi olla vuorovaikutuksessa paljastettujen MCP-palvelintyökalujen kanssa sen sijaan, että se lukisi kuvakaappauksia:

```
                    TIETOKONEEN KÄYTTÖSARJA
                     
  [ LLM-agentti ]                               [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Text Editor") ====>| (Paikantaa kohteen)
        |<=== Palauttaa ikkunan tunnuksen ja rajat ===|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (Generoi TOONin)
        |<=== Palauttaa pakatun Markdown-puun ========|
        |     "[ID: 12] Painike: 'Tallenna'"          |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (Suorittaa toiminnon)
        |<=== Palauttaa onnistumistilan ==============|
```

---

## Järjestelmän optimointi (Osallistujan opas)

Saavuttaaksesi benchmark-testeissämme näkyvät suorituskykyprofiilit, isäntäytimesi on oltava määritetty järjestelmätason muistikuvaukseen.

### 1 Gt:n Huge Page -sivut
Nairobi OS käyttää 1 Gt:n Huge Page -sivuja ohittaakseen CPU:n Translation Lookaside Buffer (TLB) -käännöksen yleiskustannukset massiivisilla tietoaineistoilla.

Huge Pagen allokoimiseksi Linux-isännälläsi:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Huomautus: Jos järjestelmä ei pysty allokoimaan 1 Gt:n sivua fragmentaation vuoksi, moottori siirtyy automaattisesti käyttämään Transparent Huge Pages (THP) -sivuja.*

### D-Bus Broker -konfiguraatio
Korkeataajuisissa ympäristöissä varmista, että `dbus-broker` on asennettu perinteisen `dbus-daemonin` sijaan nopean signaalin etenemisen varmistamiseveksi ohjaustasolla.

---

## Lisenssi

Tämä projekti on lisensoitu **Apache License 2.0** -lisenssillä.  
*(Huomautus: Osa TOON-formaatista ja silta-toteutuksesta on hyvitetty TOON-tekijöille.)*

---
© 2026 Kevin Chege. Kaikki oikeudet pidätetään.  
*Sovereign Systems Lab, Nairobi, Kenia.*
