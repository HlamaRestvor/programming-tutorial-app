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
