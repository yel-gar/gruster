use std::collections::HashMap;
use std::sync::LazyLock;

pub static ENTRIES: LazyLock<HashMap<u16, &'static str>> = LazyLock::new(|| {
    let dat = [
        (67, ""), // HOW VERY, VERY, CRINGE
        (309, ""),                  // 7UST
        (478, ""),  // YOUR TAKING TOO LONG
        (8, ""), // YOUR COMPUTER IS NOW INFESTED WITH F R I E N D S
        (270, ""),                                // LETOCTF{I_L1Ke_
        (37, ""), // IF YOU WANT TO BE FASTER, YOU MUST KISS GASTER
        (46, ""),                           // GREENER, YET GREENER
        (234, ""),                                     // UND3RTALE
        (652, ""), // NOW. LET US PLAY THE GAME. YOU LOST IT BY THE WAY.
        (381, ""),                                         // EXP3RIM9NT
        (599, ""),                                              // GR33N
        (31, ""),                                      // SECRET: G5STER
        (6, ""), // YOU SHOULD NOT HAVE RUN RANDOM EXE FROM THE INTERNET
        (367, ""),                       // I GENUINELY HATE THIS NUMBER
        (567, ""),                                             // KNIGHT
        (520, ""),                                              // ISTH3
        (120, ""),                                      // G5EEN_AppLeZ}
        (162, ""),                                     // SECRET: KN1GH5
        (660, ""),                          // LEFT BUTTON: YOU CAN TYPE
        (165, ""),                                       // SECRET: RU5T
        (222, ""),                                              // APPLE
        (300, ""),                                    // T0CH1ng_PEopl3}
        (658, ""), // RIGHT BUTTON: SYSTEM32 DELETED
        (480, ""),    // SLOW DOWN IF YOU'RE A CLOWN
        (17, ""), // 165 482 30. NO SPACE. ALL SMALL. ALL GREEN.
        (477, ""),            // THIS EXPERIMENT IS TOO ANNOYING
        (510, ""),                                       // KR1S
        (30, ""),                              // SECRET: GAS1ER
        (44, ""),                        // I SEEM TO SLOWLY GET
        (665, ""),                                   // PICK ONE
        (482, ""),                              // SECRET: GR3EN
        (664, ""),                                   // PICK ONE
        (14, ""),                              // I USE ARCH BTW
        (
            650,
            "",
        ), // FINALLY. THIS EXPERIMENT HAS BEEN VERY, VERY, ENTERTAINING
    ];
    HashMap::from_iter(dat)
});

pub const FLAG_ARGON_SALT: &[u8; 16] = b"im_also_a_pirate";
pub const FLAG_ARGON_NONCE: [u8; 12] = [225u8, 146, 137, 51, 115, 72, 223, 59, 254, 5, 38, 70];
pub const FLAG_PAYLOAD: [u8; 52] = [
    228u8, 47, 172, 90, 218, 207, 109, 6, 217, 113, 170, 39, 239, 77, 134, 71, 1, 140, 153, 67, 77,
    192, 33, 130, 19, 237, 154, 172, 2, 167, 131, 79, 221, 31, 46, 74, 113, 78, 41, 68, 49, 2, 30,
    166, 136, 204, 161, 185, 170, 44, 143, 37,
];
