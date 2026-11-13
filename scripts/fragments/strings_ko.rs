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
