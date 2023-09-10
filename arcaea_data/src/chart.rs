
use std::collections::HashMap;
use crate::ChartData;
use anyhow::Result as anyResult;
pub fn get_charts() -> anyResult<HashMap<String,ChartData>> {
let mut map = HashMap::new();

            map.insert(
            "sayonarahatsukoi-0".to_string(),
            ChartData { 
                name_en: "Sayonara Hatsukoi".to_string(),
                name_jp: None,
                rating: 1.5
            }
            );
            map.insert(
            "sayonarahatsukoi-1".to_string(),
            ChartData { 
                name_en: "Sayonara Hatsukoi".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "sayonarahatsukoi-2".to_string(),
            ChartData { 
                name_en: "Sayonara Hatsukoi".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "lostcivilization-0".to_string(),
            ChartData { 
                name_en: "Lost Civilization".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "lostcivilization-1".to_string(),
            ChartData { 
                name_en: "Lost Civilization".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "lostcivilization-2".to_string(),
            ChartData { 
                name_en: "Lost Civilization".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "lostcivilization-3".to_string(),
            ChartData { 
                name_en: "Lost Civilization".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "goodtek-0".to_string(),
            ChartData { 
                name_en: "GOODTEK (Arcaea Edit)".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "goodtek-1".to_string(),
            ChartData { 
                name_en: "GOODTEK (Arcaea Edit)".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "goodtek-2".to_string(),
            ChartData { 
                name_en: "GOODTEK (Arcaea Edit)".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "goodtek-3".to_string(),
            ChartData { 
                name_en: "GOODTEK (Arcaea Edit)".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "viyella-0".to_string(),
            ChartData { 
                name_en: "cry of viyella".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "viyella-1".to_string(),
            ChartData { 
                name_en: "cry of viyella".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "viyella-2".to_string(),
            ChartData { 
                name_en: "cry of viyella".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "rise-0".to_string(),
            ChartData { 
                name_en: "Rise".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "rise-1".to_string(),
            ChartData { 
                name_en: "Rise".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "rise-2".to_string(),
            ChartData { 
                name_en: "Rise".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "lucifer-0".to_string(),
            ChartData { 
                name_en: "Lucifer".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "lucifer-1".to_string(),
            ChartData { 
                name_en: "Lucifer".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "lucifer-2".to_string(),
            ChartData { 
                name_en: "Lucifer".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "fairytale-0".to_string(),
            ChartData { 
                name_en: "Fairytale".to_string(),
                name_jp: None,
                rating: 1.0
            }
            );
            map.insert(
            "fairytale-1".to_string(),
            ChartData { 
                name_en: "Fairytale".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "fairytale-2".to_string(),
            ChartData { 
                name_en: "Fairytale".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "fairytale-3".to_string(),
            ChartData { 
                name_en: "Fairytale".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "hearditsaid-0".to_string(),
            ChartData { 
                name_en: "I've heard it said".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "hearditsaid-1".to_string(),
            ChartData { 
                name_en: "I've heard it said".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "hearditsaid-2".to_string(),
            ChartData { 
                name_en: "I've heard it said".to_string(),
                name_jp: None,
                rating: 8.1
            }
            );
            map.insert(
            "babaroque-0".to_string(),
            ChartData { 
                name_en: "Babaroque".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "babaroque-1".to_string(),
            ChartData { 
                name_en: "Babaroque".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "babaroque-2".to_string(),
            ChartData { 
                name_en: "Babaroque".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "memoryfactory-0".to_string(),
            ChartData { 
                name_en: "memoryfactory.lzh".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "memoryfactory-1".to_string(),
            ChartData { 
                name_en: "memoryfactory.lzh".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "memoryfactory-2".to_string(),
            ChartData { 
                name_en: "memoryfactory.lzh".to_string(),
                name_jp: None,
                rating: 8.9
            }
            );
            map.insert(
            "snowwhite-0".to_string(),
            ChartData { 
                name_en: "Snow White".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "snowwhite-1".to_string(),
            ChartData { 
                name_en: "Snow White".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "snowwhite-2".to_string(),
            ChartData { 
                name_en: "Snow White".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "relentless-0".to_string(),
            ChartData { 
                name_en: "Relentless".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "relentless-1".to_string(),
            ChartData { 
                name_en: "Relentless".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "relentless-2".to_string(),
            ChartData { 
                name_en: "Relentless".to_string(),
                name_jp: None,
                rating: 8.0
            }
            );
            map.insert(
            "shadesoflight-0".to_string(),
            ChartData { 
                name_en: "Shades of Light in a Transcendent Realm".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "shadesoflight-1".to_string(),
            ChartData { 
                name_en: "Shades of Light in a Transcendent Realm".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "shadesoflight-2".to_string(),
            ChartData { 
                name_en: "Shades of Light in a Transcendent Realm".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "vexaria-0".to_string(),
            ChartData { 
                name_en: "Vexaria".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "vexaria-1".to_string(),
            ChartData { 
                name_en: "Vexaria".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "vexaria-2".to_string(),
            ChartData { 
                name_en: "Vexaria".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "vexaria-3".to_string(),
            ChartData { 
                name_en: "Vexaria".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "essenceoftwilight-0".to_string(),
            ChartData { 
                name_en: "Essence of Twilight".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "essenceoftwilight-1".to_string(),
            ChartData { 
                name_en: "Essence of Twilight".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "essenceoftwilight-2".to_string(),
            ChartData { 
                name_en: "Essence of Twilight".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "qualia-0".to_string(),
            ChartData { 
                name_en: "qualia -ideaesthesia-".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "qualia-1".to_string(),
            ChartData { 
                name_en: "qualia -ideaesthesia-".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "qualia-2".to_string(),
            ChartData { 
                name_en: "qualia -ideaesthesia-".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "qualia-3".to_string(),
            ChartData { 
                name_en: "qualia -ideaesthesia-".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "pragmatism-0".to_string(),
            ChartData { 
                name_en: "PRAGMATISM".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "pragmatism-1".to_string(),
            ChartData { 
                name_en: "PRAGMATISM".to_string(),
                name_jp: None,
                rating: 8.6
            }
            );
            map.insert(
            "pragmatism-2".to_string(),
            ChartData { 
                name_en: "PRAGMATISM".to_string(),
                name_jp: None,
                rating: 10.1
            }
            );
            map.insert(
            "pragmatism-3".to_string(),
            ChartData { 
                name_en: "PRAGMATISM -RESURRECTION-".to_string(),
                name_jp: None,
                rating: 11.0
            }
            );
            map.insert(
            "sheriruth-0".to_string(),
            ChartData { 
                name_en: "Sheriruth".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "sheriruth-1".to_string(),
            ChartData { 
                name_en: "Sheriruth".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "sheriruth-2".to_string(),
            ChartData { 
                name_en: "Sheriruth".to_string(),
                name_jp: None,
                rating: 10.1
            }
            );
            map.insert(
            "lumia-0".to_string(),
            ChartData { 
                name_en: "Lumia".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "lumia-1".to_string(),
            ChartData { 
                name_en: "Lumia".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "lumia-2".to_string(),
            ChartData { 
                name_en: "Lumia".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "lumia-3".to_string(),
            ChartData { 
                name_en: "Lumia".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "dement-0".to_string(),
            ChartData { 
                name_en: "Dement ~after legend~".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "dement-1".to_string(),
            ChartData { 
                name_en: "Dement ~after legend~".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "dement-2".to_string(),
            ChartData { 
                name_en: "Dement ~after legend~".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "dement-3".to_string(),
            ChartData { 
                name_en: "Dement ~after legend~".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "dandelion-0".to_string(),
            ChartData { 
                name_en: "Dandelion".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "dandelion-1".to_string(),
            ChartData { 
                name_en: "Dandelion".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "dandelion-2".to_string(),
            ChartData { 
                name_en: "Dandelion".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "anokumene-0".to_string(),
            ChartData { 
                name_en: "Anökumene".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "anokumene-1".to_string(),
            ChartData { 
                name_en: "Anökumene".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "anokumene-2".to_string(),
            ChartData { 
                name_en: "Anökumene".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "infinityheaven-0".to_string(),
            ChartData { 
                name_en: "Infinity Heaven".to_string(),
                name_jp: None,
                rating: 1.5
            }
            );
            map.insert(
            "infinityheaven-1".to_string(),
            ChartData { 
                name_en: "Infinity Heaven".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "infinityheaven-2".to_string(),
            ChartData { 
                name_en: "Infinity Heaven".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "infinityheaven-3".to_string(),
            ChartData { 
                name_en: "Infinity Heaven".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "partyvinyl-0".to_string(),
            ChartData { 
                name_en: "Party Vinyl".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "partyvinyl-1".to_string(),
            ChartData { 
                name_en: "Party Vinyl".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "partyvinyl-2".to_string(),
            ChartData { 
                name_en: "Party Vinyl".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "flashback-0".to_string(),
            ChartData { 
                name_en: "Flashback".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "flashback-1".to_string(),
            ChartData { 
                name_en: "Flashback".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "flashback-2".to_string(),
            ChartData { 
                name_en: "Flashback".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "flyburg-0".to_string(),
            ChartData { 
                name_en: "Flyburg and Endroll".to_string(),
                name_jp: Some("フライブルクとエンドロウル".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "flyburg-1".to_string(),
            ChartData { 
                name_en: "Flyburg and Endroll".to_string(),
                name_jp: Some("フライブルクとエンドロウル".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "flyburg-2".to_string(),
            ChartData { 
                name_en: "Flyburg and Endroll".to_string(),
                name_jp: Some("フライブルクとエンドロウル".to_string()),
                rating: 9.0
            }
            );
            map.insert(
            "nirvluce-0".to_string(),
            ChartData { 
                name_en: "Nirv lucE".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "nirvluce-1".to_string(),
            ChartData { 
                name_en: "Nirv lucE".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "nirvluce-2".to_string(),
            ChartData { 
                name_en: "Nirv lucE".to_string(),
                name_jp: None,
                rating: 10.3
            }
            );
            map.insert(
            "paradise-0".to_string(),
            ChartData { 
                name_en: "Paradise".to_string(),
                name_jp: None,
                rating: 1.0
            }
            );
            map.insert(
            "paradise-1".to_string(),
            ChartData { 
                name_en: "Paradise".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "paradise-2".to_string(),
            ChartData { 
                name_en: "Paradise".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "brandnewworld-0".to_string(),
            ChartData { 
                name_en: "Brand new world".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "brandnewworld-1".to_string(),
            ChartData { 
                name_en: "Brand new world".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "brandnewworld-2".to_string(),
            ChartData { 
                name_en: "Brand new world".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "dataerror-0".to_string(),
            ChartData { 
                name_en: "DataErr0r".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "dataerror-1".to_string(),
            ChartData { 
                name_en: "DataErr0r".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "dataerror-2".to_string(),
            ChartData { 
                name_en: "DataErr0r".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "crosssoul-0".to_string(),
            ChartData { 
                name_en: "CROSS†SOUL".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "crosssoul-1".to_string(),
            ChartData { 
                name_en: "CROSS†SOUL".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "crosssoul-2".to_string(),
            ChartData { 
                name_en: "CROSS†SOUL".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "yourvoiceso-0".to_string(),
            ChartData { 
                name_en: "Your voice so... feat. Such".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "yourvoiceso-1".to_string(),
            ChartData { 
                name_en: "Your voice so... feat. Such".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "yourvoiceso-2".to_string(),
            ChartData { 
                name_en: "Your voice so... feat. Such".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "chronostasis-0".to_string(),
            ChartData { 
                name_en: "Chronostasis".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "chronostasis-1".to_string(),
            ChartData { 
                name_en: "Chronostasis".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "chronostasis-2".to_string(),
            ChartData { 
                name_en: "Chronostasis".to_string(),
                name_jp: None,
                rating: 8.9
            }
            );
            map.insert(
            "kanagawa-0".to_string(),
            ChartData { 
                name_en: "Kanagawa Cyber Culvert".to_string(),
                name_jp: Some("神奈川電脳暗渠".to_string()),
                rating: 1.0
            }
            );
            map.insert(
            "kanagawa-1".to_string(),
            ChartData { 
                name_en: "Kanagawa Cyber Culvert".to_string(),
                name_jp: Some("神奈川電脳暗渠".to_string()),
                rating: 5.5
            }
            );
            map.insert(
            "kanagawa-2".to_string(),
            ChartData { 
                name_en: "Kanagawa Cyber Culvert".to_string(),
                name_jp: Some("神奈川電脳暗渠".to_string()),
                rating: 9.0
            }
            );
            map.insert(
            "kanagawa-3".to_string(),
            ChartData { 
                name_en: "Kanagawa Cyber Culvert".to_string(),
                name_jp: Some("神奈川電脳暗渠".to_string()),
                rating: 9.8
            }
            );
            map.insert(
            "moonlightofsandcastle-0".to_string(),
            ChartData { 
                name_en: "Moonlight of Sand Castle".to_string(),
                name_jp: None,
                rating: 1.5
            }
            );
            map.insert(
            "moonlightofsandcastle-1".to_string(),
            ChartData { 
                name_en: "Moonlight of Sand Castle".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "moonlightofsandcastle-2".to_string(),
            ChartData { 
                name_en: "Moonlight of Sand Castle".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "reconstruction-0".to_string(),
            ChartData { 
                name_en: "REconstruction".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "reconstruction-1".to_string(),
            ChartData { 
                name_en: "REconstruction".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "reconstruction-2".to_string(),
            ChartData { 
                name_en: "REconstruction".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "evoltex-0".to_string(),
            ChartData { 
                name_en: "Evoltex (poppi'n mix)".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "evoltex-1".to_string(),
            ChartData { 
                name_en: "Evoltex (poppi'n mix)".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "evoltex-2".to_string(),
            ChartData { 
                name_en: "Evoltex (poppi'n mix)".to_string(),
                name_jp: None,
                rating: 8.9
            }
            );
            map.insert(
            "oracle-0".to_string(),
            ChartData { 
                name_en: "Oracle".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "oracle-1".to_string(),
            ChartData { 
                name_en: "Oracle".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "oracle-2".to_string(),
            ChartData { 
                name_en: "Oracle".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "aterlbus-0".to_string(),
            ChartData { 
                name_en: "αterlβus".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "aterlbus-1".to_string(),
            ChartData { 
                name_en: "αterlβus".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "aterlbus-2".to_string(),
            ChartData { 
                name_en: "αterlβus".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "clotho-0".to_string(),
            ChartData { 
                name_en: "Clotho and the stargazer".to_string(),
                name_jp: Some("クロートーと星の観測者".to_string()),
                rating: 2.0
            }
            );
            map.insert(
            "clotho-1".to_string(),
            ChartData { 
                name_en: "Clotho and the stargazer".to_string(),
                name_jp: Some("クロートーと星の観測者".to_string()),
                rating: 5.0
            }
            );
            map.insert(
            "clotho-2".to_string(),
            ChartData { 
                name_en: "Clotho and the stargazer".to_string(),
                name_jp: Some("クロートーと星の観測者".to_string()),
                rating: 7.5
            }
            );
            map.insert(
            "impurebird-0".to_string(),
            ChartData { 
                name_en: "Impure Bird".to_string(),
                name_jp: Some("不浄な白い鳥".to_string()),
                rating: 2.0
            }
            );
            map.insert(
            "impurebird-1".to_string(),
            ChartData { 
                name_en: "Impure Bird".to_string(),
                name_jp: Some("不浄な白い鳥".to_string()),
                rating: 5.5
            }
            );
            map.insert(
            "impurebird-2".to_string(),
            ChartData { 
                name_en: "Impure Bird".to_string(),
                name_jp: Some("不浄な白い鳥".to_string()),
                rating: 9.4
            }
            );
            map.insert(
            "ignotus-0".to_string(),
            ChartData { 
                name_en: "Ignotus".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "ignotus-1".to_string(),
            ChartData { 
                name_en: "Ignotus".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "ignotus-2".to_string(),
            ChartData { 
                name_en: "Ignotus".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "ignotus-3".to_string(),
            ChartData { 
                name_en: "Ignotus Afterburn".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "lethaeus-0".to_string(),
            ChartData { 
                name_en: "Lethaeus".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "lethaeus-1".to_string(),
            ChartData { 
                name_en: "Lethaeus".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "lethaeus-2".to_string(),
            ChartData { 
                name_en: "Lethaeus".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "romancewars-0".to_string(),
            ChartData { 
                name_en: "Romance Wars".to_string(),
                name_jp: Some("vsキミ戦争".to_string()),
                rating: 1.0
            }
            );
            map.insert(
            "romancewars-1".to_string(),
            ChartData { 
                name_en: "Romance Wars".to_string(),
                name_jp: Some("vsキミ戦争".to_string()),
                rating: 4.0
            }
            );
            map.insert(
            "romancewars-2".to_string(),
            ChartData { 
                name_en: "Romance Wars".to_string(),
                name_jp: Some("vsキミ戦争".to_string()),
                rating: 7.5
            }
            );
            map.insert(
            "blossoms-0".to_string(),
            ChartData { 
                name_en: "Blossoms".to_string(),
                name_jp: None,
                rating: 1.0
            }
            );
            map.insert(
            "blossoms-1".to_string(),
            ChartData { 
                name_en: "Blossoms".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "blossoms-2".to_string(),
            ChartData { 
                name_en: "Blossoms".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "moonheart-0".to_string(),
            ChartData { 
                name_en: "Moonheart".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "moonheart-1".to_string(),
            ChartData { 
                name_en: "Moonheart".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "moonheart-2".to_string(),
            ChartData { 
                name_en: "Moonheart".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "moonheart-3".to_string(),
            ChartData { 
                name_en: "Moonheart".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "genesis-0".to_string(),
            ChartData { 
                name_en: "Genesis".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "genesis-1".to_string(),
            ChartData { 
                name_en: "Genesis".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "genesis-2".to_string(),
            ChartData { 
                name_en: "Genesis".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "harutopia-0".to_string(),
            ChartData { 
                name_en: "Harutopia ~Utopia of Spring~".to_string(),
                name_jp: Some("ハルトピア ~Utopia of Spring~".to_string()),
                rating: 1.0
            }
            );
            map.insert(
            "harutopia-1".to_string(),
            ChartData { 
                name_en: "Harutopia ~Utopia of Spring~".to_string(),
                name_jp: Some("ハルトピア ~Utopia of Spring~".to_string()),
                rating: 4.5
            }
            );
            map.insert(
            "harutopia-2".to_string(),
            ChartData { 
                name_en: "Harutopia ~Utopia of Spring~".to_string(),
                name_jp: Some("ハルトピア ~Utopia of Spring~".to_string()),
                rating: 8.5
            }
            );
            map.insert(
            "auxesia-0".to_string(),
            ChartData { 
                name_en: "Auxesia".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "auxesia-1".to_string(),
            ChartData { 
                name_en: "Auxesia".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "auxesia-2".to_string(),
            ChartData { 
                name_en: "Auxesia".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "rabbitintheblackroom-0".to_string(),
            ChartData { 
                name_en: "Rabbit In The Black Room".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "rabbitintheblackroom-1".to_string(),
            ChartData { 
                name_en: "Rabbit In The Black Room".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "rabbitintheblackroom-2".to_string(),
            ChartData { 
                name_en: "Rabbit In The Black Room".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "modelista-0".to_string(),
            ChartData { 
                name_en: "Modelista".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "modelista-1".to_string(),
            ChartData { 
                name_en: "Modelista".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "modelista-2".to_string(),
            ChartData { 
                name_en: "Modelista".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "soundwitch-0".to_string(),
            ChartData { 
                name_en: "SOUNDWiTCH".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "soundwitch-1".to_string(),
            ChartData { 
                name_en: "SOUNDWiTCH".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "soundwitch-2".to_string(),
            ChartData { 
                name_en: "SOUNDWiTCH".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "trappola-0".to_string(),
            ChartData { 
                name_en: "trappola bewitching".to_string(),
                name_jp: Some("妖艶魔女 -trappola bewitching-".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "trappola-1".to_string(),
            ChartData { 
                name_en: "trappola bewitching".to_string(),
                name_jp: Some("妖艶魔女 -trappola bewitching-".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "trappola-2".to_string(),
            ChartData { 
                name_en: "trappola bewitching".to_string(),
                name_jp: Some("妖艶魔女 -trappola bewitching-".to_string()),
                rating: 10.0
            }
            );
            map.insert(
            "iconoclast-0".to_string(),
            ChartData { 
                name_en: "Iconoclast".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "iconoclast-1".to_string(),
            ChartData { 
                name_en: "Iconoclast".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "iconoclast-2".to_string(),
            ChartData { 
                name_en: "Iconoclast".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "conflict-0".to_string(),
            ChartData { 
                name_en: "conflict".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "conflict-1".to_string(),
            ChartData { 
                name_en: "conflict".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "conflict-2".to_string(),
            ChartData { 
                name_en: "conflict".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "axiumcrisis-0".to_string(),
            ChartData { 
                name_en: "Axium Crisis".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "axiumcrisis-1".to_string(),
            ChartData { 
                name_en: "Axium Crisis".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "axiumcrisis-2".to_string(),
            ChartData { 
                name_en: "Axium Crisis".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "grievouslady-0".to_string(),
            ChartData { 
                name_en: "Grievous Lady".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "grievouslady-1".to_string(),
            ChartData { 
                name_en: "Grievous Lady".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "grievouslady-2".to_string(),
            ChartData { 
                name_en: "Grievous Lady".to_string(),
                name_jp: None,
                rating: 11.3
            }
            );
            map.insert(
            "dreaminattraction-0".to_string(),
            ChartData { 
                name_en: "Dreamin' Attraction!!".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "dreaminattraction-1".to_string(),
            ChartData { 
                name_en: "Dreamin' Attraction!!".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "dreaminattraction-2".to_string(),
            ChartData { 
                name_en: "Dreamin' Attraction!!".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "redandblue-0".to_string(),
            ChartData { 
                name_en: "Red and Blue".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "redandblue-1".to_string(),
            ChartData { 
                name_en: "Red and Blue".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "redandblue-2".to_string(),
            ChartData { 
                name_en: "Red and Blue".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "redandblue-3".to_string(),
            ChartData { 
                name_en: "Red and Blue and Green".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "onelastdrive-0".to_string(),
            ChartData { 
                name_en: "One Last Drive".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "onelastdrive-1".to_string(),
            ChartData { 
                name_en: "One Last Drive".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "onelastdrive-2".to_string(),
            ChartData { 
                name_en: "One Last Drive".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "surrender-0".to_string(),
            ChartData { 
                name_en: "Surrender".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "surrender-1".to_string(),
            ChartData { 
                name_en: "Surrender".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "surrender-2".to_string(),
            ChartData { 
                name_en: "Surrender".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "yozakurafubuki-0".to_string(),
            ChartData { 
                name_en: "Yosakura Fubuki".to_string(),
                name_jp: Some("夜桜吹雪".to_string()),
                rating: 4.5
            }
            );
            map.insert(
            "yozakurafubuki-1".to_string(),
            ChartData { 
                name_en: "Yosakura Fubuki".to_string(),
                name_jp: Some("夜桜吹雪".to_string()),
                rating: 7.0
            }
            );
            map.insert(
            "yozakurafubuki-2".to_string(),
            ChartData { 
                name_en: "Yosakura Fubuki".to_string(),
                name_jp: Some("夜桜吹雪".to_string()),
                rating: 9.4
            }
            );
            map.insert(
            "cyanine-0".to_string(),
            ChartData { 
                name_en: "cyanine".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "cyanine-1".to_string(),
            ChartData { 
                name_en: "cyanine".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "cyanine-2".to_string(),
            ChartData { 
                name_en: "cyanine".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "dreamgoeson-0".to_string(),
            ChartData { 
                name_en: "Dream goes on".to_string(),
                name_jp: None,
                rating: 1.5
            }
            );
            map.insert(
            "dreamgoeson-1".to_string(),
            ChartData { 
                name_en: "Dream goes on".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "dreamgoeson-2".to_string(),
            ChartData { 
                name_en: "Dream goes on".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "journey-0".to_string(),
            ChartData { 
                name_en: "Journey".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "journey-1".to_string(),
            ChartData { 
                name_en: "Journey".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "journey-2".to_string(),
            ChartData { 
                name_en: "Journey".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "specta-0".to_string(),
            ChartData { 
                name_en: "Specta".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "specta-1".to_string(),
            ChartData { 
                name_en: "Specta".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "specta-2".to_string(),
            ChartData { 
                name_en: "Specta".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "quon-0".to_string(),
            ChartData { 
                name_en: "Quon".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "quon-1".to_string(),
            ChartData { 
                name_en: "Quon".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "quon-2".to_string(),
            ChartData { 
                name_en: "Quon".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "syro-0".to_string(),
            ChartData { 
                name_en: "Syro".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "syro-1".to_string(),
            ChartData { 
                name_en: "Syro".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "syro-2".to_string(),
            ChartData { 
                name_en: "Syro".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "reinvent-0".to_string(),
            ChartData { 
                name_en: "Reinvent".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "reinvent-1".to_string(),
            ChartData { 
                name_en: "Reinvent".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "reinvent-2".to_string(),
            ChartData { 
                name_en: "Reinvent".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "silentrush-0".to_string(),
            ChartData { 
                name_en: "Silent Rush".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "silentrush-1".to_string(),
            ChartData { 
                name_en: "Silent Rush".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "silentrush-2".to_string(),
            ChartData { 
                name_en: "Silent Rush".to_string(),
                name_jp: None,
                rating: 8.6
            }
            );
            map.insert(
            "singularity-0".to_string(),
            ChartData { 
                name_en: "Singularity".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "singularity-1".to_string(),
            ChartData { 
                name_en: "Singularity".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "singularity-2".to_string(),
            ChartData { 
                name_en: "Singularity".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "singularity-3".to_string(),
            ChartData { 
                name_en: "Singularity VVVIP".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "memoryforest-0".to_string(),
            ChartData { 
                name_en: "Memory Forest".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "memoryforest-1".to_string(),
            ChartData { 
                name_en: "Memory Forest".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "memoryforest-2".to_string(),
            ChartData { 
                name_en: "Memory Forest".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "strongholds-0".to_string(),
            ChartData { 
                name_en: "Strongholds".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "strongholds-1".to_string(),
            ChartData { 
                name_en: "Strongholds".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "strongholds-2".to_string(),
            ChartData { 
                name_en: "Strongholds".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "nexttoyou-0".to_string(),
            ChartData { 
                name_en: "next to you".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "nexttoyou-1".to_string(),
            ChartData { 
                name_en: "next to you".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "nexttoyou-2".to_string(),
            ChartData { 
                name_en: "next to you".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "nexttoyou-3".to_string(),
            ChartData { 
                name_en: "next to you".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "metallicpunisher-0".to_string(),
            ChartData { 
                name_en: "Metallic Punisher".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "metallicpunisher-1".to_string(),
            ChartData { 
                name_en: "Metallic Punisher".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "metallicpunisher-2".to_string(),
            ChartData { 
                name_en: "Metallic Punisher".to_string(),
                name_jp: None,
                rating: 10.3
            }
            );
            map.insert(
            "blaster-0".to_string(),
            ChartData { 
                name_en: "Blaster".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "blaster-1".to_string(),
            ChartData { 
                name_en: "Blaster".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "blaster-2".to_string(),
            ChartData { 
                name_en: "Blaster".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "guardina-0".to_string(),
            ChartData { 
                name_en: "γuarδina".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "guardina-1".to_string(),
            ChartData { 
                name_en: "γuarδina".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "guardina-2".to_string(),
            ChartData { 
                name_en: "γuarδina".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "carminescythe-0".to_string(),
            ChartData { 
                name_en: "carmine:scythe".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "carminescythe-1".to_string(),
            ChartData { 
                name_en: "carmine:scythe".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "carminescythe-2".to_string(),
            ChartData { 
                name_en: "carmine:scythe".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "bethere-0".to_string(),
            ChartData { 
                name_en: "Be There".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "bethere-1".to_string(),
            ChartData { 
                name_en: "Be There".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "bethere-2".to_string(),
            ChartData { 
                name_en: "Be There".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "cyberneciacatharsis-0".to_string(),
            ChartData { 
                name_en: "Cybernecia Catharsis".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "cyberneciacatharsis-1".to_string(),
            ChartData { 
                name_en: "Cybernecia Catharsis".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "cyberneciacatharsis-2".to_string(),
            ChartData { 
                name_en: "Cybernecia Catharsis".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "callmyname-0".to_string(),
            ChartData { 
                name_en: "Call My Name feat. Yukacco".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "callmyname-1".to_string(),
            ChartData { 
                name_en: "Call My Name feat. Yukacco".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "callmyname-2".to_string(),
            ChartData { 
                name_en: "Call My Name feat. Yukacco".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "inkarusi-0".to_string(),
            ChartData { 
                name_en: "inkar-usi".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "inkarusi-1".to_string(),
            ChartData { 
                name_en: "inkar-usi".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "inkarusi-2".to_string(),
            ChartData { 
                name_en: "inkar-usi".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "mazenine-0".to_string(),
            ChartData { 
                name_en: "Maze No.9".to_string(),
                name_jp: Some("九番目の迷路".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "mazenine-1".to_string(),
            ChartData { 
                name_en: "Maze No.9".to_string(),
                name_jp: Some("九番目の迷路".to_string()),
                rating: 3.5
            }
            );
            map.insert(
            "mazenine-2".to_string(),
            ChartData { 
                name_en: "Maze No.9".to_string(),
                name_jp: Some("九番目の迷路".to_string()),
                rating: 8.9
            }
            );
            map.insert(
            "themessage-0".to_string(),
            ChartData { 
                name_en: "The Message".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "themessage-1".to_string(),
            ChartData { 
                name_en: "The Message".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "themessage-2".to_string(),
            ChartData { 
                name_en: "The Message".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "sulfur-0".to_string(),
            ChartData { 
                name_en: "Sulfur".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "sulfur-1".to_string(),
            ChartData { 
                name_en: "Sulfur".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "sulfur-2".to_string(),
            ChartData { 
                name_en: "Sulfur".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "halcyon-0".to_string(),
            ChartData { 
                name_en: "Halcyon".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "halcyon-1".to_string(),
            ChartData { 
                name_en: "Halcyon".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "halcyon-2".to_string(),
            ChartData { 
                name_en: "Halcyon".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "etherstrike-0".to_string(),
            ChartData { 
                name_en: "Ether Strike".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "etherstrike-1".to_string(),
            ChartData { 
                name_en: "Ether Strike".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "etherstrike-2".to_string(),
            ChartData { 
                name_en: "Ether Strike".to_string(),
                name_jp: None,
                rating: 10.3
            }
            );
            map.insert(
            "fractureray-0".to_string(),
            ChartData { 
                name_en: "Fracture Ray".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "fractureray-1".to_string(),
            ChartData { 
                name_en: "Fracture Ray".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "fractureray-2".to_string(),
            ChartData { 
                name_en: "Fracture Ray".to_string(),
                name_jp: None,
                rating: 11.3
            }
            );
            map.insert(
            "suomi-0".to_string(),
            ChartData { 
                name_en: "Suomi".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "suomi-1".to_string(),
            ChartData { 
                name_en: "Suomi".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "suomi-2".to_string(),
            ChartData { 
                name_en: "Suomi".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "bookmaker-0".to_string(),
            ChartData { 
                name_en: "Bookmaker (2D Version)".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "bookmaker-1".to_string(),
            ChartData { 
                name_en: "Bookmaker (2D Version)".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "bookmaker-2".to_string(),
            ChartData { 
                name_en: "Bookmaker (2D Version)".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "bookmaker-3".to_string(),
            ChartData { 
                name_en: "Bookmaker (2D Version)".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "darakunosono-0".to_string(),
            ChartData { 
                name_en: "Illegal Paradise".to_string(),
                name_jp: Some("堕楽の園".to_string()),
                rating: 2.0
            }
            );
            map.insert(
            "darakunosono-1".to_string(),
            ChartData { 
                name_en: "Illegal Paradise".to_string(),
                name_jp: Some("堕楽の園".to_string()),
                rating: 7.0
            }
            );
            map.insert(
            "darakunosono-2".to_string(),
            ChartData { 
                name_en: "Illegal Paradise".to_string(),
                name_jp: Some("堕楽の園".to_string()),
                rating: 9.5
            }
            );
            map.insert(
            "dropdead-0".to_string(),
            ChartData { 
                name_en: "dropdead".to_string(),
                name_jp: None,
                rating: 1.5
            }
            );
            map.insert(
            "dropdead-1".to_string(),
            ChartData { 
                name_en: "dropdead".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "dropdead-2".to_string(),
            ChartData { 
                name_en: "dropdead".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "dropdead-3".to_string(),
            ChartData { 
                name_en: "overdead.".to_string(),
                name_jp: None,
                rating: 10.5
            }
            );
            map.insert(
            "fallensquare-0".to_string(),
            ChartData { 
                name_en: "Fallensquare".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "fallensquare-1".to_string(),
            ChartData { 
                name_en: "Fallensquare".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "fallensquare-2".to_string(),
            ChartData { 
                name_en: "Fallensquare".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "nhelv-0".to_string(),
            ChartData { 
                name_en: "Nhelv".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "nhelv-1".to_string(),
            ChartData { 
                name_en: "Nhelv".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "nhelv-2".to_string(),
            ChartData { 
                name_en: "Nhelv".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "espebranch-0".to_string(),
            ChartData { 
                name_en: "LunarOrbit -believe in the Espebranch road-".to_string(),
                name_jp: Some("白道、多希望羊と信じありく。".to_string()),
                rating: 3.5
            }
            );
            map.insert(
            "espebranch-1".to_string(),
            ChartData { 
                name_en: "LunarOrbit -believe in the Espebranch road-".to_string(),
                name_jp: Some("白道、多希望羊と信じありく。".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "espebranch-2".to_string(),
            ChartData { 
                name_en: "LunarOrbit -believe in the Espebranch road-".to_string(),
                name_jp: Some("白道、多希望羊と信じありく。".to_string()),
                rating: 9.6
            }
            );
            map.insert(
            "purgatorium-0".to_string(),
            ChartData { 
                name_en: "Purgatorium".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "purgatorium-1".to_string(),
            ChartData { 
                name_en: "Purgatorium".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "purgatorium-2".to_string(),
            ChartData { 
                name_en: "Purgatorium".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "purgatorium-3".to_string(),
            ChartData { 
                name_en: "Purgatorium".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "hikari-0".to_string(),
            ChartData { 
                name_en: "Hikari".to_string(),
                name_jp: Some("光".to_string()),
                rating: 2.5
            }
            );
            map.insert(
            "hikari-1".to_string(),
            ChartData { 
                name_en: "Hikari".to_string(),
                name_jp: Some("光".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "hikari-2".to_string(),
            ChartData { 
                name_en: "Hikari".to_string(),
                name_jp: Some("光".to_string()),
                rating: 8.1
            }
            );
            map.insert(
            "stager-0".to_string(),
            ChartData { 
                name_en: "STAGER (ALL STAGE CLEAR)".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "stager-1".to_string(),
            ChartData { 
                name_en: "STAGER (ALL STAGE CLEAR)".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "stager-2".to_string(),
            ChartData { 
                name_en: "STAGER (ALL STAGE CLEAR)".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "hallofmirrors-0".to_string(),
            ChartData { 
                name_en: "Hall of Mirrors".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "hallofmirrors-1".to_string(),
            ChartData { 
                name_en: "Hall of Mirrors".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "hallofmirrors-2".to_string(),
            ChartData { 
                name_en: "Hall of Mirrors".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "linearaccelerator-0".to_string(),
            ChartData { 
                name_en: "Linear Accelerator".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "linearaccelerator-1".to_string(),
            ChartData { 
                name_en: "Linear Accelerator".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "linearaccelerator-2".to_string(),
            ChartData { 
                name_en: "Linear Accelerator".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "tiferet-0".to_string(),
            ChartData { 
                name_en: "Tiferet".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "tiferet-1".to_string(),
            ChartData { 
                name_en: "Tiferet".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "tiferet-2".to_string(),
            ChartData { 
                name_en: "Tiferet".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "alexandrite-0".to_string(),
            ChartData { 
                name_en: "Alexandrite".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "alexandrite-1".to_string(),
            ChartData { 
                name_en: "Alexandrite".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "alexandrite-2".to_string(),
            ChartData { 
                name_en: "Alexandrite".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "rugie-0".to_string(),
            ChartData { 
                name_en: "Rugie".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "rugie-1".to_string(),
            ChartData { 
                name_en: "Rugie".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "rugie-2".to_string(),
            ChartData { 
                name_en: "Rugie".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "astraltale-0".to_string(),
            ChartData { 
                name_en: "Astral tale".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "astraltale-1".to_string(),
            ChartData { 
                name_en: "Astral tale".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "astraltale-2".to_string(),
            ChartData { 
                name_en: "Astral tale".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "phantasia-0".to_string(),
            ChartData { 
                name_en: "Phantasia".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "phantasia-1".to_string(),
            ChartData { 
                name_en: "Phantasia".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "phantasia-2".to_string(),
            ChartData { 
                name_en: "Phantasia".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "empireofwinter-0".to_string(),
            ChartData { 
                name_en: "Empire of Winter".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "empireofwinter-1".to_string(),
            ChartData { 
                name_en: "Empire of Winter".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "empireofwinter-2".to_string(),
            ChartData { 
                name_en: "Empire of Winter".to_string(),
                name_jp: None,
                rating: 9.0
            }
            );
            map.insert(
            "merlin-0".to_string(),
            ChartData { 
                name_en: "MERLIN".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "merlin-1".to_string(),
            ChartData { 
                name_en: "MERLIN".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "merlin-2".to_string(),
            ChartData { 
                name_en: "MERLIN".to_string(),
                name_jp: None,
                rating: 8.9
            }
            );
            map.insert(
            "merlin-3".to_string(),
            ChartData { 
                name_en: "MERLIN".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "dxfullmetal-0".to_string(),
            ChartData { 
                name_en: "DX Choseinou Full Metal Shojo".to_string(),
                name_jp: Some("DX超性能フルメタル少女".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "dxfullmetal-1".to_string(),
            ChartData { 
                name_en: "DX Choseinou Full Metal Shojo".to_string(),
                name_jp: Some("DX超性能フルメタル少女".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "dxfullmetal-2".to_string(),
            ChartData { 
                name_en: "DX Choseinou Full Metal Shojo".to_string(),
                name_jp: Some("DX超性能フルメタル少女".to_string()),
                rating: 9.8
            }
            );
            map.insert(
            "omakeno-0".to_string(),
            ChartData { 
                name_en: "OMAKENO Stroke".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "omakeno-1".to_string(),
            ChartData { 
                name_en: "OMAKENO Stroke".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "omakeno-2".to_string(),
            ChartData { 
                name_en: "OMAKENO Stroke".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "omakeno-3".to_string(),
            ChartData { 
                name_en: "OMAKENO Stroke".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "scarletlance-0".to_string(),
            ChartData { 
                name_en: "Scarlet Lance".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "scarletlance-1".to_string(),
            ChartData { 
                name_en: "Scarlet Lance".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "scarletlance-2".to_string(),
            ChartData { 
                name_en: "Scarlet Lance".to_string(),
                name_jp: None,
                rating: 10.1
            }
            );
            map.insert(
            "ouroboros-0".to_string(),
            ChartData { 
                name_en: "ouroboros -twin stroke of the end-".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "ouroboros-1".to_string(),
            ChartData { 
                name_en: "ouroboros -twin stroke of the end-".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "ouroboros-2".to_string(),
            ChartData { 
                name_en: "ouroboros -twin stroke of the end-".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "libertas-0".to_string(),
            ChartData { 
                name_en: "Libertas".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "libertas-1".to_string(),
            ChartData { 
                name_en: "Libertas".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "libertas-2".to_string(),
            ChartData { 
                name_en: "Libertas".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "solitarydream-0".to_string(),
            ChartData { 
                name_en: "Solitary Dream".to_string(),
                name_jp: Some("虚空の夢".to_string()),
                rating: 4.0
            }
            );
            map.insert(
            "solitarydream-1".to_string(),
            ChartData { 
                name_en: "Solitary Dream".to_string(),
                name_jp: Some("虚空の夢".to_string()),
                rating: 7.0
            }
            );
            map.insert(
            "solitarydream-2".to_string(),
            ChartData { 
                name_en: "Solitary Dream".to_string(),
                name_jp: Some("虚空の夢".to_string()),
                rating: 8.8
            }
            );
            map.insert(
            "antithese-0".to_string(),
            ChartData { 
                name_en: "Antithese".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "antithese-1".to_string(),
            ChartData { 
                name_en: "Antithese".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "antithese-2".to_string(),
            ChartData { 
                name_en: "Antithese".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "antithese-3".to_string(),
            ChartData { 
                name_en: "Antithese".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "corruption-0".to_string(),
            ChartData { 
                name_en: "Corruption".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "corruption-1".to_string(),
            ChartData { 
                name_en: "Corruption".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "corruption-2".to_string(),
            ChartData { 
                name_en: "Corruption".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "blackterritory-0".to_string(),
            ChartData { 
                name_en: "Black Territory".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "blackterritory-1".to_string(),
            ChartData { 
                name_en: "Black Territory".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "blackterritory-2".to_string(),
            ChartData { 
                name_en: "Black Territory".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "viciousheroism-0".to_string(),
            ChartData { 
                name_en: "Vicious Heroism".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "viciousheroism-1".to_string(),
            ChartData { 
                name_en: "Vicious Heroism".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "viciousheroism-2".to_string(),
            ChartData { 
                name_en: "Vicious Heroism".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "cyaegha-0".to_string(),
            ChartData { 
                name_en: "Cyaegha".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "cyaegha-1".to_string(),
            ChartData { 
                name_en: "Cyaegha".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "cyaegha-2".to_string(),
            ChartData { 
                name_en: "Cyaegha".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "revixy-0".to_string(),
            ChartData { 
                name_en: "ReviXy".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "revixy-1".to_string(),
            ChartData { 
                name_en: "ReviXy".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "revixy-2".to_string(),
            ChartData { 
                name_en: "ReviXy".to_string(),
                name_jp: None,
                rating: 9.0
            }
            );
            map.insert(
            "grimheart-0".to_string(),
            ChartData { 
                name_en: "Grimheart".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "grimheart-1".to_string(),
            ChartData { 
                name_en: "Grimheart".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "grimheart-2".to_string(),
            ChartData { 
                name_en: "Grimheart".to_string(),
                name_jp: None,
                rating: 8.6
            }
            );
            map.insert(
            "vector-0".to_string(),
            ChartData { 
                name_en: "VECTOЯ".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "vector-1".to_string(),
            ChartData { 
                name_en: "VECTOЯ".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "vector-2".to_string(),
            ChartData { 
                name_en: "VECTOЯ".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "supernova-0".to_string(),
            ChartData { 
                name_en: "SUPERNOVA".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "supernova-1".to_string(),
            ChartData { 
                name_en: "SUPERNOVA".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "supernova-2".to_string(),
            ChartData { 
                name_en: "SUPERNOVA".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "dottodot-0".to_string(),
            ChartData { 
                name_en: "Dot to Dot feat. shully".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "dottodot-1".to_string(),
            ChartData { 
                name_en: "Dot to Dot feat. shully".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "dottodot-2".to_string(),
            ChartData { 
                name_en: "Dot to Dot feat. shully".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "garakuta-0".to_string(),
            ChartData { 
                name_en: "Garakuta Doll Play".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "garakuta-1".to_string(),
            ChartData { 
                name_en: "Garakuta Doll Play".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "garakuta-2".to_string(),
            ChartData { 
                name_en: "Garakuta Doll Play".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "ikazuchi-0".to_string(),
            ChartData { 
                name_en: "Ikazuchi".to_string(),
                name_jp: Some("怒槌".to_string()),
                rating: 3.5
            }
            );
            map.insert(
            "ikazuchi-1".to_string(),
            ChartData { 
                name_en: "Ikazuchi".to_string(),
                name_jp: Some("怒槌".to_string()),
                rating: 7.5
            }
            );
            map.insert(
            "ikazuchi-2".to_string(),
            ChartData { 
                name_en: "Ikazuchi".to_string(),
                name_jp: Some("怒槌".to_string()),
                rating: 10.4
            }
            );
            map.insert(
            "worldvanquisher-0".to_string(),
            ChartData { 
                name_en: "World Vanquisher".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "worldvanquisher-1".to_string(),
            ChartData { 
                name_en: "World Vanquisher".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "worldvanquisher-2".to_string(),
            ChartData { 
                name_en: "World Vanquisher".to_string(),
                name_jp: None,
                rating: 10.8
            }
            );
            map.insert(
            "dreadnought-0".to_string(),
            ChartData { 
                name_en: "Dreadnought".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "dreadnought-1".to_string(),
            ChartData { 
                name_en: "Dreadnought".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "dreadnought-2".to_string(),
            ChartData { 
                name_en: "Dreadnought".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "particlearts-0".to_string(),
            ChartData { 
                name_en: "Particle Arts".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "particlearts-1".to_string(),
            ChartData { 
                name_en: "Particle Arts".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "particlearts-2".to_string(),
            ChartData { 
                name_en: "Particle Arts".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "vindication-0".to_string(),
            ChartData { 
                name_en: "Vindication".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "vindication-1".to_string(),
            ChartData { 
                name_en: "Vindication".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "vindication-2".to_string(),
            ChartData { 
                name_en: "Vindication".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "heavensdoor-0".to_string(),
            ChartData { 
                name_en: "Heavensdoor".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "heavensdoor-1".to_string(),
            ChartData { 
                name_en: "Heavensdoor".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "heavensdoor-2".to_string(),
            ChartData { 
                name_en: "Heavensdoor".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "heavensdoor-3".to_string(),
            ChartData { 
                name_en: "Heavensdoor".to_string(),
                name_jp: None,
                rating: 10.8
            }
            );
            map.insert(
            "ringedgenesis-0".to_string(),
            ChartData { 
                name_en: "Ringed Genesis".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "ringedgenesis-1".to_string(),
            ChartData { 
                name_en: "Ringed Genesis".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "ringedgenesis-2".to_string(),
            ChartData { 
                name_en: "Ringed Genesis".to_string(),
                name_jp: None,
                rating: 10.8
            }
            );
            map.insert(
            "chelsea-0".to_string(),
            ChartData { 
                name_en: "Chelsea".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "chelsea-1".to_string(),
            ChartData { 
                name_en: "Chelsea".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "chelsea-2".to_string(),
            ChartData { 
                name_en: "Chelsea".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "aiueoon-0".to_string(),
            ChartData { 
                name_en: "AI[UE]OON".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "aiueoon-1".to_string(),
            ChartData { 
                name_en: "AI[UE]OON".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "aiueoon-2".to_string(),
            ChartData { 
                name_en: "AI[UE]OON".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "melodyoflove-0".to_string(),
            ChartData { 
                name_en: "A Wandering Melody of Love".to_string(),
                name_jp: Some("迷える音色は恋の唄".to_string()),
                rating: 3.5
            }
            );
            map.insert(
            "melodyoflove-1".to_string(),
            ChartData { 
                name_en: "A Wandering Melody of Love".to_string(),
                name_jp: Some("迷える音色は恋の唄".to_string()),
                rating: 7.5
            }
            );
            map.insert(
            "melodyoflove-2".to_string(),
            ChartData { 
                name_en: "A Wandering Melody of Love".to_string(),
                name_jp: Some("迷える音色は恋の唄".to_string()),
                rating: 9.6
            }
            );
            map.insert(
            "tiemedowngently-0".to_string(),
            ChartData { 
                name_en: "Tie me down gently".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "tiemedowngently-1".to_string(),
            ChartData { 
                name_en: "Tie me down gently".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "tiemedowngently-2".to_string(),
            ChartData { 
                name_en: "Tie me down gently".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "valhallazero-0".to_string(),
            ChartData { 
                name_en: "Valhalla:0".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "valhallazero-1".to_string(),
            ChartData { 
                name_en: "Valhalla:0".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "valhallazero-2".to_string(),
            ChartData { 
                name_en: "Valhalla:0".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "mirzam-0".to_string(),
            ChartData { 
                name_en: "Mirzam".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "mirzam-1".to_string(),
            ChartData { 
                name_en: "Mirzam".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "mirzam-2".to_string(),
            ChartData { 
                name_en: "Mirzam".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "diode-0".to_string(),
            ChartData { 
                name_en: "Diode".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "diode-1".to_string(),
            ChartData { 
                name_en: "Diode".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "diode-2".to_string(),
            ChartData { 
                name_en: "Diode".to_string(),
                name_jp: None,
                rating: 8.1
            }
            );
            map.insert(
            "freefall-0".to_string(),
            ChartData { 
                name_en: "FREEF4LL".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "freefall-1".to_string(),
            ChartData { 
                name_en: "FREEF4LL".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "freefall-2".to_string(),
            ChartData { 
                name_en: "FREEF4LL".to_string(),
                name_jp: None,
                rating: 8.6
            }
            );
            map.insert(
            "gloryroad-0".to_string(),
            ChartData { 
                name_en: "GLORY：ROAD".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "gloryroad-1".to_string(),
            ChartData { 
                name_en: "GLORY：ROAD".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "gloryroad-2".to_string(),
            ChartData { 
                name_en: "GLORY：ROAD".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "monochromeprincess-0".to_string(),
            ChartData { 
                name_en: "Monochrome Princess".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "monochromeprincess-1".to_string(),
            ChartData { 
                name_en: "Monochrome Princess".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "monochromeprincess-2".to_string(),
            ChartData { 
                name_en: "Monochrome Princess".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "heavenlycaress-0".to_string(),
            ChartData { 
                name_en: "Heavenly caress".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "heavenlycaress-1".to_string(),
            ChartData { 
                name_en: "Heavenly caress".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "heavenlycaress-2".to_string(),
            ChartData { 
                name_en: "Heavenly caress".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "senkyou-0".to_string(),
            ChartData { 
                name_en: "Senkyou".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "senkyou-1".to_string(),
            ChartData { 
                name_en: "Senkyou".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "senkyou-2".to_string(),
            ChartData { 
                name_en: "Senkyou".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "filament-0".to_string(),
            ChartData { 
                name_en: "Filament".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "filament-1".to_string(),
            ChartData { 
                name_en: "Filament".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "filament-2".to_string(),
            ChartData { 
                name_en: "Filament".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "avantraze-0".to_string(),
            ChartData { 
                name_en: "Avant Raze".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "avantraze-1".to_string(),
            ChartData { 
                name_en: "Avant Raze".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "avantraze-2".to_string(),
            ChartData { 
                name_en: "Avant Raze".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "battlenoone-0".to_string(),
            ChartData { 
                name_en: "BATTLE NO.1".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "battlenoone-1".to_string(),
            ChartData { 
                name_en: "BATTLE NO.1".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "battlenoone-2".to_string(),
            ChartData { 
                name_en: "BATTLE NO.1".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "laqryma-0".to_string(),
            ChartData { 
                name_en: "La'qryma of the Wasteland".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "laqryma-1".to_string(),
            ChartData { 
                name_en: "La'qryma of the Wasteland".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "laqryma-2".to_string(),
            ChartData { 
                name_en: "La'qryma of the Wasteland".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "laqryma-3".to_string(),
            ChartData { 
                name_en: "La'qryma of the Wasteland".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "einherjar-0".to_string(),
            ChartData { 
                name_en: "Einherjar Joker".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "einherjar-1".to_string(),
            ChartData { 
                name_en: "Einherjar Joker".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "einherjar-2".to_string(),
            ChartData { 
                name_en: "Einherjar Joker".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "izana-0".to_string(),
            ChartData { 
                name_en: "IZANA".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "izana-1".to_string(),
            ChartData { 
                name_en: "IZANA".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "izana-2".to_string(),
            ChartData { 
                name_en: "IZANA".to_string(),
                name_jp: None,
                rating: 10.3
            }
            );
            map.insert(
            "saikyostronger-0".to_string(),
            ChartData { 
                name_en: "SAIKYO STRONGER".to_string(),
                name_jp: Some("最強STRONGER".to_string()),
                rating: 5.5
            }
            );
            map.insert(
            "saikyostronger-1".to_string(),
            ChartData { 
                name_en: "SAIKYO STRONGER".to_string(),
                name_jp: Some("最強STRONGER".to_string()),
                rating: 9.4
            }
            );
            map.insert(
            "saikyostronger-2".to_string(),
            ChartData { 
                name_en: "SAIKYO STRONGER".to_string(),
                name_jp: Some("最強STRONGER".to_string()),
                rating: 11.0
            }
            );
            map.insert(
            "worldexecuteme-0".to_string(),
            ChartData { 
                name_en: "world.execute(me);".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "worldexecuteme-1".to_string(),
            ChartData { 
                name_en: "world.execute(me);".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "worldexecuteme-2".to_string(),
            ChartData { 
                name_en: "world.execute(me);".to_string(),
                name_jp: None,
                rating: 8.0
            }
            );
            map.insert(
            "blrink-0".to_string(),
            ChartData { 
                name_en: "BLRINK".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "blrink-1".to_string(),
            ChartData { 
                name_en: "BLRINK".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "blrink-2".to_string(),
            ChartData { 
                name_en: "BLRINK".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "oblivia-0".to_string(),
            ChartData { 
                name_en: "Oblivia".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "oblivia-1".to_string(),
            ChartData { 
                name_en: "Oblivia".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "oblivia-2".to_string(),
            ChartData { 
                name_en: "Oblivia".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "amygdata-0".to_string(),
            ChartData { 
                name_en: "amygdata".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "amygdata-1".to_string(),
            ChartData { 
                name_en: "amygdata".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "amygdata-2".to_string(),
            ChartData { 
                name_en: "amygdata".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "corpssansorganes-0".to_string(),
            ChartData { 
                name_en: "corps-sans-organes".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "corpssansorganes-1".to_string(),
            ChartData { 
                name_en: "corps-sans-organes".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "corpssansorganes-2".to_string(),
            ChartData { 
                name_en: "corps-sans-organes".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "equilibrium-0".to_string(),
            ChartData { 
                name_en: "Equilibrium".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "equilibrium-1".to_string(),
            ChartData { 
                name_en: "Equilibrium".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "equilibrium-2".to_string(),
            ChartData { 
                name_en: "Equilibrium".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "antagonism-0".to_string(),
            ChartData { 
                name_en: "Antagonism".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "antagonism-1".to_string(),
            ChartData { 
                name_en: "Antagonism".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "antagonism-2".to_string(),
            ChartData { 
                name_en: "Antagonism".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "lostdesire-0".to_string(),
            ChartData { 
                name_en: "Lost Desire".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "lostdesire-1".to_string(),
            ChartData { 
                name_en: "Lost Desire".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "lostdesire-2".to_string(),
            ChartData { 
                name_en: "Lost Desire".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "dantalion-0".to_string(),
            ChartData { 
                name_en: "Dantalion".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "dantalion-1".to_string(),
            ChartData { 
                name_en: "Dantalion".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "dantalion-2".to_string(),
            ChartData { 
                name_en: "Dantalion".to_string(),
                name_jp: None,
                rating: 10.9
            }
            );
            map.insert(
            "ifi-0".to_string(),
            ChartData { 
                name_en: "#1f1e33".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "ifi-1".to_string(),
            ChartData { 
                name_en: "#1f1e33".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "ifi-2".to_string(),
            ChartData { 
                name_en: "#1f1e33".to_string(),
                name_jp: None,
                rating: 10.9
            }
            );
            map.insert(
            "tempestissimo-0".to_string(),
            ChartData { 
                name_en: "Tempestissimo".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "tempestissimo-1".to_string(),
            ChartData { 
                name_en: "Tempestissimo".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "tempestissimo-2".to_string(),
            ChartData { 
                name_en: "Tempestissimo".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "tempestissimo-3".to_string(),
            ChartData { 
                name_en: "Tempestissimo".to_string(),
                name_jp: None,
                rating: 11.5
            }
            );
            map.insert(
            "arcahv-0".to_string(),
            ChartData { 
                name_en: "Arcahv".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "arcahv-1".to_string(),
            ChartData { 
                name_en: "Arcahv".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "arcahv-2".to_string(),
            ChartData { 
                name_en: "Arcahv".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "altale-0".to_string(),
            ChartData { 
                name_en: "Altale".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "altale-1".to_string(),
            ChartData { 
                name_en: "Altale".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "altale-2".to_string(),
            ChartData { 
                name_en: "Altale".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "givemeanightmare-0".to_string(),
            ChartData { 
                name_en: "Give Me a Nightmare".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "givemeanightmare-1".to_string(),
            ChartData { 
                name_en: "Give Me a Nightmare".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "givemeanightmare-2".to_string(),
            ChartData { 
                name_en: "Give Me a Nightmare".to_string(),
                name_jp: None,
                rating: 8.9
            }
            );
            map.insert(
            "blacklotus-0".to_string(),
            ChartData { 
                name_en: "Black Lotus".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "blacklotus-1".to_string(),
            ChartData { 
                name_en: "Black Lotus".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "blacklotus-2".to_string(),
            ChartData { 
                name_en: "Black Lotus".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "gekka-0".to_string(),
            ChartData { 
                name_en: "Gekka (Short Version)".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "gekka-1".to_string(),
            ChartData { 
                name_en: "Gekka (Short Version)".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "gekka-2".to_string(),
            ChartData { 
                name_en: "Gekka (Short Version)".to_string(),
                name_jp: None,
                rating: 8.6
            }
            );
            map.insert(
            "vividtheory-0".to_string(),
            ChartData { 
                name_en: "Vivid Theory".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "vividtheory-1".to_string(),
            ChartData { 
                name_en: "Vivid Theory".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "vividtheory-2".to_string(),
            ChartData { 
                name_en: "Vivid Theory".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "onefr-0".to_string(),
            ChartData { 
                name_en: "1F√".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "onefr-1".to_string(),
            ChartData { 
                name_en: "1F√".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "onefr-2".to_string(),
            ChartData { 
                name_en: "1F√".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "scarletcage-0".to_string(),
            ChartData { 
                name_en: "Scarlet Cage".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "scarletcage-1".to_string(),
            ChartData { 
                name_en: "Scarlet Cage".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "scarletcage-2".to_string(),
            ChartData { 
                name_en: "Scarlet Cage".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "faintlight-0".to_string(),
            ChartData { 
                name_en: "Faint Light (Arcaea Edit)".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "faintlight-1".to_string(),
            ChartData { 
                name_en: "Faint Light (Arcaea Edit)".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "faintlight-2".to_string(),
            ChartData { 
                name_en: "Faint Light (Arcaea Edit)".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "feelssoright-0".to_string(),
            ChartData { 
                name_en: "Feels So Right feat. Renko".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "feelssoright-1".to_string(),
            ChartData { 
                name_en: "Feels So Right feat. Renko".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "feelssoright-2".to_string(),
            ChartData { 
                name_en: "Feels So Right feat. Renko".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "teriqma-0".to_string(),
            ChartData { 
                name_en: "Teriqma".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "teriqma-1".to_string(),
            ChartData { 
                name_en: "Teriqma".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "teriqma-2".to_string(),
            ChartData { 
                name_en: "Teriqma".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "mahoroba-0".to_string(),
            ChartData { 
                name_en: "MAHOROBA".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "mahoroba-1".to_string(),
            ChartData { 
                name_en: "MAHOROBA".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "mahoroba-2".to_string(),
            ChartData { 
                name_en: "MAHOROBA".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "badtek-0".to_string(),
            ChartData { 
                name_en: "BADTEK".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "badtek-1".to_string(),
            ChartData { 
                name_en: "BADTEK".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "badtek-2".to_string(),
            ChartData { 
                name_en: "BADTEK".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "maliciousmischance-0".to_string(),
            ChartData { 
                name_en: "Malicious Mischance".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "maliciousmischance-1".to_string(),
            ChartData { 
                name_en: "Malicious Mischance".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "maliciousmischance-2".to_string(),
            ChartData { 
                name_en: "Malicious Mischance".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "gothiveofra-0".to_string(),
            ChartData { 
                name_en: "Got hive of Ra".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "gothiveofra-1".to_string(),
            ChartData { 
                name_en: "Got hive of Ra".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "gothiveofra-2".to_string(),
            ChartData { 
                name_en: "Got hive of Ra".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "buchigireberserker-0".to_string(),
            ChartData { 
                name_en: "BUCHiGiRE Berserker".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "buchigireberserker-1".to_string(),
            ChartData { 
                name_en: "BUCHiGiRE Berserker".to_string(),
                name_jp: None,
                rating: 8.6
            }
            );
            map.insert(
            "buchigireberserker-2".to_string(),
            ChartData { 
                name_en: "BUCHiGiRE Berserker".to_string(),
                name_jp: None,
                rating: 10.9
            }
            );
            map.insert(
            "galaxyfriends-0".to_string(),
            ChartData { 
                name_en: "Galaxy Friends".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "galaxyfriends-1".to_string(),
            ChartData { 
                name_en: "Galaxy Friends".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "galaxyfriends-2".to_string(),
            ChartData { 
                name_en: "Galaxy Friends".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "crossover-0".to_string(),
            ChartData { 
                name_en: "CROSS†OVER".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "crossover-1".to_string(),
            ChartData { 
                name_en: "CROSS†OVER".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "crossover-2".to_string(),
            ChartData { 
                name_en: "CROSS†OVER".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "xeraphinite-0".to_string(),
            ChartData { 
                name_en: "Xeraphinite".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "xeraphinite-1".to_string(),
            ChartData { 
                name_en: "Xeraphinite".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "xeraphinite-2".to_string(),
            ChartData { 
                name_en: "Xeraphinite".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "lapis-0".to_string(),
            ChartData { 
                name_en: "Lapis".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "lapis-1".to_string(),
            ChartData { 
                name_en: "Lapis".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "lapis-2".to_string(),
            ChartData { 
                name_en: "Lapis".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "xanatos-0".to_string(),
            ChartData { 
                name_en: "Xanatos".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "xanatos-1".to_string(),
            ChartData { 
                name_en: "Xanatos".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "xanatos-2".to_string(),
            ChartData { 
                name_en: "Xanatos".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "purpleverse-0".to_string(),
            ChartData { 
                name_en: "Purple Verse".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "purpleverse-1".to_string(),
            ChartData { 
                name_en: "Purple Verse".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "purpleverse-2".to_string(),
            ChartData { 
                name_en: "Purple Verse".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "alicealamode-0".to_string(),
            ChartData { 
                name_en: "Alice à la mode".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "alicealamode-1".to_string(),
            ChartData { 
                name_en: "Alice à la mode".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "alicealamode-2".to_string(),
            ChartData { 
                name_en: "Alice à la mode".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "eccentrictale-0".to_string(),
            ChartData { 
                name_en: "Eccentric Tale".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "eccentrictale-1".to_string(),
            ChartData { 
                name_en: "Eccentric Tale".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "eccentrictale-2".to_string(),
            ChartData { 
                name_en: "Eccentric Tale".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "alicessuitcase-0".to_string(),
            ChartData { 
                name_en: "Alice's Suitcase".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "alicessuitcase-1".to_string(),
            ChartData { 
                name_en: "Alice's Suitcase".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "alicessuitcase-2".to_string(),
            ChartData { 
                name_en: "Alice's Suitcase".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "jump-0".to_string(),
            ChartData { 
                name_en: "Jump".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "jump-1".to_string(),
            ChartData { 
                name_en: "Jump".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "jump-2".to_string(),
            ChartData { 
                name_en: "Jump".to_string(),
                name_jp: None,
                rating: 9.0
            }
            );
            map.insert(
            "felis-0".to_string(),
            ChartData { 
                name_en: "Felis".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "felis-1".to_string(),
            ChartData { 
                name_en: "Felis".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "felis-2".to_string(),
            ChartData { 
                name_en: "Felis".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "besideyou-0".to_string(),
            ChartData { 
                name_en: "Beside You".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "besideyou-1".to_string(),
            ChartData { 
                name_en: "Beside You".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "besideyou-2".to_string(),
            ChartData { 
                name_en: "Beside You".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "heartjackin-0".to_string(),
            ChartData { 
                name_en: "Heart Jackin'".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "heartjackin-1".to_string(),
            ChartData { 
                name_en: "Heart Jackin'".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "heartjackin-2".to_string(),
            ChartData { 
                name_en: "Heart Jackin'".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "toaliceliddell-0".to_string(),
            ChartData { 
                name_en: "To: Alice Liddell".to_string(),
                name_jp: Some("アリス・リデルに捧ぐ".to_string()),
                rating: 4.0
            }
            );
            map.insert(
            "toaliceliddell-1".to_string(),
            ChartData { 
                name_en: "To: Alice Liddell".to_string(),
                name_jp: Some("アリス・リデルに捧ぐ".to_string()),
                rating: 7.0
            }
            );
            map.insert(
            "toaliceliddell-2".to_string(),
            ChartData { 
                name_en: "To: Alice Liddell".to_string(),
                name_jp: Some("アリス・リデルに捧ぐ".to_string()),
                rating: 10.3
            }
            );
            map.insert(
            "lazyaddiction-0".to_string(),
            ChartData { 
                name_en: "Lazy Addiction".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "lazyaddiction-1".to_string(),
            ChartData { 
                name_en: "Lazy Addiction".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "lazyaddiction-2".to_string(),
            ChartData { 
                name_en: "Lazy Addiction".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "dazzlehop-0".to_string(),
            ChartData { 
                name_en: "Dazzle hop".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "dazzlehop-1".to_string(),
            ChartData { 
                name_en: "Dazzle hop".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "dazzlehop-2".to_string(),
            ChartData { 
                name_en: "Dazzle hop".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "viyellastears-0".to_string(),
            ChartData { 
                name_en: "Viyella's Tears".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "viyellastears-1".to_string(),
            ChartData { 
                name_en: "Viyella's Tears".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "viyellastears-2".to_string(),
            ChartData { 
                name_en: "Viyella's Tears".to_string(),
                name_jp: None,
                rating: 10.3
            }
            );
            map.insert(
            "omegafour-0".to_string(),
            ChartData { 
                name_en: "ω4".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "omegafour-1".to_string(),
            ChartData { 
                name_en: "ω4".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "omegafour-2".to_string(),
            ChartData { 
                name_en: "ω4".to_string(),
                name_jp: None,
                rating: 10.8
            }
            );
            map.insert(
            "aprilshowers-0".to_string(),
            ChartData { 
                name_en: "April showers".to_string(),
                name_jp: Some("四月の雨".to_string()),
                rating: 2.0
            }
            );
            map.insert(
            "aprilshowers-1".to_string(),
            ChartData { 
                name_en: "April showers".to_string(),
                name_jp: Some("四月の雨".to_string()),
                rating: 5.5
            }
            );
            map.insert(
            "aprilshowers-2".to_string(),
            ChartData { 
                name_en: "April showers".to_string(),
                name_jp: Some("四月の雨".to_string()),
                rating: 8.3
            }
            );
            map.insert(
            "seventhsense-0".to_string(),
            ChartData { 
                name_en: "7thSense".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "seventhsense-1".to_string(),
            ChartData { 
                name_en: "7thSense".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "seventhsense-2".to_string(),
            ChartData { 
                name_en: "7thSense".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "oshamascramble-0".to_string(),
            ChartData { 
                name_en: "Oshama Scramble!".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "oshamascramble-1".to_string(),
            ChartData { 
                name_en: "Oshama Scramble!".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "oshamascramble-2".to_string(),
            ChartData { 
                name_en: "Oshama Scramble!".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "amazingmightyyyy-0".to_string(),
            ChartData { 
                name_en: "AMAZING MIGHTYYYY!!!!".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "amazingmightyyyy-1".to_string(),
            ChartData { 
                name_en: "AMAZING MIGHTYYYY!!!!".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "amazingmightyyyy-2".to_string(),
            ChartData { 
                name_en: "AMAZING MIGHTYYYY!!!!".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "climax-0".to_string(),
            ChartData { 
                name_en: "Climax".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "climax-1".to_string(),
            ChartData { 
                name_en: "Climax".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "climax-2".to_string(),
            ChartData { 
                name_en: "Climax".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "lastcelebration-0".to_string(),
            ChartData { 
                name_en: "Last Celebration".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "lastcelebration-1".to_string(),
            ChartData { 
                name_en: "Last Celebration".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "lastcelebration-2".to_string(),
            ChartData { 
                name_en: "Last Celebration".to_string(),
                name_jp: None,
                rating: 10.5
            }
            );
            map.insert(
            "gou-0".to_string(),
            ChartData { 
                name_en: "Misdeed -la bonté de Dieu et l'origine du mal-".to_string(),
                name_jp: Some("業 -善なる神とこの世の悪について-".to_string()),
                rating: 5.5
            }
            );
            map.insert(
            "gou-1".to_string(),
            ChartData { 
                name_en: "Misdeed -la bonté de Dieu et l'origine du mal-".to_string(),
                name_jp: Some("業 -善なる神とこの世の悪について-".to_string()),
                rating: 8.5
            }
            );
            map.insert(
            "gou-2".to_string(),
            ChartData { 
                name_en: "Misdeed -la bonté de Dieu et l'origine du mal-".to_string(),
                name_jp: Some("業 -善なる神とこの世の悪について-".to_string()),
                rating: 10.9
            }
            );
            map.insert(
            "glow-0".to_string(),
            ChartData { 
                name_en: "Glow".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "glow-1".to_string(),
            ChartData { 
                name_en: "Glow".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "glow-2".to_string(),
            ChartData { 
                name_en: "Glow".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "attraqtia-0".to_string(),
            ChartData { 
                name_en: "AttraqtiA".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "attraqtia-1".to_string(),
            ChartData { 
                name_en: "AttraqtiA".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "attraqtia-2".to_string(),
            ChartData { 
                name_en: "AttraqtiA".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "enchantedlove-0".to_string(),
            ChartData { 
                name_en: "enchanted love".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "enchantedlove-1".to_string(),
            ChartData { 
                name_en: "enchanted love".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "enchantedlove-2".to_string(),
            ChartData { 
                name_en: "enchanted love".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "take-0".to_string(),
            ChartData { 
                name_en: "Bamboo".to_string(),
                name_jp: Some("竹".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "take-1".to_string(),
            ChartData { 
                name_en: "Bamboo".to_string(),
                name_jp: Some("竹".to_string()),
                rating: 6.5
            }
            );
            map.insert(
            "take-2".to_string(),
            ChartData { 
                name_en: "Bamboo".to_string(),
                name_jp: Some("竹".to_string()),
                rating: 10.0
            }
            );
            map.insert(
            "gimmedablood-0".to_string(),
            ChartData { 
                name_en: "GIMME DA BLOOD".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "gimmedablood-1".to_string(),
            ChartData { 
                name_en: "GIMME DA BLOOD".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "gimmedablood-2".to_string(),
            ChartData { 
                name_en: "GIMME DA BLOOD".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "bassline-0".to_string(),
            ChartData { 
                name_en: "Can I Friend You on Bassbook? Lol".to_string(),
                name_jp: Some("ベースラインやってる？ｗ".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "bassline-1".to_string(),
            ChartData { 
                name_en: "Can I Friend You on Bassbook? Lol".to_string(),
                name_jp: Some("ベースラインやってる？ｗ".to_string()),
                rating: 6.5
            }
            );
            map.insert(
            "bassline-2".to_string(),
            ChartData { 
                name_en: "Can I Friend You on Bassbook? Lol".to_string(),
                name_jp: Some("ベースラインやってる？ｗ".to_string()),
                rating: 9.3
            }
            );
            map.insert(
            "lifeispiano-0".to_string(),
            ChartData { 
                name_en: "Life is PIANO".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "lifeispiano-1".to_string(),
            ChartData { 
                name_en: "Life is PIANO".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "lifeispiano-2".to_string(),
            ChartData { 
                name_en: "Life is PIANO".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "paperwitch-0".to_string(),
            ChartData { 
                name_en: "Paper Witch".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "paperwitch-1".to_string(),
            ChartData { 
                name_en: "Paper Witch".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "paperwitch-2".to_string(),
            ChartData { 
                name_en: "Paper Witch".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "crystalgravity-0".to_string(),
            ChartData { 
                name_en: "Crystal Gravity".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "crystalgravity-1".to_string(),
            ChartData { 
                name_en: "Crystal Gravity".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "crystalgravity-2".to_string(),
            ChartData { 
                name_en: "Crystal Gravity".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "farawaylight-0".to_string(),
            ChartData { 
                name_en: "Far Away Light".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "farawaylight-1".to_string(),
            ChartData { 
                name_en: "Far Away Light".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "farawaylight-2".to_string(),
            ChartData { 
                name_en: "Far Away Light".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "loschen-0".to_string(),
            ChartData { 
                name_en: "Löschen".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "loschen-1".to_string(),
            ChartData { 
                name_en: "Löschen".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "loschen-2".to_string(),
            ChartData { 
                name_en: "Löschen".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "aegleseeker-0".to_string(),
            ChartData { 
                name_en: "Aegleseeker".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "aegleseeker-1".to_string(),
            ChartData { 
                name_en: "Aegleseeker".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "aegleseeker-2".to_string(),
            ChartData { 
                name_en: "Aegleseeker".to_string(),
                name_jp: None,
                rating: 11.1
            }
            );
            map.insert(
            "coastalhighway-0".to_string(),
            ChartData { 
                name_en: "Coastal Highway".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "coastalhighway-1".to_string(),
            ChartData { 
                name_en: "Coastal Highway".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "coastalhighway-2".to_string(),
            ChartData { 
                name_en: "Coastal Highway".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "odysseia-0".to_string(),
            ChartData { 
                name_en: "ΟΔΥΣΣΕΙΑ".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "odysseia-1".to_string(),
            ChartData { 
                name_en: "ΟΔΥΣΣΕΙΑ".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "odysseia-2".to_string(),
            ChartData { 
                name_en: "ΟΔΥΣΣΕΙΑ".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "overwhelm-0".to_string(),
            ChartData { 
                name_en: "Overwhelm".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "overwhelm-1".to_string(),
            ChartData { 
                name_en: "Overwhelm".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "overwhelm-2".to_string(),
            ChartData { 
                name_en: "Overwhelm".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "vandalism-0".to_string(),
            ChartData { 
                name_en: "Vandalism".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "vandalism-1".to_string(),
            ChartData { 
                name_en: "Vandalism".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "vandalism-2".to_string(),
            ChartData { 
                name_en: "Vandalism".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "turbocharger-0".to_string(),
            ChartData { 
                name_en: "Turbocharger".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "turbocharger-1".to_string(),
            ChartData { 
                name_en: "Turbocharger".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "turbocharger-2".to_string(),
            ChartData { 
                name_en: "Turbocharger".to_string(),
                name_jp: None,
                rating: 9.0
            }
            );
            map.insert(
            "theultimacy-0".to_string(),
            ChartData { 
                name_en: "THE ULTIMACY".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "theultimacy-1".to_string(),
            ChartData { 
                name_en: "THE ULTIMACY".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "theultimacy-2".to_string(),
            ChartData { 
                name_en: "THE ULTIMACY".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "rekkaresonance-0".to_string(),
            ChartData { 
                name_en: "REKKA RESONANCE".to_string(),
                name_jp: Some("烈華RESONANCE".to_string()),
                rating: 5.0
            }
            );
            map.insert(
            "rekkaresonance-1".to_string(),
            ChartData { 
                name_en: "REKKA RESONANCE".to_string(),
                name_jp: Some("烈華RESONANCE".to_string()),
                rating: 8.3
            }
            );
            map.insert(
            "rekkaresonance-2".to_string(),
            ChartData { 
                name_en: "REKKA RESONANCE".to_string(),
                name_jp: Some("烈華RESONANCE".to_string()),
                rating: 10.7
            }
            );
            map.insert(
            "kyogenkigo-0".to_string(),
            ChartData { 
                name_en: "False Embellishment".to_string(),
                name_jp: Some("狂言綺語".to_string()),
                rating: 3.5
            }
            );
            map.insert(
            "kyogenkigo-1".to_string(),
            ChartData { 
                name_en: "False Embellishment".to_string(),
                name_jp: Some("狂言綺語".to_string()),
                rating: 6.5
            }
            );
            map.insert(
            "kyogenkigo-2".to_string(),
            ChartData { 
                name_en: "False Embellishment".to_string(),
                name_jp: Some("狂言綺語".to_string()),
                rating: 9.3
            }
            );
            map.insert(
            "hivemind-0".to_string(),
            ChartData { 
                name_en: "HIVEMIND".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "hivemind-1".to_string(),
            ChartData { 
                name_en: "HIVEMIND".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "hivemind-2".to_string(),
            ChartData { 
                name_en: "HIVEMIND".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "seclusion-0".to_string(),
            ChartData { 
                name_en: "Seclusion".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "seclusion-1".to_string(),
            ChartData { 
                name_en: "Seclusion".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "seclusion-2".to_string(),
            ChartData { 
                name_en: "Seclusion".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "smallcloud-0".to_string(),
            ChartData { 
                name_en: "Small Cloud Sugar Candy".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "smallcloud-1".to_string(),
            ChartData { 
                name_en: "Small Cloud Sugar Candy".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "smallcloud-2".to_string(),
            ChartData { 
                name_en: "Small Cloud Sugar Candy".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "alterale-0".to_string(),
            ChartData { 
                name_en: "AlterAle".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "alterale-1".to_string(),
            ChartData { 
                name_en: "AlterAle".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "alterale-2".to_string(),
            ChartData { 
                name_en: "AlterAle".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "divinelight-0".to_string(),
            ChartData { 
                name_en: "Divine Light of Myriad".to_string(),
                name_jp: Some("光速神授説 - Divine Light of Myriad -".to_string()),
                rating: 4.5
            }
            );
            map.insert(
            "divinelight-1".to_string(),
            ChartData { 
                name_en: "Divine Light of Myriad".to_string(),
                name_jp: Some("光速神授説 - Divine Light of Myriad -".to_string()),
                rating: 7.5
            }
            );
            map.insert(
            "divinelight-2".to_string(),
            ChartData { 
                name_en: "Divine Light of Myriad".to_string(),
                name_jp: Some("光速神授説 - Divine Light of Myriad -".to_string()),
                rating: 10.8
            }
            );
            map.insert(
            "mazymetroplex-0".to_string(),
            ChartData { 
                name_en: "Mazy Metroplex".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "mazymetroplex-1".to_string(),
            ChartData { 
                name_en: "Mazy Metroplex".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "mazymetroplex-2".to_string(),
            ChartData { 
                name_en: "Mazy Metroplex".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "quonwacca-0".to_string(),
            ChartData { 
                name_en: "Quon".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "quonwacca-1".to_string(),
            ChartData { 
                name_en: "Quon".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "quonwacca-2".to_string(),
            ChartData { 
                name_en: "Quon".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "quonwacca-3".to_string(),
            ChartData { 
                name_en: "Quon".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "withu-0".to_string(),
            ChartData { 
                name_en: "with U".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "withu-1".to_string(),
            ChartData { 
                name_en: "with U".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "withu-2".to_string(),
            ChartData { 
                name_en: "with U".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "genocider-0".to_string(),
            ChartData { 
                name_en: "GENOCIDER".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "genocider-1".to_string(),
            ChartData { 
                name_en: "GENOCIDER".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "genocider-2".to_string(),
            ChartData { 
                name_en: "GENOCIDER".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "letyoudivermx-0".to_string(),
            ChartData { 
                name_en: "Let you DIVE! (nitro rmx)".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "letyoudivermx-1".to_string(),
            ChartData { 
                name_en: "Let you DIVE! (nitro rmx)".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "letyoudivermx-2".to_string(),
            ChartData { 
                name_en: "Let you DIVE! (nitro rmx)".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "sheriruthrmx-0".to_string(),
            ChartData { 
                name_en: "Sheriruth (Laur Remix)".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "sheriruthrmx-1".to_string(),
            ChartData { 
                name_en: "Sheriruth (Laur Remix)".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "sheriruthrmx-2".to_string(),
            ChartData { 
                name_en: "Sheriruth (Laur Remix)".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "eveninginscarlet-0".to_string(),
            ChartData { 
                name_en: "Evening in Scarlet".to_string(),
                name_jp: Some("緋纏いの宵".to_string()),
                rating: 4.0
            }
            );
            map.insert(
            "eveninginscarlet-1".to_string(),
            ChartData { 
                name_en: "Evening in Scarlet".to_string(),
                name_jp: Some("緋纏いの宵".to_string()),
                rating: 7.0
            }
            );
            map.insert(
            "eveninginscarlet-2".to_string(),
            ChartData { 
                name_en: "Evening in Scarlet".to_string(),
                name_jp: Some("緋纏いの宵".to_string()),
                rating: 9.5
            }
            );
            map.insert(
            "bluecomet-0".to_string(),
            ChartData { 
                name_en: "blue comet".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "bluecomet-1".to_string(),
            ChartData { 
                name_en: "blue comet".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "bluecomet-2".to_string(),
            ChartData { 
                name_en: "blue comet".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "energysynergymatrix-0".to_string(),
            ChartData { 
                name_en: "ENERGY SYNERGY MATRIX".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "energysynergymatrix-1".to_string(),
            ChartData { 
                name_en: "ENERGY SYNERGY MATRIX".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "energysynergymatrix-2".to_string(),
            ChartData { 
                name_en: "ENERGY SYNERGY MATRIX".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "gengaozo-0".to_string(),
            ChartData { 
                name_en: "G e n g a o z o".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "gengaozo-1".to_string(),
            ChartData { 
                name_en: "G e n g a o z o".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "gengaozo-2".to_string(),
            ChartData { 
                name_en: "G e n g a o z o".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "goldenslaughterer-0".to_string(),
            ChartData { 
                name_en: "goldenslaughterer".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "goldenslaughterer-1".to_string(),
            ChartData { 
                name_en: "goldenslaughterer".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "goldenslaughterer-2".to_string(),
            ChartData { 
                name_en: "goldenslaughterer".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "lastendconductor-0".to_string(),
            ChartData { 
                name_en: "lastendconductor".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "lastendconductor-1".to_string(),
            ChartData { 
                name_en: "lastendconductor".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "lastendconductor-2".to_string(),
            ChartData { 
                name_en: "lastendconductor".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "lastendconductor-3".to_string(),
            ChartData { 
                name_en: "lastendconductor".to_string(),
                name_jp: None,
                rating: 10.1
            }
            );
            map.insert(
            "redolentshape-0".to_string(),
            ChartData { 
                name_en: "Redolent Shape".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "redolentshape-1".to_string(),
            ChartData { 
                name_en: "Redolent Shape".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "redolentshape-2".to_string(),
            ChartData { 
                name_en: "Redolent Shape".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "cosmica-0".to_string(),
            ChartData { 
                name_en: "Cosmica".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "cosmica-1".to_string(),
            ChartData { 
                name_en: "Cosmica".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "cosmica-2".to_string(),
            ChartData { 
                name_en: "Cosmica".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "ascent-0".to_string(),
            ChartData { 
                name_en: "Ascent".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "ascent-1".to_string(),
            ChartData { 
                name_en: "Ascent".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "ascent-2".to_string(),
            ChartData { 
                name_en: "Ascent".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "livefastdieyoung-0".to_string(),
            ChartData { 
                name_en: "Live Fast Die Young".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "livefastdieyoung-1".to_string(),
            ChartData { 
                name_en: "Live Fast Die Young".to_string(),
                name_jp: None,
                rating: 8.1
            }
            );
            map.insert(
            "livefastdieyoung-2".to_string(),
            ChartData { 
                name_en: "Live Fast Die Young".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "summerfireworks-0".to_string(),
            ChartData { 
                name_en: "Summer Fireworks of Love".to_string(),
                name_jp: Some("彩る夏の恋花火".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "summerfireworks-1".to_string(),
            ChartData { 
                name_en: "Summer Fireworks of Love".to_string(),
                name_jp: Some("彩る夏の恋花火".to_string()),
                rating: 6.5
            }
            );
            map.insert(
            "summerfireworks-2".to_string(),
            ChartData { 
                name_en: "Summer Fireworks of Love".to_string(),
                name_jp: Some("彩る夏の恋花火".to_string()),
                rating: 9.9
            }
            );
            map.insert(
            "firstsnow-0".to_string(),
            ChartData { 
                name_en: "First Snow".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "firstsnow-1".to_string(),
            ChartData { 
                name_en: "First Snow".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "firstsnow-2".to_string(),
            ChartData { 
                name_en: "First Snow".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "bluerose-0".to_string(),
            ChartData { 
                name_en: "Blue Rose".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "bluerose-1".to_string(),
            ChartData { 
                name_en: "Blue Rose".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "bluerose-2".to_string(),
            ChartData { 
                name_en: "Blue Rose".to_string(),
                name_jp: None,
                rating: 9.1
            }
            );
            map.insert(
            "blockedlibrary-0".to_string(),
            ChartData { 
                name_en: "Blocked Library".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "blockedlibrary-1".to_string(),
            ChartData { 
                name_en: "Blocked Library".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "blockedlibrary-2".to_string(),
            ChartData { 
                name_en: "Blocked Library".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "neokosmo-0".to_string(),
            ChartData { 
                name_en: "nέο κόsmo".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "neokosmo-1".to_string(),
            ChartData { 
                name_en: "nέο κόsmo".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "neokosmo-2".to_string(),
            ChartData { 
                name_en: "nέο κόsmo".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "lightningscrew-0".to_string(),
            ChartData { 
                name_en: "Lightning Screw".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "lightningscrew-1".to_string(),
            ChartData { 
                name_en: "Lightning Screw".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "lightningscrew-2".to_string(),
            ChartData { 
                name_en: "Lightning Screw".to_string(),
                name_jp: None,
                rating: 10.5
            }
            );
            map.insert(
            "lightsofmuse-0".to_string(),
            ChartData { 
                name_en: "Lights of Muse".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "lightsofmuse-1".to_string(),
            ChartData { 
                name_en: "Lights of Muse".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "lightsofmuse-2".to_string(),
            ChartData { 
                name_en: "Lights of Muse".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "finalstep-0".to_string(),
            ChartData { 
                name_en: "Final Step!".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "finalstep-1".to_string(),
            ChartData { 
                name_en: "Final Step!".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "finalstep-2".to_string(),
            ChartData { 
                name_en: "Final Step!".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "akinokagerou-0".to_string(),
            ChartData { 
                name_en: "Haze of Autumn".to_string(),
                name_jp: Some("秋の陽炎".to_string()),
                rating: 3.5
            }
            );
            map.insert(
            "akinokagerou-1".to_string(),
            ChartData { 
                name_en: "Haze of Autumn".to_string(),
                name_jp: Some("秋の陽炎".to_string()),
                rating: 7.0
            }
            );
            map.insert(
            "akinokagerou-2".to_string(),
            ChartData { 
                name_en: "Haze of Autumn".to_string(),
                name_jp: Some("秋の陽炎".to_string()),
                rating: 9.8
            }
            );
            map.insert(
            "medusa-0".to_string(),
            ChartData { 
                name_en: "Medusa".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "medusa-1".to_string(),
            ChartData { 
                name_en: "Medusa".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "medusa-2".to_string(),
            ChartData { 
                name_en: "Medusa".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "init-0".to_string(),
            ChartData { 
                name_en: "init()".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "init-1".to_string(),
            ChartData { 
                name_en: "init()".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "init-2".to_string(),
            ChartData { 
                name_en: "init()".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "internetoverdose-0".to_string(),
            ChartData { 
                name_en: "INTERNET OVERDOSE".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "internetoverdose-1".to_string(),
            ChartData { 
                name_en: "INTERNET OVERDOSE".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "internetoverdose-2".to_string(),
            ChartData { 
                name_en: "INTERNET OVERDOSE".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "sakurafubuki-0".to_string(),
            ChartData { 
                name_en: "Sakura Fubuki".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "sakurafubuki-1".to_string(),
            ChartData { 
                name_en: "Sakura Fubuki".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "sakurafubuki-2".to_string(),
            ChartData { 
                name_en: "Sakura Fubuki".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "nulctrl-0".to_string(),
            ChartData { 
                name_en: "NULCTRL".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "nulctrl-1".to_string(),
            ChartData { 
                name_en: "NULCTRL".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "nulctrl-2".to_string(),
            ChartData { 
                name_en: "NULCTRL".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "macromod-0".to_string(),
            ChartData { 
                name_en: "Macrocosmic Modulation".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "macromod-1".to_string(),
            ChartData { 
                name_en: "Macrocosmic Modulation".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "macromod-2".to_string(),
            ChartData { 
                name_en: "Macrocosmic Modulation".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "neowings-0".to_string(),
            ChartData { 
                name_en: "NEO WINGS".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "neowings-1".to_string(),
            ChartData { 
                name_en: "NEO WINGS".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "neowings-2".to_string(),
            ChartData { 
                name_en: "NEO WINGS".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "kissinglucifer-0".to_string(),
            ChartData { 
                name_en: "Kissing Lucifer".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "kissinglucifer-1".to_string(),
            ChartData { 
                name_en: "Kissing Lucifer".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "kissinglucifer-2".to_string(),
            ChartData { 
                name_en: "Kissing Lucifer".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "avril-0".to_string(),
            ChartData { 
                name_en: "Ävril -Flicka i krans-".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "avril-1".to_string(),
            ChartData { 
                name_en: "Ävril -Flicka i krans-".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "avril-2".to_string(),
            ChartData { 
                name_en: "Ävril -Flicka i krans-".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "aurgelmir-0".to_string(),
            ChartData { 
                name_en: "Aurgelmir".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "aurgelmir-1".to_string(),
            ChartData { 
                name_en: "Aurgelmir".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "aurgelmir-2".to_string(),
            ChartData { 
                name_en: "Aurgelmir".to_string(),
                name_jp: None,
                rating: 10.5
            }
            );
            map.insert(
            "headbonkache-0".to_string(),
            ChartData { 
                name_en: "Head BONK ache".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "headbonkache-1".to_string(),
            ChartData { 
                name_en: "Head BONK ache".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "headbonkache-2".to_string(),
            ChartData { 
                name_en: "Head BONK ache".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "ddd-0".to_string(),
            ChartData { 
                name_en: "DDD".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "ddd-1".to_string(),
            ChartData { 
                name_en: "DDD".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "ddd-2".to_string(),
            ChartData { 
                name_en: "DDD".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "prism-0".to_string(),
            ChartData { 
                name_en: "Prism".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "prism-1".to_string(),
            ChartData { 
                name_en: "Prism".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "prism-2".to_string(),
            ChartData { 
                name_en: "Prism".to_string(),
                name_jp: None,
                rating: 8.0
            }
            );
            map.insert(
            "protoflicker-0".to_string(),
            ChartData { 
                name_en: "Protoflicker".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "protoflicker-1".to_string(),
            ChartData { 
                name_en: "Protoflicker".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "protoflicker-2".to_string(),
            ChartData { 
                name_en: "Protoflicker".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "stasis-0".to_string(),
            ChartData { 
                name_en: "Stasis".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "stasis-1".to_string(),
            ChartData { 
                name_en: "Stasis".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "stasis-2".to_string(),
            ChartData { 
                name_en: "Stasis".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "picopicotranslation-0".to_string(),
            ChartData { 
                name_en: "PICO-Pico-Translation!".to_string(),
                name_jp: Some("ピコPico*とらんすれーしょんっ！".to_string()),
                rating: 2.0
            }
            );
            map.insert(
            "picopicotranslation-1".to_string(),
            ChartData { 
                name_en: "PICO-Pico-Translation!".to_string(),
                name_jp: Some("ピコPico*とらんすれーしょんっ！".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "picopicotranslation-2".to_string(),
            ChartData { 
                name_en: "PICO-Pico-Translation!".to_string(),
                name_jp: Some("ピコPico*とらんすれーしょんっ！".to_string()),
                rating: 9.3
            }
            );
            map.insert(
            "nekonote-0".to_string(),
            ChartData { 
                name_en: "Dancin' on a Cat's Paw".to_string(),
                name_jp: Some("ネコノテ・カリタガール".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "nekonote-1".to_string(),
            ChartData { 
                name_en: "Dancin' on a Cat's Paw".to_string(),
                name_jp: Some("ネコノテ・カリタガール".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "nekonote-2".to_string(),
            ChartData { 
                name_en: "Dancin' on a Cat's Paw".to_string(),
                name_jp: Some("ネコノテ・カリタガール".to_string()),
                rating: 9.2
            }
            );
            map.insert(
            "mu-0".to_string(),
            ChartData { 
                name_en: "μ".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "mu-1".to_string(),
            ChartData { 
                name_en: "μ".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "mu-2".to_string(),
            ChartData { 
                name_en: "μ".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "sanskia-0".to_string(),
            ChartData { 
                name_en: "san skia".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "sanskia-1".to_string(),
            ChartData { 
                name_en: "san skia".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "sanskia-2".to_string(),
            ChartData { 
                name_en: "san skia".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "altair-0".to_string(),
            ChartData { 
                name_en: "Altair (feat. *spiLa*)".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "altair-1".to_string(),
            ChartData { 
                name_en: "Altair (feat. *spiLa*)".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "altair-2".to_string(),
            ChartData { 
                name_en: "Altair (feat. *spiLa*)".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "mukishitsu-0".to_string(),
            ChartData { 
                name_en: "Redraw the Colorless World".to_string(),
                name_jp: Some("無機質世界に彩を".to_string()),
                rating: 4.0
            }
            );
            map.insert(
            "mukishitsu-1".to_string(),
            ChartData { 
                name_en: "Redraw the Colorless World".to_string(),
                name_jp: Some("無機質世界に彩を".to_string()),
                rating: 6.5
            }
            );
            map.insert(
            "mukishitsu-2".to_string(),
            ChartData { 
                name_en: "Redraw the Colorless World".to_string(),
                name_jp: Some("無機質世界に彩を".to_string()),
                rating: 9.2
            }
            );
            map.insert(
            "trapcrow-0".to_string(),
            ChartData { 
                name_en: "Trap Crow".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "trapcrow-1".to_string(),
            ChartData { 
                name_en: "Trap Crow".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "trapcrow-2".to_string(),
            ChartData { 
                name_en: "Trap Crow".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "pupa-0".to_string(),
            ChartData { 
                name_en: "PUPA".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "pupa-1".to_string(),
            ChartData { 
                name_en: "PUPA".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "pupa-2".to_string(),
            ChartData { 
                name_en: "PUPA".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "defection-0".to_string(),
            ChartData { 
                name_en: "Defection".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "defection-1".to_string(),
            ChartData { 
                name_en: "Defection".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "defection-2".to_string(),
            ChartData { 
                name_en: "Defection".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "infinitestrife-0".to_string(),
            ChartData { 
                name_en: "Infinite Strife,".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "infinitestrife-1".to_string(),
            ChartData { 
                name_en: "Infinite Strife,".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "infinitestrife-2".to_string(),
            ChartData { 
                name_en: "Infinite Strife,".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "infinitestrife-3".to_string(),
            ChartData { 
                name_en: "Infinite Strife,".to_string(),
                name_jp: None,
                rating: 10.9
            }
            );
            map.insert(
            "worldender-0".to_string(),
            ChartData { 
                name_en: "World Ender".to_string(),
                name_jp: Some("魔王".to_string()),
                rating: 4.0
            }
            );
            map.insert(
            "worldender-1".to_string(),
            ChartData { 
                name_en: "World Ender".to_string(),
                name_jp: Some("魔王".to_string()),
                rating: 7.5
            }
            );
            map.insert(
            "worldender-2".to_string(),
            ChartData { 
                name_en: "World Ender".to_string(),
                name_jp: Some("魔王".to_string()),
                rating: 9.9
            }
            );
            map.insert(
            "worldender-3".to_string(),
            ChartData { 
                name_en: "World Ender".to_string(),
                name_jp: Some("魔王".to_string()),
                rating: 11.2
            }
            );
            map.insert(
            "pentiment-0".to_string(),
            ChartData { 
                name_en: "Pentiment".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "pentiment-1".to_string(),
            ChartData { 
                name_en: "Pentiment".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "pentiment-2".to_string(),
            ChartData { 
                name_en: "Pentiment".to_string(),
                name_jp: None,
                rating: 10.3
            }
            );
            map.insert(
            "pentiment-3".to_string(),
            ChartData { 
                name_en: "Pentiment".to_string(),
                name_jp: None,
                rating: 11.4
            }
            );
            map.insert(
            "arcanaeden-0".to_string(),
            ChartData { 
                name_en: "Arcana Eden".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "arcanaeden-1".to_string(),
            ChartData { 
                name_en: "Arcana Eden".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "arcanaeden-2".to_string(),
            ChartData { 
                name_en: "Arcana Eden".to_string(),
                name_jp: None,
                rating: 10.5
            }
            );
            map.insert(
            "arcanaeden-3".to_string(),
            ChartData { 
                name_en: "Arcana Eden".to_string(),
                name_jp: None,
                rating: 11.4
            }
            );
            map.insert(
            "testify-0".to_string(),
            ChartData { 
                name_en: "Testify".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "testify-1".to_string(),
            ChartData { 
                name_en: "Testify".to_string(),
                name_jp: None,
                rating: 9.4
            }
            );
            map.insert(
            "testify-2".to_string(),
            ChartData { 
                name_en: "Testify".to_string(),
                name_jp: None,
                rating: 10.8
            }
            );
            map.insert(
            "testify-3".to_string(),
            ChartData { 
                name_en: "Testify".to_string(),
                name_jp: None,
                rating: 12.0
            }
            );
            map.insert(
            "lovelessdress-0".to_string(),
            ChartData { 
                name_en: "Loveless Dress".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "lovelessdress-1".to_string(),
            ChartData { 
                name_en: "Loveless Dress".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "lovelessdress-2".to_string(),
            ChartData { 
                name_en: "Loveless Dress".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "last-0".to_string(),
            ChartData { 
                name_en: "Last".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "last-1".to_string(),
            ChartData { 
                name_en: "Last".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "last-2".to_string(),
            ChartData { 
                name_en: "Last".to_string(),
                name_jp: None,
                rating: 9.0
            }
            );
            map.insert(
            "last-3".to_string(),
            ChartData { 
                name_en: "Last | Moment".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "lasteternity-0".to_string(),
            ChartData { 
                name_en: "Last | Eternity".to_string(),
                name_jp: None,
                rating: 0.0
            }
            );
            map.insert(
            "lasteternity-1".to_string(),
            ChartData { 
                name_en: "Last | Eternity".to_string(),
                name_jp: None,
                rating: 0.0
            }
            );
            map.insert(
            "lasteternity-2".to_string(),
            ChartData { 
                name_en: "Last | Eternity".to_string(),
                name_jp: None,
                rating: 0.0
            }
            );
            map.insert(
            "lasteternity-3".to_string(),
            ChartData { 
                name_en: "Last | Eternity".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "callimakarma-0".to_string(),
            ChartData { 
                name_en: "Callima Karma".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "callimakarma-1".to_string(),
            ChartData { 
                name_en: "Callima Karma".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "callimakarma-2".to_string(),
            ChartData { 
                name_en: "Callima Karma".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "kokoro-0".to_string(),
            ChartData { 
                name_en: "Heart".to_string(),
                name_jp: Some("心".to_string()),
                rating: 1.0
            }
            );
            map.insert(
            "kokoro-1".to_string(),
            ChartData { 
                name_en: "Heart".to_string(),
                name_jp: Some("心".to_string()),
                rating: 5.0
            }
            );
            map.insert(
            "kokoro-2".to_string(),
            ChartData { 
                name_en: "Heart".to_string(),
                name_jp: Some("心".to_string()),
                rating: 9.3
            }
            );
            map.insert(
            "aidrew-0".to_string(),
            ChartData { 
                name_en: "Ai Drew".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "aidrew-1".to_string(),
            ChartData { 
                name_en: "Ai Drew".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "aidrew-2".to_string(),
            ChartData { 
                name_en: "Ai Drew".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "fluffyflash-0".to_string(),
            ChartData { 
                name_en: "FLUFFY FLASH".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "fluffyflash-1".to_string(),
            ChartData { 
                name_en: "FLUFFY FLASH".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "fluffyflash-2".to_string(),
            ChartData { 
                name_en: "FLUFFY FLASH".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "goodbyemerry-0".to_string(),
            ChartData { 
                name_en: "Good bye, Merry-Go-Round.".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "goodbyemerry-1".to_string(),
            ChartData { 
                name_en: "Good bye, Merry-Go-Round.".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "goodbyemerry-2".to_string(),
            ChartData { 
                name_en: "Good bye, Merry-Go-Round.".to_string(),
                name_jp: None,
                rating: 10.5
            }
            );
            map.insert(
            "lamia-0".to_string(),
            ChartData { 
                name_en: "LAMIA".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "lamia-1".to_string(),
            ChartData { 
                name_en: "LAMIA".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "lamia-2".to_string(),
            ChartData { 
                name_en: "LAMIA".to_string(),
                name_jp: None,
                rating: 10.9
            }
            );
            map.insert(
            "freefall-3".to_string(),
            ChartData { 
                name_en: "FREEF4LL".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "freemyself-0".to_string(),
            ChartData { 
                name_en: "Free Myself".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "freemyself-1".to_string(),
            ChartData { 
                name_en: "Free Myself".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "freemyself-2".to_string(),
            ChartData { 
                name_en: "Free Myself".to_string(),
                name_jp: None,
                rating: 10.0
            }
            );
            map.insert(
            "cocorocosmetic-0".to_string(),
            ChartData { 
                name_en: "cocoro*cosmetic".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "cocorocosmetic-1".to_string(),
            ChartData { 
                name_en: "cocoro*cosmetic".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "cocorocosmetic-2".to_string(),
            ChartData { 
                name_en: "cocoro*cosmetic".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "partyvinyl-3".to_string(),
            ChartData { 
                name_en: "Party Vinyl".to_string(),
                name_jp: None,
                rating: 10.1
            }
            );
            map.insert(
            "capella-0".to_string(),
            ChartData { 
                name_en: "Capella".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "capella-1".to_string(),
            ChartData { 
                name_en: "Capella".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "capella-2".to_string(),
            ChartData { 
                name_en: "Capella".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "dialnote-0".to_string(),
            ChartData { 
                name_en: "Dialnote".to_string(),
                name_jp: Some("だいあるのーと".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "dialnote-1".to_string(),
            ChartData { 
                name_en: "Dialnote".to_string(),
                name_jp: Some("だいあるのーと".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "dialnote-2".to_string(),
            ChartData { 
                name_en: "Dialnote".to_string(),
                name_jp: Some("だいあるのーと".to_string()),
                rating: 8.1
            }
            );
            map.insert(
            "tsukinimurakumo-0".to_string(),
            ChartData { 
                name_en: "Tsuki ni Murakumo, Hana ni Kaze".to_string(),
                name_jp: Some("月に叢雲華に風".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "tsukinimurakumo-1".to_string(),
            ChartData { 
                name_en: "Tsuki ni Murakumo, Hana ni Kaze".to_string(),
                name_jp: Some("月に叢雲華に風".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "tsukinimurakumo-2".to_string(),
            ChartData { 
                name_en: "Tsuki ni Murakumo, Hana ni Kaze".to_string(),
                name_jp: Some("月に叢雲華に風".to_string()),
                rating: 8.4
            }
            );
            map.insert(
            "mantis-0".to_string(),
            ChartData { 
                name_en: "MANTIS (Arcaea Ultra-Bloodrush VIP)".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "mantis-1".to_string(),
            ChartData { 
                name_en: "MANTIS (Arcaea Ultra-Bloodrush VIP)".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "mantis-2".to_string(),
            ChartData { 
                name_en: "MANTIS (Arcaea Ultra-Bloodrush VIP)".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "worldfragments-0".to_string(),
            ChartData { 
                name_en: "World Fragments III(radio edit)".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "worldfragments-1".to_string(),
            ChartData { 
                name_en: "World Fragments III(radio edit)".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "worldfragments-2".to_string(),
            ChartData { 
                name_en: "World Fragments III(radio edit)".to_string(),
                name_jp: None,
                rating: 9.8
            }
            );
            map.insert(
            "astrawalkthrough-0".to_string(),
            ChartData { 
                name_en: "Astra walkthrough".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "astrawalkthrough-1".to_string(),
            ChartData { 
                name_en: "Astra walkthrough".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "astrawalkthrough-2".to_string(),
            ChartData { 
                name_en: "Astra walkthrough".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "chronicle-0".to_string(),
            ChartData { 
                name_en: "Chronicle".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "chronicle-1".to_string(),
            ChartData { 
                name_en: "Chronicle".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "chronicle-2".to_string(),
            ChartData { 
                name_en: "Chronicle".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "nullapophenia-0".to_string(),
            ChartData { 
                name_en: "NULL APOPHENIA".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "nullapophenia-1".to_string(),
            ChartData { 
                name_en: "NULL APOPHENIA".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "nullapophenia-2".to_string(),
            ChartData { 
                name_en: "NULL APOPHENIA".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "trappola-3".to_string(),
            ChartData { 
                name_en: "trappola bewitching".to_string(),
                name_jp: Some("妖艶魔女 -trappola bewitching-".to_string()),
                rating: 10.5
            }
            );
            map.insert(
            "crimsonthrone-0".to_string(),
            ChartData { 
                name_en: "Crimson Throne".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "crimsonthrone-1".to_string(),
            ChartData { 
                name_en: "Crimson Throne".to_string(),
                name_jp: None,
                rating: 8.0
            }
            );
            map.insert(
            "crimsonthrone-2".to_string(),
            ChartData { 
                name_en: "Crimson Throne".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "manicjeer-0".to_string(),
            ChartData { 
                name_en: "Manic Jeer".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "manicjeer-1".to_string(),
            ChartData { 
                name_en: "Manic Jeer".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "manicjeer-2".to_string(),
            ChartData { 
                name_en: "Manic Jeer".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "hiirogekka-0".to_string(),
            ChartData { 
                name_en: "Hiiro Gekka, Kyoushou no Zetsu (nayuta 2017 ver.)".to_string(),
                name_jp: Some("緋色月下、狂咲ノ絶 (nayuta 2017 ver.)".to_string()),
                rating: 3.5
            }
            );
            map.insert(
            "hiirogekka-1".to_string(),
            ChartData { 
                name_en: "Hiiro Gekka, Kyoushou no Zetsu (nayuta 2017 ver.)".to_string(),
                name_jp: Some("緋色月下、狂咲ノ絶 (nayuta 2017 ver.)".to_string()),
                rating: 6.5
            }
            );
            map.insert(
            "hiirogekka-2".to_string(),
            ChartData { 
                name_en: "Hiiro Gekka, Kyoushou no Zetsu (nayuta 2017 ver.)".to_string(),
                name_jp: Some("緋色月下、狂咲ノ絶 (nayuta 2017 ver.)".to_string()),
                rating: 10.3
            }
            );
            map.insert(
            "letsrock-0".to_string(),
            ChartData { 
                name_en: "Let's Rock (Arcaea mix)".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "letsrock-1".to_string(),
            ChartData { 
                name_en: "Let's Rock (Arcaea mix)".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "letsrock-2".to_string(),
            ChartData { 
                name_en: "Let's Rock (Arcaea mix)".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "cycles-0".to_string(),
            ChartData { 
                name_en: "CYCLES".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "cycles-1".to_string(),
            ChartData { 
                name_en: "CYCLES".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "cycles-2".to_string(),
            ChartData { 
                name_en: "CYCLES".to_string(),
                name_jp: None,
                rating: 8.8
            }
            );
            map.insert(
            "maxrage-0".to_string(),
            ChartData { 
                name_en: "MAXRAGE".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "maxrage-1".to_string(),
            ChartData { 
                name_en: "MAXRAGE".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "maxrage-2".to_string(),
            ChartData { 
                name_en: "MAXRAGE".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "infinity-0".to_string(),
            ChartData { 
                name_en: "[X]".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "infinity-1".to_string(),
            ChartData { 
                name_en: "[X]".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "infinity-2".to_string(),
            ChartData { 
                name_en: "[X]".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "temptation-0".to_string(),
            ChartData { 
                name_en: "TEmPTaTiON".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "temptation-1".to_string(),
            ChartData { 
                name_en: "TEmPTaTiON".to_string(),
                name_jp: None,
                rating: 8.2
            }
            );
            map.insert(
            "temptation-2".to_string(),
            ChartData { 
                name_en: "TEmPTaTiON".to_string(),
                name_jp: None,
                rating: 10.9
            }
            );
            map.insert(
            "shadesoflight-3".to_string(),
            ChartData { 
                name_en: "Shades of Light in a Transcendent Realm".to_string(),
                name_jp: None,
                rating: 9.0
            }
            );
            map.insert(
            "primitivelights-0".to_string(),
            ChartData { 
                name_en: "PRIMITIVE LIGHTS".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "primitivelights-1".to_string(),
            ChartData { 
                name_en: "PRIMITIVE LIGHTS".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "primitivelights-2".to_string(),
            ChartData { 
                name_en: "PRIMITIVE LIGHTS".to_string(),
                name_jp: None,
                rating: 10.7
            }
            );
            map.insert(
            "teriqma-3".to_string(),
            ChartData { 
                name_en: "Teriqma".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "cosmopop-0".to_string(),
            ChartData { 
                name_en: "Cosmo Pop Funclub".to_string(),
                name_jp: Some("コスモポップファンクラブ".to_string()),
                rating: 2.5
            }
            );
            map.insert(
            "cosmopop-1".to_string(),
            ChartData { 
                name_en: "Cosmo Pop Funclub".to_string(),
                name_jp: Some("コスモポップファンクラブ".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "cosmopop-2".to_string(),
            ChartData { 
                name_en: "Cosmo Pop Funclub".to_string(),
                name_jp: Some("コスモポップファンクラブ".to_string()),
                rating: 8.8
            }
            );
            map.insert(
            "impact-0".to_string(),
            ChartData { 
                name_en: "IMPACT".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "impact-1".to_string(),
            ChartData { 
                name_en: "IMPACT".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "impact-2".to_string(),
            ChartData { 
                name_en: "IMPACT".to_string(),
                name_jp: None,
                rating: 9.6
            }
            );
            map.insert(
            "impact-3".to_string(),
            ChartData { 
                name_en: "IMPACT".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "genesischunithm-0".to_string(),
            ChartData { 
                name_en: "Genesis".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "genesischunithm-1".to_string(),
            ChartData { 
                name_en: "Genesis".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "genesischunithm-2".to_string(),
            ChartData { 
                name_en: "Genesis".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "trrricksters-0".to_string(),
            ChartData { 
                name_en: "Trrricksters!!".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "trrricksters-1".to_string(),
            ChartData { 
                name_en: "Trrricksters!!".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "trrricksters-2".to_string(),
            ChartData { 
                name_en: "Trrricksters!!".to_string(),
                name_jp: None,
                rating: 10.1
            }
            );
            map.insert(
            "spidersthread-0".to_string(),
            ChartData { 
                name_en: "Spider's Thread".to_string(),
                name_jp: Some("蜘蛛の糸".to_string()),
                rating: 4.5
            }
            );
            map.insert(
            "spidersthread-1".to_string(),
            ChartData { 
                name_en: "Spider's Thread".to_string(),
                name_jp: Some("蜘蛛の糸".to_string()),
                rating: 7.5
            }
            );
            map.insert(
            "spidersthread-2".to_string(),
            ChartData { 
                name_en: "Spider's Thread".to_string(),
                name_jp: Some("蜘蛛の糸".to_string()),
                rating: 10.8
            }
            );
            map.insert(
            "lostemotion-0".to_string(),
            ChartData { 
                name_en: "Lost Emotion feat. nomico".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "lostemotion-1".to_string(),
            ChartData { 
                name_en: "Lost Emotion feat. nomico".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "lostemotion-2".to_string(),
            ChartData { 
                name_en: "Lost Emotion feat. nomico".to_string(),
                name_jp: None,
                rating: 9.3
            }
            );
            map.insert(
            "gimmick-0".to_string(),
            ChartData { 
                name_en: "GIMMICK".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "gimmick-1".to_string(),
            ChartData { 
                name_en: "GIMMICK".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "gimmick-2".to_string(),
            ChartData { 
                name_en: "GIMMICK".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "thesurvivor-0".to_string(),
            ChartData { 
                name_en: "The Survivor (Game Edit)".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "thesurvivor-1".to_string(),
            ChartData { 
                name_en: "The Survivor (Game Edit)".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "thesurvivor-2".to_string(),
            ChartData { 
                name_en: "The Survivor (Game Edit)".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "newyorkbackraise-0".to_string(),
            ChartData { 
                name_en: "New York Back Raise".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "newyorkbackraise-1".to_string(),
            ChartData { 
                name_en: "New York Back Raise".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "newyorkbackraise-2".to_string(),
            ChartData { 
                name_en: "New York Back Raise".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "galacticlove-0".to_string(),
            ChartData { 
                name_en: "Galactic Love".to_string(),
                name_jp: None,
                rating: 3.0
            }
            );
            map.insert(
            "galacticlove-1".to_string(),
            ChartData { 
                name_en: "Galactic Love".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "galacticlove-2".to_string(),
            ChartData { 
                name_en: "Galactic Love".to_string(),
                name_jp: None,
                rating: 9.0
            }
            );
            map.insert(
            "lawlesspoint-0".to_string(),
            ChartData { 
                name_en: "Lawless Point".to_string(),
                name_jp: Some("無法地点".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "lawlesspoint-1".to_string(),
            ChartData { 
                name_en: "Lawless Point".to_string(),
                name_jp: Some("無法地点".to_string()),
                rating: 6.0
            }
            );
            map.insert(
            "lawlesspoint-2".to_string(),
            ChartData { 
                name_en: "Lawless Point".to_string(),
                name_jp: Some("無法地点".to_string()),
                rating: 8.9
            }
            );
            map.insert(
            "lostintheabyss-0".to_string(),
            ChartData { 
                name_en: "Lost in the Abyss".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "lostintheabyss-1".to_string(),
            ChartData { 
                name_en: "Lost in the Abyss".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "lostintheabyss-2".to_string(),
            ChartData { 
                name_en: "Lost in the Abyss".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "hybris-0".to_string(),
            ChartData { 
                name_en: "Hybris (The one who shattered)".to_string(),
                name_jp: Some("ヒュブリスの頂に聳えるのは".to_string()),
                rating: 4.5
            }
            );
            map.insert(
            "hybris-1".to_string(),
            ChartData { 
                name_en: "Hybris (The one who shattered)".to_string(),
                name_jp: Some("ヒュブリスの頂に聳えるのは".to_string()),
                rating: 7.5
            }
            );
            map.insert(
            "hybris-2".to_string(),
            ChartData { 
                name_en: "Hybris (The one who shattered)".to_string(),
                name_jp: Some("ヒュブリスの頂に聳えるのは".to_string()),
                rating: 9.8
            }
            );
            map.insert(
            "tothemilkyway-0".to_string(),
            ChartData { 
                name_en: "To the Milky Way".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "tothemilkyway-1".to_string(),
            ChartData { 
                name_en: "To the Milky Way".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "tothemilkyway-2".to_string(),
            ChartData { 
                name_en: "To the Milky Way".to_string(),
                name_jp: None,
                rating: 10.5
            }
            );
            map.insert(
            "internetyamero-0".to_string(),
            ChartData { 
                name_en: "INTERNET YAMERO".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "internetyamero-1".to_string(),
            ChartData { 
                name_en: "INTERNET YAMERO".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "internetyamero-2".to_string(),
            ChartData { 
                name_en: "INTERNET YAMERO".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "bulletwaiting-0".to_string(),
            ChartData { 
                name_en: "Bullet Waiting for Me (James Landino remix)".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "bulletwaiting-1".to_string(),
            ChartData { 
                name_en: "Bullet Waiting for Me (James Landino remix)".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "bulletwaiting-2".to_string(),
            ChartData { 
                name_en: "Bullet Waiting for Me (James Landino remix)".to_string(),
                name_jp: None,
                rating: 8.1
            }
            );
            map.insert(
            "devillicsphere-0".to_string(),
            ChartData { 
                name_en: "Devillic Sphere".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "devillicsphere-1".to_string(),
            ChartData { 
                name_en: "Devillic Sphere".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "devillicsphere-2".to_string(),
            ChartData { 
                name_en: "Devillic Sphere".to_string(),
                name_jp: None,
                rating: 9.9
            }
            );
            map.insert(
            "lucidtraveler-0".to_string(),
            ChartData { 
                name_en: "Lucid Traveler".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "lucidtraveler-1".to_string(),
            ChartData { 
                name_en: "Lucid Traveler".to_string(),
                name_jp: None,
                rating: 8.3
            }
            );
            map.insert(
            "lucidtraveler-2".to_string(),
            ChartData { 
                name_en: "Lucid Traveler".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "chaos-0".to_string(),
            ChartData { 
                name_en: "CHAOS".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "chaos-1".to_string(),
            ChartData { 
                name_en: "CHAOS".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "chaos-2".to_string(),
            ChartData { 
                name_en: "CHAOS".to_string(),
                name_jp: None,
                rating: 10.9
            }
            );
            map.insert(
            "usedtobe-0".to_string(),
            ChartData { 
                name_en: "Used to be".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "usedtobe-1".to_string(),
            ChartData { 
                name_en: "Used to be".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "usedtobe-2".to_string(),
            ChartData { 
                name_en: "Used to be".to_string(),
                name_jp: None,
                rating: 9.2
            }
            );
            map.insert(
            "ultimatetaste-0".to_string(),
            ChartData { 
                name_en: "Ultimate taste".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "ultimatetaste-1".to_string(),
            ChartData { 
                name_en: "Ultimate taste".to_string(),
                name_jp: None,
                rating: 6.5
            }
            );
            map.insert(
            "ultimatetaste-2".to_string(),
            ChartData { 
                name_en: "Ultimate taste".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "syuten-0".to_string(),
            ChartData { 
                name_en: "syūten".to_string(),
                name_jp: None,
                rating: 2.0
            }
            );
            map.insert(
            "syuten-1".to_string(),
            ChartData { 
                name_en: "syūten".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "syuten-2".to_string(),
            ChartData { 
                name_en: "syūten".to_string(),
                name_jp: None,
                rating: 8.5
            }
            );
            map.insert(
            "drg-0".to_string(),
            ChartData { 
                name_en: "DRG".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "drg-1".to_string(),
            ChartData { 
                name_en: "DRG".to_string(),
                name_jp: None,
                rating: 7.0
            }
            );
            map.insert(
            "drg-2".to_string(),
            ChartData { 
                name_en: "DRG".to_string(),
                name_jp: None,
                rating: 9.5
            }
            );
            map.insert(
            "nnglooms-0".to_string(),
            ChartData { 
                name_en: "99 Glooms".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "nnglooms-1".to_string(),
            ChartData { 
                name_en: "99 Glooms".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "nnglooms-2".to_string(),
            ChartData { 
                name_en: "99 Glooms".to_string(),
                name_jp: None,
                rating: 10.3
            }
            );
            map.insert(
            "ii-0".to_string(),
            ChartData { 
                name_en: " ͟͝͞Ⅱ́̕ ".to_string(),
                name_jp: None,
                rating: 5.0
            }
            );
            map.insert(
            "ii-1".to_string(),
            ChartData { 
                name_en: " ͟͝͞Ⅱ́̕ ".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "ii-2".to_string(),
            ChartData { 
                name_en: " ͟͝͞Ⅱ́̕ ".to_string(),
                name_jp: None,
                rating: 10.8
            }
            );
            map.insert(
            "magnolia-0".to_string(),
            ChartData { 
                name_en: "Magnolia".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "magnolia-1".to_string(),
            ChartData { 
                name_en: "Magnolia".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "magnolia-2".to_string(),
            ChartData { 
                name_en: "Magnolia".to_string(),
                name_jp: None,
                rating: 10.2
            }
            );
            map.insert(
            "sacrifice-0".to_string(),
            ChartData { 
                name_en: "SACRIFICE feat. ayame".to_string(),
                name_jp: None,
                rating: 3.5
            }
            );
            map.insert(
            "sacrifice-1".to_string(),
            ChartData { 
                name_en: "SACRIFICE feat. ayame".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "sacrifice-2".to_string(),
            ChartData { 
                name_en: "SACRIFICE feat. ayame".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "rgb-0".to_string(),
            ChartData { 
                name_en: "RGB".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "rgb-1".to_string(),
            ChartData { 
                name_en: "RGB".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "rgb-2".to_string(),
            ChartData { 
                name_en: "RGB".to_string(),
                name_jp: None,
                rating: 9.7
            }
            );
            map.insert(
            "waitfordawn-0".to_string(),
            ChartData { 
                name_en: "WAIT FOR DAWN".to_string(),
                name_jp: None,
                rating: 2.5
            }
            );
            map.insert(
            "waitfordawn-1".to_string(),
            ChartData { 
                name_en: "WAIT FOR DAWN".to_string(),
                name_jp: None,
                rating: 6.0
            }
            );
            map.insert(
            "waitfordawn-2".to_string(),
            ChartData { 
                name_en: "WAIT FOR DAWN".to_string(),
                name_jp: None,
                rating: 8.7
            }
            );
            map.insert(
            "ravenspride-0".to_string(),
            ChartData { 
                name_en: "Raven's Pride".to_string(),
                name_jp: Some("レイヴンズ・プライド".to_string()),
                rating: 3.0
            }
            );
            map.insert(
            "ravenspride-1".to_string(),
            ChartData { 
                name_en: "Raven's Pride".to_string(),
                name_jp: Some("レイヴンズ・プライド".to_string()),
                rating: 6.5
            }
            );
            map.insert(
            "ravenspride-2".to_string(),
            ChartData { 
                name_en: "Raven's Pride".to_string(),
                name_jp: Some("レイヴンズ・プライド".to_string()),
                rating: 9.4
            }
            );
            map.insert(
            "riseoftheworld-0".to_string(),
            ChartData { 
                name_en: "Rise of the World".to_string(),
                name_jp: None,
                rating: 4.5
            }
            );
            map.insert(
            "riseoftheworld-1".to_string(),
            ChartData { 
                name_en: "Rise of the World".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "riseoftheworld-2".to_string(),
            ChartData { 
                name_en: "Rise of the World".to_string(),
                name_jp: None,
                rating: 10.4
            }
            );
            map.insert(
            "unknownlevels-0".to_string(),
            ChartData { 
                name_en: "UNKNOWN LEVELS".to_string(),
                name_jp: None,
                rating: 4.0
            }
            );
            map.insert(
            "unknownlevels-1".to_string(),
            ChartData { 
                name_en: "UNKNOWN LEVELS".to_string(),
                name_jp: None,
                rating: 7.5
            }
            );
            map.insert(
            "unknownlevels-2".to_string(),
            ChartData { 
                name_en: "UNKNOWN LEVELS".to_string(),
                name_jp: None,
                rating: 10.6
            }
            );
            map.insert(
            "abstrusedilemma-0".to_string(),
            ChartData { 
                name_en: "Abstruse Dilemma".to_string(),
                name_jp: None,
                rating: 5.5
            }
            );
            map.insert(
            "abstrusedilemma-1".to_string(),
            ChartData { 
                name_en: "Abstruse Dilemma".to_string(),
                name_jp: None,
                rating: 8.4
            }
            );
            map.insert(
            "abstrusedilemma-2".to_string(),
            ChartData { 
                name_en: "Abstruse Dilemma".to_string(),
                name_jp: None,
                rating: 11.1
            }
            );Ok(map)
}
