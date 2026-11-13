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
