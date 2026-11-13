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
