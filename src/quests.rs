//! Quests catalog for the Rust world ("Memory Garden").
//!
//! Data-only module: 11 chapters / 103 quests. Every quest declares its slot
//! markers inside `code`: the first slot is `{slot}`, extra slots are
//! `{slot2}`, `{slot3}`, ... (at most one marker per line). `answers[i]`
//! lists every token accepted for slot i (the first one is canonical and is
//! shown once the quest is completed); `placeholders[i]` optionally reveals
//! a visible hint for slot i. All localized texts live in `i18n.rs` (KO
//! falls back to EN). The `code` blocks themselves are ALWAYS English —
//! comments and string literals alike: this is the one part of the garden
//! we do not translate.

use std::borrow::Cow::{self, Owned, Borrowed};
use serde::Serialize;
use crate::i18n::{Translation, DEF_TRANSLATION};

// ---------------------------------------------------------------------------
// PARSING TOKENS... OH YEAH... THE FIRST STEP OF BUILDING A COMPILER...
// ---------------------------------------------------------------------------
/// PARSING TOKENS... OH, YEAH... THE FIRST STEP OF BUILDING A COMPILER!!!
/// For example:
/// * given `"&mut"`, returned `"&mut"` (valid Rust);
/// * given `" & mut "`, returned `"&mut"` (valid Rust);
/// * given `"  & mu t  "`, returned `"&mu t"` (not Rust);
/// * given:
///     * `" & mu t 1 . 2 1. 2 1 .2 1.2 t1 23 45 67 89 0_ &m _ut _a; _b, 0| _1: 2. _3/ 4 "`;
/// * returned:
///     * `"&mu t 1 . 2 1. 2 1 .2 1.2 t1 23 45 67 89 0_&m _ut _a;_b,0|_1:2._3/4"` (correct workflow!).
fn canonicalize_tokens(mut input: &str) -> Cow<'_, str> {
    input = input.trim();
    let mut whitespace_iter = input.split_whitespace();
    if whitespace_iter.clone().count() <= 1 {
        return Borrowed(input);
    }

    let mut string_buf;
    let window_iter = whitespace_iter.clone().scan(None, |state, next| {
        let prev = state.replace(next);
        Some(prev.map(|p| [p, next]))
    }).flatten();
    string_buf = match whitespace_iter.next() {
        Some(window) => window.to_owned(),
        // return Borrowed(""), which is covered ABOVE:
        None => unreachable!(),
    };
    for window in window_iter {
        match [window[0].chars().last(), window[1].chars().next()] {
            [Some('0'..='9'|'a'..='z'|'A'..='Z'|'_'), Some('0'..='9'|'a'..='z'|'A'..='Z'|'_')] => {
                string_buf.push(' ')
            },
            [Some('0'..='9'), Some('.')] | [Some('.'), Some('0'..='9')] =>string_buf.push(' '),
            [Some('.'), Some('.')] | [Some(':'), Some(':')] => string_buf.push(' '),
            [Some(_), Some(_)] => (),
            [None, _] | [_, None] => unreachable!(),
        }
        string_buf.push_str(window[1]);
    }

    Owned(string_buf)
}

// ---------------------------------------------------------------------------
// Chapters & Quests structures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct Chapter {
    pub id: &'static str,
    pub number: u8,
    pub icon: &'static str,
}

impl Chapter {
    pub fn title<'a>(&self, t: &'a Translation) -> &'a str {
        let idx = (self.number as usize).saturating_sub(1);
        t.chapters_titles.get(idx).copied().unwrap_or("")
    }

    pub fn desc<'a>(&self, t: &'a Translation) -> &'a str {
        let idx = (self.number as usize).saturating_sub(1);
        t.chapters_descs.get(idx).copied().unwrap_or("")
    }

    pub fn mascot_initial<'a>(&self, t: &'a Translation) -> &'a str {
        let idx = (self.number as usize).saturating_sub(1);
        t.chapters_mascot_initial.get(idx).copied().unwrap_or("")
    }

    pub fn mascot_success<'a>(&self, t: &'a Translation) -> &'a str {
        let idx = (self.number as usize).saturating_sub(1);
        t.chapters_mascot_success.get(idx).copied().unwrap_or("")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Quest {
    pub id: &'static str,
    pub chapter_id: &'static str,
    /// Global sequential number (1..=103) — drives progress, XP chain, nav.
    pub number: u8,
    pub xp_reward: i64,
    /// Accepted tokens per slot. `answers[i]` lists every token accepted for
    /// slot i (the i-th slot marker in `code`); the FIRST token is the
    /// canonical answer — it is shown in the input when the quest is
    /// completed. A quest with `answers.len() > 1` is a multi-slot quest.
    pub answers: &'static [&'static [&'static str]],
    /// Shown in the empty input of slot i. `Some(hint)` = a visible hint
    /// (easy quest); `None`/absent = the answer stays hidden (hard quest;
    /// the hint array in i18n still nudges).
    pub placeholders: &'static [Option<&'static str>],
    /// Full code block. Slot i is marked with `{slot}` for i = 1 and
    /// `{slot2}`, `{slot3}`, ... for the rest; at most ONE marker per line.
    /// Indentation is leading spaces; trailing ` // ...` becomes a comment.
    /// Comments and string literals here are ALWAYS English.
    pub code: &'static [&'static str],
}

impl Quest {
    pub fn chapter(&self) -> &'static Chapter {
        CHAPTERS
            .iter()
            .find(|c| c.id == self.chapter_id)
            .unwrap_or(&CHAPTERS[0])
    }

    fn idx(&self) -> usize {
        (self.number as usize).saturating_sub(1)
    }

    pub fn get_title<'a>(&self, t: &'a Translation) -> &'a str {
        let idx = self.idx();
        t.quests_titles.get(idx).copied()
            .or_else(|| DEF_TRANSLATION.quests_titles.get(idx).copied())
            .unwrap_or("Quest")
    }

    pub fn get_concept<'a>(&self, t: &'a Translation) -> &'a str {
        t.quests_concepts.get(self.idx()).copied()
            .or_else(|| DEF_TRANSLATION.quests_concepts.get(self.idx()).copied())
            .unwrap_or("")
    }

    pub fn get_lore<'a>(&self, t: &'a Translation) -> &'a str {
        t.quests_lores.get(self.idx()).copied()
            .or_else(|| DEF_TRANSLATION.quests_lores.get(self.idx()).copied())
            .unwrap_or("")
    }

    pub fn get_hint<'a>(&self, t: &'a Translation) -> &'a str {
        t.quests_hints.get(self.idx()).copied()
            .or_else(|| DEF_TRANSLATION.quests_hints.get(self.idx()).copied())
            .unwrap_or("")
    }

    pub fn get_mascot_initial<'a>(&self, t: &'a Translation) -> &'a str {
        self.chapter().mascot_initial(t)
    }

    pub fn get_mascot_success<'a>(&self, t: &'a Translation) -> &'a str {
        self.chapter().mascot_success(t)
    }

    /// Marker text of slot i (1-based): slot 1 = `{slot}`, slot 2+ = `{slotN}`.
    /// Note: `{slot}` is NOT a substring of `{slot2}`, so the markers never
    /// collide during a literal search.
    pub fn slot_marker(i: usize) -> String {
        if i <= 1 { "{slot}".to_string() } else { format!("{{slot{i}}}") }
    }

    /// Strict but whitespace-tolerant comparison of the player's tokens with
    /// the accepted answers (Rust is case-sensitive; whitespace runs between
    /// word characters collapse to one space instead of vanishing — see
    /// `canonicalize_tokens`). Multi-slot quests expect one token per slot,
    /// in slot order, and every slot must match.
    pub fn validate_tokens(&self, inputs: &[&str]) -> bool {
        if inputs.len() != self.answers.len() {
            return false;
        }
        self.answers.iter().zip(inputs.iter()).all(|(accepted, input)| {
            let canon_input = canonicalize_tokens(input);
            accepted.iter().any(|ans| *canon_input == *canonicalize_tokens(ans))
        })
    }
}

// ---------------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------------

pub static CHAPTERS: &[Chapter] = &[
    Chapter { id: "basics",      number: 1,  icon: "🌱" },
    Chapter { id: "flow",        number: 2,  icon: "🧭" },
    Chapter { id: "forms",       number: 3,  icon: "🏛️" },
    Chapter { id: "collections", number: 4,  icon: "🗃️" },
    Chapter { id: "strings",     number: 5,  icon: "📜" },
    Chapter { id: "traits",      number: 6,  icon: "🧬" },
    Chapter { id: "formatting",  number: 7,  icon: "🖋️" },
    Chapter { id: "errors",      number: 8,  icon: "⚖️" },
    Chapter { id: "pointers",    number: 9,  icon: "🧠" },
    Chapter { id: "threads",     number: 10, icon: "🧵" },
    Chapter { id: "files",       number: 11, icon: "🗄️" },
];

/// Finds a quest by id ("rust-07-floats").
pub fn find_quest(id: &str) -> Option<&'static Quest> {
    QUESTS.iter().find(|q| q.id == id)
}

///Resolves a chapter by slug ("basics") or by 1-based number ("1").
pub fn find_chapter(slug_or_number: &str) -> Option<&'static Chapter> {
    if let Ok(n) = slug_or_number.parse::<u8>() {
        CHAPTERS.iter().find(|c| c.number == n)
    } else {
        CHAPTERS.iter().find(|c| c.id == slug_or_number)
    }
}

/// The quest immediately before (number - 1), if any.
pub fn prev_quest(number: u8) -> Option<&'static Quest> {
    if number <= 1 { return None; }
    QUESTS.iter().find(|q| q.number == number - 1)
}

/// The quest immediately after (number + 1), if any.
pub fn next_quest(number: u8) -> Option<&'static Quest> {
    QUESTS.iter().find(|q| q.number == number + 1)
}

/// All quests of a chapter, already ordered by number.
pub fn chapter_quests(chapter_id: &str) -> Vec<&'static Quest> {
    QUESTS.iter().filter(|q| q.chapter_id == chapter_id).collect()
}

pub fn total_quest_count() -> usize {
    QUESTS.len()
}

// ---------------------------------------------------------------------------
// Canonical Quests Data — Chapter 1: Basics (1..=12)
// Quests 1..=5 keep their historical ids (DB achievements reference them:
// first_spark .. grand_architect, chapter1_conqueror); their XP was leveled
// down to the standard 25 — no more Gemini-era ascending scale.
// The `code` blocks are ALWAYS English, regardless of the interface language.
// ---------------------------------------------------------------------------

pub static QUESTS: &[Quest] = &[
    Quest {
        id: "rust-01-mutability", chapter_id: "basics", number: 1, xp_reward: 25,
        answers: &[&["mut"]], placeholders: &[Some("mut")],
        code: &[
            "fn main() {",
            "    // 1. By default the rune is sealed: the variable is immutable",
            "    let {slot} fairy_energy: i32 = 100;",
            "    // 2. Add +50 Garden energy",
            "    fairy_energy += 50;",
            "    println!(\"Energy: {}\", fairy_energy);",
            "}",
        ],
    },
    Quest {
        id: "rust-02-shadowing", chapter_id: "basics", number: 2, xp_reward: 25,
        answers: &[&["let"]], placeholders: &[Some("let")],
        code: &[
            "fn main() {",
            "    let raw_scroll = \"42\"; // input data: text",
            "    // Shadow the variable: parse the number",
            "    {slot} raw_scroll: i32 = raw_scroll.trim().parse().unwrap();",
            "    println!(\"Result: {}\", raw_scroll + 8);",
            "}",
        ],
    },
    Quest {
        id: "rust-03-ownership", chapter_id: "basics", number: 3, xp_reward: 25,
        answers: &[&["clone"]], placeholders: &[Some("clone")],
        code: &[
            "fn main() {",
            "    let scroll = String::from(\"Rust Codex\");",
            "    // Create an independent copy of the scroll",
            "    let library_copy = scroll.{slot}();",
            "    println!(\"Original: {}, Copy: {}\", scroll, library_copy);",
            "}",
        ],
    },
    Quest {
        id: "rust-04-references", chapter_id: "basics", number: 4, xp_reward: 25,
        answers: &[&["&mut"]], placeholders: &[Some("&mut")],
        code: &[
            "fn boost_energy(energy: {slot} i32) {",
            "    *energy += 50; // dereference and add",
            "}",
            "fn main() {",
            "    let mut current_energy = 100;",
            "    boost_energy(&mut current_energy);",
            "    println!(\"Energy: {}\", current_energy);",
            "}",
        ],
    },
    Quest {
        id: "rust-05-slices", chapter_id: "basics", number: 5, xp_reward: 25,
        answers: &[&["&"]], placeholders: &[Some("&")],
        code: &[
            "fn main() {",
            "    let spell_name = String::from(\"FerrisMagic\");",
            "    // Take the slice of the first 6 bytes: \"Ferris\"",
            "    let mascot: {slot}str = &spell_name[0..6];",
            "    println!(\"Companion name: {}\", mascot);",
            "}",
        ],
    },
    Quest {
        id: "rust-06-integers", chapter_id: "basics", number: 6, xp_reward: 25,
        answers: &[&["i64"]], placeholders: &[None],
        code: &[
            "fn drain(depth: i64) -> i64 {",
            "    depth / 2",
            "}",
            "fn main() {",
            "    // Strong typing: the signature above only accepts i64 —",
            "    // i32 or u32 would refuse to compile in this slot",
            "    let depth: {slot} = -84;",
            "    println!(\"Garden depth: {}\", drain(depth));",
            "}",
        ],
    },
    Quest {
        id: "rust-07-floats", chapter_id: "basics", number: 7, xp_reward: 25,
        answers: &[&["f64"]], placeholders: &[Some("f…")],
        code: &[
            "fn weigh(dose: f64) -> f64 {",
            "    dose * 2.0",
            "}",
            "fn main() {",
            "    // Strong typing: the scale above only accepts the default",
            "    // float type — f32 would refuse to compile in this slot",
            "    let dose: {slot} = 0.75;",
            "    println!(\"Potion dose: {}\", weigh(dose));",
            "}",
        ],
    },
    Quest {
        id: "rust-08-bools", chapter_id: "basics", number: 8, xp_reward: 25,
        answers: &[&["bool"]], placeholders: &[Some("bool")],
        code: &[
            "fn main() {",
            "    // Lantern: lit or dimmed",
            "    let lantern_lit: {slot} = true;",
            "    if lantern_lit { println!(\"Light in the garden\"); }",
            "}",
        ],
    },
    Quest {
        id: "rust-09-tuples", chapter_id: "basics", number: 9, xp_reward: 25,
        answers: &[&["255"], &["0"], &["0"]], placeholders: &[None, None, None],
        code: &[
            "fn main() {",
            "    // Scarlet rune light: red=255, green=0, blue=0",
            "    let ruby = ({slot}, {slot2}, {slot3});",
            "    println!(\"Red component: {}\", ruby.0);",
            "}",
        ],
    },
    Quest {
        id: "rust-10-arrays", chapter_id: "basics", number: 10, xp_reward: 25,
        answers: &[&["2"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    // Three crystal rows with 10, 20 and 30 crystals",
            "    let crystal_row = [10, 20, 30];",
            "    // Array slots are counted from ZERO: the third row => index 2",
            "    let third = crystal_row[{slot}];",
            "    println!(\"Third row holds: {}\", third);",
            "}",
        ],
    },
    Quest {
        id: "rust-11-const", chapter_id: "basics", number: 11, xp_reward: 25,
        answers: &[&["MAX_ENERGY"]], placeholders: &[None],
        code: &[
            "// The sigil is carved before the Garden awakens",
            "const {slot}: i32 = 9999;",
            "fn main() {",
            "    println!(\"Garden limit: {}\", MAX_ENERGY);",
            "}",
        ],
    },
    Quest {
        id: "rust-12-cast", chapter_id: "basics", number: 12, xp_reward: 25,
        answers: &[&["as u8"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let potion: u32 = 77;",
            "    // Shrink the bowl down to u8 size",
            "    let ember: u8 = potion {slot};",
            "    println!(\"Smoldering ember: {}\", ember);",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 2: Flow & Functions (13..=22)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-13-if", chapter_id: "flow", number: 13, xp_reward: 25,
        answers: &[&["else"]], placeholders: &[Some("else")],
        code: &[
            "fn main() {",
            "    let energy = 40;",
            "    if energy < 50 {",
            "        println!(\"The candle is dim — a quiet whisper\");",
            "    } {slot} {",
            "        println!(\"Heat at full blaze!\");",
            "    }",
            "}",
        ],
    },
    Quest {
        id: "rust-14-if-expr", chapter_id: "flow", number: 14, xp_reward: 25,
        answers: &[&["if"]], placeholders: &[Some("if")],
        code: &[
            "fn main() {",
            "    let lion_awake = true;",
            "    // if works as an expression here and returns a string",
            "    let status = {slot} lion_awake {",
            "        \"the guard is awake\",",
            "    } else {",
            "        \"the guard dozes\",",
            "    };",
            "    println!(\"{}\", status);",
            "}",
        ],
    },
    Quest {
        id: "rust-15-match", chapter_id: "flow", number: 15, xp_reward: 25,
        answers: &[&["match"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let essence = 2;",
            "    // Pattern dispatcher: which arm answers the call",
            "    {slot} essence {",
            "        1 => println!(\"Spark\"),",
            "        2 => println!(\"Flame\"),",
            "        _ => println!(\"Darkness\"),",
            "    }",
            "}",
        ],
    },
    Quest {
        id: "rust-16-match-arm", chapter_id: "flow", number: 16, xp_reward: 25,
        answers: &[&["_"]], placeholders: &[Some("_")],
        code: &[
            "fn main() {",
            "    let rune = 'ζ';",
            "    match rune {",
            "        'α' => println!(\"The first rune\"),",
            "        'ω' => println!(\"The last rune\"),",
            "        {slot} => println!(\"An unknown rune\"),",
            "    }",
            "}",
        ],
    },
    Quest {
        id: "rust-17-match-range", chapter_id: "flow", number: 17, xp_reward: 25,
        answers: &[&["..="]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let rolls = 7;",
            "    match rolls {",
            "        1..=5 => println!(\"Too few\"),",
            "        6 {slot} 9 => println!(\"Rows galore\"),",
            "        _ => println!(\"Overflow\"),",
            "    }",
            "}",
        ],
    },
    Quest {
        id: "rust-18-loop-break", chapter_id: "flow", number: 18, xp_reward: 25,
        answers: &[&["break"]], placeholders: &[Some("break")],
        code: &[
            "fn main() {",
            "    let mut energy = 0;",
            "    // The eternal wheel — until we reach the limit",
            "    loop {",
            "        energy += 10;",
            "        if energy >= 50 {",
            "            {slot};",
            "        }",
            "    }",
            "    println!(\"Charge: {}\", energy);",
            "}",
        ],
    },
    Quest {
        id: "rust-19-while", chapter_id: "flow", number: 19, xp_reward: 25,
        answers: &[&["while"], &["100"]], placeholders: &[Some("while"), None],
        code: &[
            "fn main() {",
            "    let mut energy = 0;",
            "    // Spin the wheel until the charge reaches one hundred",
            "    {slot} energy < {slot2} {",
            "        energy += 25;",
            "    }",
            "    println!(\"Charge: {}\", energy);",
            "}",
        ],
    },
    Quest {
        id: "rust-20-for-range", chapter_id: "flow", number: 20, xp_reward: 25,
        answers: &[&["for"]], placeholders: &[Some("for")],
        code: &[
            "fn main() {",
            "    // Walk the garden rows from 1 to 5",
            "    {slot} drop in 1..=5 {",
            "        println!(\"Dew {} has dropped\", drop);",
            "    }",
            "}",
        ],
    },
    Quest {
        id: "rust-21-fn-def", chapter_id: "flow", number: 21, xp_reward: 25,
        answers: &[&["fn"]], placeholders: &[Some("fn")],
        code: &[
            "// Declare a named spell",
            "{slot} charge(price: i32) -> i32 {",
            "    price * 2",
            "}",
            "fn main() {",
            "    println!(\"To pay: {}\", charge(21));",
            "}",
        ],
    },
    Quest {
        id: "rust-22-fn-return", chapter_id: "flow", number: 22, xp_reward: 25,
        answers: &[&["->"]], placeholders: &[Some("->")],
        code: &[
            "fn fuel(minutes: i32) {slot} i32 {",
            "    minutes * 60",
            "}",
            "fn main() {",
            "    println!(\"Burning seconds: {}\", fuel(9));",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 3: Structs, Enums & Option (23..=32)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-23-struct", chapter_id: "forms", number: 23, xp_reward: 25,
        answers: &[&["struct"], &["spear"]], placeholders: &[Some("struct"), None],
        code: &[
            "// A stamp for future guardians",
            "{slot} Guardian {",
            "    {slot2}: u16,",
            "    shield: u16,",
            "}",
            "fn main() {",
            "    let g = Guardian { spear: 7, shield: 9 };",
            "    println!(\"Spear: {}\", g.spear);",
            "}",
        ],
    },
    Quest {
        id: "rust-24-field-access", chapter_id: "forms", number: 24, xp_reward: 25,
        answers: &[&["power"]], placeholders: &[Some("power")],
        code: &[
            "struct Guardian { power: u16, shield: u16 }",
            "fn main() {",
            "    let guardian = Guardian { power: 42, shield: 10 };",
            "    // The sacred dot opens the chest of values",
            "    let strength = guardian.{slot};",
            "    println!(\"Strength: {}\", strength);",
            "}",
        ],
    },
    Quest {
        id: "rust-25-struct-update", chapter_id: "forms", number: 25, xp_reward: 25,
        answers: &[&["..base"]], placeholders: &[None],
        code: &[
            "struct Guardian { spear: u16, shield: u16 }",
            "fn main() {",
            "    let base = Guardian { spear: 10, shield: 8 };",
            "    // Copy the rest from the donor: spear gets a new one",
            "    let hardier = Guardian { spear: 12, {slot} };",
            "    println!(\"New shield: {}\", hardier.shield);",
            "}",
        ],
    },
    Quest {
        id: "rust-26-impl", chapter_id: "forms", number: 26, xp_reward: 25,
        answers: &[&["impl"]], placeholders: &[Some("impl")],
        code: &[
            "struct Guardian { spear: u16, shield: u16 }",
            "// The altar of Guardian's methods",
            "{slot} Guardian {",
            "    fn total(&self) -> u16 { self.spear + self.shield }",
            "}",
            "fn main() {",
            "    let g = Guardian { spear: 5, shield: 5 };",
            "    println!(\"Sum: {}\", g.total());",
            "}",
        ],
    },
    Quest {
        id: "rust-27-self", chapter_id: "forms", number: 27, xp_reward: 25,
        answers: &[&["self"]], placeholders: &[Some("self")],
        code: &[
            "struct Guardian { name: &'static str }",
            "impl Guardian {",
            "    fn name(&{slot}) -> &'static str { self.name }",
            "}",
            "fn main() {",
            "    let g = Guardian { name: \"Cora\" };",
            "    println!(\"Name: {}\", g.name());",
            "}",
        ],
    },
    Quest {
        id: "rust-28-enum", chapter_id: "forms", number: 28, xp_reward: 25,
        answers: &[&["enum"], &["Locked"]], placeholders: &[Some("enum"), None],
        code: &[
            "// The tablet of permitted states of the Gate",
            "{slot} GateStatus {",
            "    Locked,",
            "    Guarded,",
            "    Open,",
            "}",
            "fn main() {",
            "    let gate = GateStatus::{slot2};",
            "    let _ = gate;",
            "}",
        ],
    },
    Quest {
        id: "rust-29-variant", chapter_id: "forms", number: 29, xp_reward: 25,
        answers: &[&["Ready(42)"]], placeholders: &[None],
        code: &[
            "enum ScrollStatus { Ready(i32), Lost }",
            "fn main() {",
            "    // The variant carries cargo: the scroll is ready with power 42",
            "    let scroll = ScrollStatus::{slot};",
            "    if let ScrollStatus::Ready(n) = scroll {",
            "        println!(\"The scroll is mighty: {}\", n);",
            "    }",
            "}",
        ],
    },
    Quest {
        id: "rust-30-option", chapter_id: "forms", number: 30, xp_reward: 25,
        answers: &[&["Option"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    // The shrine of absence: the artifact is not found yet",
            "    let relic: {slot}<i32> = None;",
            "    println!(\"Is the relic found? {:?}\", relic);",
            "}",
        ],
    },
    Quest {
        id: "rust-31-if-let", chapter_id: "forms", number: 31, xp_reward: 25,
        answers: &[&["if let"]], placeholders: &[Some("if…")],
        code: &[
            "fn main() {",
            "    let relic: Option<i32> = Some(7);",
            "    // If the shrine holds — unwrap it gently",
            "    {slot} Some(power) = relic {",
            "        println!(\"Artifact power: {}\", power);",
            "    }",
            "}",
        ],
    },
    Quest {
        id: "rust-32-unwrap-or", chapter_id: "forms", number: 32, xp_reward: 25,
        answers: &[&["unwrap_or"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let relic: Option<i32> = None;",
            "    // Empty? Then the lot — a spare coin",
            "    let fallback = relic.{slot}(0);",
            "    println!(\"In hands: {}\", fallback);",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 4: Collections & Iterators (33..=42)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-33-push", chapter_id: "collections", number: 33, xp_reward: 25,
        answers: &[&["push"]], placeholders: &[Some("push")],
        code: &[
            "fn main() {",
            "    let mut shelf: Vec<String> = Vec::new();",
            "    // Place the first volume on the shelf",
            "    shelf.{slot}(\"Memory Avengers\".to_string());",
            "    println!(\"Volumes: {}\", shelf.len());",
            "}",
        ],
    },
    Quest {
        id: "rust-34-vec", chapter_id: "collections", number: 34, xp_reward: 25,
        answers: &[&["vec!"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    // The vec! sigil mints the vector in one strike",
            "    let scrolls = {slot}[1, 2, 3];",
            "    println!(\"{} scrolls in a row\", scrolls.len());",
            "}",
        ],
    },
    Quest {
        id: "rust-35-pop", chapter_id: "collections", number: 35, xp_reward: 25,
        answers: &[&["pop"]], placeholders: &[Some("pop")],
        code: &[
            "fn main() {",
            "    let mut scrolls = vec![1, 2, 3];",
            "    // Take the last scroll off the shelf",
            "    let last = scrolls.{slot}();",
            "    if let Some(n) = last { println!(\"Taken: {}\", n); }",
            "}",
        ],
    },
    Quest {
        id: "rust-36-len", chapter_id: "collections", number: 36, xp_reward: 25,
        answers: &[&["len"]], placeholders: &[Some("len")],
        code: &[
            "fn main() {",
            "    let scrolls = vec![\"a\", \"b\", \"c\"];",
            "    // The fair keykeeper's gauge",
            "    println!(\"Count: {}\", scrolls.{slot}());",
            "}",
        ],
    },
    Quest {
        id: "rust-37-get", chapter_id: "collections", number: 37, xp_reward: 25,
        answers: &[&["get"]], placeholders: &[Some("get")],
        code: &[
            "fn main() {",
            "    let scrolls = vec![10, 20, 30];",
            "    // A cautious query: no panic, returns an Option",
            "    let first = scrolls.{slot}(0);",
            "    println!(\"{:?}\", first);",
            "}",
        ],
    },
    Quest {
        id: "rust-38-map", chapter_id: "collections", number: 38, xp_reward: 25,
        answers: &[&["map"]], placeholders: &[Some("m…")],
        code: &[
            "fn main() {",
            "    let rolls = vec![1, 2, 3];",
            "    // Raise every crystal to the square",
            "    let squares: Vec<i32> = rolls.iter().{slot}(|&x| x * x).collect();",
            "    println!(\"{:?}\", squares);",
            "}",
        ],
    },
    Quest {
        id: "rust-39-filter", chapter_id: "collections", number: 39, xp_reward: 25,
        answers: &[&["filter"]], placeholders: &[Some("filter")],
        code: &[
            "fn main() {",
            "    let rolls = vec![1, 2, 3, 4, 5, 6];",
            "    // The sieve keeps only the even ones",
            "    let evens: Vec<i32> = rolls.iter().copied().{slot}(|&x| x % 2 == 0).collect();",
            "    println!(\"{:?}\", evens);",
            "}",
        ],
    },
    Quest {
        id: "rust-40-sum", chapter_id: "collections", number: 40, xp_reward: 25,
        answers: &[&["sum", "sum::<i32>", "sum::<_>"]], placeholders: &[Some("sum")],
        code: &[
            "fn main() {",
            "    let sparks = vec![3, 5, 7];",
            "    // Gather the sparks into one fountain",
            "    let total: i32 = sparks.iter().{slot}();",
            "    println!(\"Fountain: {}\", total);",
            "}",
        ],
    },
    Quest {
        id: "rust-41-collect", chapter_id: "collections", number: 41, xp_reward: 25,
        answers: &[&["collect"]], placeholders: &[Some("collect")],
        code: &[
            "fn main() {",
            "    let rolls = vec![1, 2, 3];",
            "    // Pour the doubled stream into the named vessel",
            "    let doubled: Vec<i32> = rolls.iter().map(|&x| x * 2).{slot}();",
            "    println!(\"{:?}\", doubled);",
            "}",
        ],
    },
    Quest {
        id: "rust-42-hashmap", chapter_id: "collections", number: 42, xp_reward: 25,
        answers: &[&["HashMap"], &["HashMap"]], placeholders: &[None, Some("H…")],
        code: &[
            "use std::collections::HashMap;",
            "fn main() {",
            "    // The treasury: a key leads to a coin",
            "    let mut vault: {slot}<String, i32> = {slot2}::new();",
            "    vault.insert(\"keys\".to_string(), 3);",
            "    println!(\"Coins on key: {:?}\", vault.get(\"keys\"));",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 5: Strings (43..=51)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-43-string", chapter_id: "strings", number: 43, xp_reward: 25,
        answers: &[&["String"]], placeholders: &[Some("String")],
        code: &[
            "fn main() {",
            "    // An owning scroll grows on the heap",
            "    let mut diary: {slot} = String::new();",
            "    diary.push_str(\"Day one\");",
            "    println!(\"{}\", diary);",
            "}",
        ],
    },
    Quest {
        id: "rust-44-to-string", chapter_id: "strings", number: 44, xp_reward: 25,
        answers: &[&["to_string"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let fragment = \"Ferris\";",
            "    // The blessing turns a fragment into a scroll",
            "    let scroll: String = fragment.{slot}();",
            "    println!(\"{}\", scroll);",
            "}",
        ],
    },
    Quest {
        id: "rust-45-push-str", chapter_id: "strings", number: 45, xp_reward: 25,
        answers: &[&["push_str"]], placeholders: &[Some("push_…")],
        code: &[
            "fn main() {",
            "    let mut diary = String::from(\"Garden notes:\");",
            "    // Grow the scroll with text",
            "    diary.{slot}(\" a new day added\");",
            "    println!(\"{}\", diary);",
            "}",
        ],
    },
    Quest {
        id: "rust-46-format", chapter_id: "strings", number: 46, xp_reward: 25,
        answers: &[&["format!"]], placeholders: &[Some("f…!")],
        code: &[
            "fn main() {",
            "    // The workshop assembles a fresh String from parts",
            "    let title = {slot}(\"Scroll #{}\", 4);",
            "    println!(\"{}\", title);",
            "}",
        ],
    },
    Quest {
        id: "rust-47-trim", chapter_id: "strings", number: 47, xp_reward: 25,
        answers: &[&["trim"]], placeholders: &[Some("trim")],
        code: &[
            "fn main() {",
            "    let quote = \"   runic word   \";",
            "    // The tailor trims the edges",
            "    let clean = quote.{slot}();",
            "    println!(\"[{}]\", clean);",
            "}",
        ],
    },
    Quest {
        id: "rust-48-uppercase", chapter_id: "strings", number: 48, xp_reward: 25,
        answers: &[&["to_uppercase"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let whisper = \"sunset drive\";",
            "    // Every character rises to its capital form",
            "    let shout = whisper.{slot}();",
            "    println!(\"{}\", shout);",
            "}",
        ],
    },
    Quest {
        id: "rust-49-chars", chapter_id: "strings", number: 49, xp_reward: 25,
        answers: &[&["chars"]], placeholders: &[Some("chars")],
        code: &[
            "fn main() {",
            "    let s = String::from(\"runes\");",
            "    // A caravan across the characters of the string",
            "    let letters: Vec<char> = s.{slot}().collect();",
            "    println!(\"First letter: {}\", letters[0]);",
            "}",
        ],
    },
    Quest {
        id: "rust-50-split", chapter_id: "strings", number: 50, xp_reward: 25,
        answers: &[&["split"]], placeholders: &[Some("s…")],
        code: &[
            "fn main() {",
            "    let log = \"claw, flame, spark\";",
            "    // Cut the chronicle by comma-with-space",
            "    let notes: Vec<&str> = log.{slot}(\", \").collect();",
            "    println!(\"First: {}\", notes[0]);",
            "}",
        ],
    },
    Quest {
        id: "rust-51-replace", chapter_id: "strings", number: 51, xp_reward: 25,
        answers: &[&["replace"]], placeholders: &[Some("replace")],
        code: &[
            "fn main() {",
            "    let curse = \"the cursed tower\";",
            "    // Replace every occurrence",
            "    let blessed = curse.{slot}(\"cursed\", \"blessed\");",
            "    println!(\"{}\", blessed);",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 6: Traits & Generics (52..=60)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-52-trait", chapter_id: "traits", number: 52, xp_reward: 25,
        answers: &[&["trait"], &["self"]], placeholders: &[Some("trait"), None],
        code: &[
            "// The contract types put their signature on",
            "{slot} Spell {",
            "    fn cast(&{slot2}) -> String;",
            "}",
            "fn main() {}",
        ],
    },
    Quest {
        id: "rust-53-impl-trait", chapter_id: "traits", number: 53, xp_reward: 25,
        answers: &[&["impl"]], placeholders: &[None],
        code: &[
            "struct Fireball;",
            "trait Spell {",
            "    fn cast(&self) -> String;",
            "}",
            "    // Fireball signs the contract",
            "{slot} Spell for Fireball {",
            "    fn cast(&self) -> String { String::from(\"fsssh!\") }",
            "}",
            "fn main() {}",
        ],
    },
    Quest {
        id: "rust-54-clone-trait", chapter_id: "traits", number: 54, xp_reward: 25,
        answers: &[&["Clone"]], placeholders: &[Some("Clone")],
        code: &[
            "#[derive({slot})]",
            "struct Scroll { title: String }",
            "fn main() {",
            "    let s1 = Scroll { title: String::from(\"Covenant\") };",
            "    let s2 = s1.clone();",
            "    println!(\"{} / {}\", s1.title, s2.title);",
            "}",
        ],
    },
    Quest {
        id: "rust-55-partial-eq", chapter_id: "traits", number: 55, xp_reward: 25,
        answers: &[&["PartialEq"]], placeholders: &[Some("P…")],
        code: &[
            "#[derive(Clone, {slot})]",
            "struct Key(u8);",
            "fn main() {",
            "    let k1 = Key(1);",
            "    let k2 = Key(1);",
            "    println!(\"Duel: {}\", k1 == k2);",
            "}",
        ],
    },
    Quest {
        id: "rust-56-generic-t", chapter_id: "traits", number: 56, xp_reward: 25,
        answers: &[&["T"], &["T"]], placeholders: &[Some("T"), None],
        code: &[
            "// The token of generality: any type arrives from outside",
            "fn double<{slot}>(x: {slot2}) -> (T, T)",
            "where T: Clone {",
            "    (x.clone(), x.clone())",
            "}",
            "fn main() {",
            "    let pair = double(9);",
            "    println!(\"{:?}\", pair);",
            "}",
        ],
    },
    Quest {
        id: "rust-57-generic-pair", chapter_id: "traits", number: 57, xp_reward: 25,
        answers: &[&["T"], &["T"]], placeholders: &[None, Some("T")],
        code: &[
            "// One token serves both parameters at once",
            "fn first<{slot}>(a: {slot2}, b: T) -> T {",
            "    a",
            "}",
            "fn main() {",
            "    let won = first(\"scroll\", \"stone\");",
            "    println!(\"Winner: {}\", won);",
            "}",
        ],
    },
    Quest {
        id: "rust-58-display", chapter_id: "traits", number: 58, xp_reward: 25,
        answers: &[&["Display"]], placeholders: &[Some("D…")],
        code: &[
            "struct Guardian { name: String }",
            "    // The courteous presentation of a guardian for the reader",
            "impl std::fmt::{slot} for Guardian {",
            "    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {",
            "        write!(f, \"Guardian {}\", self.name)",
            "    }",
            "}",
            "fn main() {",
            "    let g = Guardian { name: String::from(\"Cora\") };",
            "    println!(\"{}\", g);",
            "}",
        ],
    },
    Quest {
        id: "rust-59-dyn", chapter_id: "traits", number: 59, xp_reward: 25,
        answers: &[&["dyn"]], placeholders: &[Some("dyn")],
        code: &[
            "struct Fish;",
            "trait Swimmer { fn swim(&self) -> &'static str; }",
            "impl Swimmer for Fish { fn swim(&self) -> &'static str { \"blub\" } }",
            "fn main() {",
            "    let fish = Fish;",
            "    // A trait object: the fish hides behind a common contract",
            "    let swimmer: &{slot} Swimmer = &fish;",
            "    println!(\"{}\", swimmer.swim());",
            "}",
        ],
    },
    Quest {
        id: "rust-60-where", chapter_id: "traits", number: 60, xp_reward: 25,
        answers: &[&["where"]], placeholders: &[Some("where")],
        code: &[
            "trait Spell { fn cast(&self) -> String; }",
            "struct Ice;",
            "impl Spell for Ice { fn cast(&self) -> String { String::from(\"swish!\") } }",
            "// The where hall lines the bounds up in an even row",
            "fn chant<T>(spell: T) -> String",
            "    {slot}",
            "        T: Spell,",
            "{",
            "    spell.cast()",
            "}",
            "fn main() {",
            "    println!(\"{}\", chant(Ice));",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 7: Display & Debug (61..=68)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-61-debug", chapter_id: "formatting", number: 61, xp_reward: 25,
        answers: &[&["Debug"], &["{:?}"]], placeholders: &[Some("Debug"), None],
        code: &[
            "  // The badge of honesty: {:?} requires this trait",
            "#[derive({slot})]",
            "struct Scroll { id: u32 }",
            "fn main() {",
            "    let s = Scroll { id: 7 };",
            "    println!(\"{slot2}\", s);",
            "}",
        ],
    },
    Quest {
        id: "rust-62-debug-fmt", chapter_id: "formatting", number: 62, xp_reward: 25,
        answers: &[&["Debug"], &["{:?}"]], placeholders: &[None, Some("{:…")],
        code: &[
            "  // derive(Debug) is already in place",
            "#[derive({slot})]",
            "struct Scroll { id: u32 }",
            "fn main() {",
            "    let s = Scroll { id: 7 };",
            "    // The mirror of honesty: asking for Debug output",
            "    println!(\"Scroll: {slot2}\", s);",
            "}",
        ],
    },
    Quest {
        id: "rust-63-pretty-debug", chapter_id: "formatting", number: 63, xp_reward: 25,
        answers: &[&["Debug"], &["{:#?}"]], placeholders: &[None, None],
        code: &[
            "  // derive(Debug) is already in place",
            "#[derive({slot})]",
            "struct Scroll { id: u32, master: &'static str }",
            "fn main() {",
            "    let s = Scroll { id: 7, master: \"Ferris\" };",
            "    // Indentation lays the bones out on the shelves",
            "    println!(\"{slot2}\", s);",
            "}",
        ],
    },
    Quest {
        id: "rust-64-display-impl", chapter_id: "formatting", number: 64, xp_reward: 25,
        answers: &[&["Display"]], placeholders: &[Some("D…")],
        code: &[
            "struct Rune { label: &'static str }",
            "    // The type learns to present itself in words",
            "impl std::fmt::{slot} for Rune {",
            "    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {",
            "        write!(f, \"⟨{}⟩\", self.label)",
            "    }",
            "}",
            "fn main() {",
            "    let r = Rune { label: \"alpha\" };",
            "    println!(\"{}\", r);",
            "}",
        ],
    },
    Quest {
        id: "rust-65-write", chapter_id: "formatting", number: 65, xp_reward: 25,
        answers: &[&["write!"]], placeholders: &[Some("w…!")],
        code: &[
            "struct Rune { label: &'static str }",
            "impl std::fmt::Display for Rune {",
            "    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {",
            "        // Ink flows into the formatter",
            "        let out = {slot}(f, \"⟨{}⟩\", self.label);",
            "        out",
            "    }",
            "}",
            "fn main() {",
            "    let r = Rune { label: \"omega\" };",
            "    println!(\"{}\", r);",
            "}",
        ],
    },
    Quest {
        id: "rust-66-decimal", chapter_id: "formatting", number: 66, xp_reward: 25,
        answers: &[&["{:.2}"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let power = 3.14159;",
            "    // Precise scales: two digits after the decimal point",
            "    println!(\"Power: {slot}\", power);",
            "}",
        ],
    },
    Quest {
        id: "rust-67-width-right", chapter_id: "formatting", number: 67, xp_reward: 25,
        answers: &[&["{:>8}"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let title = \"Ferris\";",
            "    // Right edge, field width 8",
            "    println!(\"{slot}|\", title);",
            "}",
        ],
    },
    Quest {
        id: "rust-68-zero-pad", chapter_id: "formatting", number: 68, xp_reward: 25,
        answers: &[&["{:08}"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let scroll_id = 42;",
            "    // Zero padding: the number in ceremonial width",
            "    println!(\"Artifact #{slot}\", scroll_id);",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 8: Result & Errors (69..=77)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-69-result", chapter_id: "errors", number: 69, xp_reward: 25,
        answers: &[&["Result"]], placeholders: &[Some("Result")],
        code: &[
            "// The keeper of the solemn seal: the function promises both outcomes",
            "fn find_relic() -> {slot}<i32, String> {",
            "    Ok(9001)",
            "}",
            "fn main() {",
            "    println!(\"{:?}\", find_relic());",
            "}",
        ],
    },
    Quest {
        id: "rust-70-err", chapter_id: "errors", number: 70, xp_reward: 25,
        answers: &[&["Err"]], placeholders: &[Some("Err")],
        code: &[
            "fn find_relic() -> Result<i32, String> {",
            "    // The relic is not found — return the cargo of failure",
            "    {slot}(\"The chest is empty\".to_string())",
            "}",
            "fn main() {",
            "    println!(\"{:?}\", find_relic());",
            "}",
        ],
    },
    Quest {
        id: "rust-71-unwrap", chapter_id: "errors", number: 71, xp_reward: 25,
        answers: &[&["unwrap"]], placeholders: &[None],
        code: &[
            "fn find_relic() -> Result<i32, String> { Ok(7) }",
            "fn main() {",
            "    // The bold unsealing: panic on Err",
            "    let relic = find_relic().{slot}();",
            "    println!(\"Relic: {}\", relic);",
            "}",
        ],
    },
    Quest {
        id: "rust-72-expect", chapter_id: "errors", number: 72, xp_reward: 25,
        answers: &[&["expect"]], placeholders: &[Some("e…")],
        code: &[
            "fn find_relic() -> Result<i32, String> { Err(String::from(\"empty\")) }",
            "fn main() {",
            "    // Unsealing with your own sorrow",
            "    let relic = find_relic().{slot}(\"The relic is gone!\");",
            "    let _ = relic;",
            "}",
        ],
    },
    Quest {
        id: "rust-73-question", chapter_id: "errors", number: 73, xp_reward: 25,
        answers: &[&["?"]], placeholders: &[None],
        code: &[
            "// The relay rune carries the error upward in one sign",
            "fn read_scroll(path: &str) -> Result<String, std::io::Error> {",
            "    let text = std::fs::read_to_string(path){slot};",
            "    Ok(text)",
            "}",
            "fn main() {",
            "    let out = read_scroll(\"Chronicles.txt\");",
            "    println!(\"{:?}\", out);",
            "}",
        ],
    },
    Quest {
        id: "rust-74-unwrap-or", chapter_id: "errors", number: 74, xp_reward: 25,
        answers: &[&["unwrap_or"]], placeholders: &[Some("unwrap_…")],
        code: &[
            "fn find_relic() -> Result<i32, String> { Err(String::from(\"empty\")) }",
            "fn main() {",
            "    // Misfortune swapped for a coin kept at hand",
            "    let relic = find_relic().{slot}(-1);",
            "    println!(\"In hands: {}\", relic);",
            "}",
        ],
    },
    Quest {
        id: "rust-75-map-err", chapter_id: "errors", number: 75, xp_reward: 25,
        answers: &[&["map_err"]], placeholders: &[Some("map_…")],
        code: &[
            "fn parse_key(raw: &str) -> Result<i32, String> {",
            "    let n: i32 = raw.trim().parse()",
            "        // Reforge the error into a courteous form",
            "        .{slot}(|_| \"Key is not a number\".to_string())?;",
            "    Ok(n)",
            "}",
            "fn main() {",
            "    println!(\"{:?}\", parse_key(\"13\"));",
            "}",
        ],
    },
    Quest {
        id: "rust-76-match-ok-err", chapter_id: "errors", number: 76, xp_reward: 25,
        answers: &[&["Ok(n)"]], placeholders: &[Some("Ok(…)")],
        code: &[
            "fn main() {",
            "    // Both hands: greet Ok with joy, keep Err in the rear",
            "    let report = match \"18\".parse::<i32>() {",
            "        {slot} => format!(\"Read: {}\", n),",
            "        Err(e) => format!(\"Failed: {}\", e),",
            "    };",
            "    println!(\"{}\", report);",
            "}",
        ],
    },
    Quest {
        id: "rust-77-io", chapter_id: "errors", number: 77, xp_reward: 25,
        answers: &[&["io"]], placeholders: &[Some("i…")],
        code: &[
            "// The input-output gate of the standard library",
            "use std::{slot};",
            "fn main() {",
            "    let mut line = String::new();",
            "    io::stdin().read_line(&mut line).unwrap();",
            "    println!(\"You said: {}\", line.trim());",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 9: Smart Pointers (78..=86)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-78-box", chapter_id: "pointers", number: 78, xp_reward: 25,
        answers: &[&["Box"], &["Box"]], placeholders: &[Some("Box"), None],
        code: &[
            "fn main() {",
            "    // Heap apartments for a bulky tenant",
            "    let heap_scroll: {slot}<u64> = {slot2}::new(9000);",
            "    println!(\"On the heap: {}\", heap_scroll);",
            "}",
        ],
    },
    Quest {
        id: "rust-79-deref", chapter_id: "pointers", number: 79, xp_reward: 25,
        answers: &[&["*"]], placeholders: &[Some("*")],
        code: &[
            "fn main() {",
            "    let heap_scroll = Box::new(77);",
            "    // Tunnel through the pointer to the very value",
            "    println!(\"Treasure: {}\", {slot}heap_scroll);",
            "}",
        ],
    },
    Quest {
        id: "rust-80-dyn-box", chapter_id: "pointers", number: 80, xp_reward: 25,
        answers: &[&["dyn"]], placeholders: &[Some("d…")],
        code: &[
            "trait Spell { fn cast(&self) -> &'static str; }",
            "struct Fireball;",
            "impl Spell for Fireball { fn cast(&self) -> &'static str { \"fsssh!\" } }",
            "fn main() {",
            "    // The heap does not care what kind of guest: all speak the trait's tongue",
            "    let spell: Box<{slot} Spell> = Box::new(Fireball);",
            "    println!(\"{}\", spell.cast());",
            "}",
        ],
    },
    Quest {
        id: "rust-81-rc", chapter_id: "pointers", number: 81, xp_reward: 25,
        answers: &[&["Rc"], &["Rc"]], placeholders: &[None, Some("R…")],
        code: &[
            "use std::rc::Rc;",
            "fn main() {",
            "    // The registry of owners: everyone reads the scroll, no copies",
            "    let scroll: {slot}<String> = {slot2}::new(\"Rare Scroll\".to_string());",
            "    println!(\"{}\", scroll);",
            "}",
        ],
    },
    Quest {
        id: "rust-82-rc-clone", chapter_id: "pointers", number: 82, xp_reward: 25,
        answers: &[&["Rc::clone"]], placeholders: &[Some("Rc::c…")],
        code: &[
            "use std::rc::Rc;",
            "fn main() {",
            "    let scroll = Rc::new(\"Covenant\".to_string());",
            "    // One more name in the registry (data is not copied)",
            "    let second = {slot}(&scroll);",
            "    println!(\"{} / {}\", scroll, second);",
            "}",
        ],
    },
    Quest {
        id: "rust-83-strong-count", chapter_id: "pointers", number: 83, xp_reward: 25,
        answers: &[&["strong_count"]], placeholders: &[Some("s…_count")],
        code: &[
            "use std::rc::Rc;",
            "fn main() {",
            "    let scroll = Rc::new(\"Runic scroll\".to_string());",
            "    let _copy = Rc::clone(&scroll);",
            "    // How many hands hold the rune?",
            "    println!(\"Owners: {}\", Rc::{slot}(&scroll));",
            "}",
        ],
    },
    Quest {
        id: "rust-84-refcell", chapter_id: "pointers", number: 84, xp_reward: 25,
        answers: &[&["RefCell"], &["RefCell"]], placeholders: &[Some("RefCell"), None],
        code: &[
            "use std::cell::RefCell;",
            "fn main() {",
            "    // The cell of interior mutation",
            "    let power_cell: {slot}<i32> = {slot2}::new(10);",
            "    println!(\"Power: {:?}\", power_cell);",
            "}",
        ],
    },
    Quest {
        id: "rust-85-borrow", chapter_id: "pointers", number: 85, xp_reward: 25,
        answers: &[&["borrow"]], placeholders: &[Some("borrow")],
        code: &[
            "use std::cell::RefCell;",
            "fn main() {",
            "    let power_cell = RefCell::new(10);",
            "    // The lens of reading: look, do not touch",
            "    let n = power_cell.{slot}();",
            "    println!(\"Power: {}\", n);",
            "}",
        ],
    },
    Quest {
        id: "rust-86-borrow-mut", chapter_id: "pointers", number: 86, xp_reward: 25,
        answers: &[&["borrow_mut"]], placeholders: &[Some("borrow_…")],
        code: &[
            "use std::cell::RefCell;",
            "fn main() {",
            "    let power_cell = RefCell::new(10);",
            "    // The only handle: write under runtime supervision",
            "    *power_cell.{slot}() += 5;",
            "    println!(\"New power: {}\", power_cell.borrow());",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 10: Threads (87..=95)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-87-spawn", chapter_id: "threads", number: 87, xp_reward: 25,
        answers: &[&["spawn"]], placeholders: &[Some("spawn")],
        code: &[
            "fn main() {",
            "    // Ignite the second heart of the Garden",
            "    std::thread::{slot}(|| println!(\"The thread sings\"));",
            "    println!(\"The main thread hums\");",
            "}",
        ],
    },
    Quest {
        id: "rust-88-move", chapter_id: "threads", number: 88, xp_reward: 25,
        answers: &[&["move"]], placeholders: &[Some("move")],
        code: &[
            "fn main() {",
            "    let owner_name = String::from(\"Cora\");",
            "    // The caravan takes ownership of the name",
            "    std::thread::spawn({slot}|| println!(\"Keeping the name: {}\", owner_name));",
            "}",
        ],
    },
    Quest {
        id: "rust-89-join", chapter_id: "threads", number: 89, xp_reward: 25,
        answers: &[&["join"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    let handle = std::thread::spawn(|| 42);",
            "    // Let us await the traveler's return",
            "    let answer = handle.{slot}().unwrap();",
            "    println!(\"Answer: {}\", answer);",
            "}",
        ],
    },
    Quest {
        id: "rust-90-arc", chapter_id: "threads", number: 90, xp_reward: 25,
        answers: &[&["Arc"], &["Arc"]], placeholders: &[Some("Arc"), None],
        code: &[
            "use std::sync::Arc;",
            "fn main() {",
            "    // The atomic registry of owners",
            "    let counter: {slot}<u32> = {slot2}::new(0);",
            "    println!(\"{:?}\", counter);",
            "}",
        ],
    },
    Quest {
        id: "rust-91-arc-clone", chapter_id: "threads", number: 91, xp_reward: 25,
        answers: &[&["Arc::clone"]], placeholders: &[Some("Arc::c…")],
        code: &[
            "use std::sync::Arc;",
            "fn main() {",
            "    let counter = Arc::new(0);",
            "    // The thread-safe caravan: the counter grows",
            "    let c2 = {slot}(&counter);",
            "    println!(\"{}, {}\", c2, counter);",
            "}",
        ],
    },
    Quest {
        id: "rust-92-mutex", chapter_id: "threads", number: 92, xp_reward: 25,
        answers: &[&["Mutex"], &["Mutex"]], placeholders: &[None, Some("M…")],
        code: &[
            "use std::sync::Mutex;",
            "fn main() {",
            "    // The golden chest: one hand at a time",
            "    let vault: {slot}<i32> = {slot2}::new(500);",
            "    println!(\"{:?}\", vault);",
            "}",
        ],
    },
    Quest {
        id: "rust-93-lock", chapter_id: "threads", number: 93, xp_reward: 25,
        answers: &[&["lock"]], placeholders: &[Some("lock")],
        code: &[
            "use std::sync::Mutex;",
            "fn main() {",
            "    let vault = Mutex::new(500);",
            "    // The guard unlocks when the scope ends",
            "    let mut gold = vault.{slot}().unwrap();",
            "    *gold += 100;",
            "    println!(\"Gold: {}\", gold);",
            "}",
        ],
    },
    Quest {
        id: "rust-94-mpsc", chapter_id: "threads", number: 94, xp_reward: 25,
        answers: &[&["mpsc"]], placeholders: &[Some("m…")],
        code: &[
            "fn main() {",
            "    // The channel: many senders, one receiver",
            "    let (tx, rx) = std::sync::{slot}::channel();",
            "    tx.send(\"the message\").unwrap();",
            "    println!(\"Received: {}\", rx.recv().unwrap());",
            "}",
        ],
    },
    Quest {
        id: "rust-95-send", chapter_id: "threads", number: 95, xp_reward: 25,
        answers: &[&["send"]], placeholders: &[Some("send")],
        code: &[
            "fn main() {",
            "    let (tx, rx) = std::sync::mpsc::channel();",
            "    // Release the lantern into the channel's flow",
            "    tx.{slot}(7).unwrap();",
            "    println!(\"Lantern: {}\", rx.recv().unwrap());",
            "}",
        ],
    },
    // ---------------------------------------------------------------
    // Chapter 11: Files (96..=103)
    // ---------------------------------------------------------------
    Quest {
        id: "rust-96-read-to-string", chapter_id: "files", number: 96, xp_reward: 25,
        answers: &[&["read_to_string"]], placeholders: &[Some("read_to_string")],
        code: &[
            "fn main() {",
            "    // The whole ritual: the entire file into one String",
            "    let text = std::fs::{slot}(\"Chronicles.txt\").unwrap();",
            "    println!(\"Chars: {}\", text.len());",
            "}",
        ],
    },
    Quest {
        id: "rust-97-fs-write", chapter_id: "files", number: 97, xp_reward: 25,
        answers: &[&["write"]], placeholders: &[Some("write")],
        code: &[
            "fn main() {",
            "    // The ink altar: rewrite the file in one stroke",
            "    std::fs::{slot}(\"Beacon.txt\", \"Ferris was here\").unwrap();",
            "    println!(\"The ink is drying!\");",
            "}",
        ],
    },
    Quest {
        id: "rust-98-file", chapter_id: "files", number: 98, xp_reward: 25,
        answers: &[&["File"]], placeholders: &[Some("File")],
        code: &[
            "fn main() {",
            "    // Bind to the already existing volume",
            "    let mut scroll_file = std::fs::{slot}::open(\"Chronicles.txt\").unwrap();",
            "    let _ = &mut scroll_file;",
            "    println!(\"The tome is open!\");",
            "}",
        ],
    },
    Quest {
        id: "rust-99-write-all", chapter_id: "files", number: 99, xp_reward: 25,
        answers: &[&["write_all"]], placeholders: &[Some("write_…")],
        code: &[
            "use std::io::Write;",
            "fn main() {",
            "    let mut file = std::fs::File::create(\"Covenant.txt\").unwrap();",
            "    // Seal it byte by byte",
            "    file.{slot}(b\"Ferris was here\").unwrap();",
            "}",
        ],
    },
    Quest {
        id: "rust-100-bufreader", chapter_id: "files", number: 100, xp_reward: 25,
        answers: &[&["BufReader"]], placeholders: &[Some("Buf…")],
        code: &[
            "use std::io::BufReader;",
            "fn main() {",
            "    let file = std::fs::File::open(\"Chronicles.txt\").unwrap();",
            "    // A caravan with a leather bag: fewer trips",
            "    let reader = {slot}::new(file);",
            "    let _ = reader;",
            "}",
        ],
    },
    Quest {
        id: "rust-101-read-line", chapter_id: "files", number: 101, xp_reward: 25,
        answers: &[&["read_line"]], placeholders: &[Some("read_line")],
        code: &[
            "fn main() {",
            "    let mut buf = String::new();",
            "    // Sip by sip: one line with the newline",
            "    let n = std::io::stdin().{slot}(&mut buf).unwrap();",
            "    println!(\"Bytes read: {}\", n);",
            "}",
        ],
    },
    Quest {
        id: "rust-102-metadata", chapter_id: "files", number: 102, xp_reward: 25,
        answers: &[&["metadata"]], placeholders: &[None],
        code: &[
            "fn main() {",
            "    // Weigh the tome without opening it",
            "    let meta = std::fs::{slot}(\"Chronicles.txt\").unwrap();",
            "    println!(\"Size: {} bytes\", meta.len());",
            "}",
        ],
    },
    Quest {
        id: "rust-103-remove-file", chapter_id: "files", number: 103, xp_reward: 25,
        answers: &[&["remove_file"]], placeholders: &[Some("remove_…")],
        code: &[
            "fn main() {",
            "    // The archive will forget this page forever",
            "    std::fs::{slot}(\"Ancient_drafts.txt\").unwrap();",
            "    println!(\"The rune is extinguished!\");",
            "}",
        ],
    },
];

// ---------------------------------------------------------------------------
// Integrity tests: exact counts, orderings, i18n coverage, single slot per
// quest, and answer validation semantics.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_and_ordering() {
        assert_eq!(QUESTS.len(), 103);
        assert_eq!(CHAPTERS.len(), 11);

        // Global numbers are strictly contiguous and ordered: 1..=103.
        for (i, q) in QUESTS.iter().enumerate() {
            assert_eq!(q.number as usize, i + 1, "quest {} breaks ordering", q.id);
        }
        // Chapter numbers are strictly contiguous: 1..=11.
        for (i, c) in CHAPTERS.iter().enumerate() {
            assert_eq!(c.number as usize, i + 1);
        }
        // Historical quest ids remain intact (DB achievements reference them).
        for qid in [
            "rust-01-mutability", "rust-02-shadowing", "rust-03-ownership",
            "rust-04-references", "rust-05-slices",
        ] {
            assert!(find_quest(qid).is_some(), "missing historical quest {qid}");
        }

        // Every quest's code block declares its slots in order: slot 1 =
        // `{slot}`, slot i = `{sloti}`; exactly one marker per declared slot,
        // no stray markers, at most one marker per line, and the
        // placeholders array covers every slot.
        for q in QUESTS {
            let expected = q.answers.len();
            assert!(
                expected >= 1 && expected <= 9,
                "quest {} has an impossible slot count {expected}",
                q.id
            );
            assert_eq!(
                q.placeholders.len(),
                expected,
                "quest {} must cover every slot in placeholders",
                q.id
            );
            for i in 1..=expected {
                let marker = Quest::slot_marker(i);
                assert_eq!(
                    q.code.iter().map(|l| l.matches(&marker).count()).sum::<usize>(),
                    1,
                    "quest {} slot {i}: marker {marker} must appear exactly once",
                    q.id
                );
            }
            // No stray `{slot`-shaped markers beyond the declared slots.
            // Counted as substring occurrences — several slots may share one
            // line, e.g. `let ruby = ({slot}, {slot2}, {slot3});`.
            assert_eq!(
                q.code.iter().map(|l| l.matches("{slot").count()).sum::<usize>(),
                expected,
                "quest {} has stray slot markers",
                q.id
            );
        }
    }

    #[test]
    fn i18n_coverage() {
        let en = crate::i18n::get_translation("en");
        let ru = crate::i18n::get_translation("ru");
        let ko = crate::i18n::get_translation("ko");

        for t in [en, ru] {
            assert_eq!(t.quests_titles.len(), 103, "titles for {}", t.lang);
            assert_eq!(t.quests_concepts.len(), 103, "concepts for {}", t.lang);
            assert_eq!(t.quests_lores.len(), 103, "lores for {}", t.lang);
            assert_eq!(t.quests_hints.len(), 103, "hints for {}", t.lang);
        }
        // KO intentionally stays at the historical 5 entries and falls back
        // to EN for the rest (see getters).
        assert_eq!(ko.quests_titles.len(), 5);

        // Chapter-level arrays exist for every language in full.
        for t in [en, ru, ko] {
            assert_eq!(t.chapters_titles.len(), 11);
            assert_eq!(t.chapters_descs.len(), 11);
            assert_eq!(t.chapters_mascot_initial.len(), 11);
            assert_eq!(t.chapters_mascot_success.len(), 11);
        }
    }

    #[test]
    fn fallback_and_mascots() {
        let ko = crate::i18n::get_translation("ko");
        let en = crate::i18n::get_translation("en");

        // KO quest 103 (index 102) falls back to the EN title.
        let q103 = find_quest("rust-103-remove-file").unwrap();
        assert_eq!(q103.get_title(ko), en.quests_titles[102]);

        // Mascot speech is chapter-level: quests of one chapter share it.
        let q1 = find_quest("rust-01-mutability").unwrap();
        let q12 = find_quest("rust-12-cast").unwrap();
        assert_eq!(q1.get_mascot_initial(en), q12.get_mascot_initial(en));
    }

    #[test]
    fn answer_validation() {
        let q1 = find_quest("rust-01-mutability").unwrap();
        assert!(q1.validate_tokens(&["mut"]));
        assert!(q1.validate_tokens(&["  mut "]));
        assert!(!q1.validate_tokens(&["let"]));

        // Multi-slot quest 9: three slots, strict order, all must match.
        let q9 = find_quest("rust-09-tuples").unwrap();
        assert!(q9.validate_tokens(&["255", "0", "0"]));
        assert!(q9.validate_tokens(&[" 255 ", "0", "0"]));
        assert!(!q9.validate_tokens(&["0", "255", "0"])); // wrong order
        assert!(!q9.validate_tokens(&["255", "1", "0"])); // wrong value
        assert!(!q9.validate_tokens(&["255"]));           // wrong slot count

        // Quest 40 accepts three alternative spellings in its single slot.
        let q40 = find_quest("rust-40-sum").unwrap();
        assert!(q40.validate_tokens(&["sum"]));
        assert!(q40.validate_tokens(&["sum::<i32>"]));
        assert!(q40.validate_tokens(&["sum::<_>"]));
        assert!(!q40.validate_tokens(&["fold"]));

        // Strong typing: quest 6 wants i64 (pinned by the fn signature).
        let q6 = find_quest("rust-06-integers").unwrap();
        assert!(q6.validate_tokens(&["i64"]));
        assert!(!q6.validate_tokens(&["i32"]));

        // Quest 10: the third crystal row sits at index 2.
        let q10 = find_quest("rust-10-arrays").unwrap();
        assert!(q10.validate_tokens(&["2"]));
        assert!(!q10.validate_tokens(&["[10, 20, 30]"]));
    }

    #[test]
    fn canonicalization_rules() {
        // One token passes through untouched (trim only).
        assert_eq!(canonicalize_tokens("mut"), "mut");
        assert_eq!(canonicalize_tokens("  &mut  "), "&mut");

        // Whitespace between two word chars collapses to ONE space, never
        // vanishes; if either neighbour is a non-word char, it vanishes.
        assert_eq!(canonicalize_tokens("& mut"), "&mut");
        assert_eq!(canonicalize_tokens("& mu t"), "&mu t");
        assert_eq!(canonicalize_tokens("a b"), "a b");
        assert_eq!(canonicalize_tokens("a  b"), "a b");
        assert_eq!(canonicalize_tokens("a b; c"), "a b;c");

        // The digit·dot·digit exception keeps four distinct spellings apart.
        let four = "1 . 2 1. 2 1 .2 1.2";
        assert_eq!(canonicalize_tokens(four), four);

        // The docstring example, verbatim.
        assert_eq!(
            canonicalize_tokens(
                " & mu t 1 . 2 1. 2 1 .2 1.2 t1 23 45 67 89 0_ &m _ut _a; _b, 0| _1: 2. _3/ 4 "
            ),
            "&mu t 1 . 2 1. 2 1 .2 1.2 t1 23 45 67 89 0_&m _ut _a;_b,0|_1:2._3/4"
        );
    }

    #[test]
    fn xp_of_first_five() {
        // The old ascending scale is gone: quests 1..=5 cost a flat 25 each.
        for q in QUESTS.iter().take(5) {
            assert_eq!(q.xp_reward, 25, "quest {}", q.id);
        }
    }

    #[test]
    fn navigation_helpers() {
        assert!(prev_quest(1).is_none());
        assert!(prev_quest(2).is_some());
        assert_eq!(next_quest(102).map(|q| q.id), Some("rust-103-remove-file"));
        assert!(next_quest(104).is_none());

        // Chapter lookup by slug AND by number both resolve.
        assert_eq!(find_chapter("basics").unwrap().number, 1);
        assert_eq!(find_chapter("11").unwrap().id, "files");
        assert!(find_chapter("nope").is_none());

        // Chapter sizes: 12 + 10 + 10 + 10 + 9 + 9 + 8 + 9 + 9 + 9 + 8 = 103.
        let sizes: Vec<usize> = CHAPTERS.iter()
            .map(|c| chapter_quests(c.id).len())
            .collect();
        assert_eq!(sizes, vec![12, 10, 10, 10, 9, 9, 8, 9, 9, 9, 8]);
        assert_eq!(sizes.iter().sum::<usize>(), 103);
    }

    #[test]
    fn check_answers_length() {
        for (i, q) in QUESTS.iter().enumerate() {
            for &ans_per_slot in q.answers {
                for &a in ans_per_slot {
                    println!("[{i}] = {}", a.len());
                }
            }
        }
    }
}
