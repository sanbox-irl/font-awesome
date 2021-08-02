#![no_std]
#![doc = include_str!("../README.md")]

pub const MINIMUM_CODEPOINT: usize = 0xe005;
pub const MAXIMUM_CODEPOINT: usize = 0xf8ff;

pub const AD: char = '\u{f641}';
pub const ADDRESS_BOOK: char = '\u{f2b9}';
pub const ADDRESS_CARD: char = '\u{f2bb}';
pub const ADJUST: char = '\u{f042}';
pub const AIR_FRESHENER: char = '\u{f5d0}';
pub const ALIGN_CENTER: char = '\u{f037}';
pub const ALIGN_JUSTIFY: char = '\u{f039}';
pub const ALIGN_LEFT: char = '\u{f036}';
pub const ALIGN_RIGHT: char = '\u{f038}';
pub const ALLERGIES: char = '\u{f461}';
pub const AMBULANCE: char = '\u{f0f9}';
pub const AMERICAN_SIGN_LANGUAGE_INTERPRETING: char = '\u{f2a3}';
pub const ANCHOR: char = '\u{f13d}';
pub const ANGLE_DOUBLE_DOWN: char = '\u{f103}';
pub const ANGLE_DOUBLE_LEFT: char = '\u{f100}';
pub const ANGLE_DOUBLE_RIGHT: char = '\u{f101}';
pub const ANGLE_DOUBLE_UP: char = '\u{f102}';
pub const ANGLE_DOWN: char = '\u{f107}';
pub const ANGLE_LEFT: char = '\u{f104}';
pub const ANGLE_RIGHT: char = '\u{f105}';
pub const ANGLE_UP: char = '\u{f106}';
pub const ANGRY: char = '\u{f556}';
pub const ANKH: char = '\u{f644}';
pub const APPLE_ALT: char = '\u{f5d1}';
pub const ARCHIVE: char = '\u{f187}';
pub const ARCHWAY: char = '\u{f557}';
pub const ARROW_ALT_CIRCLE_DOWN: char = '\u{f358}';
pub const ARROW_ALT_CIRCLE_LEFT: char = '\u{f359}';
pub const ARROW_ALT_CIRCLE_RIGHT: char = '\u{f35a}';
pub const ARROW_ALT_CIRCLE_UP: char = '\u{f35b}';
pub const ARROW_CIRCLE_DOWN: char = '\u{f0ab}';
pub const ARROW_CIRCLE_LEFT: char = '\u{f0a8}';
pub const ARROW_CIRCLE_RIGHT: char = '\u{f0a9}';
pub const ARROW_CIRCLE_UP: char = '\u{f0aa}';
pub const ARROW_DOWN: char = '\u{f063}';
pub const ARROW_LEFT: char = '\u{f060}';
pub const ARROW_RIGHT: char = '\u{f061}';
pub const ARROW_UP: char = '\u{f062}';
pub const ARROWS_ALT: char = '\u{f0b2}';
pub const ARROWS_ALT_H: char = '\u{f337}';
pub const ARROWS_ALT_V: char = '\u{f338}';
pub const ASSISTIVE_LISTENING_SYSTEMS: char = '\u{f2a2}';
pub const ASTERISK: char = '\u{f069}';
pub const AT: char = '\u{f1fa}';
pub const ATLAS: char = '\u{f558}';
pub const ATOM: char = '\u{f5d2}';
pub const AUDIO_DESCRIPTION: char = '\u{f29e}';
pub const AWARD: char = '\u{f559}';
pub const BABY: char = '\u{f77c}';
pub const BABY_CARRIAGE: char = '\u{f77d}';
pub const BACKSPACE: char = '\u{f55a}';
pub const BACKWARD: char = '\u{f04a}';
pub const BACON: char = '\u{f7e5}';
pub const BACTERIA: char = '\u{e059}';
pub const BACTERIUM: char = '\u{e05a}';
pub const BAHAI: char = '\u{f666}';
pub const BALANCE_SCALE: char = '\u{f24e}';
pub const BALANCE_SCALE_LEFT: char = '\u{f515}';
pub const BALANCE_SCALE_RIGHT: char = '\u{f516}';
pub const BAN: char = '\u{f05e}';
pub const BAND_AID: char = '\u{f462}';
pub const BARCODE: char = '\u{f02a}';
pub const BARS: char = '\u{f0c9}';
pub const BASEBALL_BALL: char = '\u{f433}';
pub const BASKETBALL_BALL: char = '\u{f434}';
pub const BATH: char = '\u{f2cd}';
pub const BATTERY_EMPTY: char = '\u{f244}';
pub const BATTERY_FULL: char = '\u{f240}';
pub const BATTERY_HALF: char = '\u{f242}';
pub const BATTERY_QUARTER: char = '\u{f243}';
pub const BATTERY_THREE_QUARTERS: char = '\u{f241}';
pub const BED: char = '\u{f236}';
pub const BEER: char = '\u{f0fc}';
pub const BELL: char = '\u{f0f3}';
pub const BELL_SLASH: char = '\u{f1f6}';
pub const BEZIER_CURVE: char = '\u{f55b}';
pub const BIBLE: char = '\u{f647}';
pub const BICYCLE: char = '\u{f206}';
pub const BIKING: char = '\u{f84a}';
pub const BINOCULARS: char = '\u{f1e5}';
pub const BIOHAZARD: char = '\u{f780}';
pub const BIRTHDAY_CAKE: char = '\u{f1fd}';
pub const BLENDER: char = '\u{f517}';
pub const BLENDER_PHONE: char = '\u{f6b6}';
pub const BLIND: char = '\u{f29d}';
pub const BLOG: char = '\u{f781}';
pub const BOLD: char = '\u{f032}';
pub const BOLT: char = '\u{f0e7}';
pub const BOMB: char = '\u{f1e2}';
pub const BONE: char = '\u{f5d7}';
pub const BONG: char = '\u{f55c}';
pub const BOOK: char = '\u{f02d}';
pub const BOOK_DEAD: char = '\u{f6b7}';
pub const BOOK_MEDICAL: char = '\u{f7e6}';
pub const BOOK_OPEN: char = '\u{f518}';
pub const BOOK_READER: char = '\u{f5da}';
pub const BOOKMARK: char = '\u{f02e}';
pub const BORDER_ALL: char = '\u{f84c}';
pub const BORDER_NONE: char = '\u{f850}';
pub const BORDER_STYLE: char = '\u{f853}';
pub const BOWLING_BALL: char = '\u{f436}';
pub const BOX: char = '\u{f466}';
pub const BOX_OPEN: char = '\u{f49e}';
pub const BOX_TISSUE: char = '\u{e05b}';
pub const BOXES: char = '\u{f468}';
pub const BRAILLE: char = '\u{f2a1}';
pub const BRAIN: char = '\u{f5dc}';
pub const BREAD_SLICE: char = '\u{f7ec}';
pub const BRIEFCASE: char = '\u{f0b1}';
pub const BRIEFCASE_MEDICAL: char = '\u{f469}';
pub const BROADCAST_TOWER: char = '\u{f519}';
pub const BROOM: char = '\u{f51a}';
pub const BRUSH: char = '\u{f55d}';
pub const BUG: char = '\u{f188}';
pub const BUILDING: char = '\u{f1ad}';
pub const BULLHORN: char = '\u{f0a1}';
pub const BULLSEYE: char = '\u{f140}';
pub const BURN: char = '\u{f46a}';
pub const BUS: char = '\u{f207}';
pub const BUS_ALT: char = '\u{f55e}';
pub const BUSINESS_TIME: char = '\u{f64a}';
pub const CALCULATOR: char = '\u{f1ec}';
pub const CALENDAR: char = '\u{f133}';
pub const CALENDAR_ALT: char = '\u{f073}';
pub const CALENDAR_CHECK: char = '\u{f274}';
pub const CALENDAR_DAY: char = '\u{f783}';
pub const CALENDAR_MINUS: char = '\u{f272}';
pub const CALENDAR_PLUS: char = '\u{f271}';
pub const CALENDAR_TIMES: char = '\u{f273}';
pub const CALENDAR_WEEK: char = '\u{f784}';
pub const CAMERA: char = '\u{f030}';
pub const CAMERA_RETRO: char = '\u{f083}';
pub const CAMPGROUND: char = '\u{f6bb}';
pub const CANDY_CANE: char = '\u{f786}';
pub const CANNABIS: char = '\u{f55f}';
pub const CAPSULES: char = '\u{f46b}';
pub const CAR: char = '\u{f1b9}';
pub const CAR_ALT: char = '\u{f5de}';
pub const CAR_BATTERY: char = '\u{f5df}';
pub const CAR_CRASH: char = '\u{f5e1}';
pub const CAR_SIDE: char = '\u{f5e4}';
pub const CARAVAN: char = '\u{f8ff}';
pub const CARET_DOWN: char = '\u{f0d7}';
pub const CARET_LEFT: char = '\u{f0d9}';
pub const CARET_RIGHT: char = '\u{f0da}';
pub const CARET_SQUARE_DOWN: char = '\u{f150}';
pub const CARET_SQUARE_LEFT: char = '\u{f191}';
pub const CARET_SQUARE_RIGHT: char = '\u{f152}';
pub const CARET_SQUARE_UP: char = '\u{f151}';
pub const CARET_UP: char = '\u{f0d8}';
pub const CARROT: char = '\u{f787}';
pub const CART_ARROW_DOWN: char = '\u{f218}';
pub const CART_PLUS: char = '\u{f217}';
pub const CASH_REGISTER: char = '\u{f788}';
pub const CAT: char = '\u{f6be}';
pub const CERTIFICATE: char = '\u{f0a3}';
pub const CHAIR: char = '\u{f6c0}';
pub const CHALKBOARD: char = '\u{f51b}';
pub const CHALKBOARD_TEACHER: char = '\u{f51c}';
pub const CHARGING_STATION: char = '\u{f5e7}';
pub const CHART_AREA: char = '\u{f1fe}';
pub const CHART_BAR: char = '\u{f080}';
pub const CHART_LINE: char = '\u{f201}';
pub const CHART_PIE: char = '\u{f200}';
pub const CHECK: char = '\u{f00c}';
pub const CHECK_CIRCLE: char = '\u{f058}';
pub const CHECK_DOUBLE: char = '\u{f560}';
pub const CHECK_SQUARE: char = '\u{f14a}';
pub const CHEESE: char = '\u{f7ef}';
pub const CHESS: char = '\u{f439}';
pub const CHESS_BISHOP: char = '\u{f43a}';
pub const CHESS_BOARD: char = '\u{f43c}';
pub const CHESS_KING: char = '\u{f43f}';
pub const CHESS_KNIGHT: char = '\u{f441}';
pub const CHESS_PAWN: char = '\u{f443}';
pub const CHESS_QUEEN: char = '\u{f445}';
pub const CHESS_ROOK: char = '\u{f447}';
pub const CHEVRON_CIRCLE_DOWN: char = '\u{f13a}';
pub const CHEVRON_CIRCLE_LEFT: char = '\u{f137}';
pub const CHEVRON_CIRCLE_RIGHT: char = '\u{f138}';
pub const CHEVRON_CIRCLE_UP: char = '\u{f139}';
pub const CHEVRON_DOWN: char = '\u{f078}';
pub const CHEVRON_LEFT: char = '\u{f053}';
pub const CHEVRON_RIGHT: char = '\u{f054}';
pub const CHEVRON_UP: char = '\u{f077}';
pub const CHILD: char = '\u{f1ae}';
pub const CHURCH: char = '\u{f51d}';
pub const CIRCLE: char = '\u{f111}';
pub const CIRCLE_NOTCH: char = '\u{f1ce}';
pub const CITY: char = '\u{f64f}';
pub const CLINIC_MEDICAL: char = '\u{f7f2}';
pub const CLIPBOARD: char = '\u{f328}';
pub const CLIPBOARD_CHECK: char = '\u{f46c}';
pub const CLIPBOARD_LIST: char = '\u{f46d}';
pub const CLOCK: char = '\u{f017}';
pub const CLONE: char = '\u{f24d}';
pub const CLOSED_CAPTIONING: char = '\u{f20a}';
pub const CLOUD: char = '\u{f0c2}';
pub const CLOUD_DOWNLOAD_ALT: char = '\u{f381}';
pub const CLOUD_MEATBALL: char = '\u{f73b}';
pub const CLOUD_MOON: char = '\u{f6c3}';
pub const CLOUD_MOON_RAIN: char = '\u{f73c}';
pub const CLOUD_RAIN: char = '\u{f73d}';
pub const CLOUD_SHOWERS_HEAVY: char = '\u{f740}';
pub const CLOUD_SUN: char = '\u{f6c4}';
pub const CLOUD_SUN_RAIN: char = '\u{f743}';
pub const CLOUD_UPLOAD_ALT: char = '\u{f382}';
pub const COCKTAIL: char = '\u{f561}';
pub const CODE: char = '\u{f121}';
pub const CODE_BRANCH: char = '\u{f126}';
pub const COFFEE: char = '\u{f0f4}';
pub const COG: char = '\u{f013}';
pub const COGS: char = '\u{f085}';
pub const COINS: char = '\u{f51e}';
pub const COLUMNS: char = '\u{f0db}';
pub const COMMENT: char = '\u{f075}';
pub const COMMENT_ALT: char = '\u{f27a}';
pub const COMMENT_DOLLAR: char = '\u{f651}';
pub const COMMENT_DOTS: char = '\u{f4ad}';
pub const COMMENT_MEDICAL: char = '\u{f7f5}';
pub const COMMENT_SLASH: char = '\u{f4b3}';
pub const COMMENTS: char = '\u{f086}';
pub const COMMENTS_DOLLAR: char = '\u{f653}';
pub const COMPACT_DISC: char = '\u{f51f}';
pub const COMPASS: char = '\u{f14e}';
pub const COMPRESS: char = '\u{f066}';
pub const COMPRESS_ALT: char = '\u{f422}';
pub const COMPRESS_ARROWS_ALT: char = '\u{f78c}';
pub const CONCIERGE_BELL: char = '\u{f562}';
pub const COOKIE: char = '\u{f563}';
pub const COOKIE_BITE: char = '\u{f564}';
pub const COPY: char = '\u{f0c5}';
pub const COPYRIGHT: char = '\u{f1f9}';
pub const COUCH: char = '\u{f4b8}';
pub const CREDIT_CARD: char = '\u{f09d}';
pub const CROP: char = '\u{f125}';
pub const CROP_ALT: char = '\u{f565}';
pub const CROSS: char = '\u{f654}';
pub const CROSSHAIRS: char = '\u{f05b}';
pub const CROW: char = '\u{f520}';
pub const CROWN: char = '\u{f521}';
pub const CRUTCH: char = '\u{f7f7}';
pub const CUBE: char = '\u{f1b2}';
pub const CUBES: char = '\u{f1b3}';
pub const CUT: char = '\u{f0c4}';
pub const DATABASE: char = '\u{f1c0}';
pub const DEAF: char = '\u{f2a4}';
pub const DEMOCRAT: char = '\u{f747}';
pub const DESKTOP: char = '\u{f108}';
pub const DHARMACHAKRA: char = '\u{f655}';
pub const DIAGNOSES: char = '\u{f470}';
pub const DICE: char = '\u{f522}';
pub const DICE_D20: char = '\u{f6cf}';
pub const DICE_D6: char = '\u{f6d1}';
pub const DICE_FIVE: char = '\u{f523}';
pub const DICE_FOUR: char = '\u{f524}';
pub const DICE_ONE: char = '\u{f525}';
pub const DICE_SIX: char = '\u{f526}';
pub const DICE_THREE: char = '\u{f527}';
pub const DICE_TWO: char = '\u{f528}';
pub const DIGITAL_TACHOGRAPH: char = '\u{f566}';
pub const DIRECTIONS: char = '\u{f5eb}';
pub const DISEASE: char = '\u{f7fa}';
pub const DIVIDE: char = '\u{f529}';
pub const DIZZY: char = '\u{f567}';
pub const DNA: char = '\u{f471}';
pub const DOG: char = '\u{f6d3}';
pub const DOLLAR_SIGN: char = '\u{f155}';
pub const DOLLY: char = '\u{f472}';
pub const DOLLY_FLATBED: char = '\u{f474}';
pub const DONATE: char = '\u{f4b9}';
pub const DOOR_CLOSED: char = '\u{f52a}';
pub const DOOR_OPEN: char = '\u{f52b}';
pub const DOT_CIRCLE: char = '\u{f192}';
pub const DOVE: char = '\u{f4ba}';
pub const DOWNLOAD: char = '\u{f019}';
pub const DRAFTING_COMPASS: char = '\u{f568}';
pub const DRAGON: char = '\u{f6d5}';
pub const DRAW_POLYGON: char = '\u{f5ee}';
pub const DRUM: char = '\u{f569}';
pub const DRUM_STEELPAN: char = '\u{f56a}';
pub const DRUMSTICK_BITE: char = '\u{f6d7}';
pub const DUMBBELL: char = '\u{f44b}';
pub const DUMPSTER: char = '\u{f793}';
pub const DUMPSTER_FIRE: char = '\u{f794}';
pub const DUNGEON: char = '\u{f6d9}';
pub const EDIT: char = '\u{f044}';
pub const EGG: char = '\u{f7fb}';
pub const EJECT: char = '\u{f052}';
pub const ELLIPSIS_H: char = '\u{f141}';
pub const ELLIPSIS_V: char = '\u{f142}';
pub const ENVELOPE: char = '\u{f0e0}';
pub const ENVELOPE_OPEN: char = '\u{f2b6}';
pub const ENVELOPE_OPEN_TEXT: char = '\u{f658}';
pub const ENVELOPE_SQUARE: char = '\u{f199}';
pub const EQUALS: char = '\u{f52c}';
pub const ERASER: char = '\u{f12d}';
pub const ETHERNET: char = '\u{f796}';
pub const EURO_SIGN: char = '\u{f153}';
pub const EXCHANGE_ALT: char = '\u{f362}';
pub const EXCLAMATION: char = '\u{f12a}';
pub const EXCLAMATION_CIRCLE: char = '\u{f06a}';
pub const EXCLAMATION_TRIANGLE: char = '\u{f071}';
pub const EXPAND: char = '\u{f065}';
pub const EXPAND_ALT: char = '\u{f424}';
pub const EXPAND_ARROWS_ALT: char = '\u{f31e}';
pub const EXTERNAL_LINK_ALT: char = '\u{f35d}';
pub const EXTERNAL_LINK_SQUARE_ALT: char = '\u{f360}';
pub const EYE: char = '\u{f06e}';
pub const EYE_DROPPER: char = '\u{f1fb}';
pub const EYE_SLASH: char = '\u{f070}';
pub const FAN: char = '\u{f863}';
pub const FAST_BACKWARD: char = '\u{f049}';
pub const FAST_FORWARD: char = '\u{f050}';
pub const FAUCET: char = '\u{e005}';
pub const FAX: char = '\u{f1ac}';
pub const FEATHER: char = '\u{f52d}';
pub const FEATHER_ALT: char = '\u{f56b}';
pub const FEMALE: char = '\u{f182}';
pub const FIGHTER_JET: char = '\u{f0fb}';
pub const FILE: char = '\u{f15b}';
pub const FILE_ALT: char = '\u{f15c}';
pub const FILE_ARCHIVE: char = '\u{f1c6}';
pub const FILE_AUDIO: char = '\u{f1c7}';
pub const FILE_CODE: char = '\u{f1c9}';
pub const FILE_CONTRACT: char = '\u{f56c}';
pub const FILE_CSV: char = '\u{f6dd}';
pub const FILE_DOWNLOAD: char = '\u{f56d}';
pub const FILE_EXCEL: char = '\u{f1c3}';
pub const FILE_EXPORT: char = '\u{f56e}';
pub const FILE_IMAGE: char = '\u{f1c5}';
pub const FILE_IMPORT: char = '\u{f56f}';
pub const FILE_INVOICE: char = '\u{f570}';
pub const FILE_INVOICE_DOLLAR: char = '\u{f571}';
pub const FILE_MEDICAL: char = '\u{f477}';
pub const FILE_MEDICAL_ALT: char = '\u{f478}';
pub const FILE_PDF: char = '\u{f1c1}';
pub const FILE_POWERPOINT: char = '\u{f1c4}';
pub const FILE_PRESCRIPTION: char = '\u{f572}';
pub const FILE_SIGNATURE: char = '\u{f573}';
pub const FILE_UPLOAD: char = '\u{f574}';
pub const FILE_VIDEO: char = '\u{f1c8}';
pub const FILE_WORD: char = '\u{f1c2}';
pub const FILL: char = '\u{f575}';
pub const FILL_DRIP: char = '\u{f576}';
pub const FILM: char = '\u{f008}';
pub const FILTER: char = '\u{f0b0}';
pub const FINGERPRINT: char = '\u{f577}';
pub const FIRE: char = '\u{f06d}';
pub const FIRE_ALT: char = '\u{f7e4}';
pub const FIRE_EXTINGUISHER: char = '\u{f134}';
pub const FIRST_AID: char = '\u{f479}';
pub const FISH: char = '\u{f578}';
pub const FIST_RAISED: char = '\u{f6de}';
pub const FLAG: char = '\u{f024}';
pub const FLAG_CHECKERED: char = '\u{f11e}';
pub const FLAG_USA: char = '\u{f74d}';
pub const FLASK: char = '\u{f0c3}';
pub const FLUSHED: char = '\u{f579}';
pub const FOLDER: char = '\u{f07b}';
pub const FOLDER_MINUS: char = '\u{f65d}';
pub const FOLDER_OPEN: char = '\u{f07c}';
pub const FOLDER_PLUS: char = '\u{f65e}';
pub const FONT: char = '\u{f031}';
pub const FONT_AWESOME_LOGO_FULL: char = '\u{f4e6}';
pub const FOOTBALL_BALL: char = '\u{f44e}';
pub const FORWARD: char = '\u{f04e}';
pub const FROG: char = '\u{f52e}';
pub const FROWN: char = '\u{f119}';
pub const FROWN_OPEN: char = '\u{f57a}';
pub const FUNNEL_DOLLAR: char = '\u{f662}';
pub const FUTBOL: char = '\u{f1e3}';
pub const GAMEPAD: char = '\u{f11b}';
pub const GAS_PUMP: char = '\u{f52f}';
pub const GAVEL: char = '\u{f0e3}';
pub const GEM: char = '\u{f3a5}';
pub const GENDERLESS: char = '\u{f22d}';
pub const GHOST: char = '\u{f6e2}';
pub const GIFT: char = '\u{f06b}';
pub const GIFTS: char = '\u{f79c}';
pub const GLASS_CHEERS: char = '\u{f79f}';
pub const GLASS_MARTINI: char = '\u{f000}';
pub const GLASS_MARTINI_ALT: char = '\u{f57b}';
pub const GLASS_WHISKEY: char = '\u{f7a0}';
pub const GLASSES: char = '\u{f530}';
pub const GLOBE: char = '\u{f0ac}';
pub const GLOBE_AFRICA: char = '\u{f57c}';
pub const GLOBE_AMERICAS: char = '\u{f57d}';
pub const GLOBE_ASIA: char = '\u{f57e}';
pub const GLOBE_EUROPE: char = '\u{f7a2}';
pub const GOLF_BALL: char = '\u{f450}';
pub const GOPURAM: char = '\u{f664}';
pub const GRADUATION_CAP: char = '\u{f19d}';
pub const GREATER_THAN: char = '\u{f531}';
pub const GREATER_THAN_EQUAL: char = '\u{f532}';
pub const GRIMACE: char = '\u{f57f}';
pub const GRIN: char = '\u{f580}';
pub const GRIN_ALT: char = '\u{f581}';
pub const GRIN_BEAM: char = '\u{f582}';
pub const GRIN_BEAM_SWEAT: char = '\u{f583}';
pub const GRIN_HEARTS: char = '\u{f584}';
pub const GRIN_SQUINT: char = '\u{f585}';
pub const GRIN_SQUINT_TEARS: char = '\u{f586}';
pub const GRIN_STARS: char = '\u{f587}';
pub const GRIN_TEARS: char = '\u{f588}';
pub const GRIN_TONGUE: char = '\u{f589}';
pub const GRIN_TONGUE_SQUINT: char = '\u{f58a}';
pub const GRIN_TONGUE_WINK: char = '\u{f58b}';
pub const GRIN_WINK: char = '\u{f58c}';
pub const GRIP_HORIZONTAL: char = '\u{f58d}';
pub const GRIP_LINES: char = '\u{f7a4}';
pub const GRIP_LINES_VERTICAL: char = '\u{f7a5}';
pub const GRIP_VERTICAL: char = '\u{f58e}';
pub const GUITAR: char = '\u{f7a6}';
pub const H_SQUARE: char = '\u{f0fd}';
pub const HAMBURGER: char = '\u{f805}';
pub const HAMMER: char = '\u{f6e3}';
pub const HAMSA: char = '\u{f665}';
pub const HAND_HOLDING: char = '\u{f4bd}';
pub const HAND_HOLDING_HEART: char = '\u{f4be}';
pub const HAND_HOLDING_MEDICAL: char = '\u{e05c}';
pub const HAND_HOLDING_USD: char = '\u{f4c0}';
pub const HAND_HOLDING_WATER: char = '\u{f4c1}';
pub const HAND_LIZARD: char = '\u{f258}';
pub const HAND_MIDDLE_FINGER: char = '\u{f806}';
pub const HAND_PAPER: char = '\u{f256}';
pub const HAND_PEACE: char = '\u{f25b}';
pub const HAND_POINT_DOWN: char = '\u{f0a7}';
pub const HAND_POINT_LEFT: char = '\u{f0a5}';
pub const HAND_POINT_RIGHT: char = '\u{f0a4}';
pub const HAND_POINT_UP: char = '\u{f0a6}';
pub const HAND_POINTER: char = '\u{f25a}';
pub const HAND_ROCK: char = '\u{f255}';
pub const HAND_SCISSORS: char = '\u{f257}';
pub const HAND_SPARKLES: char = '\u{e05d}';
pub const HAND_SPOCK: char = '\u{f259}';
pub const HANDS: char = '\u{f4c2}';
pub const HANDS_HELPING: char = '\u{f4c4}';
pub const HANDS_WASH: char = '\u{e05e}';
pub const HANDSHAKE: char = '\u{f2b5}';
pub const HANDSHAKE_ALT_SLASH: char = '\u{e05f}';
pub const HANDSHAKE_SLASH: char = '\u{e060}';
pub const HANUKIAH: char = '\u{f6e6}';
pub const HARD_HAT: char = '\u{f807}';
pub const HASHTAG: char = '\u{f292}';
pub const HAT_COWBOY: char = '\u{f8c0}';
pub const HAT_COWBOY_SIDE: char = '\u{f8c1}';
pub const HAT_WIZARD: char = '\u{f6e8}';
pub const HDD: char = '\u{f0a0}';
pub const HEAD_SIDE_COUGH: char = '\u{e061}';
pub const HEAD_SIDE_COUGH_SLASH: char = '\u{e062}';
pub const HEAD_SIDE_MASK: char = '\u{e063}';
pub const HEAD_SIDE_VIRUS: char = '\u{e064}';
pub const HEADING: char = '\u{f1dc}';
pub const HEADPHONES: char = '\u{f025}';
pub const HEADPHONES_ALT: char = '\u{f58f}';
pub const HEADSET: char = '\u{f590}';
pub const HEART: char = '\u{f004}';
pub const HEART_BROKEN: char = '\u{f7a9}';
pub const HEARTBEAT: char = '\u{f21e}';
pub const HELICOPTER: char = '\u{f533}';
pub const HIGHLIGHTER: char = '\u{f591}';
pub const HIKING: char = '\u{f6ec}';
pub const HIPPO: char = '\u{f6ed}';
pub const HISTORY: char = '\u{f1da}';
pub const HOCKEY_PUCK: char = '\u{f453}';
pub const HOLLY_BERRY: char = '\u{f7aa}';
pub const HOME: char = '\u{f015}';
pub const HORSE: char = '\u{f6f0}';
pub const HORSE_HEAD: char = '\u{f7ab}';
pub const HOSPITAL: char = '\u{f0f8}';
pub const HOSPITAL_ALT: char = '\u{f47d}';
pub const HOSPITAL_SYMBOL: char = '\u{f47e}';
pub const HOSPITAL_USER: char = '\u{f80d}';
pub const HOT_TUB: char = '\u{f593}';
pub const HOTDOG: char = '\u{f80f}';
pub const HOTEL: char = '\u{f594}';
pub const HOURGLASS: char = '\u{f254}';
pub const HOURGLASS_END: char = '\u{f253}';
pub const HOURGLASS_HALF: char = '\u{f252}';
pub const HOURGLASS_START: char = '\u{f251}';
pub const HOUSE_DAMAGE: char = '\u{f6f1}';
pub const HOUSE_USER: char = '\u{e065}';
pub const HRYVNIA: char = '\u{f6f2}';
pub const I_CURSOR: char = '\u{f246}';
pub const ICE_CREAM: char = '\u{f810}';
pub const ICICLES: char = '\u{f7ad}';
pub const ICONS: char = '\u{f86d}';
pub const ID_BADGE: char = '\u{f2c1}';
pub const ID_CARD: char = '\u{f2c2}';
pub const ID_CARD_ALT: char = '\u{f47f}';
pub const IGLOO: char = '\u{f7ae}';
pub const IMAGE: char = '\u{f03e}';
pub const IMAGES: char = '\u{f302}';
pub const INBOX: char = '\u{f01c}';
pub const INDENT: char = '\u{f03c}';
pub const INDUSTRY: char = '\u{f275}';
pub const INFINITY: char = '\u{f534}';
pub const INFO: char = '\u{f129}';
pub const INFO_CIRCLE: char = '\u{f05a}';
pub const ITALIC: char = '\u{f033}';
pub const JEDI: char = '\u{f669}';
pub const JOINT: char = '\u{f595}';
pub const JOURNAL_WHILLS: char = '\u{f66a}';
pub const KAABA: char = '\u{f66b}';
pub const KEY: char = '\u{f084}';
pub const KEYBOARD: char = '\u{f11c}';
pub const KHANDA: char = '\u{f66d}';
pub const KISS: char = '\u{f596}';
pub const KISS_BEAM: char = '\u{f597}';
pub const KISS_WINK_HEART: char = '\u{f598}';
pub const KIWI_BIRD: char = '\u{f535}';
pub const LANDMARK: char = '\u{f66f}';
pub const LANGUAGE: char = '\u{f1ab}';
pub const LAPTOP: char = '\u{f109}';
pub const LAPTOP_CODE: char = '\u{f5fc}';
pub const LAPTOP_HOUSE: char = '\u{e066}';
pub const LAPTOP_MEDICAL: char = '\u{f812}';
pub const LAUGH: char = '\u{f599}';
pub const LAUGH_BEAM: char = '\u{f59a}';
pub const LAUGH_SQUINT: char = '\u{f59b}';
pub const LAUGH_WINK: char = '\u{f59c}';
pub const LAYER_GROUP: char = '\u{f5fd}';
pub const LEAF: char = '\u{f06c}';
pub const LEMON: char = '\u{f094}';
pub const LESS_THAN: char = '\u{f536}';
pub const LESS_THAN_EQUAL: char = '\u{f537}';
pub const LEVEL_DOWN_ALT: char = '\u{f3be}';
pub const LEVEL_UP_ALT: char = '\u{f3bf}';
pub const LIFE_RING: char = '\u{f1cd}';
pub const LIGHTBULB: char = '\u{f0eb}';
pub const LINK: char = '\u{f0c1}';
pub const LIRA_SIGN: char = '\u{f195}';
pub const LIST: char = '\u{f03a}';
pub const LIST_ALT: char = '\u{f022}';
pub const LIST_OL: char = '\u{f0cb}';
pub const LIST_UL: char = '\u{f0ca}';
pub const LOCATION_ARROW: char = '\u{f124}';
pub const LOCK: char = '\u{f023}';
pub const LOCK_OPEN: char = '\u{f3c1}';
pub const LONG_ARROW_ALT_DOWN: char = '\u{f309}';
pub const LONG_ARROW_ALT_LEFT: char = '\u{f30a}';
pub const LONG_ARROW_ALT_RIGHT: char = '\u{f30b}';
pub const LONG_ARROW_ALT_UP: char = '\u{f30c}';
pub const LOW_VISION: char = '\u{f2a8}';
pub const LUGGAGE_CART: char = '\u{f59d}';
pub const LUNGS: char = '\u{f604}';
pub const LUNGS_VIRUS: char = '\u{e067}';
pub const MAGIC: char = '\u{f0d0}';
pub const MAGNET: char = '\u{f076}';
pub const MAIL_BULK: char = '\u{f674}';
pub const MALE: char = '\u{f183}';
pub const MAP: char = '\u{f279}';
pub const MAP_MARKED: char = '\u{f59f}';
pub const MAP_MARKED_ALT: char = '\u{f5a0}';
pub const MAP_MARKER: char = '\u{f041}';
pub const MAP_MARKER_ALT: char = '\u{f3c5}';
pub const MAP_PIN: char = '\u{f276}';
pub const MAP_SIGNS: char = '\u{f277}';
pub const MARKER: char = '\u{f5a1}';
pub const MARS: char = '\u{f222}';
pub const MARS_DOUBLE: char = '\u{f227}';
pub const MARS_STROKE: char = '\u{f229}';
pub const MARS_STROKE_H: char = '\u{f22b}';
pub const MARS_STROKE_V: char = '\u{f22a}';
pub const MASK: char = '\u{f6fa}';
pub const MEDAL: char = '\u{f5a2}';
pub const MEDKIT: char = '\u{f0fa}';
pub const MEH: char = '\u{f11a}';
pub const MEH_BLANK: char = '\u{f5a4}';
pub const MEH_ROLLING_EYES: char = '\u{f5a5}';
pub const MEMORY: char = '\u{f538}';
pub const MENORAH: char = '\u{f676}';
pub const MERCURY: char = '\u{f223}';
pub const METEOR: char = '\u{f753}';
pub const MICROCHIP: char = '\u{f2db}';
pub const MICROPHONE: char = '\u{f130}';
pub const MICROPHONE_ALT: char = '\u{f3c9}';
pub const MICROPHONE_ALT_SLASH: char = '\u{f539}';
pub const MICROPHONE_SLASH: char = '\u{f131}';
pub const MICROSCOPE: char = '\u{f610}';
pub const MINUS: char = '\u{f068}';
pub const MINUS_CIRCLE: char = '\u{f056}';
pub const MINUS_SQUARE: char = '\u{f146}';
pub const MITTEN: char = '\u{f7b5}';
pub const MOBILE: char = '\u{f10b}';
pub const MOBILE_ALT: char = '\u{f3cd}';
pub const MONEY_BILL: char = '\u{f0d6}';
pub const MONEY_BILL_ALT: char = '\u{f3d1}';
pub const MONEY_BILL_WAVE: char = '\u{f53a}';
pub const MONEY_BILL_WAVE_ALT: char = '\u{f53b}';
pub const MONEY_CHECK: char = '\u{f53c}';
pub const MONEY_CHECK_ALT: char = '\u{f53d}';
pub const MONUMENT: char = '\u{f5a6}';
pub const MOON: char = '\u{f186}';
pub const MORTAR_PESTLE: char = '\u{f5a7}';
pub const MOSQUE: char = '\u{f678}';
pub const MOTORCYCLE: char = '\u{f21c}';
pub const MOUNTAIN: char = '\u{f6fc}';
pub const MOUSE: char = '\u{f8cc}';
pub const MOUSE_POINTER: char = '\u{f245}';
pub const MUG_HOT: char = '\u{f7b6}';
pub const MUSIC: char = '\u{f001}';
pub const NETWORK_WIRED: char = '\u{f6ff}';
pub const NEUTER: char = '\u{f22c}';
pub const NEWSPAPER: char = '\u{f1ea}';
pub const NOT_EQUAL: char = '\u{f53e}';
pub const NOTES_MEDICAL: char = '\u{f481}';
pub const OBJECT_GROUP: char = '\u{f247}';
pub const OBJECT_UNGROUP: char = '\u{f248}';
pub const OIL_CAN: char = '\u{f613}';
pub const OM: char = '\u{f679}';
pub const OTTER: char = '\u{f700}';
pub const OUTDENT: char = '\u{f03b}';
pub const PAGER: char = '\u{f815}';
pub const PAINT_BRUSH: char = '\u{f1fc}';
pub const PAINT_ROLLER: char = '\u{f5aa}';
pub const PALETTE: char = '\u{f53f}';
pub const PALLET: char = '\u{f482}';
pub const PAPER_PLANE: char = '\u{f1d8}';
pub const PAPERCLIP: char = '\u{f0c6}';
pub const PARACHUTE_BOX: char = '\u{f4cd}';
pub const PARAGRAPH: char = '\u{f1dd}';
pub const PARKING: char = '\u{f540}';
pub const PASSPORT: char = '\u{f5ab}';
pub const PASTAFARIANISM: char = '\u{f67b}';
pub const PASTE: char = '\u{f0ea}';
pub const PAUSE: char = '\u{f04c}';
pub const PAUSE_CIRCLE: char = '\u{f28b}';
pub const PAW: char = '\u{f1b0}';
pub const PEACE: char = '\u{f67c}';
pub const PEN: char = '\u{f304}';
pub const PEN_ALT: char = '\u{f305}';
pub const PEN_FANCY: char = '\u{f5ac}';
pub const PEN_NIB: char = '\u{f5ad}';
pub const PEN_SQUARE: char = '\u{f14b}';
pub const PENCIL_ALT: char = '\u{f303}';
pub const PENCIL_RULER: char = '\u{f5ae}';
pub const PEOPLE_ARROWS: char = '\u{e068}';
pub const PEOPLE_CARRY: char = '\u{f4ce}';
pub const PEPPER_HOT: char = '\u{f816}';
pub const PERCENT: char = '\u{f295}';
pub const PERCENTAGE: char = '\u{f541}';
pub const PERSON_BOOTH: char = '\u{f756}';
pub const PHONE: char = '\u{f095}';
pub const PHONE_ALT: char = '\u{f879}';
pub const PHONE_SLASH: char = '\u{f3dd}';
pub const PHONE_SQUARE: char = '\u{f098}';
pub const PHONE_SQUARE_ALT: char = '\u{f87b}';
pub const PHONE_VOLUME: char = '\u{f2a0}';
pub const PHOTO_VIDEO: char = '\u{f87c}';
pub const PIGGY_BANK: char = '\u{f4d3}';
pub const PILLS: char = '\u{f484}';
pub const PIZZA_SLICE: char = '\u{f818}';
pub const PLACE_OF_WORSHIP: char = '\u{f67f}';
pub const PLANE: char = '\u{f072}';
pub const PLANE_ARRIVAL: char = '\u{f5af}';
pub const PLANE_DEPARTURE: char = '\u{f5b0}';
pub const PLANE_SLASH: char = '\u{e069}';
pub const PLAY: char = '\u{f04b}';
pub const PLAY_CIRCLE: char = '\u{f144}';
pub const PLUG: char = '\u{f1e6}';
pub const PLUS: char = '\u{f067}';
pub const PLUS_CIRCLE: char = '\u{f055}';
pub const PLUS_SQUARE: char = '\u{f0fe}';
pub const PODCAST: char = '\u{f2ce}';
pub const POLL: char = '\u{f681}';
pub const POLL_H: char = '\u{f682}';
pub const POO: char = '\u{f2fe}';
pub const POO_STORM: char = '\u{f75a}';
pub const POOP: char = '\u{f619}';
pub const PORTRAIT: char = '\u{f3e0}';
pub const POUND_SIGN: char = '\u{f154}';
pub const POWER_OFF: char = '\u{f011}';
pub const PRAY: char = '\u{f683}';
pub const PRAYING_HANDS: char = '\u{f684}';
pub const PRESCRIPTION: char = '\u{f5b1}';
pub const PRESCRIPTION_BOTTLE: char = '\u{f485}';
pub const PRESCRIPTION_BOTTLE_ALT: char = '\u{f486}';
pub const PRINT: char = '\u{f02f}';
pub const PROCEDURES: char = '\u{f487}';
pub const PROJECT_DIAGRAM: char = '\u{f542}';
pub const PUMP_MEDICAL: char = '\u{e06a}';
pub const PUMP_SOAP: char = '\u{e06b}';
pub const PUZZLE_PIECE: char = '\u{f12e}';
pub const QRCODE: char = '\u{f029}';
pub const QUESTION: char = '\u{f128}';
pub const QUESTION_CIRCLE: char = '\u{f059}';
pub const QUIDDITCH: char = '\u{f458}';
pub const QUOTE_LEFT: char = '\u{f10d}';
pub const QUOTE_RIGHT: char = '\u{f10e}';
pub const QURAN: char = '\u{f687}';
pub const RADIATION: char = '\u{f7b9}';
pub const RADIATION_ALT: char = '\u{f7ba}';
pub const RAINBOW: char = '\u{f75b}';
pub const RANDOM: char = '\u{f074}';
pub const RECEIPT: char = '\u{f543}';
pub const RECORD_VINYL: char = '\u{f8d9}';
pub const RECYCLE: char = '\u{f1b8}';
pub const REDO: char = '\u{f01e}';
pub const REDO_ALT: char = '\u{f2f9}';
pub const REGISTERED: char = '\u{f25d}';
pub const REMOVE_FORMAT: char = '\u{f87d}';
pub const REPLY: char = '\u{f3e5}';
pub const REPLY_ALL: char = '\u{f122}';
pub const REPUBLICAN: char = '\u{f75e}';
pub const RESTROOM: char = '\u{f7bd}';
pub const RETWEET: char = '\u{f079}';
pub const RIBBON: char = '\u{f4d6}';
pub const RING: char = '\u{f70b}';
pub const ROAD: char = '\u{f018}';
pub const ROBOT: char = '\u{f544}';
pub const ROCKET: char = '\u{f135}';
pub const ROUTE: char = '\u{f4d7}';
pub const RSS: char = '\u{f09e}';
pub const RSS_SQUARE: char = '\u{f143}';
pub const RUBLE_SIGN: char = '\u{f158}';
pub const RULER: char = '\u{f545}';
pub const RULER_COMBINED: char = '\u{f546}';
pub const RULER_HORIZONTAL: char = '\u{f547}';
pub const RULER_VERTICAL: char = '\u{f548}';
pub const RUNNING: char = '\u{f70c}';
pub const RUPEE_SIGN: char = '\u{f156}';
pub const SAD_CRY: char = '\u{f5b3}';
pub const SAD_TEAR: char = '\u{f5b4}';
pub const SATELLITE: char = '\u{f7bf}';
pub const SATELLITE_DISH: char = '\u{f7c0}';
pub const SAVE: char = '\u{f0c7}';
pub const SCHOOL: char = '\u{f549}';
pub const SCREWDRIVER: char = '\u{f54a}';
pub const SCROLL: char = '\u{f70e}';
pub const SD_CARD: char = '\u{f7c2}';
pub const SEARCH: char = '\u{f002}';
pub const SEARCH_DOLLAR: char = '\u{f688}';
pub const SEARCH_LOCATION: char = '\u{f689}';
pub const SEARCH_MINUS: char = '\u{f010}';
pub const SEARCH_PLUS: char = '\u{f00e}';
pub const SEEDLING: char = '\u{f4d8}';
pub const SERVER: char = '\u{f233}';
pub const SHAPES: char = '\u{f61f}';
pub const SHARE: char = '\u{f064}';
pub const SHARE_ALT: char = '\u{f1e0}';
pub const SHARE_ALT_SQUARE: char = '\u{f1e1}';
pub const SHARE_SQUARE: char = '\u{f14d}';
pub const SHEKEL_SIGN: char = '\u{f20b}';
pub const SHIELD_ALT: char = '\u{f3ed}';
pub const SHIELD_VIRUS: char = '\u{e06c}';
pub const SHIP: char = '\u{f21a}';
pub const SHIPPING_FAST: char = '\u{f48b}';
pub const SHOE_PRINTS: char = '\u{f54b}';
pub const SHOPPING_BAG: char = '\u{f290}';
pub const SHOPPING_BASKET: char = '\u{f291}';
pub const SHOPPING_CART: char = '\u{f07a}';
pub const SHOWER: char = '\u{f2cc}';
pub const SHUTTLE_VAN: char = '\u{f5b6}';
pub const SIGN: char = '\u{f4d9}';
pub const SIGN_IN_ALT: char = '\u{f2f6}';
pub const SIGN_LANGUAGE: char = '\u{f2a7}';
pub const SIGN_OUT_ALT: char = '\u{f2f5}';
pub const SIGNAL: char = '\u{f012}';
pub const SIGNATURE: char = '\u{f5b7}';
pub const SIM_CARD: char = '\u{f7c4}';
pub const SINK: char = '\u{e06d}';
pub const SITEMAP: char = '\u{f0e8}';
pub const SKATING: char = '\u{f7c5}';
pub const SKIING: char = '\u{f7c9}';
pub const SKIING_NORDIC: char = '\u{f7ca}';
pub const SKULL: char = '\u{f54c}';
pub const SKULL_CROSSBONES: char = '\u{f714}';
pub const SLASH: char = '\u{f715}';
pub const SLEIGH: char = '\u{f7cc}';
pub const SLIDERS_H: char = '\u{f1de}';
pub const SMILE: char = '\u{f118}';
pub const SMILE_BEAM: char = '\u{f5b8}';
pub const SMILE_WINK: char = '\u{f4da}';
pub const SMOG: char = '\u{f75f}';
pub const SMOKING: char = '\u{f48d}';
pub const SMOKING_BAN: char = '\u{f54d}';
pub const SMS: char = '\u{f7cd}';
pub const SNOWBOARDING: char = '\u{f7ce}';
pub const SNOWFLAKE: char = '\u{f2dc}';
pub const SNOWMAN: char = '\u{f7d0}';
pub const SNOWPLOW: char = '\u{f7d2}';
pub const SOAP: char = '\u{e06e}';
pub const SOCKS: char = '\u{f696}';
pub const SOLAR_PANEL: char = '\u{f5ba}';
pub const SORT: char = '\u{f0dc}';
pub const SORT_ALPHA_DOWN: char = '\u{f15d}';
pub const SORT_ALPHA_DOWN_ALT: char = '\u{f881}';
pub const SORT_ALPHA_UP: char = '\u{f15e}';
pub const SORT_ALPHA_UP_ALT: char = '\u{f882}';
pub const SORT_AMOUNT_DOWN: char = '\u{f160}';
pub const SORT_AMOUNT_DOWN_ALT: char = '\u{f884}';
pub const SORT_AMOUNT_UP: char = '\u{f161}';
pub const SORT_AMOUNT_UP_ALT: char = '\u{f885}';
pub const SORT_DOWN: char = '\u{f0dd}';
pub const SORT_NUMERIC_DOWN: char = '\u{f162}';
pub const SORT_NUMERIC_DOWN_ALT: char = '\u{f886}';
pub const SORT_NUMERIC_UP: char = '\u{f163}';
pub const SORT_NUMERIC_UP_ALT: char = '\u{f887}';
pub const SORT_UP: char = '\u{f0de}';
pub const SPA: char = '\u{f5bb}';
pub const SPACE_SHUTTLE: char = '\u{f197}';
pub const SPELL_CHECK: char = '\u{f891}';
pub const SPIDER: char = '\u{f717}';
pub const SPINNER: char = '\u{f110}';
pub const SPLOTCH: char = '\u{f5bc}';
pub const SPRAY_CAN: char = '\u{f5bd}';
pub const SQUARE: char = '\u{f0c8}';
pub const SQUARE_FULL: char = '\u{f45c}';
pub const SQUARE_ROOT_ALT: char = '\u{f698}';
pub const STAMP: char = '\u{f5bf}';
pub const STAR: char = '\u{f005}';
pub const STAR_AND_CRESCENT: char = '\u{f699}';
pub const STAR_HALF: char = '\u{f089}';
pub const STAR_HALF_ALT: char = '\u{f5c0}';
pub const STAR_OF_DAVID: char = '\u{f69a}';
pub const STAR_OF_LIFE: char = '\u{f621}';
pub const STEP_BACKWARD: char = '\u{f048}';
pub const STEP_FORWARD: char = '\u{f051}';
pub const STETHOSCOPE: char = '\u{f0f1}';
pub const STICKY_NOTE: char = '\u{f249}';
pub const STOP: char = '\u{f04d}';
pub const STOP_CIRCLE: char = '\u{f28d}';
pub const STOPWATCH: char = '\u{f2f2}';
pub const STOPWATCH_20: char = '\u{e06f}';
pub const STORE: char = '\u{f54e}';
pub const STORE_ALT: char = '\u{f54f}';
pub const STORE_ALT_SLASH: char = '\u{e070}';
pub const STORE_SLASH: char = '\u{e071}';
pub const STREAM: char = '\u{f550}';
pub const STREET_VIEW: char = '\u{f21d}';
pub const STRIKETHROUGH: char = '\u{f0cc}';
pub const STROOPWAFEL: char = '\u{f551}';
pub const SUBSCRIPT: char = '\u{f12c}';
pub const SUBWAY: char = '\u{f239}';
pub const SUITCASE: char = '\u{f0f2}';
pub const SUITCASE_ROLLING: char = '\u{f5c1}';
pub const SUN: char = '\u{f185}';
pub const SUPERSCRIPT: char = '\u{f12b}';
pub const SURPRISE: char = '\u{f5c2}';
pub const SWATCHBOOK: char = '\u{f5c3}';
pub const SWIMMER: char = '\u{f5c4}';
pub const SWIMMING_POOL: char = '\u{f5c5}';
pub const SYNAGOGUE: char = '\u{f69b}';
pub const SYNC: char = '\u{f021}';
pub const SYNC_ALT: char = '\u{f2f1}';
pub const SYRINGE: char = '\u{f48e}';
pub const TABLE: char = '\u{f0ce}';
pub const TABLE_TENNIS: char = '\u{f45d}';
pub const TABLET: char = '\u{f10a}';
pub const TABLET_ALT: char = '\u{f3fa}';
pub const TABLETS: char = '\u{f490}';
pub const TACHOMETER_ALT: char = '\u{f3fd}';
pub const TAG: char = '\u{f02b}';
pub const TAGS: char = '\u{f02c}';
pub const TAPE: char = '\u{f4db}';
pub const TASKS: char = '\u{f0ae}';
pub const TAXI: char = '\u{f1ba}';
pub const TEETH: char = '\u{f62e}';
pub const TEETH_OPEN: char = '\u{f62f}';
pub const TEMPERATURE_HIGH: char = '\u{f769}';
pub const TEMPERATURE_LOW: char = '\u{f76b}';
pub const TENGE: char = '\u{f7d7}';
pub const TERMINAL: char = '\u{f120}';
pub const TEXT_HEIGHT: char = '\u{f034}';
pub const TEXT_WIDTH: char = '\u{f035}';
pub const TH: char = '\u{f00a}';
pub const TH_LARGE: char = '\u{f009}';
pub const TH_LIST: char = '\u{f00b}';
pub const THEATER_MASKS: char = '\u{f630}';
pub const THERMOMETER: char = '\u{f491}';
pub const THERMOMETER_EMPTY: char = '\u{f2cb}';
pub const THERMOMETER_FULL: char = '\u{f2c7}';
pub const THERMOMETER_HALF: char = '\u{f2c9}';
pub const THERMOMETER_QUARTER: char = '\u{f2ca}';
pub const THERMOMETER_THREE_QUARTERS: char = '\u{f2c8}';
pub const THUMBS_DOWN: char = '\u{f165}';
pub const THUMBS_UP: char = '\u{f164}';
pub const THUMBTACK: char = '\u{f08d}';
pub const TICKET_ALT: char = '\u{f3ff}';
pub const TIMES: char = '\u{f00d}';
pub const TIMES_CIRCLE: char = '\u{f057}';
pub const TINT: char = '\u{f043}';
pub const TINT_SLASH: char = '\u{f5c7}';
pub const TIRED: char = '\u{f5c8}';
pub const TOGGLE_OFF: char = '\u{f204}';
pub const TOGGLE_ON: char = '\u{f205}';
pub const TOILET: char = '\u{f7d8}';
pub const TOILET_PAPER: char = '\u{f71e}';
pub const TOILET_PAPER_SLASH: char = '\u{e072}';
pub const TOOLBOX: char = '\u{f552}';
pub const TOOLS: char = '\u{f7d9}';
pub const TOOTH: char = '\u{f5c9}';
pub const TORAH: char = '\u{f6a0}';
pub const TORII_GATE: char = '\u{f6a1}';
pub const TRACTOR: char = '\u{f722}';
pub const TRADEMARK: char = '\u{f25c}';
pub const TRAFFIC_LIGHT: char = '\u{f637}';
pub const TRAILER: char = '\u{e041}';
pub const TRAIN: char = '\u{f238}';
pub const TRAM: char = '\u{f7da}';
pub const TRANSGENDER: char = '\u{f224}';
pub const TRANSGENDER_ALT: char = '\u{f225}';
pub const TRASH: char = '\u{f1f8}';
pub const TRASH_ALT: char = '\u{f2ed}';
pub const TRASH_RESTORE: char = '\u{f829}';
pub const TRASH_RESTORE_ALT: char = '\u{f82a}';
pub const TREE: char = '\u{f1bb}';
pub const TROPHY: char = '\u{f091}';
pub const TRUCK: char = '\u{f0d1}';
pub const TRUCK_LOADING: char = '\u{f4de}';
pub const TRUCK_MONSTER: char = '\u{f63b}';
pub const TRUCK_MOVING: char = '\u{f4df}';
pub const TRUCK_PICKUP: char = '\u{f63c}';
pub const TSHIRT: char = '\u{f553}';
pub const TTY: char = '\u{f1e4}';
pub const TV: char = '\u{f26c}';
pub const UMBRELLA: char = '\u{f0e9}';
pub const UMBRELLA_BEACH: char = '\u{f5ca}';
pub const UNDERLINE: char = '\u{f0cd}';
pub const UNDO: char = '\u{f0e2}';
pub const UNDO_ALT: char = '\u{f2ea}';
pub const UNIVERSAL_ACCESS: char = '\u{f29a}';
pub const UNIVERSITY: char = '\u{f19c}';
pub const UNLINK: char = '\u{f127}';
pub const UNLOCK: char = '\u{f09c}';
pub const UNLOCK_ALT: char = '\u{f13e}';
pub const UPLOAD: char = '\u{f093}';
pub const USER: char = '\u{f007}';
pub const USER_ALT: char = '\u{f406}';
pub const USER_ALT_SLASH: char = '\u{f4fa}';
pub const USER_ASTRONAUT: char = '\u{f4fb}';
pub const USER_CHECK: char = '\u{f4fc}';
pub const USER_CIRCLE: char = '\u{f2bd}';
pub const USER_CLOCK: char = '\u{f4fd}';
pub const USER_COG: char = '\u{f4fe}';
pub const USER_EDIT: char = '\u{f4ff}';
pub const USER_FRIENDS: char = '\u{f500}';
pub const USER_GRADUATE: char = '\u{f501}';
pub const USER_INJURED: char = '\u{f728}';
pub const USER_LOCK: char = '\u{f502}';
pub const USER_MD: char = '\u{f0f0}';
pub const USER_MINUS: char = '\u{f503}';
pub const USER_NINJA: char = '\u{f504}';
pub const USER_NURSE: char = '\u{f82f}';
pub const USER_PLUS: char = '\u{f234}';
pub const USER_SECRET: char = '\u{f21b}';
pub const USER_SHIELD: char = '\u{f505}';
pub const USER_SLASH: char = '\u{f506}';
pub const USER_TAG: char = '\u{f507}';
pub const USER_TIE: char = '\u{f508}';
pub const USER_TIMES: char = '\u{f235}';
pub const USERS: char = '\u{f0c0}';
pub const USERS_COG: char = '\u{f509}';
pub const USERS_SLASH: char = '\u{e073}';
pub const UTENSIL_SPOON: char = '\u{f2e5}';
pub const UTENSILS: char = '\u{f2e7}';
pub const VECTOR_SQUARE: char = '\u{f5cb}';
pub const VENUS: char = '\u{f221}';
pub const VENUS_DOUBLE: char = '\u{f226}';
pub const VENUS_MARS: char = '\u{f228}';
pub const VEST: char = '\u{e085}';
pub const VEST_PATCHES: char = '\u{e086}';
pub const VIAL: char = '\u{f492}';
pub const VIALS: char = '\u{f493}';
pub const VIDEO: char = '\u{f03d}';
pub const VIDEO_SLASH: char = '\u{f4e2}';
pub const VIHARA: char = '\u{f6a7}';
pub const VIRUS: char = '\u{e074}';
pub const VIRUS_SLASH: char = '\u{e075}';
pub const VIRUSES: char = '\u{e076}';
pub const VOICEMAIL: char = '\u{f897}';
pub const VOLLEYBALL_BALL: char = '\u{f45f}';
pub const VOLUME_DOWN: char = '\u{f027}';
pub const VOLUME_MUTE: char = '\u{f6a9}';
pub const VOLUME_OFF: char = '\u{f026}';
pub const VOLUME_UP: char = '\u{f028}';
pub const VOTE_YEA: char = '\u{f772}';
pub const VR_CARDBOARD: char = '\u{f729}';
pub const WALKING: char = '\u{f554}';
pub const WALLET: char = '\u{f555}';
pub const WAREHOUSE: char = '\u{f494}';
pub const WATER: char = '\u{f773}';
pub const WAVE_SQUARE: char = '\u{f83e}';
pub const WEIGHT: char = '\u{f496}';
pub const WEIGHT_HANGING: char = '\u{f5cd}';
pub const WHEELCHAIR: char = '\u{f193}';
pub const WIFI: char = '\u{f1eb}';
pub const WIND: char = '\u{f72e}';
pub const WINDOW_CLOSE: char = '\u{f410}';
pub const WINDOW_MAXIMIZE: char = '\u{f2d0}';
pub const WINDOW_MINIMIZE: char = '\u{f2d1}';
pub const WINDOW_RESTORE: char = '\u{f2d2}';
pub const WINE_BOTTLE: char = '\u{f72f}';
pub const WINE_GLASS: char = '\u{f4e3}';
pub const WINE_GLASS_ALT: char = '\u{f5ce}';
pub const WON_SIGN: char = '\u{f159}';
pub const WRENCH: char = '\u{f0ad}';
pub const X_RAY: char = '\u{f497}';
pub const YEN_SIGN: char = '\u{f157}';
pub const YIN_YANG: char = '\u{f6ad}';

pub const fn symbols() -> [char; 1002] {
    [
        AD,
        ADDRESS_BOOK,
        ADDRESS_CARD,
        ADJUST,
        AIR_FRESHENER,
        ALIGN_CENTER,
        ALIGN_JUSTIFY,
        ALIGN_LEFT,
        ALIGN_RIGHT,
        ALLERGIES,
        AMBULANCE,
        AMERICAN_SIGN_LANGUAGE_INTERPRETING,
        ANCHOR,
        ANGLE_DOUBLE_DOWN,
        ANGLE_DOUBLE_LEFT,
        ANGLE_DOUBLE_RIGHT,
        ANGLE_DOUBLE_UP,
        ANGLE_DOWN,
        ANGLE_LEFT,
        ANGLE_RIGHT,
        ANGLE_UP,
        ANGRY,
        ANKH,
        APPLE_ALT,
        ARCHIVE,
        ARCHWAY,
        ARROW_ALT_CIRCLE_DOWN,
        ARROW_ALT_CIRCLE_LEFT,
        ARROW_ALT_CIRCLE_RIGHT,
        ARROW_ALT_CIRCLE_UP,
        ARROW_CIRCLE_DOWN,
        ARROW_CIRCLE_LEFT,
        ARROW_CIRCLE_RIGHT,
        ARROW_CIRCLE_UP,
        ARROW_DOWN,
        ARROW_LEFT,
        ARROW_RIGHT,
        ARROW_UP,
        ARROWS_ALT,
        ARROWS_ALT_H,
        ARROWS_ALT_V,
        ASSISTIVE_LISTENING_SYSTEMS,
        ASTERISK,
        AT,
        ATLAS,
        ATOM,
        AUDIO_DESCRIPTION,
        AWARD,
        BABY,
        BABY_CARRIAGE,
        BACKSPACE,
        BACKWARD,
        BACON,
        BACTERIA,
        BACTERIUM,
        BAHAI,
        BALANCE_SCALE,
        BALANCE_SCALE_LEFT,
        BALANCE_SCALE_RIGHT,
        BAN,
        BAND_AID,
        BARCODE,
        BARS,
        BASEBALL_BALL,
        BASKETBALL_BALL,
        BATH,
        BATTERY_EMPTY,
        BATTERY_FULL,
        BATTERY_HALF,
        BATTERY_QUARTER,
        BATTERY_THREE_QUARTERS,
        BED,
        BEER,
        BELL,
        BELL_SLASH,
        BEZIER_CURVE,
        BIBLE,
        BICYCLE,
        BIKING,
        BINOCULARS,
        BIOHAZARD,
        BIRTHDAY_CAKE,
        BLENDER,
        BLENDER_PHONE,
        BLIND,
        BLOG,
        BOLD,
        BOLT,
        BOMB,
        BONE,
        BONG,
        BOOK,
        BOOK_DEAD,
        BOOK_MEDICAL,
        BOOK_OPEN,
        BOOK_READER,
        BOOKMARK,
        BORDER_ALL,
        BORDER_NONE,
        BORDER_STYLE,
        BOWLING_BALL,
        BOX,
        BOX_OPEN,
        BOX_TISSUE,
        BOXES,
        BRAILLE,
        BRAIN,
        BREAD_SLICE,
        BRIEFCASE,
        BRIEFCASE_MEDICAL,
        BROADCAST_TOWER,
        BROOM,
        BRUSH,
        BUG,
        BUILDING,
        BULLHORN,
        BULLSEYE,
        BURN,
        BUS,
        BUS_ALT,
        BUSINESS_TIME,
        CALCULATOR,
        CALENDAR,
        CALENDAR_ALT,
        CALENDAR_CHECK,
        CALENDAR_DAY,
        CALENDAR_MINUS,
        CALENDAR_PLUS,
        CALENDAR_TIMES,
        CALENDAR_WEEK,
        CAMERA,
        CAMERA_RETRO,
        CAMPGROUND,
        CANDY_CANE,
        CANNABIS,
        CAPSULES,
        CAR,
        CAR_ALT,
        CAR_BATTERY,
        CAR_CRASH,
        CAR_SIDE,
        CARAVAN,
        CARET_DOWN,
        CARET_LEFT,
        CARET_RIGHT,
        CARET_SQUARE_DOWN,
        CARET_SQUARE_LEFT,
        CARET_SQUARE_RIGHT,
        CARET_SQUARE_UP,
        CARET_UP,
        CARROT,
        CART_ARROW_DOWN,
        CART_PLUS,
        CASH_REGISTER,
        CAT,
        CERTIFICATE,
        CHAIR,
        CHALKBOARD,
        CHALKBOARD_TEACHER,
        CHARGING_STATION,
        CHART_AREA,
        CHART_BAR,
        CHART_LINE,
        CHART_PIE,
        CHECK,
        CHECK_CIRCLE,
        CHECK_DOUBLE,
        CHECK_SQUARE,
        CHEESE,
        CHESS,
        CHESS_BISHOP,
        CHESS_BOARD,
        CHESS_KING,
        CHESS_KNIGHT,
        CHESS_PAWN,
        CHESS_QUEEN,
        CHESS_ROOK,
        CHEVRON_CIRCLE_DOWN,
        CHEVRON_CIRCLE_LEFT,
        CHEVRON_CIRCLE_RIGHT,
        CHEVRON_CIRCLE_UP,
        CHEVRON_DOWN,
        CHEVRON_LEFT,
        CHEVRON_RIGHT,
        CHEVRON_UP,
        CHILD,
        CHURCH,
        CIRCLE,
        CIRCLE_NOTCH,
        CITY,
        CLINIC_MEDICAL,
        CLIPBOARD,
        CLIPBOARD_CHECK,
        CLIPBOARD_LIST,
        CLOCK,
        CLONE,
        CLOSED_CAPTIONING,
        CLOUD,
        CLOUD_DOWNLOAD_ALT,
        CLOUD_MEATBALL,
        CLOUD_MOON,
        CLOUD_MOON_RAIN,
        CLOUD_RAIN,
        CLOUD_SHOWERS_HEAVY,
        CLOUD_SUN,
        CLOUD_SUN_RAIN,
        CLOUD_UPLOAD_ALT,
        COCKTAIL,
        CODE,
        CODE_BRANCH,
        COFFEE,
        COG,
        COGS,
        COINS,
        COLUMNS,
        COMMENT,
        COMMENT_ALT,
        COMMENT_DOLLAR,
        COMMENT_DOTS,
        COMMENT_MEDICAL,
        COMMENT_SLASH,
        COMMENTS,
        COMMENTS_DOLLAR,
        COMPACT_DISC,
        COMPASS,
        COMPRESS,
        COMPRESS_ALT,
        COMPRESS_ARROWS_ALT,
        CONCIERGE_BELL,
        COOKIE,
        COOKIE_BITE,
        COPY,
        COPYRIGHT,
        COUCH,
        CREDIT_CARD,
        CROP,
        CROP_ALT,
        CROSS,
        CROSSHAIRS,
        CROW,
        CROWN,
        CRUTCH,
        CUBE,
        CUBES,
        CUT,
        DATABASE,
        DEAF,
        DEMOCRAT,
        DESKTOP,
        DHARMACHAKRA,
        DIAGNOSES,
        DICE,
        DICE_D20,
        DICE_D6,
        DICE_FIVE,
        DICE_FOUR,
        DICE_ONE,
        DICE_SIX,
        DICE_THREE,
        DICE_TWO,
        DIGITAL_TACHOGRAPH,
        DIRECTIONS,
        DISEASE,
        DIVIDE,
        DIZZY,
        DNA,
        DOG,
        DOLLAR_SIGN,
        DOLLY,
        DOLLY_FLATBED,
        DONATE,
        DOOR_CLOSED,
        DOOR_OPEN,
        DOT_CIRCLE,
        DOVE,
        DOWNLOAD,
        DRAFTING_COMPASS,
        DRAGON,
        DRAW_POLYGON,
        DRUM,
        DRUM_STEELPAN,
        DRUMSTICK_BITE,
        DUMBBELL,
        DUMPSTER,
        DUMPSTER_FIRE,
        DUNGEON,
        EDIT,
        EGG,
        EJECT,
        ELLIPSIS_H,
        ELLIPSIS_V,
        ENVELOPE,
        ENVELOPE_OPEN,
        ENVELOPE_OPEN_TEXT,
        ENVELOPE_SQUARE,
        EQUALS,
        ERASER,
        ETHERNET,
        EURO_SIGN,
        EXCHANGE_ALT,
        EXCLAMATION,
        EXCLAMATION_CIRCLE,
        EXCLAMATION_TRIANGLE,
        EXPAND,
        EXPAND_ALT,
        EXPAND_ARROWS_ALT,
        EXTERNAL_LINK_ALT,
        EXTERNAL_LINK_SQUARE_ALT,
        EYE,
        EYE_DROPPER,
        EYE_SLASH,
        FAN,
        FAST_BACKWARD,
        FAST_FORWARD,
        FAUCET,
        FAX,
        FEATHER,
        FEATHER_ALT,
        FEMALE,
        FIGHTER_JET,
        FILE,
        FILE_ALT,
        FILE_ARCHIVE,
        FILE_AUDIO,
        FILE_CODE,
        FILE_CONTRACT,
        FILE_CSV,
        FILE_DOWNLOAD,
        FILE_EXCEL,
        FILE_EXPORT,
        FILE_IMAGE,
        FILE_IMPORT,
        FILE_INVOICE,
        FILE_INVOICE_DOLLAR,
        FILE_MEDICAL,
        FILE_MEDICAL_ALT,
        FILE_PDF,
        FILE_POWERPOINT,
        FILE_PRESCRIPTION,
        FILE_SIGNATURE,
        FILE_UPLOAD,
        FILE_VIDEO,
        FILE_WORD,
        FILL,
        FILL_DRIP,
        FILM,
        FILTER,
        FINGERPRINT,
        FIRE,
        FIRE_ALT,
        FIRE_EXTINGUISHER,
        FIRST_AID,
        FISH,
        FIST_RAISED,
        FLAG,
        FLAG_CHECKERED,
        FLAG_USA,
        FLASK,
        FLUSHED,
        FOLDER,
        FOLDER_MINUS,
        FOLDER_OPEN,
        FOLDER_PLUS,
        FONT,
        FONT_AWESOME_LOGO_FULL,
        FOOTBALL_BALL,
        FORWARD,
        FROG,
        FROWN,
        FROWN_OPEN,
        FUNNEL_DOLLAR,
        FUTBOL,
        GAMEPAD,
        GAS_PUMP,
        GAVEL,
        GEM,
        GENDERLESS,
        GHOST,
        GIFT,
        GIFTS,
        GLASS_CHEERS,
        GLASS_MARTINI,
        GLASS_MARTINI_ALT,
        GLASS_WHISKEY,
        GLASSES,
        GLOBE,
        GLOBE_AFRICA,
        GLOBE_AMERICAS,
        GLOBE_ASIA,
        GLOBE_EUROPE,
        GOLF_BALL,
        GOPURAM,
        GRADUATION_CAP,
        GREATER_THAN,
        GREATER_THAN_EQUAL,
        GRIMACE,
        GRIN,
        GRIN_ALT,
        GRIN_BEAM,
        GRIN_BEAM_SWEAT,
        GRIN_HEARTS,
        GRIN_SQUINT,
        GRIN_SQUINT_TEARS,
        GRIN_STARS,
        GRIN_TEARS,
        GRIN_TONGUE,
        GRIN_TONGUE_SQUINT,
        GRIN_TONGUE_WINK,
        GRIN_WINK,
        GRIP_HORIZONTAL,
        GRIP_LINES,
        GRIP_LINES_VERTICAL,
        GRIP_VERTICAL,
        GUITAR,
        H_SQUARE,
        HAMBURGER,
        HAMMER,
        HAMSA,
        HAND_HOLDING,
        HAND_HOLDING_HEART,
        HAND_HOLDING_MEDICAL,
        HAND_HOLDING_USD,
        HAND_HOLDING_WATER,
        HAND_LIZARD,
        HAND_MIDDLE_FINGER,
        HAND_PAPER,
        HAND_PEACE,
        HAND_POINT_DOWN,
        HAND_POINT_LEFT,
        HAND_POINT_RIGHT,
        HAND_POINT_UP,
        HAND_POINTER,
        HAND_ROCK,
        HAND_SCISSORS,
        HAND_SPARKLES,
        HAND_SPOCK,
        HANDS,
        HANDS_HELPING,
        HANDS_WASH,
        HANDSHAKE,
        HANDSHAKE_ALT_SLASH,
        HANDSHAKE_SLASH,
        HANUKIAH,
        HARD_HAT,
        HASHTAG,
        HAT_COWBOY,
        HAT_COWBOY_SIDE,
        HAT_WIZARD,
        HDD,
        HEAD_SIDE_COUGH,
        HEAD_SIDE_COUGH_SLASH,
        HEAD_SIDE_MASK,
        HEAD_SIDE_VIRUS,
        HEADING,
        HEADPHONES,
        HEADPHONES_ALT,
        HEADSET,
        HEART,
        HEART_BROKEN,
        HEARTBEAT,
        HELICOPTER,
        HIGHLIGHTER,
        HIKING,
        HIPPO,
        HISTORY,
        HOCKEY_PUCK,
        HOLLY_BERRY,
        HOME,
        HORSE,
        HORSE_HEAD,
        HOSPITAL,
        HOSPITAL_ALT,
        HOSPITAL_SYMBOL,
        HOSPITAL_USER,
        HOT_TUB,
        HOTDOG,
        HOTEL,
        HOURGLASS,
        HOURGLASS_END,
        HOURGLASS_HALF,
        HOURGLASS_START,
        HOUSE_DAMAGE,
        HOUSE_USER,
        HRYVNIA,
        I_CURSOR,
        ICE_CREAM,
        ICICLES,
        ICONS,
        ID_BADGE,
        ID_CARD,
        ID_CARD_ALT,
        IGLOO,
        IMAGE,
        IMAGES,
        INBOX,
        INDENT,
        INDUSTRY,
        INFINITY,
        INFO,
        INFO_CIRCLE,
        ITALIC,
        JEDI,
        JOINT,
        JOURNAL_WHILLS,
        KAABA,
        KEY,
        KEYBOARD,
        KHANDA,
        KISS,
        KISS_BEAM,
        KISS_WINK_HEART,
        KIWI_BIRD,
        LANDMARK,
        LANGUAGE,
        LAPTOP,
        LAPTOP_CODE,
        LAPTOP_HOUSE,
        LAPTOP_MEDICAL,
        LAUGH,
        LAUGH_BEAM,
        LAUGH_SQUINT,
        LAUGH_WINK,
        LAYER_GROUP,
        LEAF,
        LEMON,
        LESS_THAN,
        LESS_THAN_EQUAL,
        LEVEL_DOWN_ALT,
        LEVEL_UP_ALT,
        LIFE_RING,
        LIGHTBULB,
        LINK,
        LIRA_SIGN,
        LIST,
        LIST_ALT,
        LIST_OL,
        LIST_UL,
        LOCATION_ARROW,
        LOCK,
        LOCK_OPEN,
        LONG_ARROW_ALT_DOWN,
        LONG_ARROW_ALT_LEFT,
        LONG_ARROW_ALT_RIGHT,
        LONG_ARROW_ALT_UP,
        LOW_VISION,
        LUGGAGE_CART,
        LUNGS,
        LUNGS_VIRUS,
        MAGIC,
        MAGNET,
        MAIL_BULK,
        MALE,
        MAP,
        MAP_MARKED,
        MAP_MARKED_ALT,
        MAP_MARKER,
        MAP_MARKER_ALT,
        MAP_PIN,
        MAP_SIGNS,
        MARKER,
        MARS,
        MARS_DOUBLE,
        MARS_STROKE,
        MARS_STROKE_H,
        MARS_STROKE_V,
        MASK,
        MEDAL,
        MEDKIT,
        MEH,
        MEH_BLANK,
        MEH_ROLLING_EYES,
        MEMORY,
        MENORAH,
        MERCURY,
        METEOR,
        MICROCHIP,
        MICROPHONE,
        MICROPHONE_ALT,
        MICROPHONE_ALT_SLASH,
        MICROPHONE_SLASH,
        MICROSCOPE,
        MINUS,
        MINUS_CIRCLE,
        MINUS_SQUARE,
        MITTEN,
        MOBILE,
        MOBILE_ALT,
        MONEY_BILL,
        MONEY_BILL_ALT,
        MONEY_BILL_WAVE,
        MONEY_BILL_WAVE_ALT,
        MONEY_CHECK,
        MONEY_CHECK_ALT,
        MONUMENT,
        MOON,
        MORTAR_PESTLE,
        MOSQUE,
        MOTORCYCLE,
        MOUNTAIN,
        MOUSE,
        MOUSE_POINTER,
        MUG_HOT,
        MUSIC,
        NETWORK_WIRED,
        NEUTER,
        NEWSPAPER,
        NOT_EQUAL,
        NOTES_MEDICAL,
        OBJECT_GROUP,
        OBJECT_UNGROUP,
        OIL_CAN,
        OM,
        OTTER,
        OUTDENT,
        PAGER,
        PAINT_BRUSH,
        PAINT_ROLLER,
        PALETTE,
        PALLET,
        PAPER_PLANE,
        PAPERCLIP,
        PARACHUTE_BOX,
        PARAGRAPH,
        PARKING,
        PASSPORT,
        PASTAFARIANISM,
        PASTE,
        PAUSE,
        PAUSE_CIRCLE,
        PAW,
        PEACE,
        PEN,
        PEN_ALT,
        PEN_FANCY,
        PEN_NIB,
        PEN_SQUARE,
        PENCIL_ALT,
        PENCIL_RULER,
        PEOPLE_ARROWS,
        PEOPLE_CARRY,
        PEPPER_HOT,
        PERCENT,
        PERCENTAGE,
        PERSON_BOOTH,
        PHONE,
        PHONE_ALT,
        PHONE_SLASH,
        PHONE_SQUARE,
        PHONE_SQUARE_ALT,
        PHONE_VOLUME,
        PHOTO_VIDEO,
        PIGGY_BANK,
        PILLS,
        PIZZA_SLICE,
        PLACE_OF_WORSHIP,
        PLANE,
        PLANE_ARRIVAL,
        PLANE_DEPARTURE,
        PLANE_SLASH,
        PLAY,
        PLAY_CIRCLE,
        PLUG,
        PLUS,
        PLUS_CIRCLE,
        PLUS_SQUARE,
        PODCAST,
        POLL,
        POLL_H,
        POO,
        POO_STORM,
        POOP,
        PORTRAIT,
        POUND_SIGN,
        POWER_OFF,
        PRAY,
        PRAYING_HANDS,
        PRESCRIPTION,
        PRESCRIPTION_BOTTLE,
        PRESCRIPTION_BOTTLE_ALT,
        PRINT,
        PROCEDURES,
        PROJECT_DIAGRAM,
        PUMP_MEDICAL,
        PUMP_SOAP,
        PUZZLE_PIECE,
        QRCODE,
        QUESTION,
        QUESTION_CIRCLE,
        QUIDDITCH,
        QUOTE_LEFT,
        QUOTE_RIGHT,
        QURAN,
        RADIATION,
        RADIATION_ALT,
        RAINBOW,
        RANDOM,
        RECEIPT,
        RECORD_VINYL,
        RECYCLE,
        REDO,
        REDO_ALT,
        REGISTERED,
        REMOVE_FORMAT,
        REPLY,
        REPLY_ALL,
        REPUBLICAN,
        RESTROOM,
        RETWEET,
        RIBBON,
        RING,
        ROAD,
        ROBOT,
        ROCKET,
        ROUTE,
        RSS,
        RSS_SQUARE,
        RUBLE_SIGN,
        RULER,
        RULER_COMBINED,
        RULER_HORIZONTAL,
        RULER_VERTICAL,
        RUNNING,
        RUPEE_SIGN,
        SAD_CRY,
        SAD_TEAR,
        SATELLITE,
        SATELLITE_DISH,
        SAVE,
        SCHOOL,
        SCREWDRIVER,
        SCROLL,
        SD_CARD,
        SEARCH,
        SEARCH_DOLLAR,
        SEARCH_LOCATION,
        SEARCH_MINUS,
        SEARCH_PLUS,
        SEEDLING,
        SERVER,
        SHAPES,
        SHARE,
        SHARE_ALT,
        SHARE_ALT_SQUARE,
        SHARE_SQUARE,
        SHEKEL_SIGN,
        SHIELD_ALT,
        SHIELD_VIRUS,
        SHIP,
        SHIPPING_FAST,
        SHOE_PRINTS,
        SHOPPING_BAG,
        SHOPPING_BASKET,
        SHOPPING_CART,
        SHOWER,
        SHUTTLE_VAN,
        SIGN,
        SIGN_IN_ALT,
        SIGN_LANGUAGE,
        SIGN_OUT_ALT,
        SIGNAL,
        SIGNATURE,
        SIM_CARD,
        SINK,
        SITEMAP,
        SKATING,
        SKIING,
        SKIING_NORDIC,
        SKULL,
        SKULL_CROSSBONES,
        SLASH,
        SLEIGH,
        SLIDERS_H,
        SMILE,
        SMILE_BEAM,
        SMILE_WINK,
        SMOG,
        SMOKING,
        SMOKING_BAN,
        SMS,
        SNOWBOARDING,
        SNOWFLAKE,
        SNOWMAN,
        SNOWPLOW,
        SOAP,
        SOCKS,
        SOLAR_PANEL,
        SORT,
        SORT_ALPHA_DOWN,
        SORT_ALPHA_DOWN_ALT,
        SORT_ALPHA_UP,
        SORT_ALPHA_UP_ALT,
        SORT_AMOUNT_DOWN,
        SORT_AMOUNT_DOWN_ALT,
        SORT_AMOUNT_UP,
        SORT_AMOUNT_UP_ALT,
        SORT_DOWN,
        SORT_NUMERIC_DOWN,
        SORT_NUMERIC_DOWN_ALT,
        SORT_NUMERIC_UP,
        SORT_NUMERIC_UP_ALT,
        SORT_UP,
        SPA,
        SPACE_SHUTTLE,
        SPELL_CHECK,
        SPIDER,
        SPINNER,
        SPLOTCH,
        SPRAY_CAN,
        SQUARE,
        SQUARE_FULL,
        SQUARE_ROOT_ALT,
        STAMP,
        STAR,
        STAR_AND_CRESCENT,
        STAR_HALF,
        STAR_HALF_ALT,
        STAR_OF_DAVID,
        STAR_OF_LIFE,
        STEP_BACKWARD,
        STEP_FORWARD,
        STETHOSCOPE,
        STICKY_NOTE,
        STOP,
        STOP_CIRCLE,
        STOPWATCH,
        STOPWATCH_20,
        STORE,
        STORE_ALT,
        STORE_ALT_SLASH,
        STORE_SLASH,
        STREAM,
        STREET_VIEW,
        STRIKETHROUGH,
        STROOPWAFEL,
        SUBSCRIPT,
        SUBWAY,
        SUITCASE,
        SUITCASE_ROLLING,
        SUN,
        SUPERSCRIPT,
        SURPRISE,
        SWATCHBOOK,
        SWIMMER,
        SWIMMING_POOL,
        SYNAGOGUE,
        SYNC,
        SYNC_ALT,
        SYRINGE,
        TABLE,
        TABLE_TENNIS,
        TABLET,
        TABLET_ALT,
        TABLETS,
        TACHOMETER_ALT,
        TAG,
        TAGS,
        TAPE,
        TASKS,
        TAXI,
        TEETH,
        TEETH_OPEN,
        TEMPERATURE_HIGH,
        TEMPERATURE_LOW,
        TENGE,
        TERMINAL,
        TEXT_HEIGHT,
        TEXT_WIDTH,
        TH,
        TH_LARGE,
        TH_LIST,
        THEATER_MASKS,
        THERMOMETER,
        THERMOMETER_EMPTY,
        THERMOMETER_FULL,
        THERMOMETER_HALF,
        THERMOMETER_QUARTER,
        THERMOMETER_THREE_QUARTERS,
        THUMBS_DOWN,
        THUMBS_UP,
        THUMBTACK,
        TICKET_ALT,
        TIMES,
        TIMES_CIRCLE,
        TINT,
        TINT_SLASH,
        TIRED,
        TOGGLE_OFF,
        TOGGLE_ON,
        TOILET,
        TOILET_PAPER,
        TOILET_PAPER_SLASH,
        TOOLBOX,
        TOOLS,
        TOOTH,
        TORAH,
        TORII_GATE,
        TRACTOR,
        TRADEMARK,
        TRAFFIC_LIGHT,
        TRAILER,
        TRAIN,
        TRAM,
        TRANSGENDER,
        TRANSGENDER_ALT,
        TRASH,
        TRASH_ALT,
        TRASH_RESTORE,
        TRASH_RESTORE_ALT,
        TREE,
        TROPHY,
        TRUCK,
        TRUCK_LOADING,
        TRUCK_MONSTER,
        TRUCK_MOVING,
        TRUCK_PICKUP,
        TSHIRT,
        TTY,
        TV,
        UMBRELLA,
        UMBRELLA_BEACH,
        UNDERLINE,
        UNDO,
        UNDO_ALT,
        UNIVERSAL_ACCESS,
        UNIVERSITY,
        UNLINK,
        UNLOCK,
        UNLOCK_ALT,
        UPLOAD,
        USER,
        USER_ALT,
        USER_ALT_SLASH,
        USER_ASTRONAUT,
        USER_CHECK,
        USER_CIRCLE,
        USER_CLOCK,
        USER_COG,
        USER_EDIT,
        USER_FRIENDS,
        USER_GRADUATE,
        USER_INJURED,
        USER_LOCK,
        USER_MD,
        USER_MINUS,
        USER_NINJA,
        USER_NURSE,
        USER_PLUS,
        USER_SECRET,
        USER_SHIELD,
        USER_SLASH,
        USER_TAG,
        USER_TIE,
        USER_TIMES,
        USERS,
        USERS_COG,
        USERS_SLASH,
        UTENSIL_SPOON,
        UTENSILS,
        VECTOR_SQUARE,
        VENUS,
        VENUS_DOUBLE,
        VENUS_MARS,
        VEST,
        VEST_PATCHES,
        VIAL,
        VIALS,
        VIDEO,
        VIDEO_SLASH,
        VIHARA,
        VIRUS,
        VIRUS_SLASH,
        VIRUSES,
        VOICEMAIL,
        VOLLEYBALL_BALL,
        VOLUME_DOWN,
        VOLUME_MUTE,
        VOLUME_OFF,
        VOLUME_UP,
        VOTE_YEA,
        VR_CARDBOARD,
        WALKING,
        WALLET,
        WAREHOUSE,
        WATER,
        WAVE_SQUARE,
        WEIGHT,
        WEIGHT_HANGING,
        WHEELCHAIR,
        WIFI,
        WIND,
        WINDOW_CLOSE,
        WINDOW_MAXIMIZE,
        WINDOW_MINIMIZE,
        WINDOW_RESTORE,
        WINE_BOTTLE,
        WINE_GLASS,
        WINE_GLASS_ALT,
        WON_SIGN,
        WRENCH,
        X_RAY,
        YEN_SIGN,
        YIN_YANG,
    ]
}
