use std::fmt::Display;

use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
pub enum Character {
    Alisa,
    Anna,
    ArmorKing,
    Asuka,
    Azucena,
    Bryan,
    Claudio,
    Clive,
    DevilJin,
    Dragunov,
    Eddy,
    Fahkumram,
    Feng,
    Heihachi,
    Hwoarang,
    Jack8,
    Jin,
    Jun,
    Kazuya,
    King,
    Kuma,
    Lars,
    Law,
    Lee,
    Leo,
    Leroy,
    Lidia,
    Lili,
    Nina,
    MiaryZo,
    Panda,
    Paul,
    Raven,
    Reina,
    Shaheen,
    Steve,
    Victor,
    Xiaoyu,
    Yoshimitsu,
    Zafina,
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Character::Alisa => "Alisa",
            Character::Anna => "Anna",
            Character::ArmorKing => "Armor King",
            Character::Asuka => "Asuka",
            Character::Azucena => "Azucena",
            Character::Bryan => "Bryan",
            Character::Claudio => "Claudio",
            Character::Clive => "Clive",
            Character::DevilJin => "Devil Jin",
            Character::Dragunov => "Dragunov",
            Character::Eddy => "Eddy",
            Character::Fahkumram => "Fahkumram",
            Character::Feng => "Feng",
            Character::Heihachi => "Heihachi",
            Character::Hwoarang => "Hwoarang",
            Character::Jack8 => "Jack-8",
            Character::Jin => "Jin",
            Character::Jun => "Jun",
            Character::Kazuya => "Kazuya",
            Character::King => "King",
            Character::Kuma => "Kuma",
            Character::Lars => "Lars",
            Character::Law => "Law",
            Character::Lee => "Lee",
            Character::Leo => "Leo",
            Character::Leroy => "Leroy",
            Character::Lidia => "Lidia",
            Character::Lili => "Lili",
            Character::Nina => "Nina",
            Character::MiaryZo => "Miary Zo",
            Character::Panda => "Panda",
            Character::Paul => "Paul",
            Character::Raven => "Raven",
            Character::Reina => "Reina",
            Character::Shaheen => "Shaheen",
            Character::Steve => "Steve",
            Character::Victor => "Victor",
            Character::Xiaoyu => "Xiaoyu",
            Character::Yoshimitsu => "Yoshimitsu",
            Character::Zafina => "Zafina",
        };

        write!(f, "{str}")
    }
}

impl Character {
    pub fn portrait_url(&self) -> &'static str {
        match self {
            Character::Alisa => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426928779673997433/alisa-portrait.png?ex=68ed027c&is=68ebb0fc&hm=2ccbd2ac72962212c4cf2eace8c7b1a83331ac3c9dedd94e649cc8c005bbc9c3&=&format=webp&quality=lossless"
            }
            Character::Anna => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945301310345266/anna-portrait.png?ex=68ed11df&is=68ebc05f&hm=226471e5fe6248aeb723fe21e6270970a0b689f9f12bd755ecf07bdfd7cf61c5&=&format=webp&quality=lossless"
            }
            Character::ArmorKing => {
                "https://media.discordapp.net/attachments/1394056479169843271/1427749563049050142/armor_king.webp?ex=68effee6&is=68eead66&hm=6b9b8c8a3b64611efac042e3cd046c2790a38d411882d62527044f547f79bc56&=&format=webp"
            }
            Character::Asuka => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945166576713871/asuka-portrait.png?ex=68ed11bf&is=68ebc03f&hm=6f4086d691e96637d728f20f80bab2c615411859f595041377d42f1cca362cf8&=&format=webp&quality=lossless"
            }
            Character::Azucena => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945166278922240/azucena-portrait.png?ex=68ed11bf&is=68ebc03f&hm=b8dcbcede5f777b90acf142fbafa96b0b35068e870c14e294fc3f2aeeac3c45b&=&format=webp&quality=lossless"
            }
            Character::Bryan => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945300983185428/bryan-portrait.png?ex=68ed11df&is=68ebc05f&hm=7ba14a16899fa254e55fc1a2cf09c60ea4f1c0b7805cc3e5bf79a33297b62d38&=&format=webp&quality=lossless"
            }
            Character::Claudio => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945300592984288/claudio-portrait.png?ex=68ed11df&is=68ebc05f&hm=334988903a3ac9a338ddf02320ac059b806813d158bf1e1c567a151c97a6f74b&=&format=webp&quality=lossless"
            }
            Character::Clive => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945300311834664/clive-portrait.png?ex=68ed11de&is=68ebc05e&hm=71f6f9b1ebb51ea44b5851fe29cd3594e1cc6b06c96015f98140b4eef3a1563a&=&format=webp&quality=lossless"
            }
            Character::DevilJin => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945299925958837/devil-jin-portrait.png?ex=68ed11de&is=68ebc05e&hm=b3c0f91083437c5e5d757a307a6060f9178a16d6d807b35d9186a95a53e294c9&=&format=webp&quality=lossless"
            }
            Character::Dragunov => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945299527762130/dragunov-portrait.png?ex=68ed11de&is=68ebc05e&hm=c77bb3acf96939c832f05bfde2e89f45a397672c4cfc996dd954d3af1c41f089&=&format=webp&quality=lossless"
            }
            Character::Eddy => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945299183833178/eddy-portrait.png?ex=68ed11de&is=68ebc05e&hm=d2e6f401e45acaf977f793967e885567f86aa9cf05cd3d1d1bb3059f31a4e3fe&=&format=webp&quality=lossless"
            }
            Character::Fahkumram => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945276412952586/fahkumram-portrait.png?ex=68ed11d9&is=68ebc059&hm=34e9243cbf5ceba49ac344100e7c6c1f71147b25cb2d42f9f462203a1f91ea08&=&format=webp&quality=lossless"
            }
            Character::Feng => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945275959836944/feng-portrait.png?ex=68ed11d9&is=68ebc059&hm=c8c410719a010caf35e340d9357a7d551e136b2c3c1ee8b7918b23d3b4a19d25&=&format=webp&quality=lossless"
            }
            Character::Heihachi => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945275716440116/heihachi-portrait.png?ex=68ed11d9&is=68ebc059&hm=3a82fcacd4fe0f9c50600d4218b3264518546d26b40419e1134d487dd8a6cefa&=&format=webp&quality=lossless"
            }
            Character::Hwoarang => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945275393736864/hwoarang-portrait.png?ex=68ed11d9&is=68ebc059&hm=04bc7edd29b756e0cf5670f12aefc3e52c1e696b26c53ea8f521e493215dd51c&=&format=webp&quality=lossless"
            }
            Character::Jack8 => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945275095810149/jack-8-portrait.png?ex=68ed11d8&is=68ebc058&hm=96c701543ed0854cc6d02213c0f3fa43d866e301dab6b8e8132cc9613894f6cd&=&format=webp&quality=lossless"
            }
            Character::Jin => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945274827247788/jin-portrait.png?ex=68ed11d8&is=68ebc058&hm=63fb373651cddb41eeeb634e9b30e998c891ed33ed3176cdb40c51018551afed&=&format=webp&quality=lossless"
            }
            Character::Jun => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945274533904394/jun-portrait.png?ex=68ed11d8&is=68ebc058&hm=8b42ec12544a5fddd468f94d862f4e690cf018ff51c443ff349726722ba07c70&=&format=webp&quality=lossless"
            }
            Character::Kazuya => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945274248560812/kazuya-portrait.png?ex=68ed11d8&is=68ebc058&hm=96723a68eadcae12db64f944f8d7aee8cbdc847d92b82b867c616e720d3bbce7&=&format=webp&quality=lossless"
            }
            Character::King => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945273950638180/king-portrait.png?ex=68ed11d8&is=68ebc058&hm=26e262b2cbac95de70995587cc315878891151b71f36c50df8b2b8621066ba34&=&format=webp&quality=lossless"
            }
            Character::Kuma => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945273585864887/kuma-portrait.png?ex=68ed11d8&is=68ebc058&hm=0ff00e0111888b1359fe97f8883eb69176626ddf71c3ce18e1e4840bb43cd551&=&format=webp&quality=lossless"
            }
            Character::Lars => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945229151408170/lars-portrait.png?ex=68ed11cd&is=68ebc04d&hm=a8b4f080eb51bbbd5d6becaec134b4fab1fbbc1564bc9d72105ee8fcbac26838&=&format=webp&quality=lossless"
            }
            Character::Law => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945228731842742/law-portrait.png?ex=68ed11cd&is=68ebc04d&hm=0f6b8eb8af150e7d240b94c15fafc2c59f13c7b9a4f1882769a5706310e01ecd&=&format=webp&quality=lossless"
            }
            Character::Lee => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945227763224730/lee-portrait.png?ex=68ed11cd&is=68ebc04d&hm=d83acaef4d2615ae3763e6fb6ab59177ef39410bb21028b1dd9dbd8da3988745&=&format=webp&quality=lossless"
            }
            Character::Leo => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945227364634875/leo-portrait.png?ex=68ed11cd&is=68ebc04d&hm=093c8926f11e90046cb74f877489e38d14f81198946e31b0e783b903f8cae638&=&format=webp&quality=lossless"
            }
            Character::Leroy => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945226551066624/leroy-portrait.png?ex=68ed11cd&is=68ebc04d&hm=8550aaa0b5005013288a789ac2a40560d89d3e1af6a6688306b05075c5e2d638&=&format=webp&quality=lossless"
            }
            Character::Lidia => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945226010005718/lidia-portrait.png?ex=68ed11cd&is=68ebc04d&hm=bae4dc6df31f2bfe73c78194bbdea6e135dd44aeecb72a8b3ff46b75486fae16&=&format=webp&quality=lossless"
            }
            Character::Lili => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945225502359663/lili-portrait.png?ex=68ed11cd&is=68ebc04d&hm=aef6ce5f15d806e5a6b6903f7d40e17c7cee4c84b948caf32a144f557b155767&=&format=webp&quality=lossless"
            }
            Character::Nina => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945224894320731/nina-portrait.png?ex=68ed11cc&is=68ebc04c&hm=573fe709d7899232505d012be53af90b5f63d593331c1d8a5e8be51492ace158&=&format=webp&quality=lossless"
            }
            Character::MiaryZo => {
                "https://media.discordapp.net/attachments/1394056479169843271/1449143628583669850/cmg9xoson37gn07ljubup6hwa.png?ex=693dd3ac&is=693c822c&hm=4f3210fd62774b4d2ea8fce9d44ae0f4a924e0a3bef5b71955e0f0d9d8133dc4&=&format=webp&quality=lossless"
            }
            Character::Panda => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945224348930201/panda-portrait.png?ex=68ed11cc&is=68ebc04c&hm=3efb95de6dfc7c03fa61560eab7e2fdfb2aa685a996520640828ea9a61fdd59e&=&format=webp&quality=lossless"
            }
            Character::Paul => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945223896076452/paul-portrait.png?ex=68ed11cc&is=68ebc04c&hm=f9efe2cdf7eb4966d8f8c9d9472dbd35bcf57e362cfba192051f269be0ccf068&=&format=webp&quality=lossless"
            }
            Character::Raven => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945169038774396/raven-portrait.png?ex=68ed11bf&is=68ebc03f&hm=0945c4f20b300362951f782390b76cbf71b6f05cf7f6244d1607bc3615a37ec7&=&format=webp&quality=lossless"
            }
            Character::Reina => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945168740716575/reina-portrait.png?ex=68ed11bf&is=68ebc03f&hm=e5fb51a2368066e632bda08ee7ce7e5995de29d83907c3f144888db7ca6fe495&=&format=webp&quality=lossless"
            }
            Character::Shaheen => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945168438984805/shaheen-portrait.png?ex=68ed11bf&is=68ebc03f&hm=721eea7c97909a5baa3aa58a6d8e0123170023a1ae0514507c27d2b7fd72eace&=&format=webp&quality=lossless"
            }
            Character::Steve => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945168136999072/steve-portrait.png?ex=68ed11bf&is=68ebc03f&hm=76e90a59b874cc1e06e988fdbf5ebe7acbd74367e270fbb267157818880909fd&=&format=webp&quality=lossless"
            }
            Character::Victor => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945167838941284/victor-portrait.png?ex=68ed11bf&is=68ebc03f&hm=892f9dc85184878c6c3a51cf5273f35dae73b84536e2d627f49c6e5490c3761a&=&format=webp&quality=lossless"
            }
            Character::Xiaoyu => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945167516237824/xiaoyu-portrait.png?ex=68ed11bf&is=68ebc03f&hm=f1db45f50a33fadc627431c5e3aed3244008d219987c3eac5d83964a569d4631&=&format=webp&quality=lossless"
            }
            Character::Yoshimitsu => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945167210057728/yoshimitsu-portrait.png?ex=68ed11bf&is=68ebc03f&hm=f7e3efbc96c9064ea7fc1fcf99d39f66c4c2fea639ab3221df0be90a1e415377&=&format=webp&quality=lossless"
            }
            Character::Zafina => {
                "https://media.discordapp.net/attachments/1394056479169843271/1426945166899417118/zafina-portrait.png?ex=68ed11bf&is=68ebc03f&hm=58a65a98d13cfb63750c2e8581637602f7d49753a56eb2a440c27b7937e3126b&=&format=webp&quality=lossless"
            }
        }
    }
}
