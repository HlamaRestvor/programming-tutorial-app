    // C5 strings
    Article {
        chapter_id: "strings",
        lang: "en",
        title: "Scrolls of Strings",
        paragraphs: &[
            "<p>Numbers are honest; text is <i>crafty</i>. A number is a fixed size on the stack, but a string is a story that can grow, shrink, be stitched and rewritten. This chapter unrolls the Scrolls: the two-string doctrine of <code class=\"kw\">String</code> and <code class=\"tp\">&amp;str</code>, growing text with <code class=\"kw\">push_str</code>, composing with <code class=\"kw\">format!</code>, and the cleaning kit — <code class=\"kw\">trim</code>, <code class=\"kw\">to_uppercase</code>, <code class=\"kw\">chars</code>, <code class=\"kw\">split</code>, <code class=\"kw\">replace</code>. Master these and text stops being a mystery.</p>",
            "<h3>Two strings: the owned scroll and the borrowed view</h3>",
            "<p><code class=\"kw\">String</code> is an <b>owned</b>, growable, heap-allocated text — a scroll you keep in your vault. <code class=\"tp\">&amp;str</code> («string slice») is a <b>borrowed view</b> onto some text that lives elsewhere — a bookmark pointing at a scroll. A literal like <code class=\"tp\">\"garden\"</code> is a <code class=\"tp\">&amp;str</code>: the characters are baked into the program's memory, and the literal is just a window onto them. This is why <code class=\"tp\">&amp;str</code> cannot grow: a view has nowhere of its own to grow into.</p>",
            "<pre><code>let owned: String = String::from(\"fairy dust\");  // heap scroll
let view: &amp;str = \"fairy dust\";                   // literal = &amp;str
let borrowed: &amp;str = &amp;owned;                     // view onto the scroll
</code></pre>",
            "<p>The bridge between the worlds: <code class=\"kw\">to_string()</code> mints a fresh owned <code class=\"kw\">String</code> from any borrowed view, and <code class=\"kw\">String::from(\"…\")</code> is the elder sibling doing the same. Both <b>copy the characters</b> onto a new heap buffer — the view keeps pointing at the old place, the String owns the new one. Converting back is free: <code class=\"tp\">&amp;some_string</code> derefs to <code class=\"tp\">&amp;str</code> (deref coercion), which is why functions that take <code class=\"tp\">&amp;str</code> happily accept <code class=\"tp\">&amp;String</code> too.</p>",
            "<pre><code>let view = \"garden\";
let word = view.to_string();          // &amp;str -&gt; String (copies)
let magic = String::from(\"sparkle\");  // same rite, older incantation
</code></pre>",
            "<h3>Growing the scroll: push_str &amp; push</h3>",
            "<p>A <code class=\"kw\">String</code> grows like a Vec (it <i>is</i> a Vec of bytes wearing a costume): <code class=\"kw\">push_str(text)</code> appends a whole slice, <code class=\"kw\">push('c')</code> appends a single character. Both need <code class=\"kw\">mut</code> — growing is mutation. The buffer may reallocate behind the scenes; the scroll simply gets longer, and any borrowed view taken <b>before</b> the push is invalidated (the shelf-trap from the Vault, again).</p>",
            "<pre><code>let mut spell = String::from(\"abra\");
spell.push_str(\"cadabra\");   // append a slice
spell.push('!');             // append one char
// spell == \"abracadabra!\"
</code></pre>",
            "<h3>format!: the composer</h3>",
            "<p><code class=\"kw\">format!</code> is the polite way to build text: it takes a template and values, and <b>mints a brand-new String</b> — no borrowing fights, no mutation of the inputs. Positional <code class=\"tp\">{}</code> slots consume arguments in order; inline captures like <code class=\"tp\">{name}</code> grab variables by name straight from the scope. The same mini-language powers <code class=\"kw\">println!</code> — learn it once, use it everywhere.</p>",
            "<pre><code>let name = \"Ferris\";
let level = 7;
let banner = format!(\"{} reached level {}!\", name, level);
let banner2 = format!(\"{name} reached {level}!\");  // inline captures
</code></pre>",
            "<h3>Cleaning kit: trim &amp; case</h3>",
            "<p><code class=\"kw\">trim()</code> shaves whitespace (spaces, tabs, newlines) off <b>both ends</b> and returns a <code class=\"tp\">&amp;str</code> — a narrower view into the same scroll, so it costs nothing. It does not touch the middle: <code class=\"tp\">\"a  b\".trim()</code> keeps its double space. Perfect for sanitizing user input before judging it. Case moves the other way: <code class=\"kw\">to_uppercase()</code> / <code class=\"kw\">to_lowercase()</code> are not views — they may change byte lengths (İ is two bytes!), so they mint a <b>new String</b>.</p>",
            "<pre><code>let raw = \"  RustGarden  \";
let clean = raw.trim();           // \"RustGarden\" (&amp;str view)
let loud = clean.to_uppercase();  // \"RUSTGARDEN\" (new String)
</code></pre>",
            "<h3>chars: the glyph conveyor</h3>",
            "<p>A String is bytes (UTF-8), and a glyph may be 1–4 bytes — so indexing <code class=\"tp\">s[0]</code> is forbidden: byte 0 is not necessarily a letter. To walk text <i>logically</i>, use <code class=\"kw\">chars()</code>: it yields <code class=\"kw\">char</code>s one by one. <code class=\"kw\">nth(n)</code> peeks at the n-th glyph (an <code class=\"kw\">Option</code> — it may run out), <code class=\"kw\">count()</code> counts glyphs — <i>not</i> bytes, and <code class=\"kw\">len()</code> on a String counts <b>bytes</b>. Cyrillic and Hangul make the difference vivid: «краб» is 4 glyphs but 8 bytes.</p>",
            "<pre><code>let runes = \"crab\";
for c in runes.chars() { /* 'c', 'r', 'a', 'b' */ }
let third = runes.chars().nth(2);   // Some('a')
let glyph_count = runes.chars().count();  // 4, not bytes
</code></pre>",
            "<h3>split: the seam ripper</h3>",
            "<p><code class=\"kw\">split(sep)</code> cuts a view into pieces along a separator and hands you a lazy iterator of <code class=\"tp\">&amp;str</code> views — collect them into a <code class=\"kw\">Vec&lt;&amp;str&gt;</code> when you want a shelf. <code class=\"kw\">split_whitespace()</code> is the CSV-free cousin: it splits on any run of whitespace and never yields empty pieces. The pieces are <b>views</b>, not copies — collect to <code class=\"kw\">String</code>s only if you need owned text.</p>",
            "<pre><code>let scroll = \"mana,spell,garden\";
let parts: Vec&lt;&amp;str&gt; = scroll.split(',').collect();
// [\"mana\", \"spell\", \"garden\"]

let words: Vec&lt;&amp;str&gt; = \"a b  c\".split_whitespace().collect();
// [\"a\", \"b\", \"c\"]
</code></pre>",
            "<p><code class=\"kw\">replace(from, to)</code> is the ink rewriter: every occurrence of <code class=\"tp\">from</code> becomes <code class=\"tp\">to</code>, and the result is a <b>fresh String</b> — the original scroll is never touched. Chain it with trim or lowercase freely; each step in the chain that produces a String hands its ownerhip to the next.</p>",
            "<pre><code>let toast = \"old spell\";
let fresh = toast.replace(\"old\", \"new\");  // \"new spell\" (new String)
// toast still reads \"old spell\": replace never edits in place
</code></pre>",
            "<h3>Which string type when?</h3>",
            "<ul><li>Storing, growing, owning text — <code class=\"kw\">String</code>.</li><li>Function parameters that only read — accept <code class=\"tp\">&amp;str</code> (callers may pass literals or <code class=\"tp\">&amp;String</code>).</li><li>Composing from parts — <code class=\"kw\">format!</code> (mints a new String).</li><li>Counting user-visible glyphs — <code class=\"kw\">chars().count()</code>; counting storage — <code class=\"kw\">len()</code>.</li><li>Cutting text apart — <code class=\"kw\">split</code>/<code class=\"kw\">split_whitespace</code> + <code class=\"kw\">collect</code>.</li></ul>",
            "<h3>The ownership ledger of text</h3>",
            "<p>The two-string doctrine is just the ownership chapter wearing a text costume. <code class=\"kw\">String::from</code> and <code class=\"kw\">to_string</code> <b>copy into new ownership</b>; <code class=\"tp\">&amp;str</code> <b>borrow</b>s; passing a String by value <b>moves</b> it; taking <code class=\"tp\">&amp;String</code> by value leaves the caller empty-handed. When the compiler complains about a text puzzle, ask the ledger: who owns these characters, and who merely looks at them?</p>",
            "<p>Training ritual: take a sentence, trim it, lowercase it, split it into words, join them with <code class=\"kw\">format!</code> back into one line with numbers — and count the glyphs at each stage. Every method you use is one of this chapter's runes; after one pass they stop feeling like API trivia and start feeling like a workshop.</p>",
            "<div class=\"article-banner\">The Scroll rule: &amp;str is a borrowed view, String is the owned scroll; trim and split lend views, to_uppercase, format! and replace mint fresh scrolls.</div>",
        ],
    },
