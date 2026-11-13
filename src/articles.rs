//! Codex deep-dive articles — one long-form self-study essay per chapter,
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

pub static ARTICLES: &[Article] = &[

    // =====================================================================
    // EN — English editions
    // =====================================================================
    // C1 basics
    Article {
        chapter_id: "basics",
        lang: "en",
        title: "Foundations of the Memory Garden",
        paragraphs: &[
            "<p>Welcome, traveler. This chapter is the soil from which everything else in <b>Ferris Garden</b> grows. Rust made its name by promising <i>memory safety without a garbage collector</i>, and that promise is kept not at runtime, but at <b>compile time</b>: the compiler simply refuses to build a program that could leak, use-after-free or double-free. Before you can fight dragons in threads or rear castles of structs, your hands must learn three humble tools: keywords, expression types and the immutable-first mindset.</p>",
            "<h3>Immutable by default</h3>",
            "<p>In most languages a variable is a bowl you may refill freely. Rust flips the default: <code class=\"kw\">let</code> binds a name to a value <b>immutably</b>. Reassigning is a compile error — <code class=\"kw\">cannot assign twice to immutable variable</code>. Why? Because immutability kills entire bug families before they are born: no accidental overwrites, no surprise mutation through shared references, and the compiler may optimize harder when it knows a binding never changes.</p>",
            "<p>Mutation is not forbidden, it is <b>explicit</b>. The keyword <code class=\"kw\">mut</code> shouts your intent: «this place WILL change». The discipline is strict: every mutable binding is a deliberate choice, and it deserves a visible keyword.</p>",
            "<h3>Expressions all the way down</h3>",
            "<p>Rust is an <i>expression-oriented</i> language: almost everything computes a value. A <code class=\"kw\">let</code> statement binds a value to a name; a block <code class=\"tp\">{ … }</code> evaluates to its last expression; even <code class=\"kw\">if</code> and <code class=\"kw\">match</code> return values. A statement followed by a semicolon <code class=\"tp\">;</code> throws its value away and evaluates to the unit type <code class=\"kw\">()</code> — Rust's polite word for «nothing».</p>",
            "<pre><code>let harvest = {
    let seeds = 3;
    seeds * 4          // no semicolon: this is the block's value
};                     // harvest == 12
</code></pre>",
            "<p>The classic beginner trap hides in that semicolon: adding <code class=\"tp\">;</code> after the last line of a function body silently changes the return type to <code class=\"kw\">()</code>, and the compiler complains about a «mismatched type». When an error makes no sense at first glance, hunt for a stray semicolon first — it is the most common ghost in the garden.</p>",
            "<h3>A first tour of the primitive types</h3>",
            "<p>Integers come in signed and unsigned families, eight widths each: <code class=\"kw\">i8</code>/<code class=\"kw\">u8</code> up to <code class=\"kw\">i128</code>/<code class=\"kw\">u128</code>, plus the machine-sized <code class=\"kw\">isize</code>/<code class=\"kw\">usize</code> used for indexing. The default is <code class=\"kw\">i32</code> — the general-purpose choice the compiler falls back to whenever context does not suggest something better.</p>",
            "<p>Floating point has two widths: <code class=\"kw\">f32</code> and <code class=\"kw\">f64</code> (the default, and the right choice 95% of the time). <code class=\"kw\">bool</code> is strictly <code class=\"kw\">true</code> or <code class=\"kw\">false</code> — no truthy numbers sneak in. <code class=\"kw\">char</code> is not a byte: it is a full <b>Unicode scalar</b>, so <code class=\"tp\">'Ф'</code>, <code class=\"tp\">'한'</code> and <code class=\"tp\">'🦀'</code> are all legal single characters. And the empty tuple <code class=\"kw\">()</code> («unit») is the type that means «no meaningful value».</p>",
            "<pre><code>let level: u8 = 7;          // 0..=255
let price: f64 = 12.5;      // default float
let awake: bool = true;     // strictly true | false
let crab: char = '🦀';      // full Unicode scalar, 4 bytes
</code></pre>",
            "<h3>Shadowing vs mutation</h3>",
            "<p>Beginners often conflate <i>shadowing</i> and <i>mutation</i>. Shadowing mutates nothing: <code class=\"kw\">let y = y + 1;</code> creates a brand-new binding, leaving the old one intact but hidden. It may even change the type: text becomes a number, a number becomes a boolean. The data flow stays linear and auditable: each transformation is its own declaration. Mutation, on the other hand, rewrites the exact same cell in place.</p>",
            "<p>Shadowing shines when a value passes through stages: parse a string into a number and keep the same name, convert units in place, or reuse a loop variable with a friendlier type. Because every stage is a fresh binding, the compiler remembers each stage's exact type, and the code reads like a diary — one line per transformation.</p>",
            "<h3>The Three Golden Rules of Ownership</h3>",
            "<ul><li>1. Each value has exactly one <b>owner</b> at any moment.</li><li>2. When the owner leaves its scope, the value is <b>dropped</b> (memory freed automatically, no GC).</li><li>3. Assignment or passing <b>moves</b> the value; the old name becomes unusable.</li></ul>",
            "<p>Move semantics frighten newcomers and delight veterans. A <code class=\"kw\">String</code> holds its bytes on the heap; silently copying it would breed use-after-free bugs, so Rust refuses to copy unless you ask. When you truly need a parallel copy, you call <code class=\"kw\">.clone()</code> and pay for every duplicated byte. Want several readers of the same data? You shall meet <code class=\"kw\">&amp;</code> references soon.</p>",
            "<p>Moves happen on every assignment, function argument and return — not only for <code class=\"kw\">String</code>. A function that receives a <code class=\"kw\">String</code> by value becomes its new owner, and the caller's name dies at the call site. Returning the value hands ownership back. That is the whole «layered contract» of Rust's data flow, and the compiler enforces it mechanically — no runtime guard dogs required.</p>",
            "<h3>Copy vs Clone</h3>",
            "<p>Small, plain values (<code class=\"kw\">i32</code>, <code class=\"kw\">f64</code>, <code class=\"kw\">bool</code>, <code class=\"kw\">char</code>, tuples of such types) carry the special <code class=\"kw\">Copy</code> marker: assignments duplicate them silently, because a bitwise copy cannot create memory bugs. Everything heap-based needs an explicit <code class=\"kw\">.clone()</code>: the pause is the price of honesty — you see exactly where the expensive duplication happens.</p>",
            "<pre><code>let n = 42;
let m = n;          // Copy: n is still alive
let s = String::from(\"Ferris\");
let t = s.clone();  // explicit: s is still alive
// let u = s;       // would MOVE s — s dies here
</code></pre>",
            "<h3>Borrowing &amp; the Borrow Checker</h3>",
            "<p>A reference is a temporary key to borrowed territory. The doctrine is <b>Aliasing XOR Mutation</b>: any number of readers (<code class=\"kw\">&amp;T</code>) OR exactly one writer (<code class=\"kw\">&amp;mut T</code>), never both. That single rule dissolves data races at the root — thread chapters will lean on it hard.</p>",
            "<pre><code>let mut mana = String::from(\"full\");
let r1 = &amp;mana;
mana.push('?');      // ✗ E0502: r1 is still read below
println!(\"{r1}\");    // last use of r1
// swap the two lines above — and the code compiles!
</code></pre>",
            "<p>Modern Rust softens the timeline: with <b>Non-Lexical Lifetimes</b> a borrow ends at its <i>last use</i>, not at the closing brace. Move the <code class=\"kw\">println!</code> above the <code class=\"kw\">push</code> and the code compiles — the compiler tracks actual usage, not brace-shaped ranges. Argue with the checker for a day and the discipline starts to feel like a guardrail rather than a cage.</p>",
            "<h3>Slices: views instead of copies</h3>",
            "<p>The last piece is <code class=\"kw\">&amp;str</code> — a <i>fat pointer</i> to a byte window <code class=\"tp\">(ptr, len)</code>. Slices read part of the data without touching the heap: zero allocation, full lifetime checks. That is the entrance ticket to iterator chapters: view first, copy only at need.</p>",
            "<p>Slices also teach the safety habit of bounds checking: indexing past the end panics with <code class=\"kw\">index out of bounds</code> instead of reading garbage. For access that may miss, prefer <code class=\"kw\">.get(i)</code>, which returns an <code class=\"kw\">Option&lt;T&gt;</code> — the forms chapter will turn that pattern into a daily reflex.</p>",
            "<h3>Strings: &amp;str vs String</h3>",
            "<p>Two string types confuse everyone exactly once. <code class=\"kw\">String</code> is an <b>owned</b> heap buffer you may grow, shrink and rewrite. <code class=\"kw\">&amp;str</code> is a <b>borrowed</b> view onto bytes somebody else owns — string literals are the classic case: <code class=\"tp\">\"Ferris\"</code> in your code is a <code class=\"kw\">&amp;'static str</code> baked into the binary. Pass <code class=\"kw\">&amp;str</code> to functions that only read; keep <code class=\"kw\">String</code> where you own and mutate.</p>",
            "<h3>Tuples, arrays, destructuring</h3>",
            "<p>Tuples <code class=\"kw\">(T, U)</code> bundle fixed sets of mixed types; arrays <code class=\"tp\">[T; N]</code> hold exactly <code class=\"kw\">N</code> same-type values right on the stack. Both destructure beautifully: <code class=\"kw\">let (a, b) = pair;</code> or <code class=\"kw\">let [first, .., last] = arr;</code> name the pieces without method noise. An array knows its length at compile time — a different length is a different type.</p>",
            "<pre><code>let potion = (\"heal\", 40);          // (&amp;str, i32)
let (name, power) = potion;         // destructure
let cells = [1, 2, 3];              // [i32; 3]
let mid = cells[1];                 // bounds-checked
</code></pre>",
            "<h3>const, static and casts</h3>",
            "<p>Compile-time constants live under <code class=\"kw\">const</code>: they are inlined wherever used and MUST carry a type annotation. Naming is SCREAMING_SNAKE_CASE (<code class=\"tp\">MAX_CHARGE</code>). <code class=\"kw\">static</code> is the long-lived sibling: one value for the whole program run, handy for global configuration — same naming convention.</p>",
            "<p>Casting between primitives uses <code class=\"kw\">as</code> and it is <b>lossy by design</b>: <code class=\"kw\">f64</code> → <code class=\"kw\">i64</code> truncates toward zero, and a large number squeezed into a small type wraps around. Overflow behavior differs by build: debug panics, release wraps silently — treat <code class=\"kw\">as</code> as a deliberate, commented decision, never a habit.</p>",
            "<h3>Reading the compiler's letters</h3>",
            "<p>Rust error messages are a curriculum. Each one carries an error code (like <code class=\"tp\">E0384</code>), the offending span, and often a <i>suggestion line</i> with the exact fix. Read them as letters from a mentor — deliberately breaking code to study the message is a legitimate (and fun) training technique, and every quest of this garden encourages it.</p>",
            "<h3>Everyday toolbox</h3>",
            "<p>The chapter also packs the daily instrument set: <code class=\"kw\">i32</code>/<code class=\"kw\">f64</code>/<code class=\"kw\">bool</code> types, tuples <code class=\"kw\">(T, U)</code>, fixed arrays <code class=\"tp\">[T; N]</code>, compile-time constants <code class=\"kw\">const</code>, and explicit primitives casts through <code class=\"kw\">as</code>. None of these is rocket science, yet all of them appear daily — walk the quests at your pace and let the fingers remember.</p>",
            "<p>A practical ritual for this chapter: after each quest, try to <i>break</i> your own solution — reassign an immutable, move a string twice, index past the end. Watching the compiler block each crime teaches more than a hundred paragraphs of theory.</p>",
            "<p>Walk each quest slowly. Type the answer before reading the hint, turn the feedback box green and feel the garden breathe. Foundations + quests = stage one complete; the next chapter argues with <code class=\"kw\">match</code> and reaches for the loops.</p>",
            "<div class=\"article-banner\">The soil is ready: twelve quests turn these foundations into reflexes. Walk them slowly — the garden rewards patience more than speed.</div>",
        ],
    },

    // C2 flow
    Article {
        chapter_id: "flow",
        lang: "en",
        title: "Runes of Flow & Functions",
        paragraphs: &[
            "<p>A program that never argues is a stone. This chapter teaches your program to <b>make decisions</b>: branch, dispatch and repeat. The star of the show is <code class=\"kw\">match</code> — Rust's pattern-matching engine, far stronger than the if/else its competition uses.</p>",
            "<h3>if, and it shall return</h3>",
            "<p>Branching looks familiar, but Rust adds a twist: <code class=\"kw\">if</code> is an <b>expression</b>, not a statement. It evaluates to a value, so you can write <code class=\"kw\">let status = if awake { \"up\" } else { \"asleep\" };</code> — no ternary gymnastics, branches keep their types equal, and the code reads like a sentence.</p>",
            "<pre><code>let mood = if hunger &gt; 8 { \"grumpy\" } else { \"sunny\" };
</code></pre>",
            "<p>One hard rule: a condition must be a genuine <code class=\"kw\">bool</code>. There is no truthiness in Rust — <code class=\"tp\">if 1 { … }</code> refuses to compile, because <code class=\"kw\">i32</code> and <code class=\"kw\">bool</code> are different species. Strict? Yes. But it also means no «accidentally-true» values: every condition says exactly what it means.</p>",
            "<h3>match: the altar of patterns</h3>",
            "<p><code class=\"kw\">match</code> compares a value against a list of <b>patterns</b> and executes the first arm that fits. Three rules to love it:</p>",
            "<ul><li><b>Exhaustiveness</b>: the compiler won't let you forget an arm: «non-exhaustive patterns» stops the build.</li><li><b>Payload destructuring</b>: arms may bind inner values straight away — <code class=\"kw\">Ok(n) =&gt; println!(\"{}\", n)</code>.</li><li><b>Range arms</b>: <code class=\"kw\">1..=5</code> matches the whole corridor of numbers, and <code class=\"kw\">_</code> is the net catching everything else.</li></ul>",
            "<p>The wildcard <code class=\"kw\">_</code> and exhaustive checking together make the compiler your personal auditor: add a new enum variant later and every unguarded <code class=\"kw\">match</code> site will refuse to compile until its arm has been written. That is why refactoring giants are less scary in Rust.</p>",
            "<pre><code>match scroll {
    0              =&gt; println!(\"empty\"),
    1..=9          =&gt; println!(\"a few pages\"),
    n @ 10..=99    =&gt; println!(\"{n} pages\"),
    _              =&gt; println!(\"a tome!\"),
}
</code></pre>",
            "<h3>The pattern zoo</h3>",
            "<p>Patterns are a language of their own. Literals match themselves (<code class=\"kw\">0</code>), ranges match corridors (<code class=\"kw\">1..=9</code>), tuples and structs destructure in place (<code class=\"kw\">(a, b)</code>, <code class=\"tp\">Guardian { spear, .. }</code>), enum variants match their payloads (<code class=\"kw\">Some(x)</code>), and <code class=\"kw\">_</code> politely ignores what it does not care to name. One <code class=\"kw\">match</code> can therefore replace a whole ladder of if-statements.</p>",
            "<p>Two extensions make patterns surgical. A <b>guard</b> adds a condition to an arm: <code class=\"kw\">n if n &gt; 10 =&gt; …</code> fires only when the extra test passes. And <code class=\"kw\">n @ 10..=99</code> <b>binds</b> the matched value to a name while still matching the range — extraction and naming in one breath.</p>",
            "<pre><code>match point {
    (0, y)                  =&gt; println!(\"on the Y axis: {y}\"),
    (x, y) if x == y        =&gt; println!(\"diagonal\"),
    (x, y)                  =&gt; println!(\"somewhere: {x}, {y}\"),
}
</code></pre>",
            "<h3>if let: the light ceremony</h3>",
            "<p>When only <i>one</i> pattern interests you, <code class=\"kw\">match</code> is a full orchestra for a single note. <code class=\"kw\">if let Some(key) = relic { … }</code> unpacks just the case you care about and leaves the rest in silence. Quest 31 drills exactly this idiom — the everyday tool for <code class=\"kw\">Option</code> and <code class=\"kw\">Result</code>.</p>",
            "<h3>while let: draining the queue</h3>",
            "<p>Its looping sibling repeats while the pattern keeps matching: <code class=\"kw\">while let Some(msg) = rx.recv() { … }</code> consumes messages until the channel runs dry. It is the idiomatic way to drain stacks, queues and iterators without hand-rolling a counter.</p>",
            "<h3>Loops: loop, while, for</h3>",
            "<p>Rust gives three spins. <code class=\"kw\">loop</code> is <i>maybe infinite</i> — honest about that; you leave it only through <code class=\"kw\">break</code> (which can also <b>return a value</b>: <code class=\"kw\">let fuel = loop { break 42; };</code>). <code class=\"kw\">while</code> re-examines a condition before each round. <code class=\"kw\">for</code> walks over <b>iterators and ranges</b> (<code class=\"kw\">1..=5</code>, <code class=\"tp\">vec.iter()</code>) — the idiomatic default: no off-by-one fingers, no manual index checking.</p>",
            "<p>Nested loops raise a practical question: which <code class=\"kw\">break</code> exits which loop? Rust answers with <b>labels</b>: <code class=\"kw\">'outer: loop { … }</code> can be exited from deep inside with <code class=\"kw\">break 'outer;</code>. Labels work for <code class=\"kw\">continue</code> too — no boolean-flag dances required.</p>",
            "<pre><code>'outer: for pot in pots {
    for seed in seeds {
        if seed.spoiled { continue 'outer; }
    }
    println!(\"{} planted\", pot);
}
</code></pre>",
            "<h3>for and the three iterators</h3>",
            "<p><code class=\"kw\">for</code> quietly calls one of three iterator flavors. <code class=\"kw\">for x in vec</code> <b>moves</b> values out (the vector is consumed); <code class=\"kw\">for x in vec.iter()</code> yields <code class=\"kw\">&amp;T</code> — read-only peeks; <code class=\"kw\">for x in vec.iter_mut()</code> yields <code class=\"kw\">&amp;mut T</code> — a license to rewrite. Choosing the flavor consciously is the difference between an accidental move bug and a clean pass.</p>",
            "<h3>fn, parameters, returns</h3>",
            "<p><code class=\"kw\">fn</code> declares a spell: a name, a typed parameter list, and the <code class=\"kw\">-&gt;</code> arrow announcing the returned type. The last expression of a body is implicitly returned — no <code class=\"kw\">return</code> keyword needed in most cases. Rust style prefers <b>whole verbs</b>: <code class=\"kw\">fn charge(price: i32) -&gt; i32</code>.</p>",
            "<p>Why does a Rust function never omit parameter types? Because generic inference belongs to <code class=\"kw\">let</code> only; function signatures are the <b>contract interface</b> between modules — they must be legible without peeking into the body.</p>",
            "<p>A function without <code class=\"kw\">-&gt;</code> returns <code class=\"kw\">()</code> — and that is a fact, not a formality: it lets <code class=\"kw\">println!</code> live freely as a statement, and it explains why a forgotten semicolon sometimes «changes» what a function returns. Types in signatures are loud; the return type is no exception.</p>",
            "<h3>Early returns and the ? relay</h3>",
            "<p>A mid-function <code class=\"kw\">return</code> is perfectly legal — early guards at the top of a function («if invalid, bail out now») keep the main path flat. And the arrowhead <code class=\"kw\">?</code> you will meet in chapter 8 is the same idea compressed: «return the error upward if there is one, otherwise unwrap the success».</p>",
            "<h3>Functions are values too</h3>",
            "<p>A <code class=\"kw\">fn</code> can live in a variable and travel as an argument: <code class=\"kw\">let f: fn(i32) -&gt; i32 = double;</code>. This is a first taste of the <b>closure</b> country that iterator chapters will explore properly — but even now you can pass behavior around like data, which is exactly what callbacks and strategies are.</p>",
            "<pre><code>fn double(n: i32) -&gt; i32 { n * 2 }

fn apply(f: fn(i32) -&gt; i32, v: i32) -&gt; i32 {
    f(v)
}
</code></pre>",
            "<h3>Putting it together</h3>",
            "<p>Master these runes and you may compose <i>anything</i>: <code class=\"kw\">fn</code>-spells exchange messages, <code class=\"kw\">match</code> routes data, <code class=\"kw\">for</code> walks shelves. Every subsequent chapter is literally the same dance with fancier decorations: struct fields flow through match arms, threads receive verbs, files travel over io via (?) relays.</p>",
            "<p>Style notes worth stealing: prefer <b>small verb-named</b> functions that do one thing; let the last expression return implicitly; keep parameter lists short — if a function asks for five arguments, it probably wants to become a struct instead. Future-you, reading this code a year from now, will send thanks.</p>",
            "<p>Start from the least-familiar rune: for many this is <code class=\"kw\">match</code> with its destructuring. Type the quest answers by hand, break them once to read the compiler's words, then let the next quest build on it. See you in the Halls of Structures.</p>",
            "<div class=\"article-banner\">Branches, patterns, loops, spells — the four gears of every Rust program. Twelve quests wait to grind them into muscle memory.</div>",
        ],
    },

    // C3 forms
    Article {
        chapter_id: "forms",
        lang: "en",
        title: "Halls of Structures & Enums",
        paragraphs: &[
            "<p>Scalars only get you so far. Real domains are shaped: guardians with spears and shields, gates with lawful states, relics that may or may not exist. Rust gives you <b>product types</b> (structs), <b>sum types</b> (enums), and a special ambassador for absence — <code class=\"kw\">Option&lt;T&gt;</code>. This chapter is the forge where raw values become <i>types of your own</i>.</p>",
            "<h3>struct: the data mould</h3>",
            "<p><code class=\"kw\">struct Guardian { spear: u16, shield: u16 }</code> casts a named group of fields. You read them with the dot, update fragment-wise with the <code class=\"kw\">..base</code> syntax and — most importantly — attach behavior through <code class=\"kw\">impl</code> blocks. The receiver <code class=\"kw\">&amp;self</code> keeps the method read-only, <code class=\"kw\">&amp;mut self</code> grants mutation, <code class=\"kw\">self</code> consumes (rare, but honest).</p>",
            "<pre><code>struct Guardian {\n    spear: u16,\n    shield: u16,\n}\n\nlet mut hero = Guardian { spear: 12, shield: 30 };\nhero.shield -= 5;      // field access through the dot\n</code></pre>",
            "<h3>Tuple structs and the newtype pattern</h3>",
            "<p>When a struct has no named fields you get a <b>tuple struct</b>: <code class=\"kw\">struct Meters(f64);</code>. It looks minimal, yet it is one of Rust's mightiest habits — the <i>newtype pattern</i>. <code class=\"kw\">struct Meters(f64)</code> and <code class=\"kw\">struct Seconds(f64)</code> are different types, so the compiler will never let you add a distance to a duration. Zero runtime cost, full compile-time safety.</p>",
            "<h3>derive: the free toolkit</h3>",
            "<p>Behavior every type deserves — printing, comparing, copying — should not be handwritten. Above a struct or enum you may place <code class=\"kw\">#[derive(Debug, Clone, PartialEq, Default)]</code> and the compiler writes the boilerplate for you: <code class=\"tp\">Debug</code> enables <code class=\"tp\">{:?}</code> printing, <code class=\"kw\">Clone</code> unlocks <code class=\"kw\">.clone()</code>, <code class=\"kw\">PartialEq</code> allows <code class=\"kw\">==</code> between values, <code class=\"kw\">Default</code> provides <code class=\"tp\">Guardian::default()</code>. The quests of this chapter lean on this one line heavily.</p>",
            "<pre><code>#[derive(Debug, Clone, PartialEq, Default)]\nstruct Relic {\n    name: String,\n    power: u8,\n}\n\nlet r1 = Relic::default();\nlet r2 = r1.clone();\nprintln!(\"{:?}\", r1 == r2);   // true\n</code></pre>",
            "<h3>impl: methods and receivers</h3>",
            "<p><code class=\"kw\">impl Guardian { … }</code> attaches behavior to the type. The first parameter — the <b>receiver</b> — decides how the method may touch its data: <code class=\"kw\">&amp;self</code> reads, <code class=\"kw\">&amp;mut self</code> mutates, <code class=\"kw\">self</code> consumes the value entirely (rare, but honest — think of <code class=\"tp\">into_*</code> conversions). Methods are called through the dot: <code class=\"kw\">hero.sharpen()</code>.</p>",
            "<pre><code>struct Guardian { spear: u16, shield: u16 }\n\nimpl Guardian {\n    fn new(spear: u16, shield: u16) -&gt; Self {   // associated fn\n        Self { spear, shield }\n    }\n    fn total(&amp;self) -&gt; u16 { self.spear + self.shield }  // borrows\n    fn buff(&amp;mut self) { self.spear += 1; }              // mutates\n}\n\nlet mut hero = Guardian::new(12, 30);\nhero.buff();\nprintln!(\"{}\", hero.total());    // 43\n</code></pre>",
            "<p>That <code class=\"kw\">impl</code> separation is why Rust has no constructors hidden behind class names: construction is an ordinary associated function <code class=\"kw\">Guardian::new()</code> — note the absence of a receiver — and behavior grows in its own room, next to — not tangled inside — the data.</p>",
            "<h3>Struct update syntax &amp; partial moves</h3>",
            "<p><code class=\"kw\">let hero2 = Guardian { spear: 40, ..hero };</code> copies all remaining fields from <code class=\"kw\">hero</code>. Careful: <code class=\"kw\">..hero</code> <b>moves</b> the fields it takes — if a struct holds a heap type like <code class=\"kw\">String</code>, the original can be left partially moved and unusable. With plain numbers the same line is a full-copy shortcut, because every field is <code class=\"kw\">Copy</code>.</p>",
            "<h3>enum: the lawful sum type</h3>",
            "<p><code class=\"kw\">enum GateStatus { Locked, Guarded, Open }</code> declares a finite set of allowed states, and each variant may carry its own <b>payload</b>: <code class=\"kw\">Ready(i32)</code>, <code class=\"kw\">Failed(String)</code>, <code class=\"kw\">Open { key: u8 }</code>. One type, several shapes. This is the heart of Rust modeling: express «one of these possibilities» in the type system itself, so illegal states simply cannot be constructed.</p>",
            "<pre><code>enum Door {\n    Locked,\n    Guarded(Guardian),\n    Open { visits: u32 },\n}\n\nlet gate = Door::Guarded(hero);\n</code></pre>",
            "<p>An enum is a <b>state machine</b> you can read: only the states you declared may exist, transitions happen through <code class=\"kw\">match</code>, and each state carries exactly the data it needs. Model a document as <code class=\"kw\">Draft | Ready(String) | Published { date: String }</code> and «a published draft with no date» instantly becomes impossible — that value simply cannot be constructed.</p>",
            "<h3>Option: absence made explicit</h3>",
            "<p>Most languages denote absence with a ghost that silently sneaks into your arithmetic (null). Rust encodes it: <code class=\"kw\">Option&lt;T&gt;</code> is either <code class=\"kw\">Some(value)</code> or <code class=\"kw\">None</code>, and the compiler <b>forces</b> you to acknowledge both before using the value. Every Option quest here drills three idioms:</p>",
            "<ul><li><code class=\"kw\">if let Some(x) = relic { … }</code> — light conditional unwrap;</li><li><code class=\"kw\">match relic { Some(n) =&gt; …, None =&gt; … }</code> — full ceremony;</li><li><code class=\"kw\">relic.unwrap_or(0)</code> — quick fallback coin.</li></ul>",
            "<p>Once absence is a value, it can be transformed like any value. The <b>combinator</b> family keeps the ceremony short: <code class=\"kw\">relic.map(|r| r.power)</code> turns an <code class=\"kw\">Option&lt;Relic&gt;</code> into <code class=\"kw\">Option&lt;u8&gt;</code>; <code class=\"kw\">and_then</code> chains the next fallible step; <code class=\"kw\">unwrap_or_else(|| fallback)</code> computes a default only when needed. Underneath they are all <code class=\"kw\">match</code> in disguise.</p>",
            "<pre><code>let relic: Option&lt;Relic&gt; = Some(Relic { name: \"🦀\".into(), power: 9 });\n\nlet power = relic\n    .map(|r| r.power)          // Option&lt;u8&gt;\n    .filter(|p| *p &gt; 5)        // keep the strong ones\n    .unwrap_or(0);             // 9\n</code></pre>",
            "<p><code class=\"kw\">unwrap()</code> exists but is rude: it panics on None. In the Garden we treat unwrap like a <i>first draft</i>: fine inside examples and quick experiments, inside real code prefer <code class=\"kw\">unwrap_or</code>, <code class=\"kw\">?</code> or a match. Chapter 8 makes this doctrine official.</p>",
            "<h3>Result: the sibling for failures</h3>",
            "<p><code class=\"kw\">Option</code> answers «is it there?», but says nothing about <i>why</i> it is missing. Its sibling <code class=\"kw\">Result&lt;T, E&gt;</code> carries an error payload in the <code class=\"kw\">Err</code> arm: <code class=\"kw\">Ok(value)</code> or <code class=\"kw\">Err(reason)</code>. The official meeting is chapter 8 — yet even here the shape should feel familiar, because it is the same enum thinking applied to failures.</p>",
            "<h3>Make illegal states unrepresentable</h3>",
            "<p>Both halves of this chapter serve one design commandment: <b>make illegal states unrepresentable</b>. A value must be positive — use <code class=\"kw\">u16</code> instead of checking <code class=\"kw\">i32</code> at every step. A field may be absent — use <code class=\"kw\">Option</code> instead of an empty-string convention. Exactly three states exist — declare an enum instead of three booleans. The compiler then becomes your domain guard, working nights and weekends for free.</p>",
            "<h3>Why does this matter?</h3>",
            "<p>Look at any real Rust code: <code class=\"tp\">Vec&lt;Item&gt;</code>, <code class=\"tp\">Option&lt;Config&gt;</code>, <code class=\"tp\">Result&lt;Reply, Error&gt;</code> — generic shells over structs and enums. Fail to master this chapter and iterators/channels/files will read as noise; master it and they become transparent glass.</p>",
            "<p>Training ritual for this hall: model something from your own day as an enum — a coffee order, a train ticket, a game state. Ask the compiler to reject every impossible variant, then turn three of your structs into newtypes. The quests below will walk you through struct, impl, Option and default one stone at a time.</p>",
            "<div class=\"article-banner\">The hall waits: struct one guardian, seal three gate variants, unwrap one relic. Doors of the next chapters will open easier.</div>",
        ],
    },

    // C7 formatting
    Article {
        chapter_id: "formatting",
        lang: "en",
        title: "The Mirror of Display & Debug",
        paragraphs: &[
            "<p>Output is a court: the value appears and speaks. Rust distinguishes two moods of self-presentation. <code class=\"kw\">Display</code> is the <b>polite face</b> for end users: <code class=\"kw\">println!(\"{}\", x)</code>. <code class=\"kw\">Debug</code> is the <b>honest confessor</b> for developers: <code class=\"kw\">println!(\"{:?}\", x)</code>.</p>",
            "<h3>Why two traits?</h3>",
            "<p>Because the audiences differ. A user wants <code class=\"tp\">4°C</code>, a developer wants <code class=\"tp\">Temperature { celsius: Some(4), scale: C }</code>. Displaying the internals to users is a leak; hiding them from developers is a torment. Two traits — no fight.</p>",
            "<h3>The macro family: println!, eprintln!, format!, write!</h3>",
            "<p>All formatting flows through the same engine — only the destination changes. <code class=\"kw\">println!</code> speaks to the terminal with a newline, <code class=\"kw\">eprintln!</code> to the error stream, <code class=\"kw\">format!</code> silently hands you a <code class=\"kw\">String</code>, and <code class=\"kw\">write!</code> pours into anything that accepts a formatter. Learn the syntax once — the whole family obeys.</p>",
            "<pre><code>let coin = 42;\n\nlet label = format!(\"coin = {}\", coin);   // string built in memory\nprintln!(\"{label}\");                     // to stdout\neprintln!(\"warning: {label}\");           // to stderr\n</code></pre>",
            "<h3>Positional and named arguments</h3>",
            "<p>Inside the braces lives its own grammar: <code class=\"tp\">{}</code> takes the next argument, <code class=\"tp\">{0}</code>/<code class=\"tp\">{1}</code> address arguments by position — the same value may appear twice — and <code class=\"tp\">{name}</code> binds to a variable captured straight from the environment. Inline capture (<code class=\"kw\">println!(\"{x}\")</code>) is the modern Rust style: less noise, fewer arguments.</p>",
            "<pre><code>let (x, y) = (3, 7);\n\nprintln!(\"{0} + {1} = {0}\", x, y);  // 3 + 7 = 3\nprintln!(\"{x} sits at {y}\");       // inline capture\nprintln!(\"{:&gt;5}|\", x);             // field width 5, right-aligned\n</code></pre>",
            "<p>Braces themselves are printable: escape them by doubling — <code class=\"tp\">{{</code> prints <code class=\"tp\">{</code> and <code class=\"tp\">}}</code> prints <code class=\"tp\">}</code>. So <code class=\"kw\">println!(\"{{}} = {{code}}\")</code> renders <code class=\"tp\">{} = {code}</code>. Templates that show literal JSON or braces lean on this trick daily.</p>",
            "<h3>Debug and the derive magic</h3>",
            "<p>Debug output is mechanical, so Rust lets you <i>derive</i> it: <code class=\"kw\">#[derive(Debug)]</code> teaches the compiler to print every field automatically. Then <code class=\"kw\">{:?}</code> calls it. Prefer <code class=\"kw\">{:#?}</code> when the anatomy is deep: pretty-printing with indentation turns nested rubble into readable shelves.</p>",
            "<pre><code>#[derive(Debug)]\nstruct Relic { name: String, power: u8 }\n\nlet vault = vec![\n    Relic { name: \"rune\".into(), power: 3 },\n    Relic { name: \"gem\".into(), power: 9 },\n];\n\nprintln!(\"{:?}\", vault);   // [Relic { name: \"rune\", power: 3 }, …]\nprintln!(\"{:#?}\", vault);  // pretty: one field per line\n</code></pre>",
            "<h3>Display vs Debug: which one when</h3>",
            "<p>A serviceable compass: <b>Debug is for you, Display is for them</b>. Debug for logs, failing tests, state snapshots — anything a human developer consumes; derive it everywhere (it composes through containers for free). Display for user-facing strings — UI lines, report rows, CLI output; write it by hand, once, deliberately. If a type deserves both, give it both: these are different contracts, not rivals.</p>",
            "<h3>Display: your own voice</h3>",
            "<p>Display cannot be derived — nobody can guess how <i>you</i> want the type addressed. You implement <code class=\"kw\">fmt::Display</code> once, and every <code class=\"kw\">{}</code> in any context honors it. Your tool is <code class=\"kw\">write!(f, \"…\", parts)</code>: the macro pours formatted text into the formatter and you return <code class=\"tp\">fmt::Result</code>.</p>",
            "<pre><code>use std::fmt;\n\nstruct Temperature { celsius: f64 }\n\nimpl fmt::Display for Temperature {\n    fn fmt(&amp;self, f: &amp;mut fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {\n        write!(f, \"{}°C\", self.celsius)\n    }\n}\n</code></pre>",
            "<h3>The specification fields: {:.2}, {:&gt;8}, {:08}</h3>",
            "<p>The miniature language inside <code class=\"kw\">{}</code> is a mini-language of specifiers: <code class=\"tp\">{:.2}</code> pins the decimal digits, <code class=\"tp\">{:&gt;8}</code> aligns right inside a field of width 8, <code class=\"tp\">{:&lt;8}</code> aligns left, <code class=\"tp\">{:08}</code> zero-pads numbers to ceremonial width (great for ids). Master these four glyphs and your output looks like a professional ledger.</p>",
            "<p>The field may carry three extra gears: <b>fill &amp; alignment</b> (<code class=\"tp\">{:&gt;8}</code>, <code class=\"tp\">{:&lt;8}</code>, <code class=\"tp\">{:^8}</code> — right, left, center; write the fill character before the arrow: <code class=\"tp\">{:*^8}</code> decorates with stars), a <b>sign</b> (<code class=\"tp\">{:+}</code> forces + onto positive numbers) and <b>zero-padded width</b> (<code class=\"tp\">{:08}</code> — perfect for identifiers like <code class=\"tp\">00042</code>).</p>",
            "<pre><code>println!(\"{:+}\", 5);      // +5\nprintln!(\"{:08}\", 42);    // 00000042\nprintln!(\"{:b}\", 9);      // 1001\nprintln!(\"{:x}\", 255);    // ff\nprintln!(\"{:o}\", 8);      // 10\nprintln!(\"{:e}\", 1234.5); // 1.2345e3\n</code></pre>",
            "<h3>Suggested training loop</h3>",
            "<ul><li>Type the <code class=\"kw\">#[derive(Debug)]</code> quests until derive feels like breathing;</li><li>Then write one Display impl by hand, with <code class=\"kw\">write!</code> — count on it becoming muscle memory;</li><li>Finish with the mini-language quests: {:.2}, {:&gt;8}, {:08}.</li></ul>",
            "<p>Close the chapter with a small habit: whenever you print an intermediate value for debugging, switch it to <code class=\"tp\">{:#?}</code> once and read its anatomy aloud. Then implement Display for one type you own — a temperature, a potion, a receipt — and never again hand-craft concatenated strings with <code class=\"kw\">+</code>. Formatting is a language; speak it on purpose.</p>",
            "<div class=\"article-banner\">After the mirror chapter, your logs stop being «random spam» and start being <i>court records</i>.</div>",
        ],
    },

    // C9 pointers
    Article {
        chapter_id: "pointers",
        lang: "en",
        title: "Mind Palace: Smart Pointers",
        paragraphs: &[
            "<p>Rust value rooms live either on the <b>stack</b> (cheap, fast, scoped) or on the <b>heap</b> (large, shared, long-lived). Owners manage the heap directly — via <b>smart pointers</b>. They are the lifeguards of the Mind Palace: each one counts, shares or guards under its own strict protocol. None of them overthrows ownership — each is a different <i>implementation</i> of it.</p>",
            "<h3>Box: the private heap suite</h3>",
            "<p><code class=\"kw\">Box&lt;T&gt;</code> allocates <code class=\"kw\">T</code> on the heap and wraps one pointer as owner: one value, one owner, deleted at scope's end. Ordinary uses: oversized structs that must not sit on the stack, recursive types where the size must be known at compile-time, and <b>trait objects</b>, <code class=\"kw\">Box&lt;dyn Trait&gt;</code> — any concrete type hiding behind one contract.</p>",
            "<pre><code>enum List {\n    Node(i32, Box&lt;List&gt;),\n    Empty,                 // recursion stops here\n}\n</code></pre>",
            "<p>Why is the Box essential in that example? A recursive type would otherwise need to know its own size, which depends on itself — an infinite regress the compiler refuses to guess. <code class=\"kw\">Box</code> breaks the cycle: a pointer is one machine word, and the size of <code class=\"kw\">List</code> becomes computable again. This is the pattern behind <code class=\"kw\">Box&lt;Node&gt;</code> in linked lists, file trees and every tree-shaped world.</p>",
            "<p>Dereferencing is the star of this palace: <code class=\"kw\">*pointer</code> digs straight to the value. Box implements <code class=\"kw\">Deref</code> — so not <code class=\"kw\">(*ptr).method()</code>, just <code class=\"kw\">ptr.method()</code> — auto-deref works with method calls transparently.</p>",
            "<p>Memory anatomy worth memorizing: the <code class=\"kw\">Box</code> itself sits on the stack, one pointer wide; the <code class=\"kw\">T</code> it guards lives on the heap; when the Box leaves its scope, <code class=\"kw\">Drop</code> frees the heap room automatically. No manual <code class=\"kw\">free()</code>, no leaks — the owner is the destructor.</p>",
            "<h3>Box&lt;dyn Trait&gt;: the contract collector</h3>",
            "<p>Sometimes the collection must hold <i>different</i> types that obey one contract. Concrete types vary in size, so the container needs a uniform shape: a <b>trait object</b>. <code class=\"kw\">Box&lt;dyn Beast&gt;</code> is a fat pointer — data plus vtable — and dynamic dispatch decides at run time which <code class=\"kw\">cry()</code> to call. The cost is one indirection; the gain is pluggable behavior.</p>",
            "<pre><code>trait Beast { fn cry(&amp;self) -&gt; String; }\n\nstruct Crab;\nimpl Beast for Crab { fn cry(&amp;self) -&gt; String { \"🦀\".into() } }\n\nlet zoo: Vec&lt;Box&lt;dyn Beast&gt;&gt; = vec![Box::new(Crab)];\nfor b in &amp;zoo { println!(\"{}\", b.cry()); }\n</code></pre>",
            "<h3>Rc: the Owner Registry</h3>",
            "<p>Sharing readable data between several owners — single-threaded contexts — is a <code class=\"kw\">Rc&lt;T&gt;</code>'s task (reference counted). <code class=\"kw\">Rc::clone(&amp;scroll)</code> does not copy bytes: it merely increments the atomic owner counter; <code class=\"kw\">Rc::strong_count(&amp;scroll)</code> tells you how many registry names currently hold the value. When the last one leaves, the data is dropped.</p>",
            "<h3>Cell &amp; RefCell: interior mutation</h3>",
            "<p>The family of <i>shared cells</i> bends one specific rule: mutation through a shared handle. <code class=\"kw\">Cell&lt;T&gt;</code> serves plain <code class=\"kw\">Copy</code> values — <code class=\"tp\">.set()</code> replaces, <code class=\"tp\">.get()</code> reads, no borrows at all. <code class=\"kw\">RefCell&lt;T&gt;</code> serves everything else: it moves the borrow-checking book from compile time to run time.</p>",
            "<pre><code>let s = Rc::new(RefCell::new(String::from(\"scroll\")));\n\nlet mut w1 = s.borrow_mut();\nw1.push_str(\"!\");\n\nlet r1 = s.borrow();   // ✗ panics: BorrowMutError\n</code></pre>",
            "<p><b>RefCell pairs with Rc nearly always</b>: Rc alone cannot offer interior mutation, because an immutable registry will never hand out a <code class=\"kw\">&amp;mut</code> borrow. The cell provides runtime-checked mutation instead: <code class=\"kw\">borrow()</code> (many readers allowed) and <code class=\"kw\">borrow_mut()</code> (exactly one writer) — and <b>panics</b> when the rules are violated mid-frame.</p>",
            "<h3>Why «runtime-checked» is a gift, not a sin</h3>",
            "<p>The borrow compiler's rules are magnificent but static. Some data structures (doubly-linked lists, shared caches, observers) cannot prove their lifetime contract at compile time. RefCell relocates that duty into runtime and keeps the Rust discipline: break the rules and panic is your judge — deterministic and easy to trace through the stack.</p>",
            "<h3>Cycles and the Weak escape hatch</h3>",
            "<p>Counted ownership has one honest weakness: two values pointing at each other never reach zero, and both leak forever. The escape hatch is <code class=\"kw\">Weak&lt;T&gt;</code> — a non-owning cousin created with <code class=\"kw\">Rc::downgrade(&amp;rc)</code>. A <code class=\"kw\">Weak</code> does not keep the value alive; <code class=\"kw\">upgrade()</code> asks politely and returns an <code class=\"kw\">Option&lt;Rc&lt;T&gt;&gt;</code>. Parent → child links use <code class=\"kw\">Rc</code>, child → parent links use <code class=\"kw\">Weak</code> — and the palace has no ghost rooms.</p>",
            "<h3>Choosing the right pointer</h3>",
            "<ul><li>Single owner, needs the heap — <code class=\"kw\">Box&lt;T&gt;</code>;</li><li>Many owners, read-only — <code class=\"kw\">Rc&lt;T&gt;</code>;</li><li>Many owners, interior mutation — <code class=\"kw\">Rc&lt;RefCell&lt;T&gt;&gt;</code>;</li><li>Threads anywhere nearby — <code class=\"kw\">Arc</code>, and <code class=\"kw\">Mutex</code> for writing (chapter ten).</li></ul>",
            "<h3>Where threads enter</h3>",
            "<p>Rc + RefCell are strictly single-threaded: the counter is not atomic. The thread chapter will upgrade the registry to <code class=\"kw\">Arc</code> and the vault to <code class=\"kw\">Mutex</code> — same psychology, new engines. First secure the palace base here: the difference between a Box, an Rc, and a RefCell is the alphabet of chapter ten.</p>",
            "<p>Training ritual: rebuild one of your earlier quest solutions so that a <code class=\"kw\">Vec</code> holds <code class=\"kw\">Box&lt;dyn Something&gt;</code>, and watch a single list accept three different species. Then deliberately borrow a RefCell twice for mutation — read the panic aloud, find the line that violates the one-writer rule, and fix it. Pointers stop being scary the moment their rules are personal experience.</p>",
            "<div class=\"article-banner\">The palace epitaph: Box owns, Rc counts owners, RefCell guards the inner revisions.</div>",
        ],
    },

    // C10 threads
    Article {
        chapter_id: "threads",
        lang: "en",
        title: "The Choir of Threads",
        paragraphs: &[
            "<p>One voice can sing one song. Ten voices can sing <i>ten songs at once</i> — or drown each other into noise. Rust's headline achievement is letting the choir fly <b>without data races</b>: fearlessness is enforced by the types you have already studied.</p>",
            "<h3>What exactly is a data race?</h3>",
            "<p>Three ingredients: <b>two or more threads</b>, <b>one shared memory location</b>, and <b>at least one writer</b> with no synchronization between the accesses. The order of reads and writes becomes scheduler roulette — the same binary yields different answers on every run, and the bug hides for months. Rust's strategy is structural: remove at least one ingredient at compile time, so the race cannot even be built.</p>",
            "<h3>spawn &amp; move</h3>",
            "<p><code class=\"kw\">std::thread::spawn(|| …)</code> kindles a new thread and launches a closure inside it. The closure captures environment values — and by default it <b>borrow</b>s them, which the thread may outlive! The <code class=\"kw\">move</code> keyword forces the closure to <b>take ownership</b> of everything it touches: the luggage travels with the caravan. That is why «use after scope» bugs never happen in safe Rust.</p>",
            "<pre><code>use std::thread;\n\nlet lore = String::from(\"Ferris\");\n\nlet handle = thread::spawn(move || {\n    format!(\"songs of {lore}\")       // lore moved in\n});\nprintln!(\"{}\", handle.join().unwrap());\n</code></pre>",
            "<h3>Scoped threads: borrowing without moving</h3>",
            "<p>Sometimes <code class=\"kw\">move</code> is wasteful: the data only needs borrowing for the duration. <code class=\"kw\">std::thread::scope</code> guarantees — at compile time — that every spawned thread is joined before the scope ends, so the closures may <b>borrow</b> surrounding variables safely. It is the bridge between the strictness of <code class=\"kw\">move</code> and the comfort of ordinary borrowing.</p>",
            "<pre><code>let mut plans = vec![\"seeds\", \"water\"];\n\nthread::scope(|s| {\n    s.spawn(|| {\n        plans.push(\"sun\");          // &amp;mut borrow, safe inside scope\n        println!(\"{}\", plans.len());\n    });\n    s.spawn(|| println!(\"assistant idle\"));\n});                  // both threads joined here\nprintln!(\"{plans:?}\");\n</code></pre>",
            "<h3>Send &amp; Sync: the border guards</h3>",
            "<p>Two marker traits police the thread border. <code class=\"kw\">Send</code> says a value may be <b>transferred</b> to another thread; <code class=\"kw\">Sync</code> says a shared reference <code class=\"kw\">&amp;T</code> may be <b>read</b> from several threads at once. Almost every plain type is both automatically.</p>",
            "<p>The registry reveals the classics: <code class=\"kw\">Rc&lt;T&gt;</code> is <i>neither</i> — its counter is not atomic — and the compiler refuses to smuggle it across. But <code class=\"kw\">Mutex&lt;T&gt;</code> makes any <code class=\"kw\">T</code> Sync, and <code class=\"kw\">Arc&lt;T&gt;</code> is Send + Sync when <code class=\"kw\">T</code> itself is. Safety here is not a runtime promise; it is a type-level proof.</p>",
            "<h3>join: keeping the score</h3>",
            "<p><code class=\"kw\">handle.join()</code> waits for the thread's finish and returns its result wrapped in <code class=\"tp\">Result</code> — the thread may have panicked. A spawned thread dropped without a join behaves like a runaway racehorse: allowed, but you never receive its result. Join the handles you care about.</p>",
            "<h3>Arc: the atomic registry</h3>",
            "<p>Sharing one value between several threads needs <code class=\"kw\">Arc&lt;T&gt;</code> — an atomically counted <b>Rc for mutants</b>. <code class=\"kw\">Arc::clone</code> stays cheap: it only increments the atomic counter; each thread gets its own handle to the same heap value. <code class=\"kw\">Send</code>/<code class=\"kw\">Sync</code> marker traits ensure that only safely-shareable data passes through these doors.</p>",
            "<h3>Mutex: the golden case</h3>",
            "<p>Reading in parallel is safe; <b>writing</b> requires an appointment. <code class=\"kw\">Mutex&lt;T&gt;</code> is the golden case: <code class=\"kw\">lock()</code> returns a guard — while the guard lives, you own the door; on <code class=\"kw\">drop</code> the door relocks. Ergonomics: <code class=\"kw\">let mut gold = vault.lock().unwrap(); *gold += 100;</code>. The <code class=\"kw\">unwrap()</code> handles the (rare) <i>poisoned lock</i> case: another thread panicked while inside.</p>",
            "<p>Why does <code class=\"kw\">lock()</code> return a <code class=\"tp\">Result</code>? Because of <i>poisoning</i>: if a thread panics while holding the guard, the mutex is marked poisoned — the protected state may be inconsistent. <code class=\"kw\">.unwrap()</code> declares «a panic anywhere is a bug everywhere» — a pragmatic default; the alternative is to inspect the error and try to salvage the half-broken state.</p>",
            "<h3>RwLock: the reading hall</h3>",
            "<p><code class=\"kw\">RwLock&lt;T&gt;</code> refines the golden case: it hands out <b>any number of read guards</b> or <b>exactly one write guard</b> — never both. When reads outnumber writes ten to one, the reading hall outperforms the one-door vault. Same ergonomics, one extra dial; reach for it when profiling shows readers queuing, not before.</p>",
            "<h3>mpsc: the message corridor</h3>",
            "<p>Where mutexes jam threads into a queue, <b>channels</b> let voices just send letters. <code class=\"kw\">mpsc::channel()</code> — <i>multi-producer, single-consumer</i>: many <code class=\"kw\">tx.send(msg)</code> senders, one <code class=\"kw\">rx.recv()</code> receiver. Ownership of each message <b>travels</b> through the channel (move again!), so races are structurally impossible. Choose: queues for state, channels for events.</p>",
            "<p>Three habits make channels sing. <b>Dropping the last sender closes the corridor</b>: <code class=\"kw\">rx.recv()</code> then returns <code class=\"kw\">Err</code> — that is how workers learn it is quitting time. <b>Cloning the sender</b> (<code class=\"kw\">tx.clone()</code>) registers more producers — cheap, no shared state. And the loop idiom <code class=\"kw\">while let Ok(msg) = rx.recv()</code> keeps consuming until the mail stops coming.</p>",
            "<pre><code>let (tx, rx) = mpsc::channel::&lt;String&gt;();\nlet rx = Arc::new(Mutex::new(rx));\n\nfor id in 0..3 {\n    let rx = Arc::clone(&amp;rx);\n    thread::spawn(move || loop {\n        match rx.lock().unwrap().recv() {\n            Ok(job) =&gt; println!(\"worker {id}: {job}\"),\n            Err(_) =&gt; break,        // channel closed: exit\n        }\n    });\n}\nfor i in 0..6 { tx.send(format!(\"job #{i}\")).unwrap(); }\ndrop(tx);                            // close the corridor\n</code></pre>",
            "<p>A closing glance over the fence: OS <b>threads</b> are heavy soloists — each carries its own stack, and the OS scheduler hands out time preemptively. <b>async/await</b> is a giant choir of cheap tasks multiplexed over a few threads, waiting cooperatively. For CPU-bound work and for learning, threads are perfect; for tens of thousands of idle network connections, async wins. Chapter ten teaches the former on purpose — the concepts transfer.</p>",
            "<h3>The choir check-list</h3>",
            "<ul><li>Do you spawn with <code class=\"kw\">move</code>? (Recommended!)</li><li>Does every joinable handle get joined (or detached deliberately)?</li><li>Is shared state wrapped in <code class=\"kw\">Arc&lt;Mutex&lt;T&gt;&gt;</code>?</li><li>Are event flows routed through the channels instead?</li></ul>",
            "<p>Training ritual: try to share a plain <code class=\"kw\">i32</code> between two spawned threads and let the compiler refuse you — read the refusal twice. Then wrap it in <code class=\"kw\">Arc&lt;Mutex&lt;i32&gt;&gt;</code>, bump it from three threads, and join them. The transition from refusal to chord is the whole chapter in one gesture.</p>",
            "<div class=\"article-banner\">The compiler prunes accidental races, Arc/Mutex prunes careless ones. After this chapter, concurrency is a choir — not a street brawl.</div>",
        ],
    },

    // C4 collections
    Article {
        chapter_id: "collections",
        lang: "en",
        title: "The Vault of Collections",
        paragraphs: &[
            "<p>Scalars and structs hold <i>one</i> thing at a time. Real adventures hold <b>shelves</b> of things: a library of scrolls that grows and shrinks, a registry that answers «have I seen this key?». This chapter opens the Vault: <code class=\"kw\">Vec&lt;T&gt;</code> — the growable array, the iterator alchemy of <code class=\"kw\">map</code>, <code class=\"kw\">filter</code> and <code class=\"kw\">collect</code>, and the key-value registry <code class=\"kw\">HashMap&lt;K, V&gt;</code>. Collections are where ownership gets <i>practical</i>: the shelf owns its contents, and borrowing decides who may touch them.</p>",
            "<h3>Vec: the growable shelf</h3>",
            "<p><code class=\"kw\">Vec&lt;T&gt;</code> is a contiguous, heap-allocated array that owns every element on it. It knows three numbers: capacity (room reserved), length (room used) and a pointer to the heap buffer. When you exceed capacity, the Vec <b>reallocates</b>: a bigger buffer is allocated, every element is moved into it, the old one is freed. That is why <code class=\"kw\">Vec::with_capacity(n)</code> exists — reserve once, skip the churn.</p>",
            "<p>Two creation rites. <code class=\"kw\">Vec::new()</code> starts empty and lets types be inferred from use; <code class=\"kw\">vec![1, 2, 3]</code> mints a vector with the values inline — the sigil every Rust programmer types a thousand times. Both compile to the same thing; the macro is just sugar with better manners.</p>",
            "<pre><code>let mut shelf: Vec&lt;String&gt; = Vec::new();
shelf.push(\"Memory Avengers\".to_string());
let scrolls = vec![1, 2, 3];          // [i32; 3] in a Vec&lt;i32&gt;
</code></pre>",
            "<p><code class=\"kw\">push</code> appends to the end (amortized O(1)), <code class=\"kw\">pop</code> removes and <b>returns</b> the last element as <code class=\"kw\">Option&lt;T&gt;</code> — <code class=\"kw\">Some(value)</code> if the shelf had one, <code class=\"kw\">None</code> if it was empty. The Option return is the collection doctrine in miniature: «might be nothing» is encoded in the type, not in a null. Note that pop needs <code class=\"kw\">mut</code> — removing is mutation.</p>",
            "<pre><code>let mut stack = vec![10, 20, 30];
let top = stack.pop();      // Some(30)
stack.push(40);             // [10, 20, 40]
let len = stack.len();      // 3
</code></pre>",
            "<p><code class=\"kw\">len()</code> is the free look: a Vec always knows its own length, so it costs nothing. Indexing <code class=\"kw\">shelf[0]</code> reads the first volume and <b>panics</b> if the index is out of bounds — Rust checks every index at runtime, no silent garbage. For a checked read, <code class=\"kw\">shelf.get(2)</code> returns <code class=\"kw\">Option&lt;&amp;T&gt;</code>: a polite <code class=\"kw\">Some</code>/<code class=\"kw\">None</code> instead of a crash. Rule of thumb: index when you have already proven the bounds, <code class=\"kw\">get()</code> when you have not.</p>",
            "<h3>Borrowing the shelf: the classic trap</h3>",
            "<p>Here is the collision the borrow checker was built for: <code class=\"kw\">let first = &amp;shelf[0];</code> takes a read-only borrow of one element, and <code class=\"kw\">shelf.push(x)</code> needs <code class=\"kw\">&amp;mut</code> of the whole shelf — because pushing may <b>reallocate</b>, invalidating every outstanding reference. Holding <code class=\"kw\">&amp;shelf[0]</code> across a <code class=\"kw\">push</code> is a compile error (E0502). Take the value, or re-borrow after the mutation — the compiler walks you through it.</p>",
            "<pre><code>let mut shelf = vec![1, 2, 3];
let first = shelf[0];       // Copy out the value — no borrow held
shelf.push(4);              // fine: nothing borrows the shelf
println!(\"{first}\");
</code></pre>",
            "<h3>Iterators: the assembly line</h3>",
            "<p>Every collection hands out an <b>iterator</b> — a conveyor that yields items one by one. <code class=\"kw\">vec.iter()</code> yields <code class=\"kw\">&amp;T</code> (read), <code class=\"kw\">vec.iter_mut()</code> yields <code class=\"kw\">&amp;mut T</code> (rewrite), and <code class=\"kw\">vec.into_iter()</code> yields owned <code class=\"kw\">T</code> (consumes the Vec). The magic is that iterators are <i>lazy</i>: nothing happens until something pulls. Adaptors like <code class=\"kw\">map</code> and <code class=\"kw\">filter</code> just wrap the conveyor; only a <b>consumer</b> — <code class=\"kw\">collect</code>, <code class=\"kw\">sum</code>, <code class=\"kw\">for</code> — starts the machinery.</p>",
            "<p><code class=\"kw\">map</code> transforms each item: <code class=\"kw\">nums.iter().map(|n| n * 2)</code> is a new conveyor producing doubled values. <code class=\"kw\">filter</code> sieves: <code class=\"kw\">nums.iter().filter(|&amp;&amp;n| n % 2 == 0)</code> keeps the evens (note the double <code class=\"kw\">&amp;</code> — the closure receives a reference to the reference the iterator yields; one dereference pattern to learn once). Chain them freely: <code class=\"kw\">filter</code> then <code class=\"kw\">map</code> reads like a sentence and costs nothing extra — no intermediate vectors are built.</p>",
            "<pre><code>let nums = vec![1, 2, 3, 4, 5, 6];
let evens_doubled: Vec&lt;i32&gt; = nums
    .iter()
    .filter(|&amp;&amp;n| n % 2 == 0)
    .map(|n| n * 2)
    .collect();               // [4, 8, 12]
</code></pre>",
            "<p><code class=\"kw\">sum()</code> drains the conveyor into one total: <code class=\"kw\">nums.iter().sum::&lt;i32&gt;()</code>. The turbofish <code class=\"tp\">::&lt;i32&gt;</code> names the type because an empty conveyor of unknown integers has no obvious sum type — the compiler refuses to guess. You may also seed with <code class=\"kw\">fold(init, |acc, x| …)</code>, the grandmother of all reducers, when the reduction is not a plain addition.</p>",
            "<p><code class=\"kw\">collect()</code> is the destination sign of the assembly line: it pours the stream into a new container. The <b>target type chooses the shape</b>: <code class=\"kw\">collect::&lt;Vec&lt;i32&gt;&gt;()</code> builds a vector, <code class=\"kw\">collect::&lt;String&gt;()</code> glues chars, <code class=\"kw\">collect::&lt;HashSet&lt;_&gt;&gt;()</code> dedupes into a set. Most often the annotation rides on the variable: <code class=\"kw\">let squared: Vec&lt;i32&gt; = …collect();</code> — declare the destination and let the pipeline fill it.</p>",
            "<h3>HashMap: the registry</h3>",
            "<p><code class=\"kw\">HashMap&lt;K, V&gt;</code> maps keys to values with near-O(1) lookup. Keys must be hashable and owned-or-borrowed per the rules of <code class=\"kw\">insert</code>: <code class=\"kw\">map.insert(String::from(\"mana\"), 90)</code> <b>moves</b> both the key and the value into the map — the map becomes their owner. That is why HashMaps of <code class=\"kw\">String</code> keys appear constantly: the registry owns its labels.</p>",
            "<pre><code>use std::collections::HashMap;

let mut mana = HashMap::new();
mana.insert(String::from(\"fire\"), 40);
mana.insert(String::from(\"frost\"), 60);
let fire = mana.get(\"fire\");         // Some(&amp;40) — a borrow, not a copy
</code></pre>",
            "<p>Reading is a borrow: <code class=\"kw\">get(\"fire\")</code> returns <code class=\"kw\">Option&lt;&amp;V&gt;</code> — a reference into the map, guarded by an Option because the key may be missing. Writing has its own etiquette: plain <code class=\"kw\">insert</code> <b>overwrites</b> and returns the old value; <code class=\"kw\">entry(key).or_insert(0)</code> is the upsert — «insert only if absent, then give me the entry to modify». The entry API is how counters and aggregators are born:</p>",
            "<pre><code>let mut counts: HashMap&lt;String, i32&gt; = HashMap::new();
for word in [\"garden\", \"garden\", \"rust\"] {
    *counts.entry(word.to_string()).or_insert(0) += 1;
}
// {\"garden\": 2, \"rust\": 1}
</code></pre>",
            "<p>Two more registry habits. Iteration yields tuples: <code class=\"kw\">for (key, value) in &amp;mana { … }</code> — order is <i>unspecified</i> (hash maps have no rows to walk in order; if you need sorted output, collect keys and sort them). And there is a <code class=\"kw\">HashSet&lt;T&gt;</code> — a HashMap pretending values do not exist, perfect for «have I seen this?» membership tests.</p>",
            "<h3>Which container when?</h3>",
            "<ul><li>Ordered list, indexing, stack behavior — <code class=\"kw\">Vec&lt;T&gt;</code> (the default 90% choice).</li><li>Lookup by key, counting, dedupe with payloads — <code class=\"kw\">HashMap&lt;K, V&gt;</code>.</li><li>Membership only, no payloads — <code class=\"kw\">HashSet&lt;T&gt;</code>.</li><li>Fixed length known at compile time — the plain array <code class=\"tp\">[T; N]</code> stays on the stack.</li></ul>",
            "<h3>The ownership ledger of collections</h3>",
            "<p>Collections make ownership <b>visible</b>. Pushing moves the value in; popping moves it out; iteration chooses between borrowing (<code class=\"kw\">iter</code>) and consuming (<code class=\"kw\">into_iter</code>). When the compiler complains about a collection loop, translate the error into this ledger and the fix usually picks itself: clone the few items you must keep, borrow the ones you only read, and let the collection keep owning the rest.</p>",
            "<p>Training ritual: take any earlier quest solution and rebuild it three ways — with <code class=\"kw\">for</code> and an index, with <code class=\"kw\">while let Some(x) = vec.pop()</code>, and with a full <code class=\"kw\">map</code>/<code class=\"kw\">filter</code>/<code class=\"kw\">collect</code> pipeline. Same output, three dialects. The pipeline version is the one Rust seniors reach for first; now you know why.</p>",
            "<div class=\"article-banner\">The Vault rule: collections own their contents, iterators lend them out, and collect() is the customs office where borrowed streams become owned containers again.</div>",
        ],
    },

    // C8 errors
    Article {
        chapter_id: "errors",
        lang: "en",
        title: "The Trials of Result & Errors",
        paragraphs: &[
            "<p>Languages hide their failures behind exceptions or nulls; Rust hands them to you in a box with a label. This chapter runs the Trials: <code class=\"kw\">Result&lt;T, E&gt;</code> — the honest return type, the courtroom of <code class=\"kw\">match</code>, the shortcuts (<code class=\"kw\">unwrap</code>, <code class=\"kw\">expect</code>, <code class=\"kw\">unwrap_or</code>), the relay baton of the <code class=\"kw\">?</code> operator, <code class=\"kw\">map_err</code> for translating the charges, and <code class=\"kw\">std::io</code> — the place where failures meet you daily.</p>",
            "<h3>Result: the honest verdict</h3>",
            "<p><code class=\"kw\">Result&lt;T, E&gt;</code> is an enum with exactly two arms: <code class=\"kw\">Ok(value)</code> — the operation succeeded and here is the goods — and <code class=\"kw\">Err(error)</code> — it failed and here is the indictment. Unlike exceptions, failure is <b>in the type signature</b>: a caller cannot pretend not to notice. The compiler forces a decision before any value of <code class=\"kw\">T</code> touches your hands.</p>",
            "<pre><code>enum Result&lt;T, E&gt; {\n    Ok(T),\n    Err(E),\n}\n\nfn parse_level(text: &amp;str) -&gt; Result&lt;i32, std::num::ParseIntError&gt; {\n    text.parse::&lt;i32&gt;()   // may fail: the type says so up front\n}\n</code></pre>",
            "<h3>match: the courtroom</h3>",
            "<p>The honest way to handle a Result is <code class=\"kw\">match</code>: both arms must be covered, so the error cannot slither away unexamined. Inside the arms the payload is yours — the <code class=\"tp\">Ok(x)</code> pattern binds the value, <code class=\"tp\">Err(e)</code> binds the error. This is the zero-ceremony form and the one the standard library docs model first.</p>",
            "<pre><code>match parse_level(\"42\") {\n    Ok(level) =&gt; println!(\"welcome, level {level}\"),\n    Err(e) =&gt; println!(\"the scroll is corrupt: {e}\"),\n}\n</code></pre>",
            "<h3>unwrap &amp; expect: the shortcuts</h3>",
            "<p><code class=\"kw\">unwrap()</code> says «I already know it is Ok — give me the value», and if you are wrong it <b>panics</b>, crashing the thread with the default message. <code class=\"kw\">expect(\"context\")</code> is the same bet with a witness statement: your text is printed on panic, which turns a mystery crash into a breadcrumb trail. Both are legitimate in tests, prototypes and demos; in real control flow they are IOUs the next reader must pay.</p>",
            "<pre><code>let good = parse_level(\"42\").unwrap();        // panics if Err\nlet better = parse_level(\"42\").expect(\"level scroll must parse\");\n// panic: level scroll must parse: invalid digit found in string\n</code></pre>",
            "<h3>The ? operator: the relay baton</h3>",
            "<p>Inside a function that itself returns a Result, <code class=\"kw\">?</code> is the polite shorthand for the courtroom: «if Ok, hand me the value and go on; if Err, <b>return the error from my function right now</b>». It is propagation in one glyph — the error travels up the call stack automatically, untouched, until someone decides to actually judge it. A chain of <code class=\"kw\">?</code> reads like a straight road.</p>",
            "<pre><code>fn read_level(path: &amp;str) -&gt; Result&lt;i32, std::io::Error&gt; {\n    let text = std::fs::read_to_string(path)?;   // Err? return now\n    let level = text.trim().parse::&lt;i32&gt;().ok().ok_or(std::io::Error::new(\n        std::io::ErrorKind::InvalidData, \"bad level\"))?;\n    Ok(level)\n}\n</code></pre>",
            "<h3>unwrap_or: the bailiff's default</h3>",
            "<p>Sometimes an error is not worth a trial: the show must go on with a default. <code class=\"kw\">unwrap_or(fallback)</code> substitutes the fallback on Err; <code class=\"kw\">unwrap_or_else(|| …)</code> builds it lazily, only when needed; <code class=\"kw\">unwrap_or_default()</code> asks the type for its zero value. These three turn «may fail» into «fail politely» — ideal for config reads, cache lookups, anything with a sensible default.</p>",
            "<pre><code>let level = parse_level(user_input).unwrap_or(1);      // default 1\nlet level2 = parse_level(user_input).unwrap_or_else(|e| {\n    println!(\"warn: {e}\");\n    1\n});\n</code></pre>",
            "<h3>map_err: translating the charges</h3>",
            "<p>When layers speak different error dialects, <code class=\"kw\">map_err</code> rewrites the Err arm while leaving Ok untouched: <code class=\"kw\">result.map_err(|e| MyError::from(e))</code>. It is the adapter between your domain error and the library error a callee returned — one closure, no ceremony. Its quiet sibling <code class=\"kw\">map</code> does the same for the Ok arm: transform the success value in place.</p>",
            "<pre><code>let level: Result&lt;i32, String&gt; =\n    parse_level(\"7x\").map_err(|e| format!(\"bad scroll: {e}\"));\n</code></pre>",
            "<h3>std::io: where failures live daily</h3>",
            "<p>Filesystem and input operations return <code class=\"kw\">io::Result&lt;T&gt;</code> — simply <code class=\"kw\">Result&lt;T, io::Error&gt;</code> — because files do not exist, disks fill up, and permissions bite. The trials in this chapter's quests are all <code class=\"kw\">io::Error</code> wrangling in costume: check the <code class=\"kw\">ErrorKind</code> (NotFound? PermissionDenied?) when you need to react differently, or just <code class=\"kw\">?</code> it upward when you do not.</p>",
            "<pre><code>use std::fs;\n\nmatch fs::read_to_string(\"garden.log\") {\n    Ok(text) =&gt; println!(\"{} bytes of gossip\", text.len()),\n    Err(e) if e.kind() == std::io::ErrorKind::NotFound =&gt; {\n        println!(\"no log yet — first visit?\");\n    }\n    Err(e) =&gt; return Err(e),   // everything else: pass upward\n}\n</code></pre>",
            "<h3>The etiquette of failure</h3>",
            "<ul><li>Library code: <b>never</b> panic on expected failures — return <code class=\"kw\">Result</code> and let the caller judge.</li><li>Tests and scripts: <code class=\"kw\">unwrap</code>/<code class=\"kw\">expect</code> are fine — a panic <i>is</i> the test failing loudly.</li><li>Top level (main, request handlers): log and degrade, or propagate with <code class=\"kw\">?</code> to a handler that answers politely.</li><li>Choosing a fallback is a decision — <code class=\"kw\">unwrap_or</code> makes it visible, <code class=\"kw\">unwrap</code> makes it a crash.</li></ul>",
            "<p>The philosophy in one breath: Rust has panics for <b>bugs</b> (broken invariants, unrecoverable states) and Result for <b>weather</b> (missing files, bad input, full disks). Confusing the two is the original sin; the Trials teach you to tell them apart by reflex.</p>",
            "<p>Training ritual: take any earlier quest that used <code class=\"kw\">unwrap</code> and rewrite it three ways — full <code class=\"kw\">match</code>, a chain of <code class=\"kw\">?</code> with a custom error, and an <code class=\"kw\">unwrap_or</code> default. Same behavior, three audiences: a courtroom, a relay race, a shrug.</p>",
            "<div class=\"article-banner\">Trial rule: failure is part of the signature — Ok and Err are both legitimate answers, ? passes the baton, and unwrap is a bet you'd better be able to show your work for.</div>",
        ],
    },

    // C11 files
    Article {
        chapter_id: "files",
        lang: "en",
        title: "The Archive of Files",
        paragraphs: &[
            "<p>Everything you have written so far died with the process: variables evaporate, Vecs dissolve, the heap is swept clean. The Archive is where data finally <b>outlives</b> the program — shelves of files that persist between runs. This chapter tours the <code class=\"kw\">std::fs</code> wing: one-shot reading with <code class=\"kw\">read_to_string</code>, the write sibling <code class=\"kw\">fs::write</code>, the handle type <code class=\"kw\">File</code>, patient buffered reading with <code class=\"kw\">BufReader</code>, measuring with <code class=\"kw\">metadata</code>, and the erasing rite <code class=\"kw\">remove_file</code>.</p>",
            "<h3>The whole-file rite: fs::read_to_string</h3>",
            "<p>The simplest spell: give a path, receive the entire file as a <code class=\"kw\">String</code> — or an <code class=\"kw\">io::Error</code> in a <code class=\"kw\">Result</code>. It is one allocation, one read, zero ceremony; perfect for configs, seed data and quest-sized scrolls. Remember the two failure modes hiding inside: the file may be missing (NotFound) and the bytes may not be valid UTF-8 (InvalidData) — both arrive as Err.</p>",
            "<pre><code>use std::fs;\n\nlet scroll = fs::read_to_string(\"garden.txt\")?;\nprintln!(\"{}\", scroll);   // the whole file, one gulp\n</code></pre>",
            "<h3>The write sibling: fs::write</h3>",
            "<p><code class=\"kw\">fs::write(path, bytes)</code> is the mirror rite: it <b>creates or truncates</b> the file and pours the whole buffer in. Truncation matters — an existing file is silently emptied before the new bytes land; there is no gentle append here (for appending you open a File with append options, below). It accepts anything that reads as bytes — a <code class=\"tp\">&amp;str</code> is fine.</p>",
            "<pre><code>use std::fs;\n\nfs::write(\"garden.txt\", \"planted: 3 ferns\\n\")?;   // create or overwrite\n</code></pre>",
            "<h3>File: the handle</h3>",
            "<p><code class=\"kw\">File</code> is not the contents — it is a <b>handle</b>: an open passage to a file on disk, holding a cursor and permissions. <code class=\"kw\">File::open(path)</code> opens read-only (fails NotFound if absent); <code class=\"kw\">File::create(path)</code> opens (or makes) for writing and truncates. The handle closes itself on drop — no close() to forget — but an unclosed handle still pins OS resources, so keep the scope tight.</p>",
            "<pre><code>use std::fs::File;\n\nlet mut ledger = File::create(\"ledger.txt\")?;   // fresh, truncated\n</code></pre>",
            "<p>Through the handle you write precisely: <code class=\"kw\">write_all(bytes)</code> pushes the whole buffer (a plain <code class=\"kw\">write</code> may write partially — loops for the diligent, <code class=\"kw\">write_all</code> for everyone else). Multiple <code class=\"kw\">write_all</code> calls append to whatever the cursor has reached, which is how you build a file in pieces.</p>",
            "<pre><code>use std::io::Write;\n\nlet mut ledger = File::create(\"ledger.txt\")?;\nledger.write_all(b\"day 1: planted ferns\\n\")?;\nledger.write_all(b\"day 2: ferns thrive\\n\")?;\n</code></pre>",
            "<h3>BufReader: the patient reader</h3>",
            "<p>Unbuffered reads ask the OS per request — expensive in tiny sips. <code class=\"kw\">BufReader</code> wraps any reader in a comfortable buffer: one big syscall fills the pantry, then lines are served from memory. Wrap a <code class=\"kw\">File</code> and iterate <code class=\"kw\">lines()</code> — each line arrives as <code class=\"kw\">io::Result&lt;String&gt;</code> (newline trimmed). This is the workhorse of log parsing and CSV-ish files.</p>",
            "<pre><code>use std::fs::File;\nuse std::io::{self, BufRead};\n\nlet reader = BufReader::new(File::open(\"garden.log\")?);\nfor line in reader.lines() {\n    let line = line?;\n    println!(\"row: {line}\");\n}\n</code></pre>",
            "<p>For an interactive half-line at a time — a prompt, then the visitor's reply — the same reader's <code class=\"kw\">read_line(&amp;mut buf)</code> fills your String with one line <b>including</b> the newline, returning how many bytes were read (0 at end of stream). Trim afterwards; the newline is part of the harvest.</p>",
            "<pre><code>use std::io::BufRead;\n\nlet mut name = String::new();\nlet bytes = reader.read_line(&amp;mut name)?;   // includes '\\n'\nlet name = name.trim().to_string();         // polite version\n</code></pre>",
            "<h3>metadata: the measuring rite</h3>",
            "<p><code class=\"kw\">fs::metadata(path)</code> returns a dossier without opening the file: <code class=\"kw\">len()</code> — size in bytes, <code class=\"kw\">is_file()</code>/<code class=\"kw\">is_dir()</code> — what shelf is this, modified times — when last touched. Reading metadata first avoids surprises: skip empty files, refuse directories dressed as files, or just display a tidy size in the UI.</p>",
            "<pre><code>use std::fs;\n\nlet meta = fs::metadata(\"garden.txt\")?;\nprintln!(\"{} bytes, file? {}\", meta.len(), meta.is_file());\n</code></pre>",
            "<h3>The erasing rite: remove_file</h3>",
            "<p><code class=\"kw\">fs::remove_file(path)</code> takes a file off the shelf — permanently: no recycle bin, no undo, and it fails politely (Err) if the file is missing or a directory. Archives owe their users care: this is the one rite where a wrong path costs real data, so the etiquette below is not decoration.</p>",
            "<pre><code>use std::fs;\n\nif fs::metadata(\"tmp_scroll.txt\").is_ok() {\n    fs::remove_file(\"tmp_scroll.txt\")?;   // gone, no second chances\n}\n</code></pre>",
            "<h3>Etiquette of the Archive</h3>",
            "<ul><li>Whole-file jobs — <code class=\"kw\">fs::read_to_string</code>/<code class=\"kw\">fs::write</code> first, handles second.</li><li>Line-by-line or huge files — <code class=\"kw\">BufReader</code>/<code class=\"kw\">lines()</code>; never slurp a 4 GB log into a String.</li><li>Appending — open with <code class=\"kw\">OpenOptions::new().append(true)</code>; <code class=\"kw\">fs::write</code> will not append.</li><li>Every rite returns <code class=\"kw\">io::Result</code> — the Trials chapter is the prerequisite you already passed; use <code class=\"kw\">?</code> and match on <code class=\"kw\">ErrorKind</code> when it matters.</li><li>Deletion is forever — check before <code class=\"kw\">remove_file</code>, and never delete on a path built from user input without validation.</li></ul>",
            "<p>The Archive closes the loop the whole garden has been building: ownership taught you who is responsible for data, the Trials taught you what to do when reality refuses, and files are where your data finally steps outside RAM and survives the night. A program that can read, write and erase responsibly is no longer a toy — it is a small, polite citizen of the disk.</p>",
            "<div class=\"article-banner\">Archive rule: read_to_string and fs::write are the one-gulp rites, File is the handle you carry, BufReader is patience in a wrapper, metadata is the dossier, and remove_file is the one rite you cast twice about.</div>",
        ],
    },

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

    // C6 traits
    Article {
        chapter_id: "traits",
        lang: "en",
        title: "Craft of Traits & Generics",
        paragraphs: &[
            "<p>Every structure so far stood alone: a <code class=\"kw\">struct</code> knew its fields, and that was its whole biography. The Workshop teaches structures to <b>promise</b>. A <code class=\"kw\">trait</code> is a contract of abilities — «anything signed here can be cloned, compared, printed» — and generics are the workshop's way of writing one blueprint that fits many materials. Together they are how Rust stays strict <i>and</i> flexible at once.</p>",
            "<h3>The contract: defining a trait</h3>",
            "<p>A trait lists method <b>signatures</b> without bodies: the promises, not the proofs. A type signs the contract with an <code class=\"kw\">impl TraitFor Type</code> block that fills in real bodies. After signing, the type's methods are callable through dot syntax, and the type itself becomes usable wherever the trait is demanded.</p>",
            "<pre><code>trait Enchantable {\n    fn power(&amp;self) -&gt; i32;              // promise, no body\n    fn describe(&amp;self) -&gt; String {       // default body allowed\n        format!(\"power: {}\", self.power())\n    }\n}\n\nstruct Wand { charges: i32 }\n\nimpl Enchantable for Wand {\n    fn power(&amp;self) -&gt; i32 { self.charges }   // proof\n}\n</code></pre>",
            "<p>Note the two speeds of promise: <code class=\"kw\">power</code> has no body, so every signer <b>must</b> write it; <code class=\"kw\">describe</code> ships a <b>default body</b>, so signers inherit it for free (and may override). Default methods are how traits stay small yet useful — a handful of mandatory promises plus a toolkit of freebies.</p>",
            "<h3>impl Trait: contract-based plumbing</h3>",
            "<p>Where a type would be written, <code class=\"kw\">impl TraitName</code> says: «any type that signed this contract». As a <b>parameter</b> it accepts every signer; as a <b>return type</b> it promises some signer while keeping the exact type private (one concrete type per function — to return different types conditionally you would need <code class=\"kw\">dyn</code>, below). It is generics with the syllabus hidden.</p>",
            "<pre><code>fn announce(item: &amp;impl Enchantable) -&gt; String {\n    item.describe()   // works for Wand, Ring, anything signed\n}\n</code></pre>",
            "<h3>Clone &amp; Copy: the duplicator contracts</h3>",
            "<p><code class=\"kw\">Clone</code> promises an explicit, visible duplicate: <code class=\"kw\">x.clone()</code> — a real method call you can point at in a code review. <code class=\"kw\">Copy</code> promises a free, silent duplicate on every assignment — allowed only for stack-only types (integers, floats, bools, char; and tuples/arrays of those). Every <code class=\"kw\">Copy</code> type is <code class=\"kw\">Clone</code>, never the reverse: Copy is the stricter, quieter sibling. Borrow checker sighs of relief are made of these two.</p>",
            "<pre><code>#[derive(Clone)]\nstruct Spell { name: String, mana: i32 }\n\nlet a = Spell { name: \"frost\".into(), mana: 30 };\nlet b = a.clone();     // explicit duplicate; `a` still owns its text\nlet n = 7;\nlet m = n;             // i32 is Copy: silent, `n` still usable\n</code></pre>",
            "<h3>PartialEq &amp; Eq: the comparison contract</h3>",
            "<p><code class=\"kw\">PartialEq</code> promises the <code class=\"tp\">==</code>/<code class=\"tp\">!=</code> operators work (its <code class=\"kw\">eq</code> method is what <code class=\"tp\">==</code> lowers to). It is «partial» because some types have values that simply refuse equality math — <code class=\"tp\">f32::NAN == NAN</code> is false. <code class=\"kw\">Eq</code> is the stricter marker: «every value compares honestly» — integers get it, floats don't. Most structs just <code class=\"kw\">#[derive(PartialEq, Eq)]</code> and field-wise comparison is generated for you.</p>",
            "<pre><code>#[derive(PartialEq, Eq)]\nstruct Rune { glyph: char, tier: i32 }\n\nlet a = Rune { glyph: 'α', tier: 2 };\nlet b = Rune { glyph: 'α', tier: 2 };\nlet same = a == b;     // true — field by field\n</code></pre>",
            "<h3>Generics: the letter T</h3>",
            "<p>A generic function stamps <code class=\"tp\">&lt;T&gt;</code> before its name and speaks of <code class=\"kw\">T</code> as «some type, decided by the caller». Nothing is duplicated in your source, yet the compiler performs <b>monomorphization</b>: for every concrete T actually used, it stamps out a specialized copy of the function at compile time — generics in Rust cost you zero at runtime. The trade: a bare <code class=\"kw\">T</code> can do <i>nothing</i> — to call methods, T must sign a trait: <code class=\"tp\">&lt;T: Enchantable&gt;</code>.</p>",
            "<pre><code>fn biggest&lt;T: PartialOrd&gt;(list: &amp;[T]) -&gt; Option&lt;&amp;T&gt; {\n    if list.is_empty() { return None; }\n    let mut best = &amp;list[0];\n    for item in &amp;list[1..] {\n        if item &gt; best { best = item; }\n    }\n    Some(best)\n}\n</code></pre>",
            "<p>Generic <b>structs</b> carry the same letter on the name: <code class=\"tp\">Pair&lt;T&gt;</code> holds two T's, <code class=\"tp\">Pair&lt;K, V&gt;</code> holds a key and a value of two different types — this is precisely how <code class=\"kw\">Vec&lt;T&gt;</code> and <code class=\"kw\">HashMap&lt;K, V&gt;</code> from the Vault are written. One blueprint, many materials, no code duplication in your repo and no dynamic dispatch at runtime.</p>",
            "<pre><code>struct Pair&lt;T&gt; { left: T, right: T }\n\nlet glyphs = Pair { left: 'α', right: 'ω' };\nlet levels = Pair { left: 3, right: 9 };   // same blueprint, other T\n</code></pre>",
            "<h3>Display: the printing contract</h3>",
            "<p><code class=\"tp\">{}</code> in <code class=\"kw\">println!</code>/<code class=\"kw\">format!</code> demands <code class=\"kw\">std::fmt::Display</code> — human-readable output. It is not automatic: you sign it yourself, writing into the formatter with <code class=\"kw\">write!</code>. The signature looks ceremonial and is worth reading once slowly: a method borrowed from the formatter that must return a <code class=\"kw\">fmt::Result</code>.</p>",
            "<pre><code>use std::fmt;\n\nimpl fmt::Display for Wand {\n    fn fmt(&amp;self, f: &amp;mut fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {\n        write!(f, \"Wand({} charges)\", self.charges)\n    }\n}\n// println!(\"{}\", wand) now speaks politely\n</code></pre>",
            "<h3>dyn Trait: the backstage pass</h3>",
            "<p>Generics and <code class=\"kw\">impl Trait</code> resolve at <b>compile time</b> (static dispatch: the exact method is stamped in). <code class=\"kw\">dyn Trait</code> is the runtime shape: a <i>trait object</i> — some signer, decided later, reached through a vtable. It costs a pointer + indirection and forbids returning different types from one function without boxing, but it lets a single <code class=\"kw\">Vec&lt;Box&lt;dyn Enchantable&gt;&gt;</code> hold wands, rings and staffs side by side. Rule: generics by default, <code class=\"kw\">dyn</code> when the set of types is genuinely open.</p>",
            "<pre><code>let shelf: Vec&lt;Box&lt;dyn Enchantable&gt;&gt; = vec![\n    Box::new(Wand { charges: 3 }),\n    Box::new(Staff { runes: 9 }),\n];\nfor item in &amp;shelf {\n    println!(\"{}\", item.describe());  // vtable call at runtime\n}\n</code></pre>",
            "<h3>where: the polite constraint ledger</h3>",
            "<p>When the <code class=\"tp\">&lt;&gt;</code> list grows heavy — two generics, each with two bounds — the signature drowns. <code class=\"kw\">where</code> moves every constraint to a readable ledger after the signature, identical in meaning. Same rules, cleaner script; large real-world signatures almost always use it.</p>",
            "<pre><code>fn archive&lt;T, U&gt;(item: T, ledger: U)\nwhere\n    T: Enchantable + Clone,\n    U: fmt::Display,\n{\n    // constraints live here, readable at a glance\n}\n</code></pre>",
            "<h3>Which tool when?</h3>",
            "<ul><li>Shared behavior across many types — define a <code class=\"kw\">trait</code>, sign with <code class=\"kw\">impl … for</code>.</li><li>«Any signer, exact type private» — <code class=\"kw\">impl Trait</code> in argument/return position.</li><li>Duplicating values — <code class=\"kw\">Clone</code> (explicit) or <code class=\"kw\">Copy</code> (silent, stack-only).</li><li>Comparing values — <code class=\"kw\">PartialEq</code> (+ <code class=\"kw\">Eq</code> for honest total equality).</li><li>One blueprint, many types, zero runtime cost — generics <code class=\"tp\">&lt;T&gt;</code>.</li><li>A mixed bag of different signers — <code class=\"kw\">Box&lt;dyn Trait&gt;</code>.</li></ul>",
            "<p>The Workshop's quiet secret: every standard-library superpower you have used — <code class=\"kw\">.clone()</code>, <code class=\"tp\">==</code>, <code class=\"tp\">{}</code> printing, <code class=\"kw\">Vec&lt;T&gt;</code> — was a trait or a generic all along. You have been signing other people's contracts since chapter one; now the contracts are yours to write.</p>",
            "<div class=\"article-banner\">Workshop rule: traits are promises, impl blocks are proofs, generics are blueprints stamped at compile time, and dyn is the backstage pass for when the guest list is open.</div>",
        ],
    },


    // =====================================================================
    // RU — Russian editions
    // =====================================================================
    // C1 basics
    Article {
        chapter_id: "basics",
        lang: "ru",
        title: "Основы Сада Памяти",
        paragraphs: &[
            "<p>Добро пожаловать, путешественник. Эта глава — почва, из которой растёт всё остальное в <b>Ferris Garden</b>. Rust обещал <i>безопасность памяти без сборщика мусора</i> — и держит обещание не в рантайме, а <b>во время компиляции</b>: компилятор просто отказывается собирать программу, в которой возможны утечка, use-after-free или двойное освобождение. Прежде чем качать замки потоков и отливать структуры, твои руки должны освоить три скромные вещи: ключевые слова, типы выражений и мышление «неизменность — по умолчанию».</p>",
            "<h3>Неизменяемость по умолчанию</h3>",
            "<p>В большинстве языков переменная — миска, которую можно доливать вечно. Rust переворачивает правило: <code class=\"kw\">let</code> связывает имя со значением <b>неизменяемо</b>. Повторное присваивание — ошибка компиляции: <code class=\"kw\">cannot assign twice to immutable variable</code>. Зачем? Неизменяемость убивает целые семейства багов до их рождения: нет случайных перезаписей, нет неожиданных мутаций через общие ссылки, а компилятор может смелее оптимизировать код, зная, что связывание не меняется.</p>",
            "<p>Мутация не запрещена — она <b>явная</b>. Ключевое слово <code class=\"kw\">mut</code> кричит о намерении: «это место БУДЕТ меняться». Дисциплина строга: каждое изменяемое связывание — обдуманный выбор, и оно заслуживает заметного ключевого слова.</p>",
            "<h3>Всё есть выражение</h3>",
            "<p>Rust — язык, <i>ориентированный на выражения</i>: почти всё вычисляет значение. Команда <code class=\"kw\">let</code> связывает значение с именем; блок <code class=\"tp\">{ … }</code> вычисляется в своё последнее выражение; даже <code class=\"kw\">if</code> и <code class=\"kw\">match</code> возвращают значения. Команда с точкой с запятой <code class=\"tp\">;</code> выбрасывает значение и вычисляется в единичный тип <code class=\"kw\">()</code> — растовское вежливое «ничто».</p>",
            "<pre><code>let harvest = {
    let seeds = 3;
    seeds * 4          // без точки с запятой: это и есть значение блока
};                     // harvest == 12
</code></pre>",
            "<p>Классическая ловушка новичка прячется в той самой точке с запятой: добавь <code class=\"tp\">;</code> после последней строки тела функции — и тип возврата молча станет <code class=\"kw\">()</code>, а компилятор пожалуется на «несовпадение типов». Если ошибка с первого взгляда не имеет смысла — сначала ищи лишнюю точку с запятой: это самый частый призрак сада.</p>",
            "<h3>Экскурсия по примитивным типам</h3>",
            "<p>Целые числа живут двумя семьями — знаковой и беззнаковой — и в восьми ширинах каждая: от <code class=\"kw\">i8</code>/<code class=\"kw\">u8</code> до <code class=\"kw\">i128</code>/<code class=\"kw\">u128</code>, плюс машинные <code class=\"kw\">isize</code>/<code class=\"kw\">usize</code> для индексации. Дефолт — <code class=\"kw\">i32</code>: универсальный выбор, к которому компилятор откатывается, когда контекст не подсказывает ничего лучшего.</p>",
            "<p>Плавающая точка бывает двух ширин: <code class=\"kw\">f32</code> и <code class=\"kw\">f64</code> (дефолт и верный выбор в 95% случаев). <code class=\"kw\">bool</code> — строго <code class=\"kw\">true</code> или <code class=\"kw\">false</code>: никакие «правдоподобные» числа не просочатся. <code class=\"kw\">char</code> — не байт: это полноценный <b>скаляр Unicode</b>, так что <code class=\"tp\">'Ф'</code>, <code class=\"tp\">'한'</code> и <code class=\"tp\">'🦀'</code> — законные одиночные символы. А пустой кортеж <code class=\"kw\">()</code> («юнит») — тип, означающий «нет значимого значения».</p>",
            "<pre><code>let level: u8 = 7;          // 0..=255
let price: f64 = 12.5;      // float по умолчанию
let awake: bool = true;     // строго true | false
let crab: char = '🦀';      // полный скаляр Unicode, 4 байта
</code></pre>",
            "<h3>Затенение против мутации</h3>",
            "<p>Новички путают <i>затенение</i> и <i>мутацию</i>. Затенение ничего не меняет: <code class=\"kw\">let y = y + 1;</code> создаёт совершенно новое связывание, старое остаётся, но скрыто. Оно даже может сменить тип: текст становится числом, число — истиной (bool). Поток данных остаётся линейным и проверяемым: каждая трансформация — собственное объявление. Мутация же переписывает ту же самую ячейку на месте.</p>",
            "<p>Затенение великолепно, когда значение проходит стадии: распарсить строку в число и оставить то же имя; пересчитать единицы на месте; переиспользовать переменную цикла с более удобным типом. Каждая стадия — свежее связывание, поэтому компилятор помнит тип каждой стадии точно, а код читается как дневник: одна строка — одна трансформация.</p>",
            "<h3>Три золотых правила владения</h3>",
            "<ul><li>1. У каждого значения в каждый момент ровно один <b>владелец</b>.</li><li>2. Когда владелец покидает область, значение <b>уничтожается</b> (память освобождается автоматически, без GC).</li><li>3. Присваивание и передача <b>перемещают</b> значение; старое имя перестаёт существовать.</li></ul>",
            "<p>Семантика перемещения пугает новичков и восхищает ветеранов. <code class=\"kw\">String</code> держит байты на куче; молчаливое копирование породило бы целый выводок use-after-free багов, поэтому Rust отказывается копировать без твоего явного слова. Когда нужна параллельная копия — вызывай <code class=\"kw\">.clone()</code> и плати за каждый продублированный байт. Нужно несколько читателей одной строки? Скоро появятся ссылки <code class=\"kw\">&amp;</code>.</p>",
            "<p>Перемещения случаются при каждом присваивании, аргументе функции и возврате — не только у <code class=\"kw\">String</code>. Функция, принимающая <code class=\"kw\">String</code> по значению, становится новым владельцем, а имя вызывающего умирает в точке вызова. Возврат значения возвращает владение обратно. Это весь «слоёный договор» потока данных Rust — и компилятор исполняет его механически, без сторожевых псов в рантайме.</p>",
            "<h3>Copy против Clone</h3>",
            "<p>Малые простые значения (<code class=\"kw\">i32</code>, <code class=\"kw\">f64</code>, <code class=\"kw\">bool</code>, <code class=\"kw\">char</code>, кортежи таких типов) носят особый маркер <code class=\"kw\">Copy</code>: присваивания молча дублируют их, ведь побитовая копия не способна породить баги памяти. Всё кучевое требует явного <code class=\"kw\">.clone()</code>: пауза — цена честности; ты видишь ровно то место, где происходит дорогое дублирование.</p>",
            "<pre><code>let n = 42;
let m = n;          // Copy: n всё ещё жив
let s = String::from(\"Ferris\");
let t = s.clone();  // явно: s тоже жив
// let u = s;       // было бы ПЕРЕМЕЩЕНИЕ — s умирает здесь
</code></pre>",
            "<h3>Заимствование и Borrow Checker</h3>",
            "<p>Ссылка — временный ключ на одолженном участке. Доктрина звучит как <b>Алиасинг XOR Мутация</b>: сколько угодно читателей (<code class=\"kw\">&amp;T</code>) ИЛИ ровно один писатель (<code class=\"kw\">&amp;mut T</code>), и никогда вместе. Это одно правило растворяет гонки данных в корне — потоки в десятой главе будут опираться на него всей тяжестью.</p>",
            "<pre><code>let mut mana = String::from(\"full\");
let r1 = &amp;mana;
mana.push('?');      // ✗ E0502: r1 ещё читается ниже
println!(\"{r1}\");    // последнее использование r1
// поменяй эти две строки местами — и код соберётся!
</code></pre>",
            "<p>Современный Rust смягчает таймлайн: при <b>Non-Lexical Lifetimes</b> заём кончается в точке <i>последнего использования</i>, а не у закрывающей скобки. Подними <code class=\"kw\">println!</code> над <code class=\"kw\">push</code> — и код соберётся: компилятор отслеживает фактическое применение, а не фигурные диапазоны. Поспорь с чекером день-другой — и дисциплина зазвучит как ограждение, а не клетка.</p>",
            "<h3>Срезы: взгляды вместо копий</h3>",
            "<p>Последний элемент — <code class=\"kw\">&amp;str</code>: <i>толстый указатель</i> на окно байтов <code class=\"tp\">(ptr, len)</code>. Срезы читают часть данных, не касаясь кучи: ноль аллокаций, полные проверки времён жизни. Это входной билет в итераторные главы: сначала взгляд, копирование — лишь по нужде.</p>",
            "<p>Срезы прививают и привычку к проверке границ: индекс за концом паникует с <code class=\"kw\">index out of bounds</code> вместо чтения мусора. Для доступа, где промах возможен, предпочти <code class=\"kw\">.get(i)</code>, возвращающий <code class=\"kw\">Option&lt;T&gt;</code>, — глава форм превратит этот приём в ежедневный рефлекс.</p>",
            "<h3>Строки: &amp;str против String</h3>",
            "<p>Два строковых типа путают каждого ровно один раз. <code class=\"kw\">String</code> — <b>владеющий</b> кучевый буфер: его можно растить, сжимать и переписывать. <code class=\"kw\">&amp;str</code> — <b>заимствованный</b> взгляд на чужие байты; строковые литералы — классический случай: <code class=\"tp\">\"Ferris\"</code> в коде — это <code class=\"kw\">&amp;'static str</code>, вшитая в бинарник. Функциям, которые только читают, передавай <code class=\"kw\">&amp;str</code>; <code class=\"kw\">String</code> держи там, где владеешь и мутируешь.</p>",
            "<h3>Кортежи, массивы, деконструкция</h3>",
            "<p>Кортежи <code class=\"kw\">(T, U)</code> связывают фиксированные наборы разных типов; массивы <code class=\"tp\">[T; N]</code> держат ровно <code class=\"kw\">N</code> однотипных значений прямо на стеке. Оба деконструируются красиво: <code class=\"kw\">let (a, b) = pair;</code> или <code class=\"kw\">let [first, .., last] = arr;</code> именуют части без методного шума. Массив знает свою длину на компиляции — другая длина означает другой тип.</p>",
            "<pre><code>let potion = (\"heal\", 40);          // (&amp;str, i32)
let (name, power) = potion;         // деконструкция
let cells = [1, 2, 3];              // [i32; 3]
let mid = cells[1];                 // с проверкой границ
</code></pre>",
            "<h3>const, static и приведения</h3>",
            "<p>Компиляторные константы живут под <code class=\"kw\">const</code>: они влетают в каждое место использования и ОБЯЗАНЫ иметь аннотацию типа. Именование — SCREAMING_SNAKE_CASE (<code class=\"tp\">MAX_CHARGE</code>). <code class=\"kw\">static</code> — долгоживущий собрат: одно значение на весь запуск программы, удобно для глобальной конфигурации; соглашение об именах то же.</p>",
            "<p>Приведение примитивов делается через <code class=\"kw\">as</code> и <b>по построению теряет данные</b>: <code class=\"kw\">f64</code> → <code class=\"kw\">i64</code> отбрасывает дробную часть к нулю, а большое число, зажатое в малый тип, заворачивается по кругу. Поведение при переполнении зависит от сборки: debug паникует, release молча заворачивает — так что считай <code class=\"kw\">as</code> обдуманным и прокомментированным решением, а не привычкой.</p>",
            "<h3>Читать письма компилятора</h3>",
            "<p>Сообщения об ошибках Rust — готовый учебник. Каждое несёт код (вроде <code class=\"tp\">E0384</code>), участок кода-виновника и часто <i>строку-подсказку</i> с точным лечением. Читай их как письма от наставника: нарочно ломать код ради изучения сообщения — легитимная (и весёлая) тренировка, и каждый квест этого сада её поощряет.</p>",
            "<h3>Повседневный ящик инструментов</h3>",
            "<p>Глава складывает в карман и ежедневный набор: типы <code class=\"kw\">i32</code>/<code class=\"kw\">f64</code>/<code class=\"kw\">bool</code>, кортежи <code class=\"kw\">(T, U)</code>, фиксированные массивы <code class=\"tp\">[T; N]</code>, константы <code class=\"kw\">const</code> и явные приведения через <code class=\"kw\">as</code>. Ничего ракетного, но всё это будет повторяться ежедневно — проходи квесты в своём темпе и нарабатывай мышечную память пальцев.</p>",
            "<p>Практический ритуал этой главы: после каждого квеста попробуй <i>сломать</i> собственное решение — переприсвой неизменяемое, перемести строку дважды, проиндексируй за конец. Наблюдать, как компилятор блокирует каждое злодеяние, полезнее ста абзацев теории.</p>",
            "<p>Иди по квестам неторопливо. Печатай ответ до чтения подсказки, зажигай зелёный фидбэк и слушай, как сад дышит. Основы пройдены — в следующей главе тебя дожидаются <code class=\"kw\">match</code> и циклы.</p>",
            "<div class=\"article-banner\">Почва готова: двенадцать квестов превратят эти основы в рефлексы. Иди неспешно — сад вознаграждает терпение больше, чем скорость.</div>",
        ],
    },

    // C2 flow
    Article {
        chapter_id: "flow",
        lang: "ru",
        title: "Руны Потока и Функций",
        paragraphs: &[
            "<p>Программа, которая никогда не спорит, — камень. Эта глава учит программу <b>принимать решения</b>: ветвиться, диспетчеризировать и повторяться. Звезда главы — <code class=\"kw\">match</code>, механизм сопоставления с образцом, куда сильнее привычного if/else.</p>",
            "<h3>if, и он возвращает</h3>",
            "<p>Ветвление выглядит знакомо, но Rust добавляет поворот: <code class=\"kw\">if</code> — это <b>выражение</b>, а не команда. Оно оценивается в значение, поэтому можно писать <code class=\"kw\">let status = if awake { \"бодрствует\" } else { \"дремлет\" };</code> — ни грибов с тернарным оператором, ни смены типов между ветвями, код читается как фраза.</p>",
            "<pre><code>let mood = if hunger &gt; 8 { \"ворчлив\" } else { \"солнечный\" };
</code></pre>",
            "<p>Жёсткое правило: условие обязано быть настоящим <code class=\"kw\">bool</code>. Истинности-по-интуиции в Rust нет — <code class=\"tp\">if 1 { … }</code> откажется компилироваться, ведь <code class=\"kw\">i32</code> и <code class=\"kw\">bool</code> — разные виды. Строго? Да. Зато ни одного «случайно-истинного» значения: каждое условие говорит ровно то, что имеет в виду.</p>",
            "<h3>match: алтарь образцов</h3>",
            "<p><code class=\"kw\">match</code> сопоставляет значение со списком <b>образцов</b> и исполняет первую совпавшую руку. Три причины влюбиться:</p>",
            "<ul><li><b>Исчерпывающая полнота</b>: компилятор не даст забыть руку — «non-exhaustive patterns» остановит сборку.</li><li><b>Разбор груза</b>: руки мгновенно именуют внутренние значения — <code class=\"kw\">Ok(n) =&gt; println!(\"{}\", n)</code>.</li><li><b>Руки-диапазоны</b>: <code class=\"kw\">1..=5</code> вмещает целый коридор чисел, а <code class=\"kw\">_</code> — сеть, ловящая всё прочее.</li></ul>",
            "<p>Джокер <code class=\"kw\">_</code> вместе с проверкой полноты делает компилятора личным аудитором: добавишь новый вариант enum — и все неохваченные <code class=\"kw\">match</code> откажутся компилироваться, пока рука не написана. Именно поэтому больших рефакторингов в Rust боятся меньше.</p>",
            "<pre><code>match scroll {
    0              =&gt; println!(\"пусто\"),
    1..=9          =&gt; println!(\"немного страниц\"),
    n @ 10..=99    =&gt; println!(\"{n} страниц\"),
    _              =&gt; println!(\"целый том!\"),
}
</code></pre>",
            "<h3>Зоопарк образцов</h3>",
            "<p>Образцы — отдельный язык. Литералы совпадают сами с собой (<code class=\"kw\">0</code>), диапазоны накрывают коридоры (<code class=\"kw\">1..=9</code>), кортежи и структуры разбираются на месте (<code class=\"kw\">(a, b)</code>, <code class=\"tp\">Guardian { spear, .. }</code>), варианты enum совпадают со своим грузом (<code class=\"kw\">Some(x)</code>), а <code class=\"kw\">_</code> вежливо игнорирует то, что не хочет называть. Один <code class=\"kw\">match</code> потому заменяет целую лестницу if-ов.</p>",
            "<p>Два расширения делают образцы хирургическими. <b>Охрана</b> добавляет руке условие: <code class=\"kw\">n if n &gt; 10 =&gt; …</code> срабатывает, лишь когда дополнительная проверка прошла. А <code class=\"kw\">n @ 10..=99</code> <b>связывает</b> совпавшее значение с именем, продолжая матчить диапазон, — извлечение и именование одним вздохом.</p>",
            "<pre><code>match point {
    (0, y)                  =&gt; println!(\"на оси Y: {y}\"),
    (x, y) if x == y        =&gt; println!(\"диагональ\"),
    (x, y)                  =&gt; println!(\"где-то: {x}, {y}\"),
}
</code></pre>",
            "<h3>if let: лёгкая церемония</h3>",
            "<p>Когда интересен только <i>один</i> образец, <code class=\"kw\">match</code> — целый оркестр ради одной ноты. <code class=\"kw\">if let Some(key) = relic { … }</code> распаковывает лишь тот случай, который тебе важен, а остальное оставляет в тишине. Квест 31 упражняет ровно эту идиому — повседневный инструмент для <code class=\"kw\">Option</code> и <code class=\"kw\">Result</code>.</p>",
            "<h3>while let: вычерпывание очереди</h3>",
            "<p>Циклический собрат повторяется, пока образец продолжает совпадать: <code class=\"kw\">while let Some(msg) = rx.recv() { … }</code> поглощает сообщения, пока канал не иссякнет. Это идиоматический способ опустошать стеки, очереди и итераторы без ручных счётчиков.</p>",
            "<h3>Циклы: loop, while, for</h3>",
            "<p>Три вида вращения. <code class=\"kw\">loop</code> честно декларирует <i>возможную вечность</i> — выход только через <code class=\"kw\">break</code>, который к тому же умеет <b>возвращать значение</b>: <code class=\"kw\">let fuel = loop { break 42; };</code>. <code class=\"kw\">while</code> перепроверяет условие перед каждым оборотом. <code class=\"kw\">for</code> обходит <b>итераторы и диапазоны</b> (<code class=\"kw\">1..=5</code>, <code class=\"tp\">vec.iter()</code>) — идиоматический дефолт: ни ошибок на единицу (off-by-one), ни ручной проверки индексов.</p>",
            "<p>Вложенные циклы ставят практический вопрос: какой <code class=\"kw\">break</code> из какого цикла выходит? Rust отвечает <b>метками</b>: <code class=\"kw\">'outer: loop { … }</code> можно покинуть с глубины командой <code class=\"kw\">break 'outer;</code>. Метки работают и для <code class=\"kw\">continue</code> — никаких танцев с булевыми флагами.</p>",
            "<pre><code>'outer: for pot in pots {
    for seed in seeds {
        if seed.spoiled { continue 'outer; }
    }
    println!(\"{} посажен\", pot);
}
</code></pre>",
            "<h3>for и три итератора</h3>",
            "<p><code class=\"kw\">for</code> молча вызывает один из трёх итераторных вкусов. <code class=\"kw\">for x in vec</code> <b>перемещает</b> значения наружу (вектор съеден); <code class=\"kw\">for x in vec.iter()</code> даёт <code class=\"kw\">&amp;T</code> — только чтение; <code class=\"kw\">for x in vec.iter_mut()</code> даёт <code class=\"kw\">&amp;mut T</code> — лицензию на переписывание. Сознательный выбор вкуса — разница между случайным move-багом и чистым проходом.</p>",
            "<h3>fn, параметры, возвраты</h3>",
            "<p><code class=\"kw\">fn</code> объявляет заклинание: имя, типизированный список параметров и стрелка <code class=\"kw\">-&gt;</code>, объявляющая тип результата. Последнее выражение тела возвращается неявно — большинство функций обходится без слова <code class=\"kw\">return</code>. Стиль Rust любит <b>цельные глаголы</b>: <code class=\"kw\">fn charge(price: i32) -&gt; i32</code>.</p>",
            "<p>Почему у параметров функции обязательны типы? Потому что вывод типов привилегия <code class=\"kw\">let</code>; сигнатуры функций — это <b>договорный интерфейс</b> между модулями, и они обязаны читаться без заглядывания в тело.</p>",
            "<p>Функция без <code class=\"kw\">-&gt;</code> возвращает <code class=\"kw\">()</code> — и это факт, а не формальность: благодаря этому <code class=\"kw\">println!</code> свободно живёт как команда, и этим же объясняется, почему забытая точка с запятой иногда «меняет» то, что возвращает функция. Типы в сигнатурах громкие; тип возврата — не исключение.</p>",
            "<h3>Ранние возвраты и реле ?</h3>",
            "<p>Серединный <code class=\"kw\">return</code> совершенно легален — караулы наверху функции («если неверно — выйди сразу») держат главный путь плоским. А стрелка <code class=\"kw\">?</code>, которую ты встретишь в 8-й главе, — та же мысль, спрессованная: «верни ошибку наверх, если она есть, иначе распакуй успех».</p>",
            "<h3>Функции тоже значения</h3>",
            "<p><code class=\"kw\">fn</code> может жить в переменной и путешествовать аргументом: <code class=\"kw\">let f: fn(i32) -&gt; i32 = double;</code>. Это первый проблеск страны <b>замыканий</b>, которую итераторные главы изучат всерьёз, — но уже сейчас поведение можно передавать как данные, а это и есть колбэки и стратегии.</p>",
            "<pre><code>fn double(n: i32) -&gt; i32 { n * 2 }

fn apply(f: fn(i32) -&gt; i32, v: i32) -&gt; i32 {
    f(v)
}
</code></pre>",
            "<h3>Соберём вместе</h3>",
            "<p>Освоив эти руны, ты можешь композировать <i>что угодно</i>: заклинания <code class=\"kw\">fn</code> обмениваются вестями, <code class=\"kw\">match</code> направляет поток данных, <code class=\"kw\">for</code> обходит полки. Все последующие главы — ровно тот же танец с более пышными хорами: поля структур протекают сквозь руки match, потоки получают глаголы, файлы путешествуют по (?)-реле.</p>",
            "<p>Заметки стиля, которые стоит украсть: маленькие функции-<b>глаголы</b>, делающие одну вещь; неявный возврат последним выражением; короткие списки параметров — если функция просит пять аргументов, ей, вероятно, пора стать структурой. Ты-в-будущем, читающий этот код год спустя, скажет спасибо.</p>",
            "<p>Начни с наименее знакомой руны: у многих это <code class=\"kw\">match</code> с разбором груза. Печатай ответы вручную, ломай код разок — прочитай слова компилятора, пока следующий квест не надстроит этаж. Встретимся в Залах Структур.</p>",
            "<div class=\"article-banner\">Ветвления, образцы, циклы, заклинания — четыре шестерни любой растовской программы. Двенадцать квестов ждут, чтобы обточить их до мышечной памяти.</div>",
        ],
    },

    // C3 forms
    Article {
        chapter_id: "forms",
        lang: "ru",
        title: "Залы Структур и Перечислений",
        paragraphs: &[
            "<p>Скалярными рунами далеко не уйти. Реальные миры имеют форму: стражи с копьями и щитами, врата с дозволенными состояниями, реликвии, которые могут существовать, а могут — нет. Rust даёт <b>произведения типов</b> (структуры), <b>суммы типов</b> (перечисления) и особого посла отсутствия — <code class=\"kw\">Option&lt;T&gt;</code>. Эта глава — кузница, где сырые значения становятся <i>типами от себя</i>.</p>",
            "<h3>struct: формовочный штамп</h3>",
            "<p><code class=\"kw\">struct Guardian { spear: u16, shield: u16 }</code> отливает именованную группу полей. Читаем точкой, обновляем кусками синтаксисом <code class=\"kw\">..base</code> и — главное — прикрепляем поведение через блоки <code class=\"kw\">impl</code>. Приёмник <code class=\"kw\">&amp;self</code> держит метод только для чтения, <code class=\"kw\">&amp;mut self</code> разрешает мутацию, а голый <code class=\"kw\">self</code> потребляет значение (редко, но честно).</p>",
            "<pre><code>struct Guardian {\n    spear: u16,\n    shield: u16,\n}\n\nlet mut hero = Guardian { spear: 12, shield: 30 };\nhero.shield -= 5;      // доступ к полю через точку\n</code></pre>",
            "<h3>Кортежные структуры и newtype</h3>",
            "<p>Если у структуры нет именованных полей, получается <b>кортежная структура</b>: <code class=\"kw\">struct Meters(f64);</code>. С виду она скромна, но это один из самых сильных растовских приёмов — <i>паттерн newtype</i>. <code class=\"kw\">struct Meters(f64)</code> и <code class=\"kw\">struct Seconds(f64)</code> — разные типы, поэтому компилятор никогда не позволит сложить дистанцию с длительностью. Ноль рантайм-расходов, полная безопасность на компиляции.</p>",
            "<h3>derive: бесплатный набор инструментов</h3>",
            "<p>Поведение, которого заслуживает каждый тип, — печать, сравнение, копирование — не стоит писать вручную. Над структурой или enum можно поместить <code class=\"kw\">#[derive(Debug, Clone, PartialEq, Default)]</code>, и компилятор сочинит шаблонный код за тебя: <code class=\"tp\">Debug</code> открывает печать через <code class=\"tp\">{:?}</code>, <code class=\"kw\">Clone</code> даёт <code class=\"kw\">.clone()</code>, <code class=\"kw\">PartialEq</code> разрешает <code class=\"kw\">==</code> между значениями, <code class=\"kw\">Default</code> — <code class=\"tp\">Guardian::default()</code>. Квесты этой главы сильно опираются на эту строку.</p>",
            "<pre><code>#[derive(Debug, Clone, PartialEq, Default)]\nstruct Relic {\n    name: String,\n    power: u8,\n}\n\nlet r1 = Relic::default();\nlet r2 = r1.clone();\nprintln!(\"{:?}\", r1 == r2);   // true\n</code></pre>",
            "<h3>impl: методы и приёмники</h3>",
            "<p><code class=\"kw\">impl Guardian { … }</code> прикрепляет поведение к типу. Первый параметр — <b>приёмник</b> — решает, как методу можно трогать данные: <code class=\"kw\">&amp;self</code> читает, <code class=\"kw\">&amp;mut self</code> мутирует, <code class=\"kw\">self</code> потребляет значение целиком (редко, но честно — таковы превращения <code class=\"tp\">into_*</code>). Методы вызываются через точку: <code class=\"kw\">hero.sharpen()</code>.</p>",
            "<pre><code>struct Guardian { spear: u16, shield: u16 }\n\nimpl Guardian {\n    fn new(spear: u16, shield: u16) -&gt; Self {   // ассоциированная fn\n        Self { spear, shield }\n    }\n    fn total(&amp;self) -&gt; u16 { self.spear + self.shield }  // заём\n    fn buff(&amp;mut self) { self.spear += 1; }              // мутация\n}\n\nlet mut hero = Guardian::new(12, 30);\nhero.buff();\nprintln!(\"{}\", hero.total());    // 43\n</code></pre>",
            "<p>Отдельность <code class=\"kw\">impl</code> — вот почему в Rust нет скрытых конструкторов: новое создание — обычная ассоциированная функция <code class=\"kw\">Guardian::new()</code> — обрати внимание, приёмника нет, — а поведение растёт в отдельной палате — рядом с данными, а не впутанным в них.</p>",
            "<h3>Синтаксис обновления и частичные перемещения</h3>",
            "<p><code class=\"kw\">let hero2 = Guardian { spear: 40, ..hero };</code> забирает остальные поля из <code class=\"kw\">hero</code>. Осторожно: <code class=\"kw\">..hero</code> <b>перемещает</b> взятые поля — если структура хранит кучевый тип вроде <code class=\"kw\">String</code>, оригинал останется частично перемещённым и непригодным. На простых числах та же строка — сокращение полного копирования, ведь каждое поле <code class=\"kw\">Copy</code>.</p>",
            "<h3>enum: дозволенная сумма</h3>",
            "<p><code class=\"kw\">enum GateStatus { Locked, Guarded, Open }</code> декларирует конечный список состояний, и каждый вариант может нести собственный <b>груз</b>: <code class=\"kw\">Ready(i32)</code>, <code class=\"kw\">Failed(String)</code>, <code class=\"kw\">Open { key: u8 }</code>. Один тип — несколько обликов. Это сердце растовского моделирования: выразить «одно из возможных» прямо в системе типов, чтобы несуществующие состояния невозможно было построить.</p>",
            "<pre><code>enum Door {\n    Locked,\n    Guarded(Guardian),\n    Open { visits: u32 },\n}\n\nlet gate = Door::Guarded(hero);\n</code></pre>",
            "<p>Enum — читаемая <b>машина состояний</b>: существовать могут лишь объявленные состояния, переходы происходят через <code class=\"kw\">match</code>, и каждое состояние несёт ровно те данные, которые нужны. Смоделируешь документ как <code class=\"kw\">Draft | Ready(String) | Published { date: String }</code> — и «опубликованный черновик без даты» мгновенно станет невозможным: такое значение просто нельзя построить.</p>",
            "<h3>Option: отсутствие, сделанное явным</h3>",
            "<p>Большинство языков обозначают отсутствие призраками, которые безмолвно ускользают в арифметику (null). Rust зашивает его: <code class=\"kw\">Option&lt;T&gt;</code> — это либо <code class=\"kw\">Some(значение)</code>, либо <code class=\"kw\">None</code>, и компилятор <b>принуждает</b> признать оба исхода перед использованием значения. Каждый квест об Option упражняет три идиомы:</p>",
            "<ul><li><code class=\"kw\">if let Some(x) = relic { … }</code> — лёгкое условное вскрытие;</li><li><code class=\"kw\">match relic { Some(n) =&gt; …, None =&gt; … }</code> — полная церемония;</li><li><code class=\"kw\">relic.unwrap_or(0)</code> — быстрая запасная монета.</li></ul>",
            "<p>Раз отсутствие — значение, его можно преобразовывать, как любое значение. Семейство <b>комбинаторов</b> сокращает церемонию: <code class=\"kw\">relic.map(|r| r.power)</code> превращает <code class=\"kw\">Option&lt;Relic&gt;</code> в <code class=\"kw\">Option&lt;u8&gt;</code>; <code class=\"kw\">and_then</code> сцепляет следующий ненадёжный шаг; <code class=\"kw\">unwrap_or_else(|| fallback)</code> вычисляет запасное значение лишь по необходимости. Под капотом все они — <code class=\"kw\">match</code> в маскировке.</p>",
            "<pre><code>let relic: Option&lt;Relic&gt; = Some(Relic { name: \"🦀\".into(), power: 9 });\n\nlet power = relic\n    .map(|r| r.power)          // Option&lt;u8&gt;\n    .filter(|p| *p &gt; 5)        // оставить сильные\n    .unwrap_or(0);             // 9\n</code></pre>",
            "<p><code class=\"kw\">unwrap()</code> существует, но невежливо: на None паникует. В Саду относись к unwrap как к <i>черновику</i>: в примерах и быстрых экспериментах — можно, в настоящем коде предпочти <code class=\"kw\">unwrap_or</code>, <code class=\"kw\">?</code> или match. Глава 8 делает эту доктрину официальной.</p>",
            "<h3>Result: родной брат для провалов</h3>",
            "<p><code class=\"kw\">Option</code> отвечает на вопрос «есть ли оно?», но молчит, <i>почему</i> его нет. Его родной брат <code class=\"kw\">Result&lt;T, E&gt;</code> кладёт в руку <code class=\"kw\">Err</code> груз ошибки: <code class=\"kw\">Ok(значение)</code> или <code class=\"kw\">Err(причина)</code>. Официальная встреча состоится в главе 8 — но и здесь форма должна быть знакомой: это то же enum-мышление, применённое к провалам.</p>",
            "<h3>Делай невозможные состояния непредставимыми</h3>",
            "<p>Обе половины главы служат одной заповеди дизайна: <b>делай невозможные состояния непредставимыми</b>. Значение обязано быть положительным — бери <code class=\"kw\">u16</code> вместо проверок <code class=\"kw\">i32</code> на каждом шагу. Поле может отсутствовать — бери <code class=\"kw\">Option</code> вместо конвенции «пустая строка». Состояний ровно три — объявляй enum вместо трёх булевых. Компилятор тогда станет стражем домена, который работает ночами и в выходные бесплатно.</p>",
            "<h3>Зачем это всё?</h3>",
            "<p>Вглянься в любой настоящий Rust: <code class=\"tp\">Vec&lt;Item&gt;</code>, <code class=\"tp\">Option&lt;Config&gt;</code>, <code class=\"tp\">Result&lt;Reply, Error&gt;</code> — обобщённые оболочки над структурами и перечислениями. Не освоишь главу — итераторы, каналы и файлы будут читаться как шум; освоишь — станут прозрачным стеклом.</p>",
            "<p>Ритуал этого зала: смоделируй что-нибудь из своего дня как enum — заказ кофе, билет на поезд, состояние игры. Попроси компилятор отвергнуть каждый невозможный вариант, а затем преврати три своих структуры в newtype. Квесты ниже проведут тебя через struct, impl, Option и default по одному камню за раз.</p>",
            "<div class=\"article-banner\">Зал ждёт: отлей стража, запечатай три варианта врат, вскрой один реликт. Двери следующих глав откроются тише.</div>",
        ],
    },

    // C7 formatting
    Article {
        chapter_id: "formatting",
        lang: "ru",
        title: "Зеркало Display и Debug",
        paragraphs: &[
            "<p>Вывод — это придворный зал: значение выходит и говорит. Rust различает два тона самопредставления. <code class=\"kw\">Display</code> — <b>учтивое лицо</b> для конечного пользователя: <code class=\"kw\">println!(\"{}\", x)</code>. <code class=\"kw\">Debug</code> — <b>честный исповедник</b> для разработчика: <code class=\"kw\">println!(\"{:?}\", x)</code>.</p>",
            "<h3>Зачем два трейта?</h3>",
            "<p>Затем, что аудитории разные. Пользователь хочет <code class=\"tp\">4°C</code>, разработчик — <code class=\"tp\">Temperature { celsius: Some(4), scale: C }</code>. Показывать внутренности пользователю — утечка; прятать их от разработчика — пытка. Два трейта — и ссоры нет.</p>",
            "<h3>Семейство макросов: println!, eprintln!, format!, write!</h3>",
            "<p>Всё форматирование течёт через один и тот же двигатель — меняется лишь пункт назначения. <code class=\"kw\">println!</code> говорит в терминал с переносом строки, <code class=\"kw\">eprintln!</code> — в поток ошибок, <code class=\"kw\">format!</code> молча вручает <code class=\"kw\">String</code>, а <code class=\"kw\">write!</code> вливает текст во всё, что принимает formatter. Выучи синтаксис один раз — всё семейство подчинится.</p>",
            "<pre><code>let coin = 42;\n\nlet label = format!(\"coin = {}\", coin);   // строка, собранная в памяти\nprintln!(\"{label}\");                     // в stdout\neprintln!(\"warning: {label}\");           // в stderr\n</code></pre>",
            "<h3>Позиционные и именованные аргументы</h3>",
            "<p>Внутри фигурных скобок живёт свой грамматический язык: <code class=\"tp\">{}</code> берёт следующий аргумент, <code class=\"tp\">{0}</code>/<code class=\"tp\">{1}</code> обращаются к аргументам по позиции — одно значение может выступить дважды — а <code class=\"tp\">{name}</code> ловит переменную прямо из окружения. Инлайн-захват (<code class=\"kw\">println!(\"{x}\")</code>) — современный стиль Rust: меньше шума, меньше аргументов.</p>",
            "<pre><code>let (x, y) = (3, 7);\n\nprintln!(\"{0} + {1} = {0}\", x, y);  // 3 + 7 = 3\nprintln!(\"{x} сидит на {y}\");       // инлайн-захват\nprintln!(\"{:&gt;5}|\", x);             // ширина поля 5, по правому краю\n</code></pre>",
            "<p>Сами скобки тоже печатаются: экранируй их удвоением — <code class=\"tp\">{{</code> печатает <code class=\"tp\">{</code>, а <code class=\"tp\">}}</code> — <code class=\"tp\">}</code>. Так <code class=\"kw\">println!(\"{{}} = {{code}}\")</code> выведет <code class=\"tp\">{} = {code}</code>. Шаблоны, показывающие буквальные JSON или скобки, пользуются этим приёмом ежедневно.</p>",
            "<h3>Debug и магия derive</h3>",
            "<p>Отладочный вывод механичен, поэтому Rust разрешает его <i>наследовать</i>: <code class=\"kw\">#[derive(Debug)]</code> учит компилятор печатать каждое поле автоматически. Затем <code class=\"kw\">{:?}</code> приводит трейт в действие. Для глубокой анатомии бери <code class=\"kw\">{:#?}</code>: с отступами вложенные развалины превращаются в читаемые полки.</p>",
            "<pre><code>#[derive(Debug)]\nstruct Relic { name: String, power: u8 }\n\nlet vault = vec![\n    Relic { name: \"rune\".into(), power: 3 },\n    Relic { name: \"gem\".into(), power: 9 },\n];\n\nprintln!(\"{:?}\", vault);   // [Relic { name: \"rune\", power: 3 }, …]\nprintln!(\"{:#?}\", vault);  // красиво: по полю на строку\n</code></pre>",
            "<h3>Display против Debug: когда какой</h3>",
            "<p>Компас простой: <b>Debug — для тебя, Display — для них</b>. Debug — для логов, падений тестов, снимков состояний — всего, что читает разработчик; наследуй его везде (через контейнеры он комбинируется бесплатно). Display — для строк, которые увидит пользователь: строки интерфейса, отчётные таблицы, вывод CLI; пишется вручную, один раз, осознанно. Если типу нужны оба — дай оба: это разные контракты, а не соперники.</p>",
            "<h3>Display: твой собственный голос</h3>",
            "<p>Display наследовать нельзя — никто не угадает, как <i>ты</i> хочешь представиться. Реализуешь <code class=\"kw\">fmt::Display</code> один раз, и всякая <code class=\"kw\">{}</code> в любом месте чтит его. Инструмент один — <code class=\"kw\">write!(f, \"…\", части)</code>: макрос вливает форматированный текст в formatter, а ты возвращаешь <code class=\"tp\">fmt::Result</code>.</p>",
            "<pre><code>use std::fmt;\n\nstruct Temperature { celsius: f64 }\n\nimpl fmt::Display for Temperature {\n    fn fmt(&amp;self, f: &amp;mut fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {\n        write!(f, \"{}°C\", self.celsius)\n    }\n}\n</code></pre>",
            "<h3>Мини-язык: {:.2}, {:&gt;8}, {:08}</h3>",
            "<p>Внутри <code class=\"kw\">{}</code> живёт мини-язык спецификаторов: <code class=\"tp\">{:.2}</code> закрепляет знаки после запятой, <code class=\"tp\">{:&gt;8}</code> выравнивает по правому краю в поле ширины 8, <code class=\"tp\">{:&lt;8}</code> — по левому, <code class=\"tp\">{:08}</code> добивает число нулями до церемониальной ширины (лучшее для идентификаторов). Пять глифов — и твои логи выглядят как страницы профессиональной учётной книги.</p>",
            "<p>Поле может нести три лишние шестерни: <b>заполнитель и выравнивание</b> (<code class=\"tp\">{:&gt;8}</code>, <code class=\"tp\">{:&lt;8}</code>, <code class=\"tp\">{:^8}</code> — вправо, влево, по центру; символ заполнителя ставится перед стрелкой: <code class=\"tp\">{:*^8}</code> украшает звёздами), <b>знак</b> (<code class=\"tp\">{:+}</code> принудительно ставит + у положительных чисел) и <b>ширина с нулевым добивом</b> (<code class=\"tp\">{:08}</code> — идеально для идентификаторов вроде <code class=\"tp\">00042</code>).</p>",
            "<pre><code>println!(\"{:+}\", 5);      // +5\nprintln!(\"{:08}\", 42);    // 00000042\nprintln!(\"{:b}\", 9);      // 1001\nprintln!(\"{:x}\", 255);    // ff\nprintln!(\"{:o}\", 8);      // 10\nprintln!(\"{:e}\", 1234.5); // 1.2345e3\n</code></pre>",
            "<h3>Рекомендуемый круг тренировки</h3>",
            "<ul><li>Прогоняй квесты <code class=\"kw\">#[derive(Debug)]</code>, пока derive не станет дыханием;</li><li>Затем одна ручная реализация Display с <code class=\"kw\">write!</code>;</li><li>Финиш — квесты мини-языка: {:.2}, {:&gt;8}, {:08}.</li></ul>",
            "<p>Закрой главу маленькой привычкой: когда печатаешь промежуточное значение для отладки, однажды переключи его на <code class=\"tp\">{:#?}</code> и вслух прочитай его анатомию. Потом реализуй Display для одного своего типа — температура, зелье, чек — и больше никогда не клей строки вручную через <code class=\"kw\">+</code>. Форматирование — это язык; говори на нём намеренно.</p>",
            "<div class=\"article-banner\">После главы Зеркала твои логи перестают быть «случайными кракозябрами» и становятся <i>протоколом суда</i>.</div>",
        ],
    },

    // C9 pointers
    Article {
        chapter_id: "pointers",
        lang: "ru",
        title: "Дворец Ума: Умные указатели",
        paragraphs: &[
            "<p>Комнаты значений живут либо на <b>стеке</b> (дёшево, быстро, области видимости), либо на <b>куче</b> (большое, общее, долгоживущее). Владельцы управляют кучей напрямую — через <b>умные указатели</b>. Это спасатели Дворца Ума: каждый считает, делит или охраняет по своему строгому протоколу. Никто из них не отменяет владение — каждый лишь его <i>иное воплощение</i>.</p>",
            "<h3>Box: приватные кучевые апартаменты</h3>",
            "<p><code class=\"kw\">Box&lt;T&gt;</code> размещает <code class=\"kw\">T</code> на куче и оборачивает один указатель-владелец: одно значение, один хозяин, уничтожение в конце области. Обычные работы: громоздкие структуры, которым не место на стеке, рекурсивные типы, где размер должен быть известен при компиляции, и <b>трейт-объекты</b> — <code class=\"kw\">Box&lt;dyn Trait&gt;</code>: любой конкретный тип прячется за общим договором.</p>",
            "<pre><code>enum List {\n    Node(i32, Box&lt;List&gt;),\n    Empty,                 // рекурсия обрывается здесь\n}\n</code></pre>",
            "<p>Почему Box здесь обязателен? Рекурсивный тип иначе должен был бы знать собственный размер, который зависит от него самого, — бесконечный регресс, который компилятор отказывается угадывать. <code class=\"kw\">Box</code> разрывает круг: указатель — одно машинное слово, и размер <code class=\"kw\">List</code> снова вычислим. Это схема за <code class=\"kw\">Box&lt;Node&gt;</code> в связных списках, деревьях файлов и любом древовидном мире.</p>",
            "<p>Разыменование — звезда дворца: <code class=\"kw\">*ptr</code> прокапывается до самого значения. Box реализует <code class=\"kw\">Deref</code>, поэтому не нужно писать <code class=\"kw\">(*ptr).method()</code> — авто-разыменование сработает прямо в вызове метода.</p>",
            "<p>Анатомия памяти, которую стоит выучить наизусть: сам <code class=\"kw\">Box</code> сидит на стеке, шириной в один указатель; охраняемый им <code class=\"kw\">T</code> живёт на куче; когда Box покидает область, <code class=\"kw\">Drop</code> освобождает кучевую комнату автоматически. Никакого ручного <code class=\"kw\">free()</code>, никаких утечек — владелец и есть деструктор.</p>",
            "<h3>Box&lt;dyn Trait&gt;: собиратель договоров</h3>",
            "<p>Иногда коллекция обязана держать <i>разные</i> типы под одним договором. Конкретные типы различаются размером, поэтому контейнеру нужна единая форма — <b>трейт-объект</b>. <code class=\"kw\">Box&lt;dyn Beast&gt;</code> — толстый указатель: данные плюс vtable; динамическая диспетчеризация в рантайме решает, какой именно <code class=\"kw\">cry()</code> позвать. Цена — одно непрямое обращение; выгода — сменное поведение.</p>",
            "<pre><code>trait Beast { fn cry(&amp;self) -&gt; String; }\n\nstruct Crab;\nimpl Beast for Crab { fn cry(&amp;self) -&gt; String { \"🦀\".into() } }\n\nlet zoo: Vec&lt;Box&lt;dyn Beast&gt;&gt; = vec![Box::new(Crab)];\nfor b in &amp;zoo { println!(\"{}\", b.cry()); }\n</code></pre>",
            "<h3>Rc: реестр владельцев</h3>",
            "<p>Общий доступ на чтение между несколькими владельцами — при однопоточности — это работа <code class=\"kw\">Rc&lt;T&gt;</code> (reference counted). <code class=\"kw\">Rc::clone(&amp;scroll)</code> не копирует байты: лишь приращает счётчик владельцев; <code class=\"kw\">Rc::strong_count(&amp;scroll)</code> докладывает, сколько имён держат значение. Когда последнее имя уходит, данные уничтожаются.</p>",
            "<h3>Cell и RefCell: внутренняя мутация</h3>",
            "<p>Семейство <i>общих клеток</i> подгибает одно конкретное правило: мутацию через общую ручку. <code class=\"kw\">Cell&lt;T&gt;</code> обслуживает простые <code class=\"kw\">Copy</code>-значения — <code class=\"tp\">.set()</code> заменяет, <code class=\"tp\">.get()</code> читает, никаких заёмов вовсе. <code class=\"kw\">RefCell&lt;T&gt;</code> обслуживает всё остальное: она переносит книгу заёмов с компиляции в рантайм.</p>",
            "<pre><code>let s = Rc::new(RefCell::new(String::from(\"scroll\")));\n\nlet mut w1 = s.borrow_mut();\nw1.push_str(\"!\");\n\nlet r1 = s.borrow();   // ✗ паника: BorrowMutError\n</code></pre>",
            "<p><b>RefCell почти всегда живёт рядом с Rc</b>: Rc недоступен для внутренней мутации, так как неизменяемый реестр не может выдать <code class=\"kw\">&amp;mut</code> на владение. Клетка предоставляет мутацию с проверкой в рантайме: <code class=\"kw\">borrow()</code> (много читателей) и <code class=\"kw\">borrow_mut()</code> (ровно один писатель) — при нарушении правил она <b>паникует</b>.</p>",
            "<h3>Почему «проверка в рантайме» — дар, а не грех</h3>",
            "<p>Правила borrow checker'а великолепны, но статичны. Некоторые структуры (двусвязные списки, общие кэши, наблюдатели) не могут доказать свои договоры времени жизни на компиляции. RefCell переносит эту обязанность в рантайм и сохраняет дисциплину: нарушивший правило получит панику — детерминированную, отлаживаемую, со стековым следом.</p>",
            "<h3>Циклы и слабая лазейка Weak</h3>",
            "<p>У счётного владения есть честная слабость: два значения, указывающие друг на друга, никогда не досчитаются до нуля — и оба утекут навсегда. Лазейка — <code class=\"kw\">Weak&lt;T&gt;</code>, невладеющий собрат, создаваемый <code class=\"kw\">Rc::downgrade(&amp;rc)</code>. <code class=\"kw\">Weak</code> не удерживает значение живым; <code class=\"kw\">upgrade()</code> вежливо просит и возвращает <code class=\"kw\">Option&lt;Rc&lt;T&gt;&gt;</code>. Связи родитель → ребёнок держи на <code class=\"kw\">Rc</code>, связи ребёнок → родитель — на <code class=\"kw\">Weak</code>, и во дворце не останется призрачных комнат.</p>",
            "<h3>Выбор верного указателя</h3>",
            "<ul><li>Один владелец, нужна куча — <code class=\"kw\">Box&lt;T&gt;</code>;</li><li>Много владельцев, только чтение — <code class=\"kw\">Rc&lt;T&gt;</code>;</li><li>Много владельцев, внутренняя мутация — <code class=\"kw\">Rc&lt;RefCell&lt;T&gt;&gt;</code>;</li><li>Рядом потоки — <code class=\"kw\">Arc</code>, а для записи <code class=\"kw\">Mutex</code> (глава десятая).</li></ul>",
            "<h3>Врата в потоки</h3>",
            "<p>Rc + RefCell строго однопоточны: счётчик не атомарный. Глава «Потоки» поднимет реестр до <code class=\"kw\">Arc</code>, а ларец — до <code class=\"kw\">Mutex</code>: та же психология, новые двигатели. Сначала закрепи базу дворца: различие Box / Rc / RefCell — это азбука десятой главы.</p>",
            "<p>Ритуал тренировки: перепиши одно из прежних решений так, чтобы <code class=\"kw\">Vec</code> держал <code class=\"kw\">Box&lt;dyn Something&gt;</code>, и посмотри, как один список принимает три разных вида. Затем нарочно позаимствуй RefCell дважды на мутацию — прочитай панику вслух, найди строку, нарушившую правило одного писателя, и почини её. Указатели перестают пугать, когда их правила становятся личным опытом.</p>",
            "<div class=\"article-banner\">Эпитафия дворца: Box владеет, Rc считает владельцев, RefCell охраняет внутренние ревизии.</div>",
        ],
    },

    // C10 threads
    Article {
        chapter_id: "threads",
        lang: "ru",
        title: "Хор Потоков",
        paragraphs: &[
            "<p>Один голос поёт одну песню. Десять голосов поют <i>десять песен разом</i> — или топят друг друга в шуме. Знаковое достижение Rust — хор летит <b>без гонок данных</b>: бесстрашие обеспечивают типы, которые ты уже изучил.</p>",
            "<h3>Что такое гонка данных?</h3>",
            "<p>Три ингредиента: <b>два и более потока</b>, <b>одна общая ячейка памяти</b> и <b>хотя бы один писатель</b> без синхронизации между доступами. Порядок чтений и записей становится рулеткой планировщика — один и тот же бинарник выдаёт разные ответы от запуска к запуску, а баг прячется месяцами. Стратегия Rust структурна: убрать хотя бы один ингредиент ещё на компиляции, чтобы гонку было невозможно даже собрать.</p>",
            "<h3>spawn и move</h3>",
            "<p><code class=\"kw\">std::thread::spawn(|| …)</code> зажигает новый поток и запускает в нём замыкание. Замыкание захватывает значения окружения — и по умолчанию <b>заимствует</b> их, а поток может пережить заём! Слово <code class=\"kw\">move</code> заставляет замыкание <b>забрать владение</b> всем, что оно трогает: багаж едет вместе с караваном. Именно поэтому «использование после области» в безопасном Rust не случается в принципе.</p>",
            "<pre><code>use std::thread;\n\nlet lore = String::from(\"Ferris\");\n\nlet handle = thread::spawn(move || {\n    format!(\"songs of {lore}\")       // lore перемещён внутрь\n});\nprintln!(\"{}\", handle.join().unwrap());\n</code></pre>",
            "<h3>Scoped-потоки: заём без перемещения</h3>",
            "<p>Иногда <code class=\"kw\">move</code> расточителен: данные нужны лишь на время, взаймы. <code class=\"kw\">std::thread::scope</code> гарантирует — на компиляции, — что все spawned-потоки будут join’ены до конца области, поэтому замыкания могут <b>заимствовать</b> окружающие переменные безопасно. Это мост между строгостью <code class=\"kw\">move</code> и уютом обычных заёмов.</p>",
            "<pre><code>let mut plans = vec![\"seeds\", \"water\"];\n\nthread::scope(|s| {\n    s.spawn(|| {\n        plans.push(\"sun\");          // &amp;mut заём, безопасно внутри scope\n        println!(\"{}\", plans.len());\n    });\n    s.spawn(|| println!(\"ассистент в праздности\"));\n});                  // оба потока join’ены здесь\nprintln!(\"{plans:?}\");\n</code></pre>",
            "<h3>Send и Sync: пограничники</h3>",
            "<p>Два маркерных трейта охраняют границу потоков. <code class=\"kw\">Send</code> говорит, что значение можно <b>передать</b> другому потоку; <code class=\"kw\">Sync</code> — что общую ссылку <code class=\"kw\">&amp;T</code> можно <b>читать</b> из нескольких потоков разом. Почти все простые типы — и то и другое автоматически.</p>",
            "<p>Реестр открывает классику: <code class=\"kw\">Rc&lt;T&gt;</code> — <i>ни то, ни другое</i>: его счётчик не атомарен, и компилятор отказывается провозить его через границу. Зато <code class=\"kw\">Mutex&lt;T&gt;</code> делает Sync любой <code class=\"kw\">T</code>, а <code class=\"kw\">Arc&lt;T&gt;</code> — Send + Sync, когда таков сам <code class=\"kw\">T</code>. Безопасность здесь не рантайм-обещание, а доказательство на уровне типов.</p>",
            "<h3>join: ведение счёта</h3>",
            "<p><code class=\"kw\">handle.join()</code> дожидается финиша потока и возвращает результат в оболочке <code class=\"tp\">Result</code> — поток мог паникнуть. Ронять spawned-поток без join — отпускать скакуна без поводка: можно, но результата его бега ты не получишь. Присоединяй (join) те ручки, которые тебе дороги.</p>",
            "<h3>Arc: атомарный реестр</h3>",
            "<p>Разделить одно значение между несколькими потоками позволяет <code class=\"kw\">Arc&lt;T&gt;</code> — атомарно считаемый <b>Rc для потоков</b>. <code class=\"kw\">Arc::clone</code> остаётся дёшевым: лишь приращение атомарного счётчика; каждый поток получает собственную ручку от того же кучевого значения. Маркеры <code class=\"kw\">Send</code>/<code class=\"kw\">Sync</code> гарантируют, что через эти двери пройдут только потоко-безопасные данные.</p>",
            "<h3>Mutex: золотой ларец</h3>",
            "<p>Чтение параллельно безопасно; <b>запись</b> требует записи в очередь. <code class=\"kw\">Mutex&lt;T&gt;</code> — золотой ларец: <code class=\"kw\">lock()</code> возвращает стража — пока страж жив, дверь твоя; при <code class=\"kw\">drop</code> запирается снова. Эргономика: <code class=\"kw\">let mut gold = vault.lock().unwrap(); *gold += 100;</code>. <code class=\"kw\">unwrap()</code> разбирает (редкий) случай <i>отравленного замка</i>: другой паникнул внутри.</p>",
            "<p>Почему <code class=\"kw\">lock()</code> возвращает <code class=\"tp\">Result</code>? Из-за <i>отравления</i>: если поток паникует, держа стража, mutex помечается отравленным — охраняемое состояние может быть рассогласовано. <code class=\"kw\">.unwrap()</code> декларирует «паника где-то — баг везде» — прагматичный дефолт; альтернатива — изучить ошибку и попытаться спасти поломанное состояние.</p>",
            "<h3>RwLock: читальный зал</h3>",
            "<p><code class=\"kw\">RwLock&lt;T&gt;</code> шлифует золотой ларец: выдаёт <b>сколько угодно читательских стражей</b> или <b>ровно один писательский</b> — и никогда оба. Когда чтений впятеро больше записей, читальный зал обгоняет ларец с одной дверью. Эргономика та же, одна лишняя ручка; бери его, когда профилировщик покажет очередь читателей, а не раньше.</p>",
            "<h3>mpsc: почтовый коридор</h3>",
            "<p>Там, где mutex строит потоки в очередь, <b>каналы</b> позволяют голосам просто отправлять письма. <code class=\"kw\">mpsc::channel()</code> — <i>много отправителей, один получатель</i>: сколько угодно <code class=\"kw\">tx.send(msg)</code>, один <code class=\"kw\">rx.recv()</code>. Владение сообщением <b>путешествует</b> по каналу (снова move!), поэтому гонки структурно невозможны. Правило выбора: очереди — для состояния, каналы — для событий.</p>",
            "<p>Три привычки заставляют каналы петь. <b>Падение последнего отправителя закрывает коридор</b>: <code class=\"kw\">rx.recv()</code> тогда вернёт <code class=\"kw\">Err</code> — так работники узнают, что время уходить. <b>Клонирование отправителя</b> (<code class=\"kw\">tx.clone()</code>) регистрирует новых производителей — дёшево, без общего состояния. А идиома цикла <code class=\"kw\">while let Ok(msg) = rx.recv()</code> продолжает поглощать, пока почта не перестанет идти.</p>",
            "<pre><code>let (tx, rx) = mpsc::channel::&lt;String&gt;();\nlet rx = Arc::new(Mutex::new(rx));\n\nfor id in 0..3 {\n    let rx = Arc::clone(&amp;rx);\n    thread::spawn(move || loop {\n        match rx.lock().unwrap().recv() {\n            Ok(job) =&gt; println!(\"worker {id}: {job}\"),\n            Err(_) =&gt; break,        // канал закрыт: выходим\n        }\n    });\n}\nfor i in 0..6 { tx.send(format!(\"job #{i}\")).unwrap(); }\ndrop(tx);                            // закрыть коридор\n</code></pre>",
            "<p>Взгляд через забор напоследок: OS-<b>потоки</b> — тяжёлые солисты — каждый таит собственный стек, и планировщик ОС раздаёт им время принудительно. <b>async/await</b> — гигантский хор дешёвых задач, мультиплексируемый на нескольких потоках и ждущий кооперативно. Для CPU-работы и для обучения потоки идеальны; для десятков тысяч праздных сетевых соединений выигрывает async. Десятая глава нарочно учит первому — концепции переносятся.</p>",
            "<h3>Чек-лист хора</h3>",
            "<ul><li>Все spawn с <code class=\"kw\">move</code>? (Совет да любовь!)</li><li>Для каждого хэндла вызван join (или он честно отпущен)?</li><li>Общее состояние завернуто в <code class=\"kw\">Arc&lt;Mutex&lt;T&gt;&gt;</code>?</li><li>Поток событий идёт через каналы, а не через толظمут?</li></ul>",
            "<p>Ритуал тренировки: попробуй делить простой <code class=\"kw\">i32</code> между двумя spawned-потоками и дай компилятору отказать тебе — прочитай отказ дважды. Затем заверни его в <code class=\"kw\">Arc&lt;Mutex&lt;i32&gt;&gt;</code>, надень с трёх потоков и join’ни. Переход от отказа к аккорду — вся глава одним жестом.</p>",
            "<div class=\"article-banner\">Компилятор вырезает случайные гонки, Arc/Mutex вырезают ленивые. После этой главы конкурентность — хор, а не уличная драка.</div>",
        ],
    },

    // C4 collections
    Article {
        chapter_id: "collections",
        lang: "ru",
        title: "Хранилище Коллекций",
        paragraphs: &[
            "<p>Скаляры и структуры хранят <i>по одной</i> вещи за раз. Настоящие приключения держат <b>полки</b> вещей: библиотеку свитков, которая растёт и сжимается, реестр, отвечающий на вопрос «видел ли я этот ключ?». Эта глава открывает Хранилище: <code class=\"kw\">Vec&lt;T&gt;</code> — растущий массив, алхимия итераторов <code class=\"kw\">map</code>, <code class=\"kw\">filter</code> и <code class=\"kw\">collect</code>, и ключ-значение реестр <code class=\"kw\">HashMap&lt;K, V&gt;</code>. Коллекции — место, где владение становится <i>практикой</i>: полка владеет содержимым, а заимствование решает, кто к нему прикасается.</p>",
            "<h3>Vec: растущая полка</h3>",
            "<p><code class=\"kw\">Vec&lt;T&gt;</code> — непрерывный кучевый массив, владеющий каждым своим элементом. Он знает три числа: capacity (зарезервированный простор), length (использованный простор) и указатель на кучевый буфер. Когда простор кончается, Vec <b>перевыделяет память</b>: аллоцируется буфер побольше, каждый элемент переезжает в него, старый освобождается. Поэтому существует <code class=\"kw\">Vec::with_capacity(n)</code> — зарезервируй один раз, отбрось суету.</p>",
            "<p>Два обряда создания. <code class=\"kw\">Vec::new()</code> начинает пустым и выводит типы из употребления; <code class=\"kw\">vec![1, 2, 3]</code> отчеканивает вектор со значениями сразу — сигил, который каждый растовод набирает тысячу раз. Оба компилируются в одно и то же; макрос — просто сахар с хорошими манерами.</p>",
            "<pre><code>let mut shelf: Vec&lt;String&gt; = Vec::new();
shelf.push(\"Memory Avengers\".to_string());
let scrolls = vec![1, 2, 3];          // [i32; 3] внутри Vec&lt;i32&gt;
</code></pre>",
            "<p><code class=\"kw\">push</code> добавляет в конец (амортизированно O(1)), <code class=\"kw\">pop</code> снимает и <b>возвращает</b> последний элемент как <code class=\"kw\">Option&lt;T&gt;</code> — <code class=\"kw\">Some(значение)</code>, если на полке что-то лежало, и <code class=\"kw\">None</code>, если она была пуста. Option-возврат — доктрина коллекций в миниатюре: «может не быть» закодировано в типе, а не в null. Заметь: pop требует <code class=\"kw\">mut</code> — снятие это мутация.</p>",
            "<pre><code>let mut stack = vec![10, 20, 30];
let top = stack.pop();      // Some(30)
stack.push(40);             // [10, 20, 40]
let len = stack.len();      // 3
</code></pre>",
            "<p><code class=\"kw\">len()</code> — бесплатный взгляд: Vec всегда знает свою длину, поэтому запрос ничего не стоит. Индексация <code class=\"kw\">shelf[0]</code> читает первый том и <b>паникует</b>, если индекс за границей — Rust проверяет каждый индекс в рантайме, никакого тихого мусора. Для проверенного чтения есть <code class=\"kw\">shelf.get(2)</code>, возвращающий <code class=\"kw\">Option&lt;&amp;T&gt;</code>: вежливые <code class=\"kw\">Some</code>/<code class=\"kw\">None</code> вместо краха. Правило: индексируй, когда границы уже доказаны; <code class=\"kw\">get()</code> — когда ещё нет.</p>",
            "<h3>Заём полки: классическая ловушка</h3>",
            "<p>Вот столкновение, ради которого строили borrow checker: <code class=\"kw\">let first = &amp;shelf[0];</code> берёт читающий заём одного элемента, а <code class=\"kw\">shelf.push(x)</code> требует <code class=\"kw\">&amp;mut</code> всей полки — потому что push может <b>перевыделить память</b> и обнулить все живые ссылки. Держать <code class=\"kw\">&amp;shelf[0]</code> поперёк <code class=\"kw\">push</code> — ошибка компиляции (E0502). Скопируй значение или перезаими после мутации — компилятор тебя проведёт.</p>",
            "<pre><code>let mut shelf = vec![1, 2, 3];
let first = shelf[0];       // скопируй значение — заём не держится
shelf.push(4);              // ок: полку никто не занимает
println!(\"{first}\");
</code></pre>",
            "<h3>Итераторы: сборочная линия</h3>",
            "<p>Каждая коллекция выдаёт <b>итератор</b> — конвейер, отдающий элементы по одному. <code class=\"kw\">vec.iter()</code> отдаёт <code class=\"kw\">&amp;T</code> (чтение), <code class=\"kw\">vec.iter_mut()</code> — <code class=\"kw\">&amp;mut T</code> (переписывание), а <code class=\"kw\">vec.into_iter()</code> — владеющие <code class=\"kw\">T</code> (потребляет Vec). Магия в том, что итераторы <i>ленивы</i>: пока никто не тянет — ничего не происходит. Адаптеры вроде <code class=\"kw\">map</code> и <code class=\"kw\">filter</code> лишь оборачивают конвейер; запускает машину только <b>потребитель</b> — <code class=\"kw\">collect</code>, <code class=\"kw\">sum</code>, <code class=\"kw\">for</code>.</p>",
            "<p><code class=\"kw\">map</code> преобразует каждый элемент: <code class=\"kw\">nums.iter().map(|n| n * 2)</code> — новый конвейер, выдающий удвоенные значения. <code class=\"kw\">filter</code> процеживает: <code class=\"kw\">nums.iter().filter(|&amp;&amp;n| n % 2 == 0)</code> оставляет чётные (заметь двойной <code class=\"kw\">&amp;</code> — замыкание получает ссылку на ссылку, которую отдаёт итератор; один паттерн разыменования — выучить один раз). Соединяй свободно: <code class=\"kw\">filter</code>, затем <code class=\"kw\">map</code> читается как фраза и не стоит ничего лишнего — промежуточные векторы не строятся.</p>",
            "<pre><code>let nums = vec![1, 2, 3, 4, 5, 6];
let evens_doubled: Vec&lt;i32&gt; = nums
    .iter()
    .filter(|&amp;&amp;n| n % 2 == 0)
    .map(|n| n * 2)
    .collect();               // [4, 8, 12]
</code></pre>",
            "<p><code class=\"kw\">sum()</code> сливает конвейер в один итог: <code class=\"kw\">nums.iter().sum::&lt;i32&gt;()</code>. Турборыба <code class=\"tp\">::&lt;i32&gt;</code> называет тип, потому что у пустого конвейера неизвестных целых нет очевидного типа суммы — компилятор отказывается угадывать. Есть и <code class=\"kw\">fold(init, |acc, x| …)</code> — бабушка всех свёрток, когда сведение не простое сложение.</p>",
            "<p><code class=\"kw\">collect()</code> — табличка назначения на сборочной линии: он вливает поток в новый контейнер. <b>Целевой тип выбирает форму</b>: <code class=\"kw\">collect::&lt;Vec&lt;i32&gt;&gt;()</code> строит вектор, <code class=\"kw\">collect::&lt;String&gt;()</code> склеивает символы, <code class=\"kw\">collect::&lt;HashSet&lt;_&gt;&gt;()</code> дедуплицирует во множество. Чаще всего аннотация едет на переменной: <code class=\"kw\">let squared: Vec&lt;i32&gt; = …collect();</code> — объяви пункт назначения и дай конвейеру его наполнить.</p>",
            "<h3>HashMap: реестр</h3>",
            "<p><code class=\"kw\">HashMap&lt;K, V&gt;</code> отображает ключи в значения с почти O(1) поиском. Ключи должны быть хэшируемыми, а <code class=\"kw\">insert</code> живёт по правилам владения: <code class=\"kw\">map.insert(String::from(\"mana\"), 90)</code> <b>перемещает</b> и ключ, и значение в карту — карта становится их владельцем. Поэтому HashMap со ключами <code class=\"kw\">String</code> встречается постоянно: реестр владеет своими ярлыками.</p>",
            "<pre><code>use std::collections::HashMap;

let mut mana = HashMap::new();
mana.insert(String::from(\"fire\"), 40);
mana.insert(String::from(\"frost\"), 60);
let fire = mana.get(\"fire\");         // Some(&amp;40) — заём, не копия
</code></pre>",
            "<p>Чтение — это заём: <code class=\"kw\">get(\"fire\")</code> возвращает <code class=\"kw\">Option&lt;&amp;V&gt;</code> — ссылку внутрь карты, упакованную в Option, потому что ключа может не быть. У записи свой этикет: простой <code class=\"kw\">insert</code> <b>перезаписывает</b> и возвращает старое значение; <code class=\"kw\">entry(key).or_insert(0)</code> — это upsert: «вставь, только если нет, и отдай мне запись на изменение». API entry — колыбель счётчиков и агрегаторов:</p>",
            "<pre><code>let mut counts: HashMap&lt;String, i32&gt; = HashMap::new();
for word in [\"garden\", \"garden\", \"rust\"] {
    *counts.entry(word.to_string()).or_insert(0) += 1;
}
// {\"garden\": 2, \"rust\": 1}
</code></pre>",
            "<p>Ещё две привычки реестра. Итерация отдаёт кортежи: <code class=\"kw\">for (key, value) in &amp;mana { … }</code> — порядок <i>не определён</i> (у хэш-карты нет рядов, по которым можно пройтись по порядку; нужен сортированный вывод — собери ключи и отсортируй). А есть <code class=\"kw\">HashSet&lt;T&gt;</code> — HashMap, притворяющаяся, что значений нет: идеален для проверки «видел ли я это?».</p>",
            "<h3>Какой контейнер когда?</h3>",
            "<ul><li>Упорядоченный список, индексация, стековое поведение — <code class=\"kw\">Vec&lt;T&gt;</code> (дефолтный выбор в 90% случаев).</li><li>Поиск по ключу, счётчики, дедупликация с данными — <code class=\"kw\">HashMap&lt;K, V&gt;</code>.</li><li>Только проверка наличия, без данных — <code class=\"kw\">HashSet&lt;T&gt;</code>.</li><li>Фиксированная длина известна на компиляции — простой массив <code class=\"tp\">[T; N]</code> живёт на стеке.</li></ul>",
            "<h3>Владельческая книга коллекций</h3>",
            "<p>Коллекции делают владение <b>зримым</b>. Push перемещает значение внутрь; pop — наружу; итерация выбирает между заимствованием (<code class=\"kw\">iter</code>) и потреблением (<code class=\"kw\">into_iter</code>). Когда компилятор ругается на цикл по коллекции, переведи его жалобу в эту книгу — и решение обычно выбирает само: клонируй немногое, что нужно хранить, заими то, что только читаешь, и оставь коллекции владеть остальным.</p>",
            "<p>Ритуал тренировки: возьми любое прежнее решение квеста и перепиши его тремя способами — через <code class=\"kw\">for</code> с индексом, через <code class=\"kw\">while let Some(x) = vec.pop()</code> и полным конвейером <code class=\"kw\">map</code>/<code class=\"kw\">filter</code>/<code class=\"kw\">collect</code>. Тот же вывод, три диалекта. Конвейерный — тот, к которому первым потянутся старшие растоводы; теперь ты знаешь почему.</p>",
            "<div class=\"article-banner\">Правило Хранилища: коллекции владеют содержимым, итераторы выдают его напрокат, а collect() — таможня, где заимствованные потоки снова становятся владеющими контейнерами.</div>",
        ],
    },

    // C8 errors
    Article {
        chapter_id: "errors",
        lang: "ru",
        title: "Испытания Result и Ошибок",
        paragraphs: &[
            "<p>Языки прячут свои неудачи за исключениями или null'ами; Rust вручает их тебе в коробке с ярлыком. Эта глава проводит Испытания: <code class=\"kw\">Result&lt;T, E&gt;</code> — честный тип возврата, суд <code class=\"kw\">match</code>, скоростные выходы (<code class=\"kw\">unwrap</code>, <code class=\"kw\">expect</code>, <code class=\"kw\">unwrap_or</code>),оператора <code class=\"kw\">?</code>, <code class=\"kw\">map_err</code> для перевода обвинений и <code class=\"kw\">std::io</code> — место, где неудачи встречают тебя ежедневно.</p>",
            "<h3>Result: честный вердикт</h3>",
            "<p><code class=\"kw\">Result&lt;T, E&gt;</code> — перечисление ровно с двумя руками: <code class=\"kw\">Ok(значение)</code> — операция удалась, вот товар — и <code class=\"kw\">Err(ошибка)</code> — провалилась, вот обвинительное заключение. В отличие от исключений, провал <b>в сигнатуре типа</b>: вызывающий не может притвориться, что не заметил. Компилятор принуждает к решению, прежде чем любое значение <code class=\"kw\">T</code> попадёт тебе в руки.</p>",
            "<pre><code>enum Result&lt;T, E&gt; {\n    Ok(T),\n    Err(E),\n}\n\nfn parse_level(text: &amp;str) -&gt; Result&lt;i32, std::num::ParseIntError&gt; {\n    text.parse::&lt;i32&gt;()   // может упасть: тип говорит об этом заранее\n}\n</code></pre>",
            "<h3>match: суд</h3>",
            "<p>Честный способ разобрать Result — <code class=\"kw\">match</code>: обе руки обязательны, поэтому ошибка не может ускользнуть нерассмотренной. Внутри рук payload твой — паттерн <code class=\"tp\">Ok(x)</code> связывает значение, <code class=\"tp\">Err(e)</code> связывает ошибку. Это форма без церемоний, и именно её стандартная библиотека показывает первой.</p>",
            "<pre><code>match parse_level(\"42\") {\n    Ok(level) =&gt; println!(\"welcome, level {level}\"),\n    Err(e) =&gt; println!(\"свиток повреждён: {e}\"),\n}\n</code></pre>",
            "<h3>unwrap и expect: скоростные выходы</h3>",
            "<p><code class=\"kw\">unwrap()</code> говорит: «я уже знаю, что это Ok — отдай значение», а если ты ошибся — <b>паникует</b>, роняя поток с дефолтным сообщением. <code class=\"kw\">expect(\"контекст\")</code> — та же ставка, но с показаниями свидетеля: твой текст печатается при панике, превращая загадочный краш в хлебные крошки. Оба законны в тестах, прототипах и демо; в настоящем потоке управления это расписки, которые следующий читатель обязан погасить.</p>",
            "<pre><code>let good = parse_level(\"42\").unwrap();        // паника, если Err\nlet better = parse_level(\"42\").expect(\"свиток уровня должен парситься\");\n// паника: свиток уровня должен парситься: invalid digit found in string\n</code></pre>",
            "<h3>Оператор ?: эстафетная палочка</h3>",
            "<p>Внутри функции, которая сама возвращает Result, <code class=\"kw\">?</code> — вежливая стенография суда: «если Ok — отдай значение и идём дальше; если Err — <b>верни ошибку из моей функции прямо сейчас</b>». Проброс в один глиф — ошибка сама поднимается по стеку вызовов, нетронутая, пока кто-то не решится её по-настоящему судить. Цепочка <code class=\"kw\">?</code> читается как прямая дорога.</p>",
            "<pre><code>fn read_level(path: &amp;str) -&gt; Result&lt;i32, std::io::Error&gt; {\n    let text = std::fs::read_to_string(path)?;   // Err? вернуть сейчас\n    let level = text.trim().parse::&lt;i32&gt;().ok().ok_or(std::io::Error::new(\n        std::io::ErrorKind::InvalidData, \"bad level\"))?;\n    Ok(level)\n}\n</code></pre>",
            "<h3>unwrap_or: дефолт пристава</h3>",
            "<p>Иногда ошибка не стоит суда: представление продолжается с дефолтом. <code class=\"kw\">unwrap_or(fallback)</code> подставляет fallback при Err; <code class=\"kw\">unwrap_or_else(|| …)</code> строит его лениво, только когда нужен; <code class=\"kw\">unwrap_or_default()</code> спрашивает у типа его нулевое значение. Эти трое превращают «может упасть» в «падает вежливо» — идеальны для чтения конфигов, кэш-проверок, всего, у чего есть разумный дефолт.</p>",
            "<pre><code>let level = parse_level(user_input).unwrap_or(1);      // дефолт 1\nlet level2 = parse_level(user_input).unwrap_or_else(|e| {\n    println!(\"warn: {e}\");\n    1\n});\n</code></pre>",
            "<h3>map_err: перевод обвинений</h3>",
            "<p>Когда слои говорят на разных диалектах ошибок, <code class=\"kw\">map_err</code> переписывает руку Err, не трогая Ok: <code class=\"kw\">result.map_err(|e| MyError::from(e))</code>. Это адаптер между ошибкой твоего домена и библиотечной ошибкой, которую вернул вызываемый, — одно замыкание, ноль церемоний. Тихий собрат <code class=\"kw\">map</code> делает то же для руки Ok: преображает значение успеха на месте.</p>",
            "<pre><code>let level: Result&lt;i32, String&gt; =\n    parse_level(\"7x\").map_err(|e| format!(\"bad scroll: {e}\"));\n</code></pre>",
            "<h3>std::io: где неудачи живут ежедневно</h3>",
            "<p>Операции файловой системы и ввода возвращают <code class=\"kw\">io::Result&lt;T&gt;</code> — попросту <code class=\"kw\">Result&lt;T, io::Error&gt;</code> — потому что файлов может не быть, диски заполняются, а права кусаются. Испытания в квестах этой главы — вся та же возня с <code class=\"kw\">io::Error</code> в маскараде: заглядывай в <code class=\"kw\">ErrorKind</code> (NotFound? PermissionDenied?), когда нужно реагировать по-разному, или просто отдавай наверх через <code class=\"kw\">?</code>, когда не нужно.</p>",
            "<pre><code>use std::fs;\n\nmatch fs::read_to_string(\"garden.log\") {\n    Ok(text) =&gt; println!(\"{} байтов сплетен\", text.len()),\n    Err(e) if e.kind() == std::io::ErrorKind::NotFound =&gt; {\n        println!(\"лога ещё нет — первый визит?\");\n    }\n    Err(e) =&gt; return Err(e),   // всё прочее: наверх\n}\n</code></pre>",
            "<h3>Этикет неудачи</h3>",
            "<ul><li>Библиотечный код: <b>никогда</b> не паникуй на ожидаемых провалах — возвращай <code class=\"kw\">Result</code> и дай вызывающему судить.</li><li>Тесты и скрипты: <code class=\"kw\">unwrap</code>/<code class=\"kw\">expect</code> уместны — паника <i>и есть</i> громкое падение теста.</li><li>Верхний уровень (main, обработчики запросов): логируй и деградируй, или пробрасывай через <code class=\"kw\">?</code> в обработчик, отвечающий вежливо.</li><li>Выбор дефолта — это решение: <code class=\"kw\">unwrap_or</code> делает его видимым, <code class=\"kw\">unwrap</code> — крахом.</li></ul>",
            "<p>Философия одним выдохом: у Rust паники — для <b>багов</b> (сломанные инварианты, невосстановимые состояния), а Result — для <b>погоды</b> (нет файла, плохой ввод, полный диск). Смешать их — первородный грех; Испытания учат различать их рефлекторно.</p>",
            "<p>Ритуал тренировки: возьми любой прежний квест с <code class=\"kw\">unwrap</code> и перепиши его тремя способами — полный <code class=\"kw\">match</code>, цепочка <code class=\"kw\">?</code> с собственным типом ошибки, и дефолт через <code class=\"kw\">unwrap_or</code>. То же поведение, три аудитории: суд, эстафета, пожимание плечами.</p>",
            "<div class=\"article-banner\">Правило Испытаний: провал — часть сигнатуры; Ok и Err — оба легитимные ответы, ? передаёт эстафету, а unwrap — ставка, за которую придётся отчитаться.</div>",
        ],
    },

    // C11 files
    Article {
        chapter_id: "files",
        lang: "ru",
        title: "Архив Файлов",
        paragraphs: &[
            "<p>Всё, что ты писал до сих пор, умирало вместе с процессом: переменные испаряются, Vec растворяются, куча выметается дочиста. Архив — место, где данные наконец <b>переживают</b> программу — полки файлов, живущие между запусками. Эта глава проводит экскурсию по крылу <code class=\"kw\">std::fs</code>: цельное чтение через <code class=\"kw\">read_to_string</code>, брат записи <code class=\"kw\">fs::write</code>, тип-ручка <code class=\"kw\">File</code>, терпеливое буферное чтение с <code class=\"kw\">BufReader</code>, обмер через <code class=\"kw\">metadata</code> и стирающий обряд <code class=\"kw\">remove_file</code>.</p>",
            "<h3>Ритуал цельного чтения: fs::read_to_string</h3>",
            "<p>Простейшее заклинание: дай путь — получи весь файл как <code class=\"kw\">String</code> — или <code class=\"kw\">io::Error</code> в <code class=\"kw\">Result</code>. Одна аллокация, одно чтение, ноль церемоний; идеально для конфигов, посевных данных и квестовых свитков. Помни о двух спрятанных режимах провала: файл может отсутствовать (NotFound), а байты могут не быть валидным UTF-8 (InvalidData) — оба приходят как Err.</p>",
            "<pre><code>use std::fs;\n\nlet scroll = fs::read_to_string(\"garden.txt\")?;\nprintln!(\"{}\", scroll);   // весь файл, один глоток\n</code></pre>",
            "<h3>Брат записи: fs::write</h3>",
            "<p><code class=\"kw\">fs::write(path, bytes)</code> — зеркальный обряд: он <b>создаёт или обрезает</b> файл и вливает весь буфер. Обрезка важна — существующий файл молча опустошается прежде, чем новые байты приземлятся; мягкого добавления здесь нет (для добавления открывают File с append-опциями, ниже). Принимает всё, что читается как байты — <code class=\"tp\">&amp;str</code> годится.</p>",
            "<pre><code>use std::fs;\n\nfs::write(\"garden.txt\", \"посажено: 3 папоротника\\n\")?;   // создать или перезаписать\n</code></pre>",
            "<h3>File: ручка</h3>",
            "<p><code class=\"kw\">File</code> — не содержимое; это <b>ручка</b>: открытый проход к файлу на диске, хранящий курсор и права. <code class=\"kw\">File::open(path)</code> открывает только для чтения (Err NotFound, если файла нет); <code class=\"kw\">File::create(path)</code> открывает (или создаёт) для записи и обрезает. Ручка закрывается сама при drop — забывать close() не нужно — но незакрытая ручка всё же держит ресурсы ОС, так что держи область видимости узкой.</p>",
            "<pre><code>use std::fs::File;\n\nlet mut ledger = File::create(\"ledger.txt\")?;   // свежий, обрезанный\n</code></pre>",
            "<p>Через ручку пишут точно: <code class=\"kw\">write_all(bytes)</code> толкает весь буфер (простой <code class=\"kw\">write</code> может записать частично — циклы для усердных, <code class=\"kw\">write_all</code> для всех остальных). Несколько вызовов <code class=\"kw\">write_all</code> добавляют к тому, куда дошёл курсор, — так файл строят по кускам.</p>",
            "<pre><code>use std::io::Write;\n\nlet mut ledger = File::create(\"ledger.txt\")?;\nledger.write_all(b\"день 1: посажены папоротники\\n\")?;\nledger.write_all(b\"день 2: папоротники процветают\\n\")?;\n</code></pre>",
            "<h3>BufReader: терпеливый читатель</h3>",
            "<p>Небуферизованные чтения дёргают ОС на каждый запрос — дорого в мелких глотках. <code class=\"kw\">BufReader</code> укутывает любого читателя в уютный буфер: один большой syscall наполняет кладовую, затем строки подаются из памяти. Оберни <code class=\"kw\">File</code> и итерируй <code class=\"kw\">lines()</code> — каждая строка приходит как <code class=\"kw\">io::Result&lt;String&gt;</code> (перевод строки отрезан). Это рабочая лошадка разбора логов и CSV-подобных файлов.</p>",
            "<pre><code>use std::fs::File;\nuse std::io::{self, BufRead};\n\nlet reader = BufReader::new(File::open(\"garden.log\")?);\nfor line in reader.lines() {\n    let line = line?;\n    println!(\"строка: {line}\");\n}\n</code></pre>",
            "<p>Для интерактивного режима — по полстроки: подсказка, затем ответ посетителя — у того же читателя есть <code class=\"kw\">read_line(&amp;mut buf)</code>: наполняет твой String одной строкой <b>вместе</b> с переводом строки и возвращает, сколько байтов прочитано (0 в конце потока). После — trim; перевод строки — часть урожая.</p>",
            "<pre><code>use std::io::BufRead;\n\nlet mut name = String::new();\nlet bytes = reader.read_line(&amp;mut name)?;   // включая '\\n'\nlet name = name.trim().to_string();         // вежливая версия\n</code></pre>",
            "<h3>metadata: обмерный обряд</h3>",
            "<p><code class=\"kw\">fs::metadata(path)</code> возвращает досье, не открывая файл: <code class=\"kw\">len()</code> — размер в байтах, <code class=\"kw\">is_file()</code>/<code class=\"kw\">is_dir()</code> — что это за полка, времена изменения — когда трогали в последний раз. Сначала досье — потом сюрпризы не страшны: пропускай пустые файлы, отказывай директориям, переодетым в файлы, или просто показывай аккуратный размер в интерфейсе.</p>",
            "<pre><code>use std::fs;\n\nlet meta = fs::metadata(\"garden.txt\")?;\nprintln!(\"{} байтов, файл? {}\", meta.len(), meta.is_file());\n</code></pre>",
            "<h3>Стирающий обряд: remove_file</h3>",
            "<p><code class=\"kw\">fs::remove_file(path)</code> снимает файл с полки — навсегда: ни корзины, ни отмены, и при отсутствии файла или наличии директории вежливо падает (Err). Архив обязан пользователю заботой: это единственный обряд, где неверный путь стоит настоящих данных, поэтому этикет ниже — не украшение.</p>",
            "<pre><code>use std::fs;\n\nif fs::metadata(\"tmp_scroll.txt\").is_ok() {\n    fs::remove_file(\"tmp_scroll.txt\")?;   // исчез, второго шанса нет\n}\n</code></pre>",
            "<h3>Этикет Архива</h3>",
            "<ul><li>Работа с файлом целиком — сначала <code class=\"kw\">fs::read_to_string</code>/<code class=\"kw\">fs::write</code>, ручки потом.</li><li>Построчно или огромные файлы — <code class=\"kw\">BufReader</code>/<code class=\"kw\">lines()</code>; никогда не вливай лог на 4 ГБ в String.</li><li>Добавление — открывай с <code class=\"kw\">OpenOptions::new().append(true)</code>; <code class=\"kw\">fs::write</code> не умеет добавлять.</li><li>Каждый обряд возвращает <code class=\"kw\">io::Result</code> — глава Испытаний уже пройдена; пользуйся <code class=\"kw\">?</code> и match-ем по <code class=\"kw\">ErrorKind</code>, когда это важно.</li><li>Удаление навсегда — проверяй перед <code class=\"kw\">remove_file</code> и никогда не удаляй по пути, собранному из пользовательского ввода без проверки.</li></ul>",
            "<p>Архив замыкает круг, который весь сад строил с начала: владение научило, кто отвечает за данные, Испытания — что делать, когда реальность сопротивляется, а файлы — место, где твои данные наконец выходят из ОЗУ и переживают ночь. Программа, умеющая читать, писать и стирать ответственно, — уже не игрушка; это маленький вежливый гражданин диска.</p>",
            "<div class=\"article-banner\">Правило Архива: read_to_string и fs::write — обряды одного глотка, File — ручка, которую несёшь с собой, BufReader — терпение в обёртке, metadata — досье, а remove_file — единственный обряд, который дважды перепроверяют.</div>",
        ],
    },

    // C5 strings
    Article {
        chapter_id: "strings",
        lang: "ru",
        title: "Свитки Строк",
        paragraphs: &[
            "<p>Числа честные; текст — <i>хитрый</i>. Число — фиксированный размер на стеке, а строка — история, которая может расти, сжиматься, сшиваться и переписываться. Эта глава разворачивает Свитки: доктрина двух строк — <code class=\"kw\">String</code> и <code class=\"tp\">&amp;str</code>, рост текста через <code class=\"kw\">push_str</code>, композиция через <code class=\"kw\">format!</code> и чистящий набор — <code class=\"kw\">trim</code>, <code class=\"kw\">to_uppercase</code>, <code class=\"kw\">chars</code>, <code class=\"kw\">split</code>, <code class=\"kw\">replace</code>. Освоишь — и текст перестанет быть загадкой.</p>",
            "<h3>Две строки: владеющий свиток и заимствованный взгляд</h3>",
            "<p><code class=\"kw\">String</code> — <b>владеющий</b>, растущий, кучевый текст — свиток, который ты держишь в хранилище. <code class=\"tp\">&amp;str</code> («срез строки») — <b>заимствованный взгляд</b> на текст, живущий в другом месте — закладка, указывающая на свиток. Литерал вроде <code class=\"tp\">\"garden\"</code> — это <code class=\"tp\">&amp;str</code>: символы вшиты в память программы, а литерал — лишь окно в них. Поэтому <code class=\"tp\">&amp;str</code> не может расти: у взгляда нет собственного места, куда расти.</p>",
            "<pre><code>let owned: String = String::from(\"fairy dust\");  // кучевый свиток
let view: &amp;str = \"fairy dust\";                   // литерал = &amp;str
let borrowed: &amp;str = &amp;owned;                     // взгляд на свиток
</code></pre>",
            "<p>Мост между мирами: <code class=\"kw\">to_string()</code> отчеканивает свежий владеющий <code class=\"kw\">String</code> из любого заимствованного взгляда, а <code class=\"kw\">String::from(\"…\")</code> — старший брат, делающий то же. Оба <b>копируют символы</b> на новый кучевый буфер — взгляд продолжает указывать на старое место, String владеет новым. Обратно — бесплатно: <code class=\"tp\">&amp;some_string</code> разыменовывается в <code class=\"tp\">&amp;str</code> (deref-коэрция), поэтому функции, принимающие <code class=\"tp\">&amp;str</code>, радостно принимают и <code class=\"tp\">&amp;String</code>.</p>",
            "<pre><code>let view = \"garden\";
let word = view.to_string();          // &amp;str -&gt; String (копия)
let magic = String::from(\"sparkle\");  // тот же обряд, старая заклинательная форма
</code></pre>",
            "<h3>Рост свитка: push_str и push</h3>",
            "<p><code class=\"kw\">String</code> растёт как Vec (он и <i>есть</i> Vec байтов в маскарадном костюме): <code class=\"kw\">push_str(text)</code> добавляет срез целиком, <code class=\"kw\">push('c')</code> — один символ. Оба требуют <code class=\"kw\">mut</code> — рост это мутация. Буфер может перевыделиться за кулисами; свиток просто становится длиннее, а любой взгляд, взятый <b>до</b> push, обнуляется (полочная ловушка из Хранилища — снова).</p>",
            "<pre><code>let mut spell = String::from(\"abra\");
spell.push_str(\"cadabra\");   // добавить срез
spell.push('!');             // добавить один символ
// spell == \"abracadabra!\"
</code></pre>",
            "<h3>format!: композитор</h3>",
            "<p><code class=\"kw\">format!</code> — вежливый способ строить текст: берёт шаблон и значения, <b>отчеканивает совершенно новый String</b> — ни заёмных драк, ни мутации входов. Позиционные слоты <code class=\"tp\">{}</code> consume-ят аргументы по порядку; инлайновые захваты вроде <code class=\"tp\">{name}</code> хватают переменные по имени прямо из области видимости. Тот же мини-язык питает <code class=\"kw\">println!</code> — выучи один раз, пользуйся везде.</p>",
            "<pre><code>let name = \"Ferris\";
let level = 7;
let banner = format!(\"{} reached level {}!\", name, level);
let banner2 = format!(\"{name} reached {level}!\");  // инлайновые захваты
</code></pre>",
            "<h3>Чистящий набор: trim и регистр</h3>",
            "<p><code class=\"kw\">trim()</code> сбривает пробельные символы (пробелы, табы, переводы строк) с <b>обоих концов</b> и возвращает <code class=\"tp\">&amp;str</code> — суженный взгляд в тот же свиток, так что стоит ничего. Середину он не трогает: <code class=\"tp\">\"a  b\".trim()</code> сохраняет двойной пробел. Идеально для дезинфекции пользовательского ввода перед вынесением вердикта. Регистр идёт с другой стороны: <code class=\"kw\">to_uppercase()</code>/<code class=\"kw\">to_lowercase()</code> — не взгляды, они могут менять длину в байтах (İ — два байта!), поэтому отчеканивают <b>новый String</b>.</p>",
            "<pre><code>let raw = \"  RustGarden  \";
let clean = raw.trim();           // \"RustGarden\" (&amp;str-взгляд)
let loud = clean.to_uppercase();  // \"RUSTGARDEN\" (новый String)
</code></pre>",
            "<h3>chars: конвейер глифов</h3>",
            "<p>String — это байты (UTF-8), а глиф может занимать 1–4 байта — поэтому индексация <code class=\"tp\">s[0]</code> запрещена: байт 0 не обязательно буква. Чтобы идти по тексту <i>логически</i>, есть <code class=\"kw\">chars()</code>: он выдаёт <code class=\"kw\">char</code>'ы по одному. <code class=\"kw\">nth(n)</code> подглядывает n-й глиф (Option — может закончиться), <code class=\"kw\">count()</code> считает глифы — <i>не</i> байты, а <code class=\"kw\">len()</code> у String считает <b>байты</b>. Кириллица и хангыль делают разницу наглядной: «краб» — 4 глифа, но 8 байтов.</p>",
            "<pre><code>let runes = \"crab\";
for c in runes.chars() { /* 'c', 'r', 'a', 'b' */ }
let third = runes.chars().nth(2);   // Some('a')
let glyph_count = runes.chars().count();  // 4, не байты
</code></pre>",
            "<h3>split: распарыватель швов</h3>",
            "<p><code class=\"kw\">split(sep)</code> разрезает взгляд на куски вдоль разделителя и выдаёт ленивый итератор <code class=\"tp\">&amp;str</code>-взглядов — собери их в <code class=\"kw\">Vec&lt;&amp;str&gt;</code>, когда нужна полка. <code class=\"kw\">split_whitespace()</code> — двоюродный брат без CSV: режет по любой серии пробельных и никогда не выдаёт пустых кусков. Куски — <b>взгляды</b>, не копии — collect-и в <code class=\"kw\">String</code>, только если нужен владеющий текст.</p>",
            "<pre><code>let scroll = \"mana,spell,garden\";
let parts: Vec&lt;&amp;str&gt; = scroll.split(',').collect();
// [\"mana\", \"spell\", \"garden\"]

let words: Vec&lt;&amp;str&gt; = \"a b  c\".split_whitespace().collect();
// [\"a\", \"b\", \"c\"]
</code></pre>",
            "<p><code class=\"kw\">replace(from, to)</code> — переписыватель чернил: каждое вхождение <code class=\"tp\">from</code> становится <code class=\"tp\">to</code>, а результат — <b>свежий String</b> — исходный свиток никогда не трогается. Свободно соединяй с trim или lowercase; каждый шаг цепочки, производящий String, передаёт владение следующему.</p>",
            "<pre><code>let toast = \"old spell\";
let fresh = toast.replace(\"old\", \"new\");  // \"new spell\" (новый String)
// toast по-прежнему читает \"old spell\": replace не правит на месте
</code></pre>",
            "<h3>Какой строковый тип когда?</h3>",
            "<ul><li>Хранение, рост, владение текстом — <code class=\"kw\">String</code>.</li><li>Параметры функций, которые только читают — принимай <code class=\"tp\">&amp;str</code> (вызывающий передаст и литерал, и <code class=\"tp\">&amp;String</code>).</li><li>Композиция из частей — <code class=\"kw\">format!</code> (отчеканивает новый String).</li><li>Счёт видимых глифов — <code class=\"kw\">chars().count()</code>; счёт хранения — <code class=\"kw\">len()</code>.</li><li>Разрезание текста — <code class=\"kw\">split</code>/<code class=\"kw\">split_whitespace</code> + <code class=\"kw\">collect</code>.</li></ul>",
            "<h3>Владельческая книга текста</h3>",
            "<p>Доктрина двух строк — просто глава о владении в текстовом костюме. <code class=\"kw\">String::from</code> и <code class=\"kw\">to_string</code> <b>копируют в новое владение</b>; <code class=\"tp\">&amp;str</code> <b>заимствует</b>; передача String по значению <b>перемещает</b> её; приём <code class=\"tp\">&amp;String</code> по значению оставляет вызывающего с пустыми руками. Когда компилятор ругается на текстовую головоломку, спроси книгу: кто владеет этими символами, а кто лишь смотрит?</p>",
            "<p>Ритуал тренировки: возьми предложение, обрежь его, смени регистр, разрежь на слова, собери обратно <code class=\"kw\">format!</code>'ом в одну строку с номерами — и считай глифы на каждом этапе. Каждый метод, который ты используешь, — одна из рун этой главы; после одного прохода они перестают быть API-тривиальностью и становятся мастерской.</p>",
            "<div class=\"article-banner\">Правило Свитков: &amp;str — заимствованный взгляд, String — владеющий свиток; trim и split дают взгляды напрокат, to_uppercase, format! и replace отчеканивают свежие свитки.</div>",
        ],
    },

    // C6 traits
    Article {
        chapter_id: "traits",
        lang: "ru",
        title: "Мастерская Трейтов и Обобщений",
        paragraphs: &[
            "<p>До сих пор каждая структура стояла одиноко: <code class=\"kw\">struct</code> знала свои поля, и этим исчерпывалась её биография. Мастерская учит структуры <b>обещать</b>. <code class=\"kw\">trait</code> — договор способностей: «всё, что подписано здесь, можно клонировать, сравнивать, печатать», а обобщения — способ мастерской писать один чертёж на много материалов. Вместе они позволяют Rust быть строгим <i>и</i> гибким одновременно.</p>",
            "<h3>Договор: объявление трейта</h3>",
            "<p>Трейт перечисляет <b>сигнатуры</b> методов без тел: обещания, не доказательства. Тип подписывает договор блоком <code class=\"kw\">impl TraitFor Type</code>, наполняющим их реальными телами. После подписи методы типа вызываются через точку, а сам тип годится везде, где требуют трейт.</p>",
            "<pre><code>trait Enchantable {\n    fn power(&amp;self) -&gt; i32;              // обещание, без тела\n    fn describe(&amp;self) -&gt; String {       // тело по умолчанию — можно\n        format!(\"power: {}\", self.power())\n    }\n}\n\nstruct Wand { charges: i32 }\n\nimpl Enchantable for Wand {\n    fn power(&amp;self) -&gt; i32 { self.charges }   // доказательство\n}\n</code></pre>",
            "<p>Заметь два темпа обещания: <code class=\"kw\">power</code> без тела — каждый подписант <b>обязан</b> его написать; <code class=\"kw\">describe</code> везёт <b>тело по умолчанию</b> — подписанты получают его бесплатно (и могут переопределить). Методы по умолчанию — способ держать трейты маленькими, но полезными: горстка обязательных обещаний плюс набор подарков.</p>",
            "<h3>impl Trait: сантехника на договорах</h3>",
            "<p>Там, где пишется тип, <code class=\"kw\">impl TraitName</code> говорит: «любой тип, подписавший этот договор». Как <b>параметр</b> он принимает всех подписантов; как <b>тип возврата</b> обещает некоего подписанта, пряча точный тип (один конкретный тип на функцию — вернуть разные типы условно можно лишь через <code class=\"kw\">dyn</code>, ниже). Это генерики со скрытой программой.</p>",
            "<pre><code>fn announce(item: &amp;impl Enchantable) -&gt; String {\n    item.describe()   // годится для Wand, Ring, любого подписанта\n}\n</code></pre>",
            "<h3>Clone и Copy: договоры дубликатора</h3>",
            "<p><code class=\"kw\">Clone</code> обещает явный, видимый дубликат: <code class=\"kw\">x.clone()</code> — настоящий вызов метода, который видно в ревью. <code class=\"kw\">Copy</code> обещает бесплатный, беззвучный дубликат при каждом присваивании — позволен только стековым типам (целые, плавающие, bool, char; и кортежи/массивы из них). Каждый <code class=\"kw\">Copy</code>-тип — <code class=\"kw\">Clone</code>, но не наоборот: Copy — строгий и тихий брат. Вздохи облегчения borrow checker'а сотканы из этих двоих.</p>",
            "<pre><code>#[derive(Clone)]\nstruct Spell { name: String, mana: i32 }\n\nlet a = Spell { name: \"frost\".into(), mana: 30 };\nlet b = a.clone();     // явный дубликат; `a` всё ещё владеет текстом\nlet n = 7;\nlet m = n;             // i32 — Copy: беззвучно, <code>n</code> жив</code></pre>",
            "<h3>PartialEq и Eq: договор сравнения</h3>",
            "<p><code class=\"kw\">PartialEq</code> обещает, что операторы <code class=\"tp\">==</code>/<code class=\"tp\">!=</code> работают (его метод <code class=\"kw\">eq</code> — то, во что опускается <code class=\"tp\">==</code>). «Частичное» — потому что некоторые типы содержат значения, отказывающиеся от равенства: <code class=\"tp\">f32::NAN == NAN</code> ложно. <code class=\"kw\">Eq</code> — строгая пометка: «каждое значение сравнивается честно» — целые её получают, плавающие нет. Большинство структур просто пишут <code class=\"kw\">#[derive(PartialEq, Eq)]</code> — и сравнение по полям генерируется само.</p>",
            "<pre><code>#[derive(PartialEq, Eq)]\nstruct Rune { glyph: char, tier: i32 }\n\nlet a = Rune { glyph: 'α', tier: 2 };\nlet b = Rune { glyph: 'α', tier: 2 };\nlet same = a == b;     // true — поле за полем\n</code></pre>",
            "<h3>Обобщения: буква T</h3>",
            "<p>Обобщённая функция ставит <code class=\"tp\">&lt;T&gt;</code> перед именем и говорит о <code class=\"kw\">T</code> как о «некоем типе, который выберет вызывающий». В исходнике ничего не дублируется, но компилятор совершает <b>мономорфизацию</b>: для каждого реально использованного T он отчеканивает специализованную копию функции на этапе компиляции — генерики в Rust бесплатны в рантайме. Расплата: голый <code class=\"kw\">T</code> не умеет <i>ничего</i> — чтобы звать методы, T должен подписать трейт: <code class=\"tp\">&lt;T: Enchantable&gt;</code>.</p>",
            "<pre><code>fn biggest&lt;T: PartialOrd&gt;(list: &amp;[T]) -&gt; Option&lt;&amp;T&gt; {\n    if list.is_empty() { return None; }\n    let mut best = &amp;list[0];\n    for item in &amp;list[1..] {\n        if item &gt; best { best = item; }\n    }\n    Some(best)\n}\n</code></pre>",
            "<p>Обобщённые <b>структуры</b> носят ту же букву на имени: <code class=\"tp\">Pair&lt;T&gt;</code> держит два T, <code class=\"tp\">Pair&lt;K, V&gt;</code> — ключ и значение двух разных типов — именно так написаны <code class=\"kw\">Vec&lt;T&gt;</code> и <code class=\"kw\">HashMap&lt;K, V&gt;</code> из Хранилища. Один чертёж, много материалов, ни дублирования кода в репозитории, ни динамической диспетчеризации в рантайме.</p>",
            "<pre><code>struct Pair&lt;T&gt; { left: T, right: T }\n\nlet glyphs = Pair { left: 'α', right: 'ω' };\nlet levels = Pair { left: 3, right: 9 };   // тот же чертёж, другой T\n</code></pre>",
            "<h3>Display: печатный договор</h3>",
            "<p><code class=\"tp\">{}</code> в <code class=\"kw\">println!</code>/<code class=\"kw\">format!</code> требует <code class=\"kw\">std::fmt::Display</code> — человекочитаемый вывод. Он не автоматический: подписываешь сам, записывая в форматтер через <code class=\"kw\">write!</code>. Сигнатура выглядит церемониально и стоит медленного прочтения: метод, заимствованный у форматтера, обязан вернуть <code class=\"kw\">fmt::Result</code>.</p>",
            "<pre><code>use std::fmt;\n\nimpl fmt::Display for Wand {\n    fn fmt(&amp;self, f: &amp;mut fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {\n        write!(f, \"Wand({} charges)\", self.charges)\n    }\n}\n// println!(\"{}\", wand) теперь говорит вежливо\n</code></pre>",
            "<h3>dyn Trait: пропуск за кулисы</h3>",
            "<p>Генерики и <code class=\"kw\">impl Trait</code> разрешаются <b>на этапе компиляции</b> (статическая диспетчеризация: точный метод вшивается). <code class=\"kw\">dyn Trait</code> — рантайм-форма: <i>трейт-объект</i> — некий подписант, решённый позже, через vtable. Цена — указатель плюс косвенность; зато единственный <code class=\"kw\">Vec&lt;Box&lt;dyn Enchantable&gt;&gt;</code> держит палочки, кольца и посохи рядом. Правило: генерики по умолчанию, <code class=\"kw\">dyn</code> — когда набор типов по-настоящему открыт.</p>",
            "<pre><code>let shelf: Vec&lt;Box&lt;dyn Enchantable&gt;&gt; = vec![\n    Box::new(Wand { charges: 3 }),\n    Box::new(Staff { runes: 9 }),\n];\nfor item in &amp;shelf {\n    println!(\"{}\", item.describe());  // вызов через vtable в рантайме\n}\n</code></pre>",
            "<h3>where: вежливая книга ограничений</h3>",
            "<p>Когда список <code class=\"tp\">&lt;&gt;</code> тяжелеет — два обобщения, у каждого по два ограничения — сигнатура тонет. <code class=\"kw\">where</code> уносит все ограничения в читаемую книгу после сигнатуры, смысл идентичен. Те же правила, более чистый свиток; большие настоящие сигнатуры почти всегда так пишут.</p>",
            "<pre><code>fn archive&lt;T, U&gt;(item: T, ledger: U)\nwhere\n    T: Enchantable + Clone,\n    U: fmt::Display,\n{\n    // ограничения живут здесь, читаются с одного взгляда\n}\n</code></pre>",
            "<h3>Какой инструмент когда?</h3>",
            "<ul><li>Общее поведение многих типов — объяви <code class=\"kw\">trait</code>, подпиши через <code class=\"kw\">impl … for</code>.</li><li>«Любой подписант, точный тип скрыт» — <code class=\"kw\">impl Trait</code> в аргументе/возврате.</li><li>Дублирование значений — <code class=\"kw\">Clone</code> (явно) или <code class=\"kw\">Copy</code> (беззвучно, стековые).</li><li>Сравнение значений — <code class=\"kw\">PartialEq</code> (плюс <code class=\"kw\">Eq</code> для честного полного равенства).</li><li>Один чертёж, много типов, нулевая цена рантайма — генерики <code class=\"tp\">&lt;T&gt;</code>.</li><li>Сборная солянка подписантов — <code class=\"kw\">Box&lt;dyn Trait&gt;</code>.</li></ul>",
            "<p>Тихая тайна Мастерской: каждая суперсила стандартной библиотеки, которой ты пользовался, — <code class=\"kw\">.clone()</code>, <code class=\"tp\">==</code>, печать через <code class=\"tp\">{}</code>, <code class=\"kw\">Vec&lt;T&gt;</code> — всё это время была трейтом или обобщением. Ты подписывал чужие договоры с первой главы; теперь договоры писать тебе.</p>",
            "<div class=\"article-banner\">Правило Мастерской: трейты — обещания, impl-блоки — доказательства, обобщения — чертежи, отчеканенные на компиляции, а dyn — пропуск за кулисы на случай открытого списка гостей.</div>",
        ],
    },


    // =====================================================================
    // KO — Korean editions
    // =====================================================================
    // C1 basics
    Article {
        chapter_id: "basics",
        lang: "ko",
        title: "메모리 정원의 기초",
        paragraphs: &[
            "<p>여행자여, 환영합니다. 이 챕터는 <b>Ferris Garden</b>의 모든 것이 자라나는 흙입니다. Rust는 <i>가비지 컬렉터 없는 메모리 안전성</i>을 약속하며 이름을 얻었고, 그 약속은 런타임이 아니라 <b>컴파일 시간</b>에 지켜집니다: 컴파일러는 누수, use-after-free, 이중 해제가 일어날 수 있는 프로그램을 그냥 빌드하지 않습니다. 스레드의 용과 싸우고 구조체의 성을 쌓기 전에, 당신의 손은 세 가지 겸손한 도구를 익혀야 합니다: 키워드, 식의 타입, 그리고 '불변이 기본' 사고방식입니다.</p>",
            "<h3>기본은 불변</h3>",
            "<p>대부분의 언어에서 변수는 마음껏 채울 수 있는 그릇입니다. Rust는 규칙을 뒤집습니다: <code class=\"kw\">let</code>은 이름과 값을 <b>불변으로</b> 묶습니다. 재대입은 컴파일 오류입니다 — <code class=\"kw\">cannot assign twice to immutable variable</code>. 왜일까요? 불변성은 버그 가족 전체를 태어나기 전에 없앱니다: 우발적인 덮어쓰기도, 공유 참조를 통한 기습 뮤테이션도 없고, 바인딩이 변하지 않음을 아는 컴파일러는 더 대담하게 최적화할 수 있습니다.</p>",
            "<p>뮤테이션은 금지되지 않았습니다 — <b>명시적</b>일 뿐입니다. 키워드 <code class=\"kw\">mut</code>는 의도를 외칩니다: '이 자리는 변할 것이다'. 규율은 엄격합니다: 모든 가변 바인딩은 신중한 선택이며, 눈에 띄는 키워드를 값으로 칩니다.</p>",
            "<h3>모든 것은 식이다</h3>",
            "<p>Rust는 <i>식 지향</i> 언어입니다: 거의 모든 것이 값을 계산합니다. <code class=\"kw\">let</code> 문은 값을 이름에 묶고, 블록 <code class=\"tp\">{ … }</code>은 마지막 식으로 평가되며, <code class=\"kw\">if</code>와 <code class=\"kw\">match</code>조차 값을 돌려줍니다. 세미콜론 <code class=\"tp\">;</code>이 붙은 문은 그 값을 버리고 유닛 타입 <code class=\"kw\">()</code>으로 평가됩니다 — Rust식 정중한 '아무것도 없음'입니다.</p>",
            "<pre><code>let harvest = {
    let seeds = 3;
    seeds * 4          // 세미콜론 없음: 이것이 블록의 값
};                     // harvest == 12
</code></pre>",
            "<p>초심자의 고전 함정이 바로 그 세미콜론에 숨어 있습니다: 함수 본문의 마지막 줄 뒤에 <code class=\"tp\">;</code>를 붙이면 반환 타입이 몰래 <code class=\"kw\">()</code>가 되고, 컴파일러는 '타입 불일치'를 불평합니다. 오류가 언뜻 이해되지 않으면 먼저 낯선 세미콜론을 찾아보세요 — 정원에서 가장 흔한 유령입니다.</p>",
            "<h3>기본 타입 첫 여행</h3>",
            "<p>정수는 부호 있는 가족과 없는 가족, 각 여덟 개의 폭으로 옵니다: <code class=\"kw\">i8</code>/<code class=\"kw\">u8</code>부터 <code class=\"kw\">i128</code>/<code class=\"kw\">u128</code>까지, 그리고 인덱싱에 쓰이는 기계 크기의 <code class=\"kw\">isize</code>/<code class=\"kw\">usize</code>. 기본값은 <code class=\"kw\">i32</code> — 문맥이 더 나은 것을 알려주지 않을 때 컴파일러가 돌아가는 만능 선택입니다.</p>",
            "<p>부동 소수점은 두 폭입니다: <code class=\"kw\">f32</code>와 <code class=\"kw\">f64</code>(기본값, 그리고 95%의 경우 옳은 선택). <code class=\"kw\">bool</code>은 엄격히 <code class=\"kw\">true</code> 또는 <code class=\"kw\">false</code>입니다 — truthy 숫자는 못 들어옵니다. <code class=\"kw\">char</code>는 바이트가 아닙니다: 완전한 <b>유니코드 스칼라</b>라서 <code class=\"tp\">'Ф'</code>, <code class=\"tp\">'한'</code>, <code class=\"tp\">'🦀'</code> 모두 합법적인 한 글자입니다. 그리고 빈 튜플 <code class=\"kw\">()</code>('유닛')은 '의미 있는 값이 없음'을 뜻하는 타입입니다.</p>",
            "<pre><code>let level: u8 = 7;          // 0..=255
let price: f64 = 12.5;      // 기본 float
let awake: bool = true;     // 엄격히 true | false
let crab: char = '🦀';      // 완전한 유니코드 스칼라, 4바이트
</code></pre>",
            "<h3>섀도잉 대 뮤테이션</h3>",
            "<p>초심자는 <i>섀도잉</i>과 <i>뮤테이션</i>을 혼동합니다. 섀도잉은 아무것도 바꾸지 않습니다: <code class=\"kw\">let y = y + 1;</code>은 완전히 새로운 바인딩을 만들고, 옛 것은 남아 있되 가려질 뿐입니다. 타입까지 바꿀 수 있습니다: 문자열이 숫자로, 숫자가 불리언으로. 데이터 흐름은 선형이고 감사 가능합니다: 변환마다 자기 선언을 가집니다. 뮤테이션은 같은 셀을 그 자리에서 다시 씁니다.</p>",
            "<p>섀도잉은 값이 단계를 거칠 때 빛을 발합니다: 문자열을 숫자로 파싱하고 같은 이름을 유지하거나, 단위를 그 자리에서 바꾸거나, 루프 변수를 더 다루기 편한 타입으로 다시 씁니다. 단계마다 새 바인딩이므로 컴파일러는 각 단계의 정확한 타입을 기억하고, 코드는 일기처럼 읽힙니다 — 변환 한 줄에 기록 한 줄.</p>",
            "<h3>소유권의 세 황금 규칙</h3>",
            "<ul><li>1. 모든 값은 항상 정확히 하나의 <b>소유자</b>를 가집니다.</li><li>2. 소유자가 스코프를 떠나면 값은 <b>drop</b>됩니다(메모리가 자동으로 해제, GC 없음).</li><li>3. 대입과 전달은 값을 <b>이동</b>시킵니다; 옛 이름은 쓸 수 없게 됩니다.</li></ul>",
            "<p>이동 의미론은 초심자를 겁주고 베테랑을 기쁘게 합니다. <code class=\"kw\">String</code>은 바이트를 힙에 담습니다; 조용히 복사하면 use-after-free 버그가 우글우글 생기므로, Rust는 당신이 묻지 않으면 복사를 거부합니다. 진짜 병렬 복사가 필요하면 <code class=\"kw\">.clone()</code>을 부르고 복제된 모든 바이트 값을 치릅니다. 같은 데이터를 여럿이 읽고 싶다면? 곧 <code class=\"kw\">&amp;</code> 참조를 만나게 됩니다.</p>",
            "<p>이동은 대입, 함수 인자, 반환마다 일어납니다 — <code class=\"kw\">String</code>만이 아닙니다. 값을 값으로 받는 함수는 새 소유자가 되고, 호출자의 이름은 호출 지점에서 죽습니다. 값을 돌려주면 소유권도 돌아옵니다. 이것이 Rust 데이터 흐름의 '층층 계약' 전부이며, 컴파일러가 기계적으로 집행합니다 — 런타임 경비견이 필요 없습니다.</p>",
            "<h3>Copy 대 Clone</h3>",
            "<p>작고 단순한 값(<code class=\"kw\">i32</code>, <code class=\"kw\">f64</code>, <code class=\"kw\">bool</code>, <code class=\"kw\">char</code>, 그런 타입들의 튜플)은 특별한 <code class=\"kw\">Copy</code> 마커를 가집니다: 대입이 조용히 복제합니다 — 비트 복사로는 메모리 버그가 생길 수 없기 때문입니다. 힙 기반의 모든 것은 명시적 <code class=\"kw\">.clone()</code>이 필요합니다: 그 잠깐의 멈춤이 정직함의 값입니다 — 비싼 복제가 어디서 일어나는지 정확히 보입니다.</p>",
            "<pre><code>let n = 42;
let m = n;          // Copy: n은 여전히 살아 있음
let s = String::from(\"Ferris\");
let t = s.clone();  // 명시적: s도 살아 있음
// let u = s;       // 이라면 이동(MOVE) — s는 여기서 죽음
</code></pre>",
            "<h3>대여와 Borrow Checker</h3>",
            "<p>참조는 빌린 땅의 임시 열쇠입니다. 교리는 <b>앨리어싱 XOR 뮤테이션</b>: 읽는 자는 몇 명이든(<code class=\"kw\">&amp;T</code>) 쓰는 자는 정확히 한 명(<code class=\"kw\">&amp;mut T</code>), 둘이 함께일 수는 없습니다. 이 한 규칙이 데이터 경합을 뿌리부터 녹입니다 — 스레드 챕터가 이것에 크게 기댑니다.</p>",
            "<pre><code>let mut mana = String::from(\"full\");
let r1 = &amp;mana;
mana.push('?');      // ✗ E0502: r1이 아래에서 아직 읽힘
println!(\"{r1}\");    // r1의 마지막 사용
// 위 두 줄을 맞바꾸면 코드가 컴파일됩니다!
</code></pre>",
            "<p>현대의 Rust는 시간표를 부드럽게 만듭니다: <b>Non-Lexical Lifetimes</b> 덕분에 대여는 닫는 중괄호가 아니라 <i>마지막 사용</i> 지점에서 끝납니다. <code class=\"kw\">println!</code>을 <code class=\"kw\">push</code> 위로 올리면 코드가 컴파일됩니다 — 컴파일러는 중괄호 모양의 범위가 아니라 실제 사용을 추적합니다. 체커와 하루 이틀 다투고 나면 규율이 새장이 아니라 안전 난간처럼 느껴지기 시작합니다.</p>",
            "<h3>슬라이스: 복제 대신 훔쳐보기</h3>",
            "<p>마지막 조각은 <code class=\"kw\">&amp;str</code> — 바이트 창 <code class=\"tp\">(ptr, len)</code>을 향한 <i>뚱뚱한 포인터</i>입니다. 슬라이스는 힙을 건드리지 않고 데이터의 일부를 읽습니다: 할당 0, 수명 검사는 만점. 이것이 이터레이터 챕터의 입장권입니다: 먼저 들여다보고, 복제는 필요할 때만.</p>",
            "<p>슬라이스는 경계 검사 습관도 길러 줍니다: 끝을 넘는 인덱스는 쓰레기를 읽는 대신 패닉합니다(<code class=\"kw\">index out of bounds</code>). 실패할 수 있는 접근은 <code class=\"kw\">Option&lt;T&gt;</code>를 돌려주는 <code class=\"kw\">.get(i)</code>를 택하세요 — 구조체 챕터가 이 패턴을 매일의 반사신경으로 만들어 줄 겁니다.</p>",
            "<h3>문자열: &amp;str 대 String</h3>",
            "<p>두 문자열 타입은 모두를 정확히 한 번 혼란스럽게 합니다. <code class=\"kw\">String</code>은 <b>소유한</b> 힙 버퍼 — 키우고, 줄이고, 다시 쓸 수 있습니다. <code class=\"kw\">&amp;str</code>은 남의 바이트를 빌려 보는 <b>빌린</b> 시점 — 문자열 리터럴이 고전적 사례입니다: 코드 속 <code class=\"tp\">\"Ferris\"</code>는 바이너리에 구워진 <code class=\"kw\">&amp;'static str</code>입니다. 읽기만 하는 함수엔 <code class=\"kw\">&amp;str</code>을, 소유하고 뮤테이션하는 자리엔 <code class=\"kw\">String</code>을 쓰세요.</p>",
            "<h3>튜플, 배열, 구조 분해</h3>",
            "<p>튜플 <code class=\"kw\">(T, U)</code>는 섞인 타입의 고정 묶음이고, 배열 <code class=\"tp\">[T; N]</code>은 같은 타입의 값 정확히 <code class=\"kw\">N</code>개를 스택에 바로 담습니다. 둘 다 아름답게 분해됩니다: <code class=\"kw\">let (a, b) = pair;</code>나 <code class=\"kw\">let [first, .., last] = arr;</code>는 메서드 소음 없이 조각에 이름을 붙입니다. 배열은 컴파일 시간에 길이를 알며 — 길이가 다르면 그 자체가 다른 타입입니다.</p>",
            "<pre><code>let potion = (\"heal\", 40);          // (&amp;str, i32)
let (name, power) = potion;         // 구조 분해
let cells = [1, 2, 3];              // [i32; 3]
let mid = cells[1];                 // 경계 검사됨
</code></pre>",
            "<h3>const, static, 캐스트</h3>",
            "<p>컴파일 시간 상수는 <code class=\"kw\">const</code> 아래 삽니다: 사용된 자리마다 들어가며 타입 어노테이션이 필수입니다. 이름은 SCREAMING_SNAKE_CASE(<code class=\"tp\">MAX_CHARGE</code>). <code class=\"kw\">static</code>은 장수하는 형제입니다: 프로그램 실행 전체에 값 하나, 전역 설정에 유용합니다 — 이름 규칙은 동일합니다.</p>",
            "<p>기본 타입 간 캐스트는 <code class=\"kw\">as</code>로 하며 <b>설계상 손실입니다</b>: <code class=\"kw\">f64</code> → <code class=\"kw\">i64</code>는 0 쪽으로 잘라내고, 큰 수를 작은 타입에 누르면 순환합니다. 오버플로 동작은 빌드에 따라 다릅니다: debug는 패닉, release는 조용히 순환 — 그러니 <code class=\"kw\">as</code>를 습관이 아니라 신중하고 주석 달린 결정으로 대하세요.</p>",
            "<h3>컴파일러의 편지 읽기</h3>",
            "<p>Rust 오류 메시지는 커리큘럼입니다. 각 메시지는 오류 코드(<code class=\"tp\">E0384</code> 같은), 범인 코드 위치, 그리고 자주 정확한 처방을 담은 <i>제안 줄</i>을 실어 옵니다. 멘토의 편지처럼 읽으세요 — 일부러 코드를 부수며 메시지를 연구하는 것은 이 정원의 모든 퀘스트가 장려하는 합법적(그리고 재미있는) 훈련입니다.</p>",
            "<h3>매일의 도구 상자</h3>",
            "<p>챕터는 매일의 도구 세트도 주머니에 넣어 줍니다: 타입 <code class=\"kw\">i32</code>/<code class=\"kw\">f64</code>/<code class=\"kw\">bool</code>, 튜플 <code class=\"kw\">(T, U)</code>, 고정 배열 <code class=\"tp\">[T; N]</code>, 컴파일 시간 상수 <code class=\"kw\">const</code>, 그리고 <code class=\"kw\">as</code>를 통한 명시적 캐스트. 하나도 로켓 과학은 아니지만 전부 매일 나옵니다 — 퀘스트를 자기 속도로 걸으며 손가락의 근육 기억을 쌓으세요.</p>",
            "<p>이 챕터의 실용적 의식: 퀘스트를 마칠 때마다 자기 해답을 <i>부수려</i> 해보세요 — 불변을 재대입하고, 문자열을 두 번 이동시키고, 끝을 넘어 인덱싱하기. 컴파일러가 각 범죄를 막는 것을 지켜보는 것이 백 단락의 이론보다 더 가르칩니다.</p>",
            "<p>퀘스트를 천천히 걸으세요. 힌트를 읽기 전에 답을 입력하고, 피드백 상자를 초록색으로 켜고, 정원이 숨 쉬는 것을 느끼세요. 기초 + 퀘스트 = 1단계 완료; 다음 챕터는 <code class=\"kw\">match</code>와 논쟁하고 루프에 손을 뻗습니다.</p>",
            "<div class=\"article-banner\">흙은 준비됐습니다: 열두 퀘스트가 이 기초를 반사신경으로 바꿉니다. 천천히 걸으세요 — 정원은 속도보다 인내에 더 큰 보상을 줍니다.</div>",
        ],
    },

    // C2 flow
    Article {
        chapter_id: "flow",
        lang: "ko",
        title: "흐름과 함수의 룬",
        paragraphs: &[
            "<p>논쟁하지 않는 프로그램은 돌덩이입니다. 이 챕터는 프로그램에게 <b>결정을 내리는 법</b>을 가르칩니다: 분기, 디스패치, 반복. 주인공은 <code class=\"kw\">match</code> — 경쟁 언어의 if/else보다 훨씬 강한 Rust의 패턴 매칭 엔진입니다.</p>",
            "<h3>if, 그리고 돌려준다</h3>",
            "<p>분기는 낯익지만 Rust는 한 수 더 둡니다: <code class=\"kw\">if</code>는 문이 아니라 <b>식</b>입니다. 값을 평가하므로 <code class=\"kw\">let status = if awake { \"기상\" } else { \"취침\" };</code>처럼 쓸 수 있습니다 — 삼항 연산자 곡예도 없고, 갈래의 타입은 서로 같아야 하며, 코드는 문장처럼 읽힙니다.</p>",
            "<pre><code>let mood = if hunger &gt; 8 { \"까칠\" } else { \"맑음\" };
</code></pre>",
            "<p>한 가지 강한 규칙: 조건은 진짜 <code class=\"kw\">bool</code>이어야 합니다. Rust에는 truthiness가 없습니다 — <code class=\"tp\">if 1 { … }</code>은 컴파일을 거부합니다, <code class=\"kw\">i32</code>와 <code class=\"kw\">bool</code>은 다른 종이기 때문입니다. 엄격한가요? 그렇습니다. 대신 '우연히 참'인 값도 없습니다: 모든 조건은 자기가 뜻하는 바를 정확히 말합니다.</p>",
            "<h3>match: 패턴의 제단</h3>",
            "<p><code class=\"kw\">match</code>는 값을 <b>패턴</b> 목록과 비교하고 첫 번째로 맞는 갈래를 실행합니다. 사랑해야 할 세 가지 규칙:</p>",
            "<ul><li><b>완전성</b>: 컴파일러가 갈래를 빼먹지 못하게 합니다 — 'non-exhaustive patterns'가 빌드를 멈춥니다.</li><li><b>페이로드 분해</b>: 갈래가 내부 값을 바로 묶습니다 — <code class=\"kw\">Ok(n) =&gt; println!(\"{}\", n)</code>.</li><li><b>범위 갈래</b>: <code class=\"kw\">1..=5</code>는 숫자의 통째 복도를 매칭하고, <code class=\"kw\">_</code>는 나머지 전부를 잡는 그물입니다.</li></ul>",
            "<p>조커 <code class=\"kw\">_</code>와 완전성 검사가 함께하면 컴파일러가 개인 감사관이 됩니다: 나중에 enum 변형을 하나 추가하면, 손대지 않은 모든 <code class=\"kw\">match</code>는 갈래를 쓸 때까지 컴파일을 거부합니다. 그래서 Rust에서 대형 리팩터링은 덜 무섭습니다.</p>",
            "<pre><code>match scroll {
    0              =&gt; println!(\"빈 책장\"),
    1..=9          =&gt; println!(\"몇 페이지\"),
    n @ 10..=99    =&gt; println!(\"{n} 페이지\"),
    _              =&gt; println!(\"한 권의 책!\"),
}
</code></pre>",
            "<h3>패턴 동물원</h3>",
            "<p>패턴은 그 자체로 하나의 언어입니다. 리터럴은 자기 자신과 매칭되고(<code class=\"kw\">0</code>), 범위는 복도를(<code class=\"kw\">1..=9</code>), 튜플과 구조체는 그 자리에서 분해되며(<code class=\"kw\">(a, b)</code>, <code class=\"tp\">Guardian { spear, .. }</code>), enum 변형은 자기 페이로드와 매칭되고(<code class=\"kw\">Some(x)</code>), <code class=\"kw\">_</code>는 이름 붙일 마음이 없는 것을 정중히 무시합니다. 그래서 <code class=\"kw\">match</code> 하나가 if 사다리 전체를 대신합니다.</p>",
            "<p>두 확장이 패턴을 수술용으로 만듭니다. <b>가드</b>는 갈래에 조건을 얹습니다: <code class=\"kw\">n if n &gt; 10 =&gt; …</code>는 추가 검사를 통과했을 때만 발동합니다. 그리고 <code class=\"kw\">n @ 10..=99</code>는 범위를 매칭하면서 <b>묶기도</b> 합니다 — 추출과 이름 붙이기가 한 호흡에.</p>",
            "<pre><code>match point {
    (0, y)                  =&gt; println!(\"Y축 위: {y}\"),\n    (x, y) if x == y        =&gt; println!(\"대각선\"),\n    (x, y)                  =&gt; println!(\"어딘가: {x}, {y}\"),\n}
</code></pre>",
            "<h3>if let: 가벼운 의식</h3>",
            "<p><i>단 하나의</i> 패턴만 관심 있을 때, <code class=\"kw\">match</code>는 한 음을 위한 풀 오케스트라입니다. <code class=\"kw\">if let Some(key) = relic { … }</code>은 당신이 신경 쓰는 경우만 풀고 나머지는 침묵에 맡깁니다. 퀘스트 31이 바로 이 관용구를 연습합니다 — <code class=\"kw\">Option</code>과 <code class=\"kw\">Result</code>의 매일의 도구.</p>",
            "<h3>while let: 큐 비우기</h3>",
            "<p>순환 버전은 패턴이 계속 맞는 동안 반복합니다: <code class=\"kw\">while let Some(msg) = rx.recv() { … }</code>는 채널이 마를 때까지 메시지를 소비합니다. 카운터를 손으로 굴리지 않고 스택, 큐, 이터레이터를 비우는 관용적 방법입니다.</p>",
            "<h3>루프: loop, while, for</h3>",
            "<p>Rust는 세 가지 회전을 줍니다. <code class=\"kw\">loop</code>는 <i>아마도 무한</i> — 솔직하게 선언합니다; <code class=\"kw\">break</code>로만 빠져나오며, break는 <b>값도 돌려줍니다</b>: <code class=\"kw\">let fuel = loop { break 42; };</code>. <code class=\"kw\">while</code>은 매 바퀴 전에 조건을 다시 봅니다. <code class=\"kw\">for</code>는 <b>이터레이터와 범위</b>를 걷습니다(<code class=\"kw\">1..=5</code>, <code class=\"tp\">vec.iter()</code>) — 관용적 기본값: 한 칸 어긋난 실수도, 수동 인덱스 검사도 없습니다.</p>",
            "<p>중첩 루프는 실용적 질문을 낳습니다: 어느 <code class=\"kw\">break</code>가 어느 루프를 빠져나올까? Rust는 <b>라벨</b>로 답합니다: <code class=\"kw\">'outer: loop { … }</code>는 깊은 곳에서 <code class=\"kw\">break 'outer;</code>로 빠져나올 수 있습니다. 라벨은 <code class=\"kw\">continue</code>에도 통합니다 — 불리언 플래그 춤은 필요 없습니다.</p>",
            "<pre><code>'outer: for pot in pots {
    for seed in seeds {
        if seed.spoiled { continue 'outer; }
    }
    println!(\"{} 심음\", pot);
}
</code></pre>",
            "<h3>for와 세 이터레이터</h3>",
            "<p><code class=\"kw\">for</code>는 조용히 세 가지 이터레이터 맛 중 하나를 부릅니다. <code class=\"kw\">for x in vec</code>는 값을 <b>이동</b>시킵니다(벡터가 소진됨); <code class=\"kw\">for x in vec.iter()</code>는 <code class=\"kw\">&amp;T</code> — 읽기 전용 훔쳐보기를, <code class=\"kw\">for x in vec.iter_mut()</code>는 <code class=\"kw\">&amp;mut T</code> — 다시 쓰기 허가증을 줍니다. 맛을 의식적으로 고르는 것이 우발적 이동 버그와 깔끔한 순회의 차이입니다.</p>",
            "<h3>fn, 매개변수, 반환</h3>",
            "<p><code class=\"kw\">fn</code>은 주문을 선언합니다: 이름, 타입이 있는 매개변수 목록, 그리고 반환 타입을 알리는 화살표 <code class=\"kw\">-&gt;</code>. 본문의 마지막 식은 암묵적으로 반환됩니다 — 대부분의 경우 <code class=\"kw\">return</code> 키워드가 필요 없습니다. Rust 스타일은 <b>온전한 동사</b>를 선호합니다: <code class=\"kw\">fn charge(price: i32) -&gt; i32</code>.</p>",
            "<p>왜 Rust 함수는 매개변수 타입을 생략하지 않을까요? 타입 추론은 <code class=\"kw\">let</code>의 특권이기 때문입니다; 함수 시그니처는 모듈 사이의 <b>계약 인터페이스</b> — 몸을 들여다보지 않고도 읽혀야 합니다.</p>",
            "<p><code class=\"kw\">-&gt;</code>가 없는 함수는 <code class=\"kw\">()</code>를 반환합니다 — 이것은 격식이 아니라 사실입니다: 그래서 <code class=\"kw\">println!</code>을 문으로 자유롭게 부를 수 있고, 잊은 세미콜론이 때때로 함수의 반환값을 '바꾼' 것처럼 보이는 이유도 여기에 있습니다. 시그니처의 타입은 요란합니다; 반환 타입도 예외가 아닙니다.</p>",
            "<h3>조기 반환과 ? 릴레이</h3>",
            "<p>중간의 <code class=\"kw\">return</code>은 완전히 합법입니다 — 함수 상단의 초병('무효하면 즉시 나간다')이 본길을 평평하게 유지합니다. 그리고 8장에서 만날 화살촉 <code class=\"kw\">?</code>는 같은 생각의 압축본입니다: '오류가 있으면 위로 돌려주고, 아니면 성공을 풀어라'.</p>",
            "<h3>함수도 값이다</h3>",
            "<p><code class=\"kw\">fn</code>은 변수에 살고 인자로 여행할 수 있습니다: <code class=\"kw\">let f: fn(i32) -&gt; i32 = double;</code>. 이터레이터 챕터가 제대로 탐험할 <b>클로저</b> 땅의 첫맛이지만, 지금도 행동을 데이터처럼 넘길 수 있습니다 — 콜백과 전략이 정확히 그것입니다.</p>",
            "<pre><code>fn double(n: i32) -&gt; i32 { n * 2 }

fn apply(f: fn(i32) -&gt; i32, v: i32) -&gt; i32 {
    f(v)
}
</code></pre>",
            "<h3>함께 엮기</h3>",
            "<p>이 룬들을 익히면 <i>무엇이든</i> 조합할 수 있습니다: <code class=\"kw\">fn</code> 주문은 소식을 주고받고, <code class=\"kw\">match</code>는 데이터를 방향 지으며, <code class=\"kw\">for</code>는 선반을 걷습니다. 이후의 모든 챕터는 문자 그대로 같은 춤에 더 화려한 장식을 얹은 것입니다: 구조체 필드는 match 갈래로 흐르고, 스레드는 동사를 받고, 파일은 (?) 릴레이를 타고 여행합니다.</p>",
            "<p>훔칠 만한 스타일 노트: 한 가지 일을 하는 <b>작은 동사</b> 함수를 선호하고, 마지막 식의 암묵적 반환을 살리며, 매개변수 목록은 짧게 — 함수가 인자 다섯 개를 요구한다면 아마 구조체가 되고 싶은 것입니다. 1년 뒤 이 코드를 읽을 미래의 나에게서 감사 편지가 올 겁니다.</p>",
            "<p>가장 낯선 룬부터 시작하세요: 많은 이에게 그것은 페이로드를 분해하는 <code class=\"kw\">match</code>입니다. 퀘스트 답을 손으로 타이핑하고, 한 번은 일부러 부숴서 컴파일러의 말을 읽은 뒤, 다음 퀘스트가 그 위에 층을 쌓게 하세요. 구조체의 전당에서 만납시다.</p>",
            "<div class=\"article-banner\">분기, 패턴, 루프, 주문 — 모든 Rust 프로그램의 네 개 톱니바퀴. 열두 퀘스트가 근육 기억으로 갈아줍니다.</div>",
        ],
    },

    // C3 forms
    Article {
        chapter_id: "forms",
        lang: "ko",
        title: "구조체와 열거형의 전당",
        paragraphs: &[
            "<p>스칼라만으로는 멀리 가지 못합니다. 실제 도메인에는 형태가 있습니다: 창과 방패를 든 수호자, 합법적 상태를 가진 문, 존재할 수도 없을 수도 있는 유물. Rust는 <b>곱 타입</b>(구조체), <b>합 타입</b>(열거형), 그리고 부재의 특별한 대사 — <code class=\"kw\">Option&lt;T&gt;</code>를 줍니다. 이 챕터는 날것의 값이 <i>나만의 타입</i>이 되는 대장간입니다.</p>",
            "<h3>struct: 데이터 거푸집</h3>",
            "<p><code class=\"kw\">struct Guardian { spear: u16, shield: u16 }</code>은 이름 붙은 필드 묶음을 주조합니다. 점으로 읽고, <code class=\"kw\">..base</code> 문법으로 조각조각 갱신하고 — 무엇보다 — <code class=\"kw\">impl</code> 블록으로 행동을 붙입니다. 리시버 <code class=\"kw\">&amp;self</code>는 메서드를 읽기 전용으로, <code class=\"kw\">&amp;mut self</code>는 뮤테이션을, 벌거벗은 <code class=\"kw\">self</code>는 값을 통째로 소비합니다(드물지만 정직합니다).</p>",
            "<pre><code>struct Guardian {\n    spear: u16,\n    shield: u16,\n}\n\nlet mut hero = Guardian { spear: 12, shield: 30 };\nhero.shield -= 5;      // 점을 통한 필드 접근\n</code></pre>",
            "<h3>튜플 구조체와 newtype 패턴</h3>",
            "<p>이름 붙은 필드가 없으면 <b>튜플 구조체</b>가 됩니다: <code class=\"kw\">struct Meters(f64);</code>. 겉보기엔 소박하지만 Rust의 가장 강력한 습관 중 하나 — <i>newtype 패턴</i>입니다. <code class=\"kw\">struct Meters(f64)</code>와 <code class=\"kw\">struct Seconds(f64)</code>는 다른 타입이라, 컴파일러는 거리와 지속시간을 더하는 것을 절대 허용하지 않습니다. 런타임 비용은 0, 컴파일 시간 안전성은 만점입니다.</p>",
            "<h3>derive: 공짜 도구 세트</h3>",
            "<p>모든 타입이 받을 자격이 있는 행동 — 출력, 비교, 복제 — 는 손으로 쓸 필요가 없습니다. 구조체나 enum 위에 <code class=\"kw\">#[derive(Debug, Clone, PartialEq, Default)]</code>를 놓으면 컴파일러가 보일러플레이트를 대신 씁니다: <code class=\"tp\">Debug</code>는 <code class=\"tp\">{:?}</code> 출력을, <code class=\"kw\">Clone</code>은 <code class=\"kw\">.clone()</code>을, <code class=\"kw\">PartialEq</code>는 값 사이의 <code class=\"kw\">==</code>를, <code class=\"kw\">Default</code>는 <code class=\"tp\">Guardian::default()</code>를 엽니다. 이 챕터의 퀘스트들은 이 한 줄에 크게 기댑니다.</p>",
            "<pre><code>#[derive(Debug, Clone, PartialEq, Default)]\nstruct Relic {\n    name: String,\n    power: u8,\n}\n\nlet r1 = Relic::default();\nlet r2 = r1.clone();\nprintln!(\"{:?}\", r1 == r2);   // true\n</code></pre>",
            "<h3>impl: 메서드와 리시버</h3>",
            "<p><code class=\"kw\">impl Guardian { … }</code>은 타입에 행동을 붙입니다. 첫 매개변수 — <b>리시버</b> — 가 메서드가 데이터를 다루는 방식을 정합니다: <code class=\"kw\">&amp;self</code>는 읽고, <code class=\"kw\">&amp;mut self</code>는 뮤테이션하고, <code class=\"kw\">self</code>는 값을 통째로 소비합니다(드물지만 정직합니다 — <code class=\"tp\">into_*</code> 변환이 그렇습니다). 메서드는 점으로 부릅니다: <code class=\"kw\">hero.sharpen()</code>.</p>",
            "<pre><code>struct Guardian { spear: u16, shield: u16 }\n\nimpl Guardian {\n    fn new(spear: u16, shield: u16) -&gt; Self {   // 연관 함수\n        Self { spear, shield }\n    }\n    fn total(&amp;self) -&gt; u16 { self.spear + self.shield }  // 대여\n    fn buff(&amp;mut self) { self.spear += 1; }              // 뮤테이션\n}\n\nlet mut hero = Guardian::new(12, 30);\nhero.buff();\nprintln!(\"{}\", hero.total());    // 43\n</code></pre>",
            "<p><code class=\"kw\">impl</code>의 분리가 Rust에 클래스 이름 뒤에 숨은 생성자가 없는 이유입니다: 생성은 보통의 연관 함수 <code class=\"kw\">Guardian::new()</code> — 리시버가 없음에 주목하세요 — 이고, 행동은 데이터 옆의 별도 방에서, 데이터에 얽히지 않고 자랍니다.</p>",
            "<h3>갱신 문법과 부분 이동</h3>",
            "<p><code class=\"kw\">let hero2 = Guardian { spear: 40, ..hero };</code>는 나머지 필드를 <code class=\"kw\">hero</code>에서 가져옵니다. 조심하세요: <code class=\"kw\">..hero</code>는 가져온 필드를 <b>이동</b>시킵니다 — 구조체가 <code class=\"kw\">String</code> 같은 힙 타입을 담고 있다면 원본은 부분적으로 이동되어 쓸 수 없게 됩니다. 평범한 숫자라면 같은 줄이 전체 복사의 지름길입니다: 모든 필드가 <code class=\"kw\">Copy</code>이기 때문입니다.</p>",
            "<h3>enum: 합법적 합 타입</h3>",
            "<p><code class=\"kw\">enum GateStatus { Locked, Guarded, Open }</code>은 허용된 상태의 유한한 목록을 선언하고, 각 변형은 자기 <b>페이로드</b>를 실을 수 있습니다: <code class=\"kw\">Ready(i32)</code>, <code class=\"kw\">Failed(String)</code>, <code class=\"kw\">Open { key: u8 }</code>. 하나의 타입, 여러 개의 모습. 이것이 Rust 모델링의 심장입니다: '이 가능성 중 하나'를 타입 시스템 안에서 표현해서 불가능한 상태는 애초에 만들어지지 않게 합니다.</p>",
            "<pre><code>enum Door {\n    Locked,\n    Guarded(Guardian),\n    Open { visits: u32 },\n}\n\nlet gate = Door::Guarded(hero);\n</code></pre>",
            "<p>enum은 읽을 수 있는 <b>상태 기계</b>입니다: 선언한 상태만 존재할 수 있고, 전이는 <code class=\"kw\">match</code>로 일어나며, 각 상태는 필요한 데이터만 정확히 실습니다. 문서를 <code class=\"kw\">Draft | Ready(String) | Published { date: String }</code>로 모델링하면 '날짜 없는 게시된 초안'은 즉시 불가능해집니다 — 그런 값은 그냥 만들어질 수 없습니다.</p>",
            "<h3>Option: 부재를 명시적으로</h3>",
            "<p>대부분의 언어는 부재를 표현할 때 조용히 산술에 스며드는 유령(null)을 씁니다. Rust는 부재를 타입으로 새깁니다: <code class=\"kw\">Option&lt;T&gt;</code>는 <code class=\"kw\">Some(값)</code>이거나 <code class=\"kw\">None</code>이며, 컴파일러가 값을 쓰기 전에 둘 다 인정하도록 <b>강제</b>합니다. 이 챕터의 모든 Option 퀘스트는 세 가지 관용구를 연습합니다:</p>",
            "<ul><li><code class=\"kw\">if let Some(x) = relic { … }</code> — 가벼운 조건부 언랩;</li><li><code class=\"kw\">match relic { Some(n) =&gt; …, None =&gt; … }</code> — 완전한 의식;</li><li><code class=\"kw\">relic.unwrap_or(0)</code> — 빠른 대체 코인.</li></ul>",
            "<p>부재가 값이라면 어떤 값처럼 변환할 수 있습니다. <b>컴비네이터</b> 가족은 의식을 짧게 줄입니다: <code class=\"kw\">relic.map(|r| r.power)</code>는 <code class=\"kw\">Option&lt;Relic&gt;</code>을 <code class=\"kw\">Option&lt;u8&gt;</code>로 바꾸고, <code class=\"kw\">and_then</code>은 다음 실패 가능 단계를 사슬처럼 잇고, <code class=\"kw\">unwrap_or_else(|| fallback)</code>은 필요할 때만 기본값을 계산합니다. 그 밑바닥은 모두 변장한 <code class=\"kw\">match</code>입니다.</p>",
            "<pre><code>let relic: Option&lt;Relic&gt; = Some(Relic { name: \"🦀\".into(), power: 9 });\n\nlet power = relic\n    .map(|r| r.power)          // Option&lt;u8&gt;\n    .filter(|p| *p &gt; 5)        // 강한 것만 남기기\n    .unwrap_or(0);             // 9\n</code></pre>",
            "<p><code class=\"kw\">unwrap()</code>은 존재하지만 무례합니다: None에서 패닉합니다. 정원에서는 unwrap을 <i>초안</i>으로 다룹니다: 예제와 빠른 실험에서는 괜찮지만, 진짜 코드에서는 <code class=\"kw\">unwrap_or</code>, <code class=\"kw\">?</code>, match를 택하세요. 8장이 이 교리를 공식화합니다.</p>",
            "<h3>Result: 실패를 위한 형제</h3>",
            "<p><code class=\"kw\">Option</code>은 '있는가?'에 답하지만 <i>왜</i> 없는지는 말해 주지 않습니다. 형제 <code class=\"kw\">Result&lt;T, E&gt;</code>는 <code class=\"kw\">Err</code> 갈래에 오류 페이로드를 실습니다: <code class=\"kw\">Ok(값)</code> 또는 <code class=\"kw\">Err(이유)</code>. 공식 만남은 8장이지만, 여기서도 형태는 낯익어야 합니다: 실패에 적용된 같은 enum 사고입니다.</p>",
            "<h3>불가능한 상태를 표현 불가능하게</h3>",
            "<p>챕터의 두 절반이 하나의 설계 계명에 봉사합니다: <b>불가능한 상태를 표현 불가능하게 만들기</b>. 값은 양수여야 한다 — 모든 단계에서 <code class=\"kw\">i32</code>를 검사하는 대신 <code class=\"kw\">u16</code>을 쓰세요. 필드는 없을 수 있다 — '빈 문자열 관례' 대신 <code class=\"kw\">Option</code>을 쓰세요. 상태는 정확히 셋이다 — 불리언 세 개 대신 enum을 선언하세요. 그러면 컴파일러가 밤에도 주말에도 무료로 일하는 도메인 파수꾼이 됩니다.</p>",
            "<h3>왜 이것이 중요한가?</h3>",
            "<p>진짜 Rust 코드를 보세요: <code class=\"tp\">Vec&lt;Item&gt;</code>, <code class=\"tp\">Option&lt;Config&gt;</code>, <code class=\"tp\">Result&lt;Reply, Error&gt;</code> — 구조체와 열거형 위의 제네릭 껍질들. 이 챕터를 마스터하지 못하면 이터레이터, 채널, 파일은 소음으로 읽힙니다; 마스터하면 투명한 유리가 됩니다.</p>",
            "<p>이 전당의 훈련 의식: 하루 중 무언가를 enum으로 모델링해 보세요 — 커피 주문, 기차표, 게임 상태. 컴파일러에게 불가능한 변형마다 거부를 부탁하고, 구조체 셋을 newtype으로 바꿔 보세요. 아래 퀘스트들이 struct, impl, Option, default를 한 번에 한 돌씩 걸어 줍니다.</p>",
            "<div class=\"article-banner\">전당이 기다립니다: 수호자를 하나 주조하고, 문 변형 셋을 봉인하고, 유물 하나를 언랩하세요. 다음 챕터의 문들이 더 쉽게 열립니다.</div>",
        ],
    },

    // C7 formatting
    Article {
        chapter_id: "formatting",
        lang: "ko",
        title: "Display와 Debug의 거울",
        paragraphs: &[
            "<p>출력은 궁정입니다: 값이 나타나서 말합니다. Rust는 자기 표현의 두 가지 어조를 구분합니다. <code class=\"kw\">Display</code>는 최종 사용자를 위한 <b>정중한 얼굴</b>입니다: <code class=\"kw\">println!(\"{}\", x)</code>. <code class=\"kw\">Debug</code>는 개발자를 위한 <b>솔직한 고백자</b>입니다: <code class=\"kw\">println!(\"{:?}\", x)</code>.</p>",
            "<h3>왜 트레이트가 둘인가?</h3>",
            "<p>청중이 다르기 때문입니다. 사용자는 <code class=\"tp\">4°C</code>를 원하고, 개발자는 <code class=\"tp\">Temperature { celsius: Some(4), scale: C }</code>를 원합니다. 내부를 사용자에게 보여 주는 것은 누수이고, 개발자에게 숨기는 것은 고문입니다. 트레이트가 둘이면 — 싸움은 없습니다.</p>",
            "<h3>매크로 가족: println!, eprintln!, format!, write!</h3>",
            "<p>모든 포매팅은 같은 엔진을 흐르고 — 목적지만 바뀝니다. <code class=\"kw\">println!</code>은 줄바꿈과 함께 터미널에 말하고, <code class=\"kw\">eprintln!</code>은 오류 스트림에, <code class=\"kw\">format!</code>은 조용히 <code class=\"kw\">String</code>을 건네주고, <code class=\"kw\">write!</code>는 formatter를 받는 무엇이든에 부어 넣습니다. 문법을 한 번만 배우면 — 가족 전체가 따릅니다.</p>",
            "<pre><code>let coin = 42;\n\nlet label = format!(\"coin = {}\", coin);   // 메모리에서 만든 문자열\nprintln!(\"{label}\");                     // stdout으로\neprintln!(\"warning: {label}\");           // stderr로\n</code></pre>",
            "<h3>위치 인자와 이름 붙은 인자</h3>",
            "<p>중괄호 안에는 자체 문법이 삽니다: <code class=\"tp\">{}</code>는 다음 인자를 가져가고, <code class=\"tp\">{0}</code>/<code class=\"tp\">{1}</code>은 위치로 인자를 부르며 — 같은 값이 두 번 나올 수 있습니다 — <code class=\"tp\">{name}</code>은 환경에서 바로 변수를 잡습니다. 인라인 캡처(<code class=\"kw\">println!(\"{x}\")</code>)가 현대의 Rust 스타일입니다: 소음도 적고 인자도 적습니다.</p>",
            "<pre><code>let (x, y) = (3, 7);\n\nprintln!(\"{0} + {1} = {0}\", x, y);  // 3 + 7 = 3\nprintln!(\"{x}는 {y}에 앉는다\");       // 인라인 캡처\nprintln!(\"{:&gt;5}|\", x);             // 필드 너비 5, 오른쪽 정렬\n</code></pre>",
            "<p>중괄호 자체도 출력할 수 있습니다: 두 배로 만들어 이스케이프하세요 — <code class=\"tp\">{{</code>는 <code class=\"tp\">{</code>를, <code class=\"tp\">}}</code>는 <code class=\"tp\">}</code>를 인쇄합니다. 그래서 <code class=\"kw\">println!(\"{{}} = {{code}}\")</code>는 <code class=\"tp\">{} = {code}</code>를 렌더합니다. 리터럴 JSON이나 중괄호를 보여 주는 템플릿은 이 기술을 매일 씁니다.</p>",
            "<h3>Debug와 derive의 마법</h3>",
            "<p>디버그 출력은 기계적이므로 Rust는 이를 <i>derive</i>하게 허용합니다: <code class=\"kw\">#[derive(Debug)]</code>가 모든 필드를 자동으로 출력하도록 컴파일러를 가르칩니다. 그다음 <code class=\"kw\">{:?}</code>가 트레이트를 불러냅니다. 해부가 깊으면 <code class=\"kw\">{:#?}</code>를 택하세요: 들여쓰기가 있는 예쁜 출력이 중첩 잔해를 읽을 수 있는 선반으로 바꿉니다.</p>",
            "<pre><code>#[derive(Debug)]\nstruct Relic { name: String, power: u8 }\n\nlet vault = vec![\n    Relic { name: \"rune\".into(), power: 3 },\n    Relic { name: \"gem\".into(), power: 9 },\n];\n\nprintln!(\"{:?}\", vault);   // [Relic { name: \"rune\", power: 3 }, …]\nprintln!(\"{:#?}\", vault);  // 예쁘게: 한 줄에 한 필드\n</code></pre>",
            "<h3>Display 대 Debug: 언제 무엇을</h3>",
            "<p>실용적인 나침반: <b>Debug는 당신을 위한 것, Display는 그들을 위한 것</b>. Debug는 로그, 실패한 테스트, 상태 스냅샷 — 개발자가 읽는 모든 것에 씁니다; 어디서나 derive하세요(컨테이너를 통해서도 공짜로 조합됩니다). Display는 사용자가 볼 문자열 — UI 줄, 보고서 행, CLI 출력 — 에 씁니다; 한 번, 의도적으로, 손으로 씁니다. 둘 다 필요한 타입이라면 둘 다 주세요: 다른 계약이지 라이벌이 아닙니다.</p>",
            "<h3>Display: 당신만의 목소리</h3>",
            "<p>Display는 derive할 수 없습니다 — <i>당신</i>이 타입을 어떻게 불리길 원하는지 아무도 추측할 수 없기 때문입니다. <code class=\"kw\">fmt::Display</code>를 한 번 구현하면 어디에서나 모든 <code class=\"kw\">{}</code>가 그것을 존중합니다. 도구는 하나 — <code class=\"kw\">write!(f, \"…\", 부분)</code>: 매크로가 포맷된 텍스트를 formatter에 부어 넣고, 당신은 <code class=\"tp\">fmt::Result</code>를 돌려줍니다.</p>",
            "<pre><code>use std::fmt;\n\nstruct Temperature { celsius: f64 }\n\nimpl fmt::Display for Temperature {\n    fn fmt(&amp;self, f: &amp;mut fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {\n        write!(f, \"{}°C\", self.celsius)\n    }\n}\n</code></pre>",
            "<h3>명세 필드: {:.2}, {:&gt;8}, {:08}</h3>",
            "<p><code class=\"kw\">{}</code> 안에는 명세의 미니 언어가 삽니다: <code class=\"tp\">{:.2}</code>는 소수 자릿수를 고정하고, <code class=\"tp\">{:&gt;8}</code>은 너비 8의 필드 안에서 오른쪽 정렬을, <code class=\"tp\">{:&lt;8}</code>은 왼쪽을, <code class=\"tp\">{:08}</code>은 숫자를 의식적인 너비까지 0으로 채웁니다(식별자에 최고). 이 네 글자를 익히면 출력이 전문 장부처럼 보입니다.</p>",
            "<p>필드는 세 개의 추가 기어를 실을 수 있습니다: <b>채움과 정렬</b>(<code class=\"tp\">{:&gt;8}</code>, <code class=\"tp\">{:&lt;8}</code>, <code class=\"tp\">{:^8}</code> — 오른쪽, 왼쪽, 중앙; 채움 문자는 화살표 앞에 씁니다: <code class=\"tp\">{:*^8}</code>은 별로 꾸밉니다), <b>부호</b>(<code class=\"tp\">{:+}</code>는 양수에 +를 강제합니다), 그리고 <b>0으로 채운 너비</b>(<code class=\"tp\">{:08}</code> — <code class=\"tp\">00042</code> 같은 식별자에 최적).</p>",
            "<pre><code>println!(\"{:+}\", 5);      // +5\nprintln!(\"{:08}\", 42);    // 00000042\nprintln!(\"{:b}\", 9);      // 1001\nprintln!(\"{:x}\", 255);    // ff\nprintln!(\"{:o}\", 8);      // 10\nprintln!(\"{:e}\", 1234.5); // 1.2345e3\n</code></pre>",
            "<h3>추천 훈련 루프</h3>",
            "<ul><li><code class=\"kw\">#[derive(Debug)]</code> 퀘스트를 derive가 숨 쉬는 것처럼 느껴질 때까지 반복하세요;</li><li>그다음 <code class=\"kw\">write!</code>를 쓴 Display 구현 하나를 손으로 쓰세요 — 근육 기억이 될 겁니다;</li><li>마무리는 미니 언어 퀘스트: {:.2}, {:&gt;8}, {:08}.</li></ul>",
            "<p>챕터를 작은 습관으로 닫으세요: 디버깅하려고 중간 값을 출력할 때, 한 번은 <code class=\"tp\">{:#?}</code>로 바꿔 그 해부학을 소리 내어 읽어 보세요. 그다음 자기 타입 하나 — 온도, 포션, 영수증 — 에 Display를 구현하고, 다시는 <code class=\"kw\">+</code>로 문자열을 수작업으로 붙이지 마세요. 포매팅은 언어입니다; 의도를 갖고 말하세요.</p>",
            "<div class=\"article-banner\">거울 챕터 이후, 당신의 로그는 '무작위 스팸'이 아니라 <i>궁정 기록</i>이 됩니다.</div>",
        ],
    },

    // C9 pointers
    Article {
        chapter_id: "pointers",
        lang: "ko",
        title: "마음의 궁전: 스마트 포인터",
        paragraphs: &[
            "<p>Rust 값의 방들은 <b>스택</b>(싸고, 빠르고, 스코프가 있음)이나 <b>힙</b>(크고, 공유되고, 오래살아 있음)에 삽니다. 소유자들은 <b>스마트 포인터</b>를 통해 힙을 직접 관리합니다. 마음의 궁전의 구조대 요원들입니다: 각자 자신의 엄격한 규약에 따라 세고, 나누고, 지킵니다. 어느 누구도 소유권을 전복하지 않습니다 — 각자 소유권의 <i>다른 구현</i>일 뿐입니다.</p>",
            "<h3>Box: 개인 힙 스위트</h3>",
            "<p><code class=\"kw\">Box&lt;T&gt;</code>는 <code class=\"kw\">T</code>를 힙에 할당하고 소유자로서 포인터 하나를 감쌉니다: 값 하나, 소유자 하나, 스코프가 끝나면 해제. 흔한 용도: 스택에 있으면 곤란한 초대형 구조체, 컴파일 시간에 크기를 알아야 하는 재귀 타입, 그리고 <b>트레이트 객체</b> — <code class=\"kw\">Box&lt;dyn Trait&gt;</code>: 어떤 구체 타입이든 하나의 계약 뒤에 숨습니다.</p>",
            "<pre><code>enum List {\n    Node(i32, Box&lt;List&gt;),\n    Empty,                 // 재귀는 여기서 멈춤\n}\n</code></pre>",
            "<p>이 예제에서 Box가 왜 필수일까요? 재귀 타입은 자기 크기를 알아야 하는데, 그 크기는 자기 자신에 의존합니다 — 컴파일러가 추측하기를 거부하는 무한 회귀입니다. <code class=\"kw\">Box</code>가 고리를 끊습니다: 포인터는 기계어 한 칸이고, <code class=\"kw\">List</code>의 크기는 다시 계산 가능해집니다. 연결 리스트, 파일 트리, 모든 트리 모양의 세계 뒤에 있는 <code class=\"kw\">Box&lt;Node&gt;</code>의 패턴이 바로 이것입니다.</p>",
            "<p>역참조가 이 궁전의 스타입니다: <code class=\"kw\">*ptr</code>이 값의 심층까지 파고듭니다. Box는 <code class=\"kw\">Deref</code>를 구현하므로 <code class=\"kw\">(*ptr).method()</code>가 아니라 <code class=\"kw\">ptr.method()</code>만 쓰면 됩니다 — 자동 역참조가 메서드 호출에서 투명하게 작동합니다.</p>",
            "<p>외워 둘 만한 메모리 해부학: <code class=\"kw\">Box</code> 자체는 스택에, 포인터 한 칸 너비로 살고, 그것이 지키는 <code class=\"kw\">T</code>는 힙에 삽니다; Box가 스코프를 떠나면 <code class=\"kw\">Drop</code>이 힙 방을 자동으로 해제합니다. 수동 <code class=\"kw\">free()</code>도, 누수도 없습니다 — 소유자가 곧 소멸자입니다.</p>",
            "<h3>Box&lt;dyn Trait&gt;: 계약 수집가</h3>",
            "<p>때로는 컬렉션이 하나의 계약을 따르는 <i>서로 다른</i> 타입을 담아야 합니다. 구체 타입들은 크기가 달라서, 컨테이너에는 균일한 형태가 필요합니다 — <b>트레이트 객체</b>입니다. <code class=\"kw\">Box&lt;dyn Beast&gt;</code>는 팻 포인터 — 데이터 + vtable — 이고, 동적 디스패치가 런타임에 어떤 <code class=\"kw\">cry()</code>를 부를지 결정합니다. 비용은 간접 한 번; 이득은 교체 가능한 행동입니다.</p>",
            "<pre><code>trait Beast { fn cry(&amp;self) -&gt; String; }\n\nstruct Crab;\nimpl Beast for Crab { fn cry(&amp;self) -&gt; String { \"🦀\".into() } }\n\nlet zoo: Vec&lt;Box&lt;dyn Beast&gt;&gt; = vec![Box::new(Crab)];\nfor b in &amp;zoo { println!(\"{}\", b.cry()); }\n</code></pre>",
            "<h3>Rc: 소유자 등록부</h3>",
            "<p>읽기 전용 데이터를 여러 소유자가 나눠 쓰기 — 싱글 스레드 맥락에서 — 는 <code class=\"kw\">Rc&lt;T&gt;</code>(reference counted)의 임무입니다. <code class=\"kw\">Rc::clone(&amp;scroll)</code>은 바이트를 복사하지 않습니다: 소유자 카운터를 증가시킬 뿐입니다; <code class=\"kw\">Rc::strong_count(&amp;scroll)</code>는 현재 몇 개의 이름이 값을 쥐고 있는지 보고합니다. 마지막 이름이 떠나면 데이터는 drop됩니다.</p>",
            "<h3>Cell과 RefCell: 내부 뮤테이션</h3>",
            "<p><i>공유 셀</i> 가족은 딱 하나의 규칙을 휘게 합니다: 공유 핸들을 통한 뮤테이션. <code class=\"kw\">Cell&lt;T&gt;</code>는 단순 <code class=\"kw\">Copy</code> 값을 다룹니다 — <code class=\"tp\">.set()</code>은 교체, <code class=\"tp\">.get()</code>은 읽기, 대여는 전혀 없습니다. <code class=\"kw\">RefCell&lt;T&gt;</code>는 나머지 전부를 다룹니다: 대여 장부를 컴파일 시간에서 런타임으로 옮기지요.</p>",
            "<pre><code>let s = Rc::new(RefCell::new(String::from(\"scroll\")));\n\nlet mut w1 = s.borrow_mut();\nw1.push_str(\"!\");\n\nlet r1 = s.borrow();   // ✗ 패닉: BorrowMutError\n</code></pre>",
            "<p><b>RefCell은 거의 항상 Rc와 짝을 이룹니다</b>: Rc만으로는 내부 뮤테이션이 불가능합니다 — 불변 등록부는 <code class=\"kw\">&amp;mut</code> 대여를 절대 내주지 않기 때문입니다. 대신 셀이 런타임 검사를 거친 뮤테이션을 제공합니다: <code class=\"kw\">borrow()</code>(여러 읽는 자 허용)와 <code class=\"kw\">borrow_mut()</code>(정확히 한 명의 쓰는 자) — 규칙이 그 자리에서 깨지면 <b>패닉</b>합니다.</p>",
            "<h3>왜 '런타임 검사'는 죄가 아니라 선물인가</h3>",
            "<p>borrow checker의 규칙은 훌륭하지만 정적입니다. 일부 자료구조(이중 연결 리스트, 공유 캐시, 관찰자)는 컴파일 시간에 수명 계약을 증명하지 못합니다. RefCell은 그 의무를 런타임으로 옮기되 Rust의 규율은 지킵니다: 규칙을 깨면 패닉이 판사입니다 — 결정론적이고, 스택 추적도 쉽습니다.</p>",
            "<h3>순환과 Weak라는 탈출구</h3>",
            "<p>카운트 소유에는 정직한 약점이 있습니다: 서로를 가리키는 두 값은 0에 도달하지 못하고, 둘 다 영원히 누수됩니다. 탈출구는 <code class=\"kw\">Weak&lt;T&gt;</code> — <code class=\"kw\">Rc::downgrade(&amp;rc)</code>로 만드는 비소유 사촌입니다. <code class=\"kw\">Weak</code>는 값을 살아 있게 유지하지 않습니다; <code class=\"kw\">upgrade()</code>는 공손히 물어 <code class=\"kw\">Option&lt;Rc&lt;T&gt;&gt;</code>를 돌려줍니다. 부모 → 자식 연결은 <code class=\"kw\">Rc</code>로, 자식 → 부모 연결은 <code class=\"kw\">Weak</code>로 하면 — 궁전에 유령 방은 없습니다.</p>",
            "<h3>올바른 포인터 고르기</h3>",
            "<ul><li>소유자 하나, 힙 필요 — <code class=\"kw\">Box&lt;T&gt;</code>;</li><li>소유자 많음, 읽기 전용 — <code class=\"kw\">Rc&lt;T&gt;</code>;</li><li>소유자 많음, 내부 뮤테이션 — <code class=\"kw\">Rc&lt;RefCell&lt;T&gt;&gt;</code>;</li><li>근처에 스레드라도 있다면 — <code class=\"kw\">Arc</code>, 쓰기엔 <code class=\"kw\">Mutex</code>(10장).</li></ul>",
            "<h3>스레드가 등장하는 지점</h3>",
            "<p>Rc + RefCell은 철저히 싱글 스레드입니다: 카운터가 원자적이지 않습니다. 스레드 챕터는 등록부를 <code class=\"kw\">Arc</code>로, 금고를 <code class=\"kw\">Mutex</code>로 업그레이드합니다 — 같은 심리, 새 엔진. 먼저 여기서 궁전 기반을 다지세요: Box, Rc, RefCell의 차이가 열 번째 챕터의 알파벳입니다.</p>",
            "<p>훈련 의식: 이전 퀘스트 해답 하나를 <code class=\"kw\">Vec</code>이 <code class=\"kw\">Box&lt;dyn Something&gt;</code>을 담도록 다시 쓰고, 하나의 목록이 세 종을 받아들이는 것을 지켜보세요. 그다음 일부러 RefCell을 뮤테이션으로 두 번 빌려 보세요 — 패닉을 소리 내어 읽고, '쓰는 자 하나' 규칙을 깬 줄을 찾아 고치세요. 포인터의 규칙이 개인 경험이 되는 순간 무서움이 사라집니다.</p>",
            "<div class=\"article-banner\">궁전의 비문: Box는 소유하고, Rc는 소유자를 세고, RefCell은 내부 개정을 지킨다.</div>",
        ],
    },

    // C10 threads
    Article {
        chapter_id: "threads",
        lang: "ko",
        title: "스레드의 합창",
        paragraphs: &[
            "<p>한 목소리는 한 노래를 부릅니다. 열 목소리는 <i>열 노래를 동시에</i> 부를 수도, 서로를 소음에 가라앉힐 수도 있습니다. Rust의 대표적 업적은 합창단이 <b>데이터 경합 없이</b> 날게 하는 것입니다: 무공포는 당신이 이미 공부한 타입들이 집행합니다.</p>",
            "<h3>데이터 경합이란 정확히 무엇인가?</h3>",
            "<p>재료 셋: <b>두 개 이상의 스레드</b>, <b>공유 메모리 위치 하나</b>, 그리고 접근 사이에 동기화 없는 <b>적어도 하나의 쓰기</b>. 읽기와 쓰기의 순서는 스케줄러 룰렛이 됩니다 — 같은 바이너리가 실행마다 다른 답을 내고, 버그는 몇 달을 숨습니다. Rust의 전략은 구조적입니다: 적어도 재료 하나를 컴파일 시간에 제거해서 경합이 아예 빌드되지 않게 합니다.</p>",
            "<h3>spawn과 move</h3>",
            "<p><code class=\"kw\">std::thread::spawn(|| …)</code>은 새 스레드를 점화하고 그 안에서 클로저를 가동합니다. 클로저는 환경 값을 캡처합니다 — 기본값으로는 <b>대여</b>하는데, 스레드는 대여보다 오래살 수 있습니다! <code class=\"kw\">move</code> 키워드는 클로저가 만지는 모든 것의 <b>소유권을 가져가게</b> 합니다: 수하물은 무리와 함께 여행합니다. 그래서 '스코프 이후 사용' 버그는 안전한 Rust에서 원천적으로 일어나지 않습니다.</p>",
            "<pre><code>use std::thread;\n\nlet lore = String::from(\"Ferris\");\n\nlet handle = thread::spawn(move || {\n    format!(\"songs of {lore}\")       // lore가 안으로 이동\n});\nprintln!(\"{}\", handle.join().unwrap());\n</code></pre>",
            "<h3>스코프드 스레드: 이동 없이 대여하기</h3>",
            "<p>때로 <code class=\"kw\">move</code>는 아깝습니다: 데이터는 그 기간 동안만 빌리면 되니까요. <code class=\"kw\">std::thread::scope</code>는 컴파일 시간에 모든 spawn된 스레드가 스코프가 끝나기 전에 join됨을 보장하므로, 클로저가 주변 변수를 안전하게 <b>대여</b>할 수 있습니다. <code class=\"kw\">move</code>의 엄격함과 보통 대여의 편안함 사이의 다리입니다.</p>",
            "<pre><code>let mut plans = vec![\"seeds\", \"water\"];\n\nthread::scope(|s| {\n    s.spawn(|| {\n        plans.push(\"sun\");          // &amp;mut 대여, 스코프 안이라 안전\n        println!(\"{}\", plans.len());\n    });\n    s.spawn(|| println!(\"조수 대기 중\"));\n});                  // 두 스레드가 여기서 join됨\nprintln!(\"{plans:?}\");\n</code></pre>",
            "<h3>Send와 Sync: 국경 수비대</h3>",
            "<p>두 마커 트레이트가 스레드 국경을 지킵니다. <code class=\"kw\">Send</code>는 값이 다른 스레드로 <b>이전</b>될 수 있음을, <code class=\"kw\">Sync</code>는 공유 참조 <code class=\"kw\">&amp;T</code>를 여러 스레드가 <b>동시에 읽을 수</b> 있음을 말합니다. 대부분의 평범한 타입은 둘 다 자동으로 해당됩니다.</p>",
            "<p>등록부가 고전을 밝힙니다: <code class=\"kw\">Rc&lt;T&gt;</code>는 <i>둘 다 아닙니다</i> — 카운터가 원자적이지 않아서 — 컴파일러는 밀수를 거부합니다. 그러나 <code class=\"kw\">Mutex&lt;T&gt;</code>는 어떤 <code class=\"kw\">T</code>든 Sync로 만들고, <code class=\"kw\">Arc&lt;T&gt;</code>는 <code class=\"kw\">T</code>가 그렇다면 Send + Sync입니다. 안전은 여기서 런타임 약속이 아니라 타입 수준의 증명입니다.</p>",
            "<h3>join: 점수 기록</h3>",
            "<p><code class=\"kw\">handle.join()</code>은 스레드의 결승을 기다리고 결과를 <code class=\"tp\">Result</code>에 감싸 돌려줍니다 — 스레드는 패닉했을 수 있습니다. join 없이 버려진 spawned 스레드는 굴레를 벗은 경주마와 같습니다: 허용되지만, 그 달리기의 결과는 받지 못합니다. 소중한 핸들은 join하세요.</p>",
            "<h3>Arc: 원자적 등록부</h3>",
            "<p>값 하나를 여러 스레드 사이에서 나누려면 <code class=\"kw\">Arc&lt;T&gt;</code> — 원자적으로 세는 <b>스레드용 Rc</b>가 필요합니다. <code class=\"kw\">Arc::clone</code>은 여전히 쌉니다: 원자적 카운터를 증가시킬 뿐; 각 스레드는 같은 힙 값에 대한 자기 핸들을 받습니다. <code class=\"kw\">Send</code>/<code class=\"kw\">Sync</code> 마커 트레이트는 스레드 안전하게 나눌 수 있는 데이터만 이 문들을 통과시킵니다.</p>",
            "<h3>Mutex: 황금 보물상자</h3>",
            "<p>병렬 읽기는 안전하지만; <b>쓰기</b>는 예약이 필요합니다. <code class=\"kw\">Mutex&lt;T&gt;</code>는 황금 보물상자입니다: <code class=\"kw\">lock()</code>이 가드를 돌려주고 — 가드가 살아 있는 동안 문은 당신 것; <code class=\"kw\">drop</code>이면 문이 다시 잠깁니다. 인체공학: <code class=\"kw\">let mut gold = vault.lock().unwrap(); *gold += 100;</code>. <code class=\"kw\">unwrap()</code>은 (드문) <i>오염된 잠금</i> — 다른 스레드가 안에서 패닉 — 을 다룹니다.</p>",
            "<p><code class=\"kw\">lock()</code>이 왜 <code class=\"tp\">Result</code>를 돌려줄까요? <i>포이저닝(poisoning)</i> 때문입니다: 스레드가 가드를 쥔 채 패닉하면 뮤텍스는 오염 표시를 받습니다 — 보호 상태가 불일치할 수 있습니다. <code class=\"kw\">.unwrap()</code>은 '어디선가의 패닉은 모든 곳의 버그'라고 선언하는 실용적 기본값입니다; 대안은 오류를 살펴보고 반쯤 깨진 상태를 구해 보는 것입니다.</p>",
            "<h3>RwLock: 열람실</h3>",
            "<p><code class=\"kw\">RwLock&lt;T&gt;</code>는 황금 보물상자를 다듬습니다: <b>읽기 가드는 몇 개든</b> 또는 <b>쓰기 가드는 정확히 하나</b> — 둘이 함께일 수는 없습니다. 읽기가 쓰기보다 열 배 많으면 열람실이 문 하나짜리 금고보다 우수합니다. 인체공학은 같고 손잡이 하나 더; 프로파일링이 읽는 자들의 줄을 보여 줄 때 찾으세요, 그 전에는 말고.</p>",
            "<h3>mpsc: 메시지 회랑</h3>",
            "<p>뮤텍스가 스레드를 줄 세우는 곳에서, <b>채널</b>은 목소리들이 그저 편지를 보내게 합니다. <code class=\"kw\">mpsc::channel()</code> — <i>multi-producer, single-consumer</i>: 여러 <code class=\"kw\">tx.send(msg)</code> 발신자, 하나의 <code class=\"kw\">rx.recv()</code> 수신자. 각 메시지의 소유권은 채널을 통해 <b>여행</b>합니다(또 move!), 그래서 경합은 구조적으로 불가능합니다. 선택은 이렇게: 상태엔 큐, 이벤트엔 채널.</p>",
            "<p>세 가지 습관이 채널을 노래하게 합니다. <b>마지막 발신자가 drop되면 회랑이 닫힙니다</b>: 그러면 <code class=\"kw\">rx.recv()</code>는 <code class=\"kw\">Err</code>를 돌려주고, 일꾼들은 하직 시간임을 압니다. <b>발신자 복제</b>(<code class=\"kw\">tx.clone()</code>)는 생산자를 추가합니다 — 싸고, 공유 상태도 없습니다. 그리고 <code class=\"kw\">while let Ok(msg) = rx.recv()</code> 루프 관용구는 우편이 멎을 때까지 계속 소비합니다.</p>",
            "<pre><code>let (tx, rx) = mpsc::channel::&lt;String&gt;();\nlet rx = Arc::new(Mutex::new(rx));\n\nfor id in 0..3 {\n    let rx = Arc::clone(&amp;rx);\n    thread::spawn(move || loop {\n        match rx.lock().unwrap().recv() {\n            Ok(job) =&gt; println!(\"worker {id}: {job}\"),\n            Err(_) =&gt; break,        // 채널 닫힘: 퇴근\n        }\n    });\n}\nfor i in 0..6 { tx.send(format!(\"job #{i}\")).unwrap(); }\ndrop(tx);                            // 회랑 닫기\n</code></pre>",
            "<p>울타리 너머 마지막 한 눈: OS <b>스레드</b>는 무거운 독창자입니다 — 각자 자기 스택을 지니고 OS 스케줄러가 시간을 선점형으로 나눕니다. <b>async/await</b>은 몇 개의 스레드 위에 싼 태스크를 다중화하는 거대 합창단이고, 협력적으로 기다립니다. CPU 바운드 작업과 학습에는 스레드가 완벽하고; 수만 개의 대기 중 네트워크 연결에는 async가 이깁니다. 10장은 의도적으로 전자를 가르칩니다 — 개념은 전이됩니다.</p>",
            "<h3>합창단 체크리스트</h3>",
            "<ul><li>spawn에 <code class=\"kw\">move</code>를 썼는가? (권장!)</li><li>join 가능한 모든 핸들이 join되었는가(또는 의도적으로 떼어놓았는가)?</li><li>공유 상태가 <code class=\"kw\">Arc&lt;Mutex&lt;T&gt;&gt;</code>로 감싸졌는가?</li><li>이벤트 흐름은 채널로 우회되는가?</li></ul>",
            "<p>훈련 의식: 평범한 <code class=\"kw\">i32</code>를 spawn된 두 스레드가 나눠 쓰게 시도하고 컴파일러의 거부를 받아 보세요 — 거부를 두 번 읽으세요. 그다음 <code class=\"kw\">Arc&lt;Mutex&lt;i32&gt;&gt;</code>로 감싸고, 세 스레드에서 올리고, join하세요. 거부에서 화음으로의 전환이 한 제스처로 담긴 챕터 전체입니다.</p>",
            "<div class=\"article-banner\">컴파일러는 우발적 경합을 가지치고, Arc/Mutex는 게으른 경합을 가지칩니다. 이 챕터 이후 동시성은 길거리 난투가 아니라 합창입니다.</div>",
        ],
    },

    // C4 collections
    Article {
        chapter_id: "collections",
        lang: "ko",
        title: "컬렉션 보관소",
        paragraphs: &[
            "<p>스칼라와 구조체는 한 번에 <i>하나</i>의 값만 담습니다. 진짜 모험은 <b>선반</b>을 필요로 합니다: 자라고 줄어드는 두루마리 서고, «이 키를 본 적 있는가?»에 답하는 대장. 이 챕터는 보관소를 엽니다: <code class=\"kw\">Vec&lt;T&gt;</code> — 자라는 배열, <code class=\"kw\">map</code>, <code class=\"kw\">filter</code>, <code class=\"kw\">collect</code>의 이터레이터 연금술, 그리고 키-값 대장 <code class=\"kw\">HashMap&lt;K, V&gt;</code>. 컬렉션은 소유권이 <i>실전</i>이 되는 곳입니다: 선반은 내용물을 소유하고, 빌림이 누가 만질지 결정합니다.</p>",
            "<h3>Vec: 자라는 선반</h3>",
            "<p><code class=\"kw\">Vec&lt;T&gt;</code>는 모든 요소를 소유하는 연속된 힙 배열입니다. 세 개의 수를 압니다: capacity(예약된 공간), length(사용 중인 공간), 그리고 힙 버퍼를 가리키는 포인터. 공간이 부족해지면 Vec는 <b>재할당</b>합니다: 더 큰 버퍼를 할당하고 모든 요소를 그곳으로 이동시키며, 옛 버퍼를 해제합니다. 그래서 <code class=\"kw\">Vec::with_capacity(n)</code>이 있습니다 — 한 번 예약하고 소요를 생략하세요.</p>",
            "<p>두 가지 생성 의식. <code class=\"kw\">Vec::new()</code>는 비어 있는 상태로 시작하며 사용처에서 타입을 추론합니다; <code class=\"kw\">vec![1, 2, 3]</code>은 값을 인라인으로 박아 벡터를 주조합니다 — 모든 러스트 사용자가 천 번쯤 입력하는 인장입니다. 둘은 같은 것으로 컴파일됩니다; 매크로는 그저 예의 바른 설탕일 뿐입니다.</p>",
            "<pre><code>let mut shelf: Vec&lt;String&gt; = Vec::new();
shelf.push(\"Memory Avengers\".to_string());
let scrolls = vec![1, 2, 3];          // Vec&lt;i32&gt; 안의 [1, 2, 3]
</code></pre>",
            "<p><code class=\"kw\">push</code>는 끝에 덧붙이고(상각 O(1)), <code class=\"kw\">pop</code>은 마지막 요소를 제거하며 <b>반환</b>합니다 — <code class=\"kw\">Option&lt;T&gt;</code>로: 선반에 무언가 있었다면 <code class=\"kw\">Some(값)</code>, 비어 있었다면 <code class=\"kw\">None</code>. Option 반환은 축소판 컬렉션 교리입니다: «없을 수도 있음»이 null이 아니라 타입에 새겨집니다. pop은 <code class=\"kw\">mut</code>를 요구한다는 점도 유의하세요 — 제거는 곧 변형입니다.</p>",
            "<pre><code>let mut stack = vec![10, 20, 30];
let top = stack.pop();      // Some(30)
stack.push(40);             // [10, 20, 40]
let len = stack.len();      // 3
</code></pre>",
            "<p><code class=\"kw\">len()</code>은 공짜 힐끗임입니다: Vec는 늘 제 길이를 알므로 질의 비용이 없습니다. 인덱싱 <code class=\"kw\">shelf[0]</code>은 첫 번째 권을 읽고 인덱스가 범위를 벗어나면 <b>패닉</b>합니다 — Rust는 모든 인덱스를 런타임에 검사하며, 조용한 쓰레기값은 없습니다. 검사된 읽기는 <code class=\"kw\">shelf.get(2)</code>가 담당합니다 — 충돌 대신 정중한 <code class=\"kw\">Option&lt;&amp;T&gt;</code>를 돌려줍니다. 경험칙: 범위를 이미 증명했으면 인덱싱, 아직이면 <code class=\"kw\">get()</code>.</p>",
            "<h3>선반 빌리기: 고전적 함정</h3>",
            "<p>빌림 검사기가 태어난 이유의 충돌입니다: <code class=\"kw\">let first = &amp;shelf[0];</code>은 한 요소의 읽기 전용 빌림을 취하고, <code class=\"kw\">shelf.push(x)</code>는 선반 전체의 <code class=\"kw\">&amp;mut</code>를 요구합니다 — push가 <b>재할당</b>으로 살아 있는 참조를 모두 무효화할 수 있기 때문입니다. <code class=\"kw\">&amp;shelf[0]</code>를 든 채 <code class=\"kw\">push</code>를 건너는 것은 컴파일 오류(E0502)입니다. 값을 복사하거나, 변형 뒤에 다시 빌리세요 — 컴파일러가 안내합니다.</p>",
            "<pre><code>let mut shelf = vec![1, 2, 3];
let first = shelf[0];       // 값을 복사 — 빌림을 쥐고 있지 않음
shelf.push(4);              // OK: 선반을 점유한 이가 없음
println!(\"{first}\");
</code></pre>",
            "<h3>이터레이터: 조립 라인</h3>",
            "<p>모든 컬렉션은 <b>이터레이터</b>를 내어줍니다 — 하나씩 항목을 내놓는 컨베이어입니다. <code class=\"kw\">vec.iter()</code>는 <code class=\"kw\">&amp;T</code>(읽기), <code class=\"kw\">vec.iter_mut()</code>는 <code class=\"kw\">&amp;mut T</code>(고치기), <code class=\"kw\">vec.into_iter()</code>는 소유의 <code class=\"kw\">T</code>(Vec 소모)를 내놓습니다. 마법은 이터레이터가 <i>게으르다</i>는 점입니다: 누군가 당길 때까지 아무 일도 일어나지 않습니다. <code class=\"kw\">map</code>, <code class=\"kw\">filter</code> 같은 어댑터는 컨베이어를 감쌀 뿐이고, <b>소비자</b> — <code class=\"kw\">collect</code>, <code class=\"kw\">sum</code>, <code class=\"kw\">for</code> — 만이 기계를 돌립니다.</p>",
            "<p><code class=\"kw\">map</code>은 각 항목을 변형합니다: <code class=\"kw\">nums.iter().map(|n| n * 2)</code>은 두 배 값을 내놓는 새 컨베이어입니다. <code class=\"kw\">filter</code>는 체입니다: <code class=\"kw\">nums.iter().filter(|&amp;&amp;n| n % 2 == 0)</code>은 짝수만 남깁니다(이중 <code class=\"kw\">&amp;</code>에 주목 — 클로저는 이터레이터가 내놓는 참조의 참조를 받습니다; 한 번 배우면 끝나는 패턴입니다). 자유롭게 연결하세요: <code class=\"kw\">filter</code> 다음 <code class=\"kw\">map</code>은 문장처럼 읽히며 중간 벡터를 전혀 만들지 않습니다.</p>",
            "<pre><code>let nums = vec![1, 2, 3, 4, 5, 6];
let evens_doubled: Vec&lt;i32&gt; = nums
    .iter()
    .filter(|&amp;&amp;n| n % 2 == 0)
    .map(|n| n * 2)
    .collect();               // [4, 8, 12]
</code></pre>",
            "<p><code class=\"kw\">sum()</code>은 컨베이어를 하나의 합계로 배출합니다: <code class=\"kw\">nums.iter().sum::&lt;i32&gt;()</code>. 터보피시 <code class=\"tp\">::&lt;i32&gt;</code>가 타입을 적습니다 — 알 수 없는 정수의 빈 컨베이어에는 명백한 합 타입이 없어 컴파일러가 추측을 거부하기 때문입니다. 평범한 덧셈이 아닌 축소는 <code class=\"kw\">fold(init, |acc, x| …)</code>, 모든 리듀서의 할머니로 하세요.</p>",
            "<p><code class=\"kw\">collect()</code>는 조립 라인의 목적지 표지판입니다: 스트림을 새 컨테이너에 부어 넣습니다. <b>목적지 타입이 형태를 고릅니다</b>: <code class=\"kw\">collect::&lt;Vec&lt;i32&gt;&gt;()</code>는 벡터를, <code class=\"kw\">collect::&lt;String&gt;()</code>은 문자를 이어붙이고, <code class=\"kw\">collect::&lt;HashSet&lt;_&gt;&gt;()</code>는 집합으로 중복을 제거합니다. 대개 표기는 변수에 얹습니다: <code class=\"kw\">let squared: Vec&lt;i32&gt; = …collect();</code> — 목적지를 선언하고 파이프라인이 채우게 하세요.</p>",
            "<h3>HashMap: 대장</h3>",
            "<p><code class=\"kw\">HashMap&lt;K, V&gt;</code>는 거의 O(1) 조회로 키를 값에 사상합니다. <code class=\"kw\">insert</code>는 소유권 규칙을 그대로 삽니다: <code class=\"kw\">map.insert(String::from(\"mana\"), 90)</code>은 키와 값을 모두 맵으로 <b>이동</b>시킵니다 — 맵이 소유자가 됩니다. 그래서 String 키의 HashMap이 끊임없이 등장합니다: 대장은 자기 라벨을 소유합니다.</p>",
            "<pre><code>use std::collections::HashMap;

let mut mana = HashMap::new();
mana.insert(String::from(\"fire\"), 40);
mana.insert(String::from(\"frost\"), 60);
let fire = mana.get(\"fire\");         // Some(&amp;40) — 복사가 아니라 빌림
</code></pre>",
            "<p>읽기는 빌림입니다: <code class=\"kw\">get(\"fire\")</code>는 <code class=\"kw\">Option&lt;&amp;V&gt;</code>를 돌려줍니다 — 맵 안을 가리키는 참조이며, 키가 없을 수 있으므로 Option에 포장됩니다. 쓰기 나름의 예법도 있습니다: 평범한 <code class=\"kw\">insert</code>는 <b>덮어쓰고</b> 옛값을 돌려주며, <code class=\"kw\">entry(key).or_insert(0)</code>는 upsert입니다 — «없을 때만 넣고, 그 항목을 고칠 수 있게 내놓아라». entry API는 카운터와 집계기의 요람입니다:</p>",
            "<pre><code>let mut counts: HashMap&lt;String, i32&gt; = HashMap::new();
for word in [\"garden\", \"garden\", \"rust\"] {
    *counts.entry(word.to_string()).or_insert(0) += 1;
}
// {\"garden\": 2, \"rust\": 1}
</code></pre>",
            "<p>대장 습관 두 가지 더. 순회는 튜플을 내놓습니다: <code class=\"kw\">for (key, value) in &amp;mana { … }</code> — 순서는 <i>미정의</i>입니다(해시 맵에는 순서대로 걸을 줄이 없습니다; 정렬된 출력이 필요하면 키를 모아 정렬하세요). 그리고 <code class=\"kw\">HashSet&lt;T&gt;</code>이 있습니다 — 값이 없는 척하는 HashMap으로, «본 적 있는가?» 멤버십 검사에 완벽합니다.</p>",
            "<h3>어떤 컨테이너를 언제?</h3>",
            "<ul><li>순서 있는 목록, 인덱싱, 스택 동작 — <code class=\"kw\">Vec&lt;T&gt;</code> (90%의 기본 선택).</li><li>키로 찾기, 카운팅, 페이로드가 있는 중복 제거 — <code class=\"kw\">HashMap&lt;K, V&gt;</code>.</li><li>멤버십만, 페이로드 없음 — <code class=\"kw\">HashSet&lt;T&gt;</code>.</li><li>컴파일 시점에 알려진 고정 길이 — 평범한 배열 <code class=\"tp\">[T; N]</code>은 스택에 삽니다.</li></ul>",
            "<h3>컬렉션의 소유권 장부</h3>",
            "<p>컬렉션은 소유권을 <b>보이게</b> 합니다. push는 값을 안으로 이동시키고, pop은 밖으로 이동시키며, 순회는 빌림(<code class=\"kw\">iter</code>)과 소모(<code class=\"kw\">into_iter</code>) 사이에서 선택합니다. 컴파일러가 컬렉션 루프에 대해 트집을 들면, 그 불평을 이 장부로 번역하세요 — 해결책은 보통 저절로 고릅니다: 꼭 갖고 있어야 할 소수는 clone하고, 읽기만 할 것은 빌리며, 나머지는 컬렉션이 소유하게 두세요.</p>",
            "<p>훈련 의식: 예전 퀘스트 풀이 하나를 골라 세 가지 방식으로 다시 지어 보세요 — <code class=\"kw\">for</code>와 인덱스로, <code class=\"kw\">while let Some(x) = vec.pop()</code>으로, 그리고 온전한 <code class=\"kw\">map</code>/<code class=\"kw\">filter</code>/<code class=\"kw\">collect</code> 파이프라인으로. 같은 출력, 세 방언. 파이프라인판이 시니어 러스터들이 먼저 손대는 형태입니다; 이제 이유를 아셨죠.</p>",
            "<div class=\"article-banner\">보관소의 규칙: 컬렉션은 내용물을 소유하고, 이터레이터는 그것을 빌려주며, collect()는 빌린 스트림이 다시 소유의 컨테이너가 되는 세관입니다.</div>",
        ],
    },

    // C8 errors
    Article {
        chapter_id: "errors",
        lang: "ko",
        title: "Result와 오류의 시련",
        paragraphs: &[
            "<p>다른 언어들은 실패를 예외나 null 뒤에 숨기지만, Rust는 라벨 붙은 상자에 담아 건넵니다. 이 챕터는 시련을 진행합니다: <code class=\"kw\">Result&lt;T, E&gt;</code> — 정직한 반환 타입, <code class=\"kw\">match</code>의 법정, 지름길들(<code class=\"kw\">unwrap</code>, <code class=\"kw\">expect</code>, <code class=\"kw\">unwrap_or</code>), <code class=\"kw\">?</code> 연산자의 릴레이 바통, 혐의를 번역하는 <code class=\"kw\">map_err</code>, 그리고 실패가 매일 만나는 곳 <code class=\"kw\">std::io</code>.</p>",
            "<h3>Result: 정직한 판결</h3>",
            "<p><code class=\"kw\">Result&lt;T, E&gt;</code>는 정확히 두 팔을 가진 열거형입니다: <code class=\"kw\">Ok(값)</code> — 연산이 성공했고 여기 물건이 있다 — 그리고 <code class=\"kw\">Err(오류)</code> — 실패했고 여기 기소장이 있다. 예외와 달리 실패가 <b>타입 시그니처에</b> 있습니다: 호출자는 못 본 척할 수 없습니다. 컴파일러는 <code class=\"kw\">T</code>의 어떤 값도 손에 닿기 전에 결정을 강요합니다.</p>",
            "<pre><code>enum Result&lt;T, E&gt; {\n    Ok(T),\n    Err(E),\n}\n\nfn parse_level(text: &amp;str) -&gt; Result&lt;i32, std::num::ParseIntError&gt; {\n    text.parse::&lt;i32&gt;()   // 실패할 수 있음: 타입이 미리 말해줌\n}\n</code></pre>",
            "<h3>match: 법정</h3>",
            "<p>Result를 다루는 정직한 방법은 <code class=\"kw\">match</code>입니다: 두 팔 모두 필수라 오류가 심사받지 않고 스며나갈 수 없습니다. 팔 안의 페이로드는 여러분 것입니다 — <code class=\"tp\">Ok(x)</code> 패턴은 값을 묶고, <code class=\"tp\">Err(e)</code>는 오류를 묶습니다. 이것이 의식 없는 형태이며 표준 라이브러리 문서가 가장 먼저 보여주는 것입니다.</p>",
            "<pre><code>match parse_level(\"42\") {\n    Ok(level) =&gt; println!(\"welcome, level {level}\"),\n    Err(e) =&gt; println!(\"두루마리가 훼손됐다: {e}\"),\n}\n</code></pre>",
            "<h3>unwrap과 expect: 지름길</h3>",
            "<p><code class=\"kw\">unwrap()</code>은 «Ok라는 걸 이미 알고 있다 — 값을 다오»라고 말하고, 여러분이 틀리면 <b>패닉</b>합니다 — 기본 메시지와 함께 스레드가 추락합니다. <code class=\"kw\">expect(\"맥락\")</code>은 증언이 붙은 같은 내기입니다: 패닉 시 여러분의 텍스트가 인쇄되어 수수께끼 크래시가 빵 부스러기로 바뀝니다. 둘 다 테스트, 프로토타입, 데모에서는 정당합니다; 진짜 제어 흐름에서는 다음 독자가 갚아야 하는 차용증입니다.</p>",
            "<pre><code>let good = parse_level(\"42\").unwrap();        // Err면 패닉\nlet better = parse_level(\"42\").expect(\"레벨 두루마리는 파싱되어야 한다\");\n// 패닉: 레벨 두루마리는 파싱되어야 한다: invalid digit found in string\n</code></pre>",
            "<h3>? 연산자: 릴레이 바통</h3>",
            "<p>자기 자신이 Result를 반환하는 함수 안에서 <code class=\"kw\">?</code>는 법정의 정중한 속기입니다: «Ok면 값을 건네주고 계속 가라; Err면 <b>내 함수에서 지금 오류를 반환하라</b>». 글리프 하나짜리 전파 — 오류는 손대지 않은 채 호출 스택을 저절로 거슬러 올라가 누군가 진짜로 판결하기로 마음먹을 때까지 갑니다. <code class=\"kw\">?</code>의 사슬은 직선 도로처럼 읽힙니다.</p>",
            "<pre><code>fn read_level(path: &amp;str) -&gt; Result&lt;i32, std::io::Error&gt; {\n    let text = std::fs::read_to_string(path)?;   // Err? 지금 반환\n    let level = text.trim().parse::&lt;i32&gt;().ok().ok_or(std::io::Error::new(\n        std::io::ErrorKind::InvalidData, \"bad level\"))?;\n    Ok(level)\n}\n</code></pre>",
            "<h3>unwrap_or: 집행관의 기본값</h3>",
            "<p>때로 오류는 재판할 가치가 없습니다: 쇼는 기본값과 함께 계속되어야 합니다. <code class=\"kw\">unwrap_or(fallback)</code>은 Err일 때 fallback으로 대체하고, <code class=\"kw\">unwrap_or_else(|| …)</code>는 필요할 때만 느긋하게 만들며, <code class=\"kw\">unwrap_or_default()</code>는 타입에게 0값을 물어봅니다. 이 셋은 «실패할 수도»를 «정중하게 실패»로 바꿉니다 — 설정 읽기, 캐시 조회, 그럴듯한 기본값이 있는 모든 것에 이상적입니다.</p>",
            "<pre><code>let level = parse_level(user_input).unwrap_or(1);      // 기본값 1\nlet level2 = parse_level(user_input).unwrap_or_else(|e| {\n    println!(\"warn: {e}\");\n    1\n});\n</code></pre>",
            "<h3>map_err: 혐의 번역</h3>",
            "<p>층마다 오류 방언이 다를 때 <code class=\"kw\">map_err</code>는 Ok를 건드리지 않고 Err 팔을 다시 씁니다: <code class=\"kw\">result.map_err(|e| MyError::from(e))</code>. 여러분 도메인 오류와 호출된 쪽이 돌려준 라이브러리 오류 사이의 어댑터입니다 — 클로저 하나, 의식 없음. 조용한 형제 <code class=\"kw\">map</code>은 Ok 팔에 대해 같은 일을 합니다: 성공 값을 자리에서 변형합니다.</p>",
            "<pre><code>let level: Result&lt;i32, String&gt; =\n    parse_level(\"7x\").map_err(|e| format!(\"bad scroll: {e}\"));\n</code></pre>",
            "<h3>std::io: 실패가 매일 사는 곳</h3>",
            "<p>파일시스템과 입출력 연산은 <code class=\"kw\">io::Result&lt;T&gt;</code> — 그냥 <code class=\"kw\">Result&lt;T, io::Error&gt;</code> —를 반환합니다. 파일이 없을 수도, 디스크가 차고, 권한이 물기 때문입니다. 이 챕터 퀘스트의 시련들은 전부 가면을 쓴 <code class=\"kw\">io::Error</code> 취급입니다: 다르게 반응해야 하면 <code class=\"kw\">ErrorKind</code>(NotFound? PermissionDenied?)를 들여다보고, 아니면 그냥 <code class=\"kw\">?</code>로 위로 넘기세요.</p>",
            "<pre><code>use std::fs;\n\nmatch fs::read_to_string(\"garden.log\") {\n    Ok(text) =&gt; println!(\"{}바이트의 수다\", text.len()),\n    Err(e) if e.kind() == std::io::ErrorKind::NotFound =&gt; {\n        println!(\"아직 로그가 없다 — 첫 방문?\");\n    }\n    Err(e) =&gt; return Err(e),   // 나머지는: 위로 전달\n}\n</code></pre>",
            "<h3>실패의 예법</h3>",
            "<ul><li>라이브러리 코드: 예상되는 실패로는 <b>절대</b> 패닉하지 마세요 — <code class=\"kw\">Result</code>를 돌려주고 호출자가 판단하게 하세요.</li><li>테스트와 스크립트: <code class=\"kw\">unwrap</code>/<code class=\"kw\">expect</code>는 괜찮습니다 — 패닉이 곧 테스트의 요란한 실패입니다.</li><li>최상위(main, 요청 핸들러): 기록하고 우아하게 저하하거나, 정중히 답하는 핸들러까지 <code class=\"kw\">?</code>로 전파하세요.</li><li>기본값 선택은 결정입니다 — <code class=\"kw\">unwrap_or</code>는 그것을 보이게 하고, <code class=\"kw\">unwrap</code>은 크래시로 만듭니다.</li></ul>",
            "<p>한 숨에 담은 철학: Rust에는 <b>버그</b>를 위한 패닉(깨진 불변식, 복구 불가 상태)과 <b>날씨</b>를 위한 Result(없는 파일, 나쁜 입력, 찬 디스크)가 있습니다. 둘을 뒤섞는 것이 원죄입니다; 시련은 반사적으로 구별하도록 가르칩니다.</p>",
            "<p>훈련 의식: <code class=\"kw\">unwrap</code>을 썼던 옛 퀘스트를 골라 세 가지 방식으로 다시 쓰세요 — 온전한 <code class=\"kw\">match</code>, 사용자 정의 오류를 곁들인 <code class=\"kw\">?</code> 사슬, 그리고 <code class=\"kw\">unwrap_or</code> 기본값. 같은 동작, 세 관객층: 법정, 릴레이, 어깨 으쓱.</p>",
            "<div class=\"article-banner\">시련의 규칙: 실패는 시그니처의 일부 — Ok와 Err 모두 정당한 답이고, ?는 바통을 넘기며, unwrap은 근거를 보여줄 수 있어야 하는 내기입니다.</div>",
        ],
    },

    // C11 files
    Article {
        chapter_id: "files",
        lang: "ko",
        title: "파일의 서고",
        paragraphs: &[
            "<p>지금까지 쓴 모든 것은 프로세스와 함께 죽었습니다: 변수는 증발하고, Vec는 녹고, 힙은 깨끗이 쓸립니다. 서고는 데이터가 마침내 프로그램보다 <b>오래 살아남는</b> 곳입니다 — 실행 사이에도 사는 파일 선반들. 이 챕터는 <code class=\"kw\">std::fs</code> 동을 둘러봅니다: <code class=\"kw\">read_to_string</code>의 단숨 읽기, 형제자매 <code class=\"kw\">fs::write</code>, 핸들 타입 <code class=\"kw\">File</code>, <code class=\"kw\">BufReader</code>의 인내심 있는 버퍼 읽기, <code class=\"kw\">metadata</code>로 재기, 그리고 지우는 의식 <code class=\"kw\">remove_file</code>.</p>",
            "<h3>온전히 읽는 의식: fs::read_to_string</h3>",
            "<p>가장 간단한 주문: 경로를 주면 파일 전체를 <code class=\"kw\">String</code>으로 받습니다 — 아니면 <code class=\"kw\">Result</code>에 담긴 <code class=\"kw\">io::Error</code>. 할당 한 번, 읽기 한 번, 의식 없음; 설정, 시드 데이터, 퀘스트 크기의 두루마리에 완벽합니다. 안에 숨은 두 실패 양상을 기억하세요: 파일이 없을 수 있고(NotFound) 바이트가 유효한 UTF-8이 아닐 수 있으며(InvalidData) — 둘 다 Err로 옵니다.</p>",
            "<pre><code>use std::fs;\n\nlet scroll = fs::read_to_string(\"garden.txt\")?;\nprintln!(\"{}\", scroll);   // 파일 전체, 한 모금\n</code></pre>",
            "<h3>쓰기 형제자매: fs::write</h3>",
            "<p><code class=\"kw\">fs::write(path, bytes)</code>는 거울 의식입니다: 파일을 <b>만들거나 잘라내고</b> 버퍼 전체를 채웁니다. 잘라내기가 중요합니다 — 기존 파일은 새 바이트가 내려앉기 전에 조용히 비워집니다; 여기엔 부드러운 덧붙이기가 없습니다(덧붙이려면 아래처럼 append 옵션으로 File을 여세요). 바이트로 읽히는 것은 무엇이든 받습니다 — <code class=\"tp\">&amp;str</code>이면 충분합니다.</p>",
            "<pre><code>use std::fs;\n\nfs::write(\"garden.txt\", \"심음: 양치식물 3그루\\n\")?;   // 생성 또는 덮어쓰기\n</code></pre>",
            "<h3>File: 핸들</h3>",
            "<p><code class=\"kw\">File</code>은 내용물이 아닙니다 — <b>핸들</b>입니다: 디스크의 파일로 통하는 열린 통로로, 커서와 권한을 쥡니다. <code class=\"kw\">File::open(path)</code>는 읽기 전용으로 엽니다(없으면 NotFound 실패); <code class=\"kw\">File::create(path)</code>는 쓰기용으로 열거나 만들며 잘라냅니다. 핸들은 drop에서 스스로 닫힙니다 — 잊을 close()가 없습니다 — 그래도 닫히지 않은 핸들은 OS 자원을 붙잡고 있으니 스코프를 좁게 유지하세요.</p>",
            "<pre><code>use std::fs::File;\n\nlet mut ledger = File::create(\"ledger.txt\")?;   // 새 것, 잘린 것\n</code></pre>",
            "<p>핸들 너머로는 정밀하게 씁니다: <code class=\"kw\">write_all(bytes)</code>는 버퍼 전체를 밀어 넣습니다(평범한 <code class=\"kw\">write</code>는 부분만 쓸 수 있습니다 — 성실한 이를 위한 루프, 나머지 모두를 위한 <code class=\"kw\">write_all</code>). <code class=\"kw\">write_all</code>을 여러 번 부르면 커서가 도달한 곳에 덧붙습니다 — 파일을 조각조각 세우는 방법이 바로 이것입니다.</p>",
            "<pre><code>use std::io::Write;\n\nlet mut ledger = File::create(\"ledger.txt\")?;\nledger.write_all(b\"1일차: 양치식물을 심다\\n\")?;\nledger.write_all(b\"2일차: 양치식물이 번성한다\\n\")?;\n</code></pre>",
            "<h3>BufReader: 인내심 있는 독자</h3>",
            "<p>버퍼 없는 읽기는 요청마다 OS를 흔듭니다 — 작은 모금에는 비쌉니다. <code class=\"kw\">BufReader</code>는 어떤 독자든 편안한 버퍼로 감쌉니다: 큰 syscall 한 번이 창고를 채우고, 줄들은 메모리에서 나갑니다. <code class=\"kw\">File</code>을 감싸고 <code class=\"kw\">lines()</code>를 순회하세요 — 각 줄은 <code class=\"kw\">io::Result&lt;String&gt;</code>로 도착합니다(줄바꿈은 잘려 있음). 로그 파싱과 CSV 비슷한 파일의 말일입니다.</p>",
            "<pre><code>use std::fs::File;\nuse std::io::{self, BufRead};\n\nlet reader = BufReader::new(File::open(\"garden.log\")?);\nfor line in reader.lines() {\n    let line = line?;\n    println!(\"행: {line}\");\n}\n</code></pre>",
            "<p>한 번에 반 줄씩 — 프롬프트를 내고 방문자의 대답을 받는 — 인터랙티브 모드에는 같은 독자의 <code class=\"kw\">read_line(&amp;mut buf)</code>가 있습니다: String에 줄바꿈을 <b>포함한</b> 한 줄을 채우고 몇 바이트를 읽었는지 돌려줍니다(스트림 끝에서 0). 이후 trim하세요; 줄바꿈도 수확의 일부입니다.</p>",
            "<pre><code>use std::io::BufRead;\n\nlet mut name = String::new();\nlet bytes = reader.read_line(&amp;mut name)?;   // '\\n' 포함\nlet name = name.trim().to_string();         // 정중한 버전\n</code></pre>",
            "<h3>metadata: 재기의 의식</h3>",
            "<p><code class=\"kw\">fs::metadata(path)</code>는 파일을 열지 않고 서류를 돌려줍니다: <code class=\"kw\">len()</code> — 바이트 크기, <code class=\"kw\">is_file()</code>/<code class=\"kw\">is_dir()</code> — 이것이 어떤 선반인지, 수정 시각 — 마지막으로 만진 때. 먼저 서류를 보면 놀랄 일이 없습니다: 빈 파일은 건너뛰고, 파일로 분장한 디렉터리는 거절하거나, UI에 깔끔한 크기를 보여주기만 하세요.</p>",
            "<pre><code>use std::fs;\n\nlet meta = fs::metadata(\"garden.txt\")?;\nprintln!(\"{}바이트, 파일? {}\", meta.len(), meta.is_file());\n</code></pre>",
            "<h3>지우는 의식: remove_file</h3>",
            "<p><code class=\"kw\">fs::remove_file(path)</code>는 파일을 선반에서 내립니다 — 영원히: 휴지통도, 취소도 없으며, 파일이 없거나 디렉터리면 정중히 실패합니다(Err). 서고는 사용자에게 배려가 의무입니다: 잘못된 경로가 진짜 데이터 값을 치르는 유일한 의식이므로 아래의 예법은 장식이 아닙니다.</p>",
            "<pre><code>use std::fs;\n\nif fs::metadata(\"tmp_scroll.txt\").is_ok() {\n    fs::remove_file(\"tmp_scroll.txt\")?;   // 사라졌다, 두 번째 기회 없음\n}\n</code></pre>",
            "<h3>서고의 예법</h3>",
            "<ul><li>파일 통째로 다루는 일 — <code class=\"kw\">fs::read_to_string</code>/<code class=\"kw\">fs::write</code>가 먼저, 핸들은 그 다음.</li><li>줄 단위나 거대 파일 — <code class=\"kw\">BufReader</code>/<code class=\"kw\">lines()</code>; 4GB 로그를 String에 단숨에 넣지 마세요.</li><li>덧붙이기 — <code class=\"kw\">OpenOptions::new().append(true)</code>로 여세요; <code class=\"kw\">fs::write</code>는 덧붙이지 못합니다.</li><li>모든 의식은 <code class=\"kw\">io::Result</code>를 돌려줍니다 — 시련 챕터는 이미 통과한 필수 과목입니다; <code class=\"kw\">?</code>를 쓰고 중요할 땐 <code class=\"kw\">ErrorKind</code>로 match하세요.</li><li>삭제는 영원합니다 — <code class=\"kw\">remove_file</code> 전에 확인하고, 검증 없이 사용자 입력으로 조립한 경로로는 절대 지우지 마세요.</li></ul>",
            "<p>서고는 정원이 처음부터 쌓아 온 고리를 닫습니다: 소유권은 누가 데이터에 책임지는지 가르쳤고, 시련은 현실이 거부할 때 무엇을 할지 가르쳤으며, 파일은 여러분의 데이터가 마침내 RAM 밖으로 나가 밤을 넘기는 곳입니다. 읽고, 쓰고, 지우기를 책임지는 프로그램은 이제 장난감이 아닙니다 — 디스크의 작고 정중한 시민입니다.</p>",
            "<div class=\"article-banner\">서고의 규칙: read_to_string과 fs::write는 한 모금의 의식, File은 들고 다니는 핸들, BufReader는 포장 속 인내심, metadata는 서류, 그리고 remove_file은 두 번 검토하는 유일한 의식입니다.</div>",
        ],
    },

    // C5 strings
    Article {
        chapter_id: "strings",
        lang: "ko",
        title: "문자열 두루마리",
        paragraphs: &[
            "<p>숫자는 정직하지만 텍스트는 <i>교활</i>합니다. 숫자는 스택의 고정 크기이지만 문자열은 자라고, 줄고, 이어지고, 다시 쓰일 수 있는 이야기입니다. 이 챕터는 두루마리를 펼칩니다: <code class=\"kw\">String</code>과 <code class=\"tp\">&amp;str</code>의 두 문자열 교리, <code class=\"kw\">push_str</code>로 키우기, <code class=\"kw\">format!</code>으로 엮기, 그리고 세척 도구 — <code class=\"kw\">trim</code>, <code class=\"kw\">to_uppercase</code>, <code class=\"kw\">chars</code>, <code class=\"kw\">split</code>, <code class=\"kw\">replace</code>. 익히면 텍스트는 더 이상 수수께끼가 아닙니다.</p>",
            "<h3>두 문자열: 소유의 두루마리와 빌린 시야</h3>",
            "<p><code class=\"kw\">String</code>은 <b>소유</b>하고 자라는 힙 텍스트 — 금고에 보관하는 두루마리입니다. <code class=\"tp\">&amp;str</code>(«문자열 슬라이스»)는 다른 곳에 사는 텍스트를 향한 <b>빌린 시야</b> — 두루마리를 가리키는 책갈피입니다. <code class=\"tp\">\"garden\"</code> 같은 리터럴은 <code class=\"tp\">&amp;str</code>입니다: 글자들은 프로그램 메모리에 구워져 있고, 리터럴은 그것을 향한 창일 뿐입니다. 그래서 <code class=\"tp\">&amp;str</code>은 자랄 수 없습니다: 시야에는 자랄 자기 고유의 땅이 없습니다.</p>",
            "<pre><code>let owned: String = String::from(\"fairy dust\");  // 힙 두루마리
let view: &amp;str = \"fairy dust\";                   // 리터럴 = &amp;str
let borrowed: &amp;str = &amp;owned;                     // 두루마리를 향한 시야
</code></pre>",
            "<p>두 세계의 다리: <code class=\"kw\">to_string()</code>은 빌린 시야에서 새 소유의 <code class=\"kw\">String</code>을 주조하고, <code class=\"kw\">String::from(\"…\")</code>은 같은 일을 하는 형님입니다. 둘 다 글자들을 <b>새 힙 버퍼로 복사</b>합니다 — 시야는 옛 자리를 계속 가리키고, String은 새 자리를 소유합니다. 되돌아가는 것은 공짜입니다: <code class=\"tp\">&amp;some_string</code>은 <code class=\"tp\">&amp;str</code>로 역참조되며(deref 강제), 그래서 <code class=\"tp\">&amp;str</code>을 받는 함수는 <code class=\"tp\">&amp;String</code>도 기꺼이 받습니다.</p>",
            "<pre><code>let view = \"garden\";
let word = view.to_string();          // &amp;str -&gt; String (복사)
let magic = String::from(\"sparkle\");  // 같은 의식, 옛 주문 형태
</code></pre>",
            "<h3>두루마리 키우기: push_str과 push</h3>",
            "<p><code class=\"kw\">String</code>은 Vec처럼 자랍니다(비밀스럽게 옷을 갈아입은 바이트 Vec일 뿐입니다): <code class=\"kw\">push_str(text)</code>은 슬라이스 통째로 덧붙이고, <code class=\"kw\">push('c')</code>는 글자 하나를 덧붙입니다. 둘 다 <code class=\"kw\">mut</code>를 요구합니다 — 자라는 것은 변형입니다. 버퍼는 무대 뒤에서 재할당될 수 있습니다; 두루마리는 그저 길어질 뿐이고, push <b>이전에</b> 빌린 시야는 무효화됩니다(보관소의 선반 함정, 다시).</p>",
            "<pre><code>let mut spell = String::from(\"abra\");
spell.push_str(\"cadabra\");   // 슬라이스 덧붙이기
spell.push('!');             // 글자 하나 덧붙이기
// spell == \"abracadabra!\"
</code></pre>",
            "<h3>format!: 작곡가</h3>",
            "<p><code class=\"kw\">format!</code>은 텍스트를 만드는 정중한 방법입니다: 템플릿과 값을 받아 <b>완전히 새로운 String을 주조</b>합니다 — 빌림 싸움도, 입력 변형도 없이. 위치 슬롯 <code class=\"tp\">{}</code>는 인수를 순서대로 소비하고, <code class=\"tp\">{name}</code> 같은 인라인 캡처는 스코프에서 변수를 이름으로 잡습니다. 같은 미니 언어가 <code class=\"kw\">println!</code>을 움직입니다 — 한 번 배워 어디서나 쓰세요.</p>",
            "<pre><code>let name = \"Ferris\";
let level = 7;
let banner = format!(\"{} reached level {}!\", name, level);
let banner2 = format!(\"{name} reached {level}!\");  // 인라인 캡처
</code></pre>",
            "<h3>세척 도구: trim과 대소문자</h3>",
            "<p><code class=\"kw\">trim()</code>은 <b>양쪽 끝</b>의 공백(스페이스, 탭, 줄바꿈)을 깎아 <code class=\"tp\">&amp;str</code>을 돌려줍니다 — 같은 두루마리 안의 좁은 시야이므로 비용이 없습니다. 가운데는 만지지 않습니다: <code class=\"tp\">\"a  b\".trim()</code>은 이중 스페이스를 그대로 둡니다. 판정 전 사용자 입력을 소독하기에 완벽합니다. 대소문자는 반대 방향입니다: <code class=\"kw\">to_uppercase()</code>/<code class=\"kw\">to_lowercase()</code>는 시야가 아니라 바이트 길이를 바꿀 수 있고(İ는 두 바이트!), <b>새 String</b>을 주조합니다.</p>",
            "<pre><code>let raw = \"  RustGarden  \";
let clean = raw.trim();           // \"RustGarden\" (&amp;str 시야)
let loud = clean.to_uppercase();  // \"RUSTGARDEN\" (새 String)
</code></pre>",
            "<h3>chars: 글리프 컨베이어</h3>",
            "<p>String은 바이트이고(UTF-8) 글리프는 1–4바이트일 수 있습니다 — 그래서 인덱싱 <code class=\"tp\">s[0]</code>는 금지됩니다: 0번 바이트가 꼭 글자가 아니니까요. 텍스트를 <i>논리적으로</i> 걷는 길은 <code class=\"kw\">chars()</code>입니다: <code class=\"kw\">char</code>를 하나씩 내놓습니다. <code class=\"kw\">nth(n)</code>은 n번째 글리프를 훔쳐보고(Option — 다 떨어질 수 있음), <code class=\"kw\">count()</code>는 글리프를 셉니다 — <i>바이트가 아니라</i>, 한편 String의 <code class=\"kw\">len()</code>은 <b>바이트</b>를 셉니다. 키릴 문자와 한글이 차이를 생생하게 보여줍니다: «краб»는 4글리프이지만 8바이트입니다.</p>",
            "<pre><code>let runes = \"crab\";
for c in runes.chars() { /* 'c', 'r', 'a', 'b' */ }
let third = runes.chars().nth(2);   // Some('a')
let glyph_count = runes.chars().count();  // 4, 바이트가 아님
</code></pre>",
            "<h3>split: 바늘 뜯기</h3>",
            "<p><code class=\"kw\">split(sep)</code>는 구분자를 따라 시야를 조각내고 <code class=\"tp\">&amp;str</code> 시야의 게으른 이터레이터를 내어줍니다 — 선반이 필요하면 <code class=\"kw\">Vec&lt;&amp;str&gt;</code>로 collect하세요. <code class=\"kw\">split_whitespace()</code>는 CSV 없는 사촌입니다: 공백의 연속이면 어디든 자르고 빈 조각은 결코 내놓지 않습니다. 조각들은 <b>시야</b>지 복사본이 아닙니다 — 소유 텍스트가 필요할 때만 String으로 collect하세요.</p>",
            "<pre><code>let scroll = \"mana,spell,garden\";
let parts: Vec&lt;&amp;str&gt; = scroll.split(',').collect();
// [\"mana\", \"spell\", \"garden\"]

let words: Vec&lt;&amp;str&gt; = \"a b  c\".split_whitespace().collect();
// [\"a\", \"b\", \"c\"]
</code></pre>",
            "<p><code class=\"kw\">replace(from, to)</code>는 잉크 재작성자입니다: <code class=\"tp\">from</code>의 모든 출현이 <code class=\"tp\">to</code>가 되고 결과는 <b>새 String</b>입니다 — 원본 두루마리는 건드리지 않습니다. trim이나 lowercase와 자유롭게 연결하세요; String을 만드는 체인의 각 단계는 소유권을 다음 단계에 넘깁니다.</p>",
            "<pre><code>let toast = \"old spell\";
let fresh = toast.replace(\"old\", \"new\");  // \"new spell\" (새 String)
// toast는 여전히 \"old spell\": replace는 자리에서 고치지 않음
</code></pre>",
            "<h3>어떤 문자열 타입을 언제?</h3>",
            "<ul><li>저장, 성장, 소유 — <code class=\"kw\">String</code>.</li><li>읽기만 하는 함수 매개변수 — <code class=\"tp\">&amp;str</code>을 받으세요(호출자는 리터럴도 <code class=\"tp\">&amp;String</code>도 넘길 수 있음).</li><li>부품으로 엮기 — <code class=\"kw\">format!</code> (새 String을 주조).</li><li>보이는 글리프 세기 — <code class=\"kw\">chars().count()</code>; 저장소 세기 — <code class=\"kw\">len()</code>.</li><li>텍스트 자르기 — <code class=\"kw\">split</code>/<code class=\"kw\">split_whitespace</code> + <code class=\"kw\">collect</code>.</li></ul>",
            "<h3>텍스트의 소유권 장부</h3>",
            "<p>두 문자열 교리는 그저 텍스트 분장을 한 소유권 챕터입니다. <code class=\"kw\">String::from</code>과 <code class=\"kw\">to_string</code>은 <b>새 소유로 복사</b>하고, <code class=\"tp\">&amp;str</code>은 <b>빌리며</b>, String을 값으로 넘기면 <b>이동</b>하고, <code class=\"tp\">&amp;String</code>을 값으로 받으면 호출자는 빈손이 됩니다. 컴파일러가 텍스트 퍼즐에 트집을 들면 장부에 물어보세요: 이 글자들의 소유자는 누구고, 누가 그저 보고 있나?</p>",
            "<p>훈련 의식: 문장 하나를 골라 다듬고, 소문자로 바꾸고, 단어로 자르고, <code class=\"kw\">format!</code>으로 번호를 붙여 한 줄로 다시 이어 보세요 — 각 단계에서 글리프를 세면서. 쓰는 메서드 하나하나가 이 챕터의 룬입니다; 한 바퀴를 돌면 API 잡지식이 아니라 하나의 작업장으로 느껴지기 시작합니다.</p>",
            "<div class=\"article-banner\">두루마리의 규칙: &amp;str은 빌린 시야, String은 소유의 두루마리; trim과 split은 시야를 빌려주고, to_uppercase, format!, replace는 새 두루마리를 주조합니다.</div>",
        ],
    },

    // C6 traits
    Article {
        chapter_id: "traits",
        lang: "ko",
        title: "트레이트와 제네릭 공방",
        paragraphs: &[
            "<p>지금까지 모든 구조체는 홀로 서 있었습니다: <code class=\"kw\">struct</code>는 자기 필드를 알 뿐, 그것이 전기의 전부였습니다. 공방은 구조체에게 <b>약속</b>을 가르칩니다. <code class=\"kw\">trait</code>은 능력의 계약입니다 — «여기 서명된 것은 복제하고, 비교하고, 인쇄할 수 있다» — 그리고 제네릭은 하나의 청사진이 여러 재료에 맞게 쓰이는 공방의 방식입니다. 둘이 합쳐져 Rust가 동시에 엄격<i>하고</i> 유연하게 유지됩니다.</p>",
            "<h3>계약: 트레이트 선언</h3>",
            "<p>트레이트는 몸통 없는 메서드 <b>시그니처</b>를 나열합니다: 약속이지 증명이 아닙니다. 타입은 <code class=\"kw\">impl TraitFor Type</code> 블록으로 계약에 서명하며 진짜 몸통을 채웁니다. 서명 후에는 타입의 메서드를 점 구문으로 호출할 수 있고, 타입 자체는 트레이트가 요구되는 어디서든 쓸 수 있습니다.</p>",
            "<pre><code>trait Enchantable {\n    fn power(&amp;self) -&gt; i32;              // 약속, 몸통 없음\n    fn describe(&amp;self) -&gt; String {       // 기본 몸통은 허용\n        format!(\"power: {}\", self.power())\n    }\n}\n\nstruct Wand { charges: i32 }\n\nimpl Enchantable for Wand {\n    fn power(&amp;self) -&gt; i32 { self.charges }   // 증명\n}\n</code></pre>",
            "<p>약속의 두 속도를 유의하세요: <code class=\"kw\">power</code>는 몸통이 없어 모든 서명자가 <b>반드시</b> 써야 합니다; <code class=\"kw\">describe</code>는 <b>기본 몸통</b>을 실어 나르므로 서명자는 공짜로 상속합니다(재정의도 가능). 기본 메서드는 트레이트를 작으면서도 유용하게 유지하는 방법입니다 — 소수의 필수 약속 plus 공짜 도구 모음.</p>",
            "<h3>impl Trait: 계약 기반 배관</h3>",
            "<p>타입이 쓰일 자리에서 <code class=\"kw\">impl TraitName</code>은 말합니다: «이 계약에 서명한 어떤 타입이든». <b>매개변수</b>로는 모든 서명자를 받고, <b>반환 타입</b>으로는 정확한 타입을 숨긴 채 어떤 서명자를 반환한다고 약속합니다(함수당 하나의 구체적 타입 — 조건에 따라 다른 타입을 돌려주려면 아래의 <code class=\"kw\">dyn</code>이 필요합니다). 강의 목록이 숨겨진 제네릭입니다.</p>",
            "<pre><code>fn announce(item: &amp;impl Enchantable) -&gt; String {\n    item.describe()   // Wand, Ring, 서명한 무엇이든 동작\n}\n</code></pre>",
            "<h3>Clone과 Copy: 복제자 계약</h3>",
            "<p><code class=\"kw\">Clone</code>은 명시적이고 보이는 복제를 약속합니다: <code class=\"kw\">x.clone()</code> — 코드 리뷰에서 손가락으로 가리킬 수 있는 진짜 메서드 호출. <code class=\"kw\">Copy</code>는 모든 대입에서 공짜이고 조용한 복제를 약속합니다 — 스택 전용 타입(정수, 실수, bool, char; 그것들의 튜플/배열)에게만 허용됩니다. 모든 <code class=\"kw\">Copy</code> 타입은 <code class=\"kw\">Clone</code>이지만 절대 반대는 아닙니다: Copy가 더 엄격하고 조용한 형제입니다. 빌림 검사기의 안도의 한숨은 이 둘로 짜입니다.</p>",
            "<pre><code>#[derive(Clone)]\nstruct Spell { name: String, mana: i32 }\n\nlet a = Spell { name: \"frost\".into(), mana: 30 };\nlet b = a.clone();     // 명시적 복제; `a`는 여전히 텍스트를 소유\nlet n = 7;\nlet m = n;             // i32는 Copy: 조용히, `n`은 여전히 사용 가능</code></pre>",
            "<h3>PartialEq와 Eq: 비교 계약</h3>",
            "<p><code class=\"kw\">PartialEq</code>는 <code class=\"tp\">==</code>/<code class=\"tp\">!=</code> 연산자가 동작한다고 약속합니다(<code class=\"tp\">==</code>가 내리는 곳이 바로 <code class=\"kw\">eq</code> 메서드). «부분적»인 이유는 일부 타입에는 등식 수학을 거부하는 값이 있기 때문입니다 — <code class=\"tp\">f32::NAN == NAN</code>은 거짓. <code class=\"kw\">Eq</code>는 더 엄격한 마커입니다: «모든 값이 정직하게 비교된다» — 정수는 갖고 실수는 못 갖습니다. 대부분의 구조체는 그저 <code class=\"kw\">#[derive(PartialEq, Eq)]</code>를 쓰며 필드별 비교가 자동 생성됩니다.</p>",
            "<pre><code>#[derive(PartialEq, Eq)]\nstruct Rune { glyph: char, tier: i32 }\n\nlet a = Rune { glyph: 'α', tier: 2 };\nlet b = Rune { glyph: 'α', tier: 2 };\nlet same = a == b;     // true — 필드마다\n</code></pre>",
            "<h3>제네릭: 문자 T</h3>",
            "<p>제네릭 함수는 이름 앞에 <code class=\"tp\">&lt;T&gt;</code>를 찍고 <code class=\"kw\">T</code>를 «호출자가 정하는 어떤 타입»으로 말합니다. 소스에는 아무것도 복제되지 않지만 컴파일러는 <b>모노모픽화</b>를 수행합니다: 실제 쓰인 구체적 T마다 컴파일 시점에 특화된 함수 복사본을 찍어냅니다 — Rust의 제네릭은 런타임 비용이 0입니다. 대가: 맨바닥의 <code class=\"kw\">T</code>는 <i>아무것도</i> 못 합니다 — 메서드를 부르려면 T가 트레이트에 서명해야 합니다: <code class=\"tp\">&lt;T: Enchantable&gt;</code>.</p>",
            "<pre><code>fn biggest&lt;T: PartialOrd&gt;(list: &amp;[T]) -&gt; Option&lt;&amp;T&gt; {\n    if list.is_empty() { return None; }\n    let mut best = &amp;list[0];\n    for item in &amp;list[1..] {\n        if item &gt; best { best = item; }\n    }\n    Some(best)\n}\n</code></pre>",
            "<p>제네릭 <b>구조체</b>는 같은 문자를 이름에 얹습니다: <code class=\"tp\">Pair&lt;T&gt;</code>는 T 둘을, <code class=\"tp\">Pair&lt;K, V&gt;</code>는 서로 다른 타입의 키와 값을 쥡니다 — 보관소의 <code class=\"kw\">Vec&lt;T&gt;</code>와 <code class=\"kw\">HashMap&lt;K, V&gt;</code>가 정확히 이렇게 쓰여 있습니다. 하나의 청사진, 많은 재료, 저장소의 코드 중복도 런타임 동적 디스패치도 없습니다.</p>",
            "<pre><code>struct Pair&lt;T&gt; { left: T, right: T }\n\nlet glyphs = Pair { left: 'α', right: 'ω' };\nlet levels = Pair { left: 3, right: 9 };   // 같은 청사진, 다른 T\n</code></pre>",
            "<h3>Display: 인쇄 계약</h3>",
            "<p><code class=\"kw\">println!</code>/<code class=\"kw\">format!</code>의 <code class=\"tp\">{}</code>는 <code class=\"kw\">std::fmt::Display</code> — 사람이 읽는 출력 —를 요구합니다. 자동이 아닙니다: 직접 서명하고 <code class=\"kw\">write!</code>로 포매터에 기록합니다. 시그니처는 의례처럼 보이지만 한 번 천천히 읽을 가치가 있습니다: 포매터에서 빌린 메서드는 반드시 <code class=\"kw\">fmt::Result</code>를 돌려줍니다.</p>",
            "<pre><code>use std::fmt;\n\nimpl fmt::Display for Wand {\n    fn fmt(&amp;self, f: &amp;mut fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {\n        write!(f, \"Wand({} charges)\", self.charges)\n    }\n}\n// println!(\"{}\", wand)가 이제 정중하게 말합니다\n</code></pre>",
            "<h3>dyn Trait: 무대 뒤 패스</h3>",
            "<p>제네릭과 <code class=\"kw\">impl Trait</code>은 <b>컴파일 시점</b>에 결정됩니다(정적 디스패치: 정확한 메서드가 찍혀 들어감). <code class=\"kw\">dyn Trait</code>은 런타임 형태입니다: <i>트레이트 객체</i> — 나중에 정해지는 어떤 서명자, vtable을 통해 도달. 포인터와 간접층의 비용이 들지만 하나의 <code class=\"kw\">Vec&lt;Box&lt;dyn Enchantable&gt;&gt;</code>에 완드, 링, 지팡이를 나란히 담을 수 있습니다. 규칙: 기본은 제네릭, 타입 집합이 진짜 열려 있을 때 <code class=\"kw\">dyn</code>.</p>",
            "<pre><code>let shelf: Vec&lt;Box&lt;dyn Enchantable&gt;&gt; = vec![\n    Box::new(Wand { charges: 3 }),\n    Box::new(Staff { runes: 9 }),\n];\nfor item in &amp;shelf {\n    println!(\"{}\", item.describe());  // 런타임 vtable 호출\n}\n</code></pre>",
            "<h3>where: 정중한 제약 장부</h3>",
            "<p><code class=\"tp\">&lt;&gt;</code> 목록이 무거워지면 — 제네릭 둘에 각각 제약 둘 — 시그니처가 익사합니다. <code class=\"kw\">where</code>는 모든 제약을 시그니처 뒤의 읽기 쉬운 장부로 옮기며 의미는 동일합니다. 같은 규칙, 더 깨끗한 필적; 큰 실전 시그니처는 거의 항상 이렇게 씁니다.</p>",
            "<pre><code>fn archive&lt;T, U&gt;(item: T, ledger: U)\nwhere\n    T: Enchantable + Clone,\n    U: fmt::Display,\n{\n    // 제약이 여기 삽니다, 한눈에 읽힙니다\n}\n</code></pre>",
            "<h3>어떤 도구를 언제?</h3>",
            "<ul><li>여러 타입의 공유 동작 — <code class=\"kw\">trait</code>를 정의하고 <code class=\"kw\">impl … for</code>로 서명.</li><li>«어떤 서명자든, 정확한 타입은 비공개» — 인수/반환 위치의 <code class=\"kw\">impl Trait</code>.</li><li>값 복제 — <code class=\"kw\">Clone</code>(명시) 또는 <code class=\"kw\">Copy</code>(조용, 스택 전용).</li><li>값 비교 — <code class=\"kw\">PartialEq</code> (정직한 전체 동등엔 <code class=\"kw\">Eq</code> 추가).</li><li>하나의 청사진, 많은 타입, 런타임 비용 0 — 제네릭 <code class=\"tp\">&lt;T&gt;</code>.</li><li>서명자들의 잡동사니 바구니 — <code class=\"kw\">Box&lt;dyn Trait&gt;</code>.</li></ul>",
            "<p>공방의 조용한 비밀: 여러분이 써 온 표준 라이브러리의 초능력 하나하나 — <code class=\"kw\">.clone()</code>, <code class=\"tp\">==</code>, <code class=\"tp\">{}</code> 인쇄, <code class=\"kw\">Vec&lt;T&gt;</code> — 처음부터 전부 트레이트나 제네릭이었습니다. 1장부터 남의 계약에 서명해 왔던 여러분; 이제 계약을 쓸 차례는 여러분입니다.</p>",
            "<div class=\"article-banner\">공방의 규칙: 트레이트는 약속, impl 블록은 증명, 제네릭은 컴파일 시점에 찍히는 청사진, 그리고 dyn은 손님 명단이 열려 있을 때의 무대 뒤 패스입니다.</div>",
        ],
    },

];

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
