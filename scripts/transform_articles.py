#!/usr/bin/env python3
"""Transform src/articles.rs from the legacy per-language-fields layout
(title_en/title_ru/title_ko, paragraphs_en/ru/ko) into the flat
(chapter_id, lang, title, paragraphs) layout, lang-major:
an EN block, then RU, then KO — a future language is just another block.

Usage:
  python scripts/transform_articles.py --gen    # one-time rebuild from legacy
  python scripts/transform_articles.py --splice # insert scripts/fragments/*_{lang}.rs
  python scripts/transform_articles.py --clean  # drop the @@INSERT markers
"""
import glob
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(ROOT, "src", "articles.rs")
FRAGMENTS = os.path.join(ROOT, "scripts", "fragments")

CHAPTER_NUMBERS = {
    "basics": 1, "flow": 2, "forms": 3, "collections": 4, "strings": 5,
    "traits": 6, "formatting": 7, "errors": 8, "pointers": 9, "threads": 10,
    "files": 11,
}
LANGS = ("en", "ru", "ko")
LANG_HEADERS = {
    "en": "EN — English editions",
    "ru": "RU — Russian editions",
    "ko": "KO — Korean editions",
}

# A historical typo: the threads KO article title must echo the i18n chapter
# title ("스레드의 합창"); the article carried an extra syllable.
TITLE_FIXES = {
    ("threads", "ko"): "스레드의 합창",
}


def extract_array(block: str, field: str) -> str:
    """Return the verbatim inner text of `field: &[ ... ]` (bracket depth +
    string-literal aware; multi-line literals included)."""
    marker = f"{field}: &["
    start = block.index(marker) + len(marker)
    i = start
    depth, in_str, esc = 1, False, False
    while i < len(block):
        c = block[i]
        if in_str:
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == '"':
                in_str = False
        else:
            if c == '"':
                in_str = True
            elif c == "[":
                depth += 1
            elif c == "]":
                depth -= 1
                if depth == 0:
                    break
        i += 1
    assert depth == 0, f"unbalanced array in {field}"
    return block[start:i]


def parse_legacy(text: str):
    starts = [m.start() for m in re.finditer(r"(?m)^    Article \{", text)]
    assert len(starts) == 6, f"expected 6 legacy Article blocks, found {len(starts)}"
    parsed = []
    for i, s in enumerate(starts):
        end = starts[i + 1] if i + 1 < len(starts) else text.index("\n];", s)
        b = text[s:end]
        chapter_id = re.search(r'chapter_id: "([^"]+)",', b).group(1)
        titles = {}
        for lang in LANGS:
            m = re.search(rf'title_{lang}: "((?:[^"\\]|\\.)*)",', b)
            assert m, f"no title_{lang} in block {chapter_id}"
            titles[lang] = TITLE_FIXES.get((chapter_id, lang), m.group(1))
        paras = {lang: extract_array(b, f"paragraphs_{lang}") for lang in LANGS}
        parsed.append((chapter_id, titles, paras))
    return parsed


TESTS = '''
#[cfg(test)]
mod tests {
    use super::*;

    /// Every entry targets a real chapter and a known language, carries a
    /// title and non-empty paragraphs, and leaks no `{slot}` markers.
    #[test]
    fn articles_follow_chapter_ids_and_langs() {
        for a in ARTICLES {
            assert!(
                crate::quests::CHAPTERS.iter().any(|c| c.id == a.chapter_id),
                "article chapter {} unknown", a.chapter_id
            );
            assert!(
                matches!(a.lang, "en" | "ru" | "ko"),
                "article lang {} unknown", a.lang
            );
            assert!(
                !a.title.is_empty(),
                "empty title for {} / {}", a.chapter_id, a.lang
            );
            assert!(
                !a.paragraphs.is_empty(),
                "empty paragraphs for {} / {}", a.chapter_id, a.lang
            );
            for p in a.paragraphs {
                assert!(
                    !p.contains("{slot}"),
                    "stray slot in {} / {}", a.chapter_id, a.lang
                );
            }
        }
    }

    /// Every chapter with an article must carry it in all three languages,
    /// and the language editions must have equal paragraph counts.
    #[test]
    fn article_langs_are_parallel() {
        let mut chapters: Vec<&str> = ARTICLES.iter().map(|a| a.chapter_id).collect();
        chapters.sort_unstable();
        chapters.dedup();

        for chapter_id in chapters {
            let edition = |lang: &str| {
                ARTICLES
                    .iter()
                    .find(|a| a.chapter_id == chapter_id && a.lang == lang)
                    .map(|a| a.paragraphs.len())
            };
            let (en, ru, ko) = (edition("en"), edition("ru"), edition("ko"));
            assert!(en.is_some(), "chapter {} misses the EN article", chapter_id);
            assert!(ru.is_some(), "chapter {} misses the RU article", chapter_id);
            assert!(ko.is_some(), "chapter {} misses the KO article", chapter_id);
            assert_eq!(en, ru, "paragraph count mismatch (en/ru) in {}", chapter_id);
            assert_eq!(en, ko, "paragraph count mismatch (en/ko) in {}", chapter_id);
        }
    }

    /// The article title must echo the localized chapter title.
    #[test]
    fn article_titles_match_chapter_titles() {
        for a in ARTICLES {
            let t = crate::i18n::get_translation(a.lang);
            let chapter = crate::quests::find_chapter(a.chapter_id)
                .unwrap_or_else(|| panic!("unknown chapter {}", a.chapter_id));
            assert_eq!(
                a.title,
                chapter.title(&t),
                "title mismatch for {} / {}", a.chapter_id, a.lang
            );
        }
    }
}
'''

HEADER = '''//! Codex deep-dive articles — one long-form self-study essay per chapter,
//! rendered as trusted HTML through `| safe`.
//!
//! Flat registry: one entry per (chapter_id, lang) pair, grouped language by
//! language (EN block, RU block, KO block) so a future language is just one
//! more appended group. Chapters without any entry render the graceful
//! "coming soon" page.

pub struct Article {
    pub chapter_id: &'static str,
    pub lang: &'static str, // languages of articles are separate
    pub title: &'static str,
    pub paragraphs: &'static [&'static str],
}

/// The article for an exact (chapter, language) pair; falls back to the
/// English edition when the requested language has no text of its own yet.
pub fn find_article(chapter_id: &str, lang: &str) -> Option<&'static Article> {
    ARTICLES
        .iter()
        .find(|a| a.chapter_id == chapter_id && a.lang == lang)
        .or_else(|| {
            ARTICLES
                .iter()
                .find(|a| a.chapter_id == chapter_id && a.lang == "en")
        })
}

/// Does the chapter carry an article in any language? (card badges, links)
pub fn has_article(chapter_id: &str) -> bool {
    ARTICLES.iter().any(|a| a.chapter_id == chapter_id)
}

pub static ARTICLES: &[Article] = &['''


def gen_file(parsed, splice=False, clean=False):
    out = [HEADER]
    for lang in LANGS:
        out.append("")
        out.append("    // " + "=" * 69)
        out.append(f"    // {LANG_HEADERS[lang]}")
        out.append("    // " + "=" * 69)
        for chapter_id, titles, paras in parsed:
            num = CHAPTER_NUMBERS.get(chapter_id)
            label = f"C{num} {chapter_id}" if num else chapter_id
            out.append(f"    // {label}")
            out.append("    Article {")
            out.append(f'        chapter_id: "{chapter_id}",')
            out.append(f'        lang: "{lang}",')
            out.append(f'        title: "{titles[lang]}",')
            # paras[lang] opens with "\n" and closes with "\n        " —
            # verbatim copy keeps the original element indentation.
            out.append("        paragraphs: &[" + paras[lang] + "],")
            out.append("    },")
            out.append("")
        if splice:
            for path in sorted(glob.glob(os.path.join(FRAGMENTS, f"*_{lang}.rs"))):
                with open(path, encoding="utf-8") as f:
                    out.append(f.read().rstrip("\n"))
                out.append("")
        if not clean:
            out.append(f"    // @@INSERT:{lang}@@")
            out.append("")
    out.append("];")
    out.append(TESTS.rstrip("\n"))
    out.append("")
    return "\n".join(out)


def splice_file(text: str) -> str:
    """Insert fragment blocks right before each `// @@INSERT:{lang}@@`
    marker of the already-flat file (markers are kept)."""
    for lang in LANGS:
        marker = f"    // @@INSERT:{lang}@@"
        if marker not in text:
            print(f"warning: no marker for {lang}")
            continue
        blocks = []
        for path in sorted(glob.glob(os.path.join(FRAGMENTS, f"*_{lang}.rs"))):
            with open(path, encoding="utf-8") as f:
                blocks.append(f.read().rstrip("\n"))
        if blocks:
            text = text.replace(marker, "\n\n".join(blocks) + "\n\n" + marker, 1)
    return text


def clean_file(text: str) -> str:
    return re.sub(r"(?m)^    // @@INSERT:(?:en|ru|ko)@@\n\n", "", text)


def read_src() -> str:
    with open(SRC, encoding="utf-8") as f:
        return f.read()


def write_src(text: str) -> None:
    with open(SRC, "w", encoding="utf-8", newline="") as f:
        f.write(text)


def main():
    if "--gen" in sys.argv:
        splice = "--splice" in sys.argv or "--clean" in sys.argv
        clean = "--clean" in sys.argv
        write_src(gen_file(parse_legacy(read_src()), splice=splice, clean=clean))
        print(f"OK: rebuilt {SRC} (legacy -> flat)")
    elif "--splice" in sys.argv:
        write_src(splice_file(read_src()))
        print("OK: fragments spliced")
    elif "--clean" in sys.argv:
        write_src(clean_file(read_src()))
        print("OK: markers removed")
    else:
        print(__doc__)


if __name__ == "__main__":
    main()
