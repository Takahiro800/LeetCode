## Rubyのソースコードについて
- 内部の実装を知らないと、どうすれば早くなるかがわからないのでRubyのソースコードを確認できるようにした
  - [ruby/ruby: The Ruby Programming Language](https://github.com/ruby/ruby)
  - [【超入門】キミにも読める！Rubyのソースコードの歩き方 - Qiita](https://qiita.com/suketa/items/f66ba5ad536d11ff0639)

# 1 Two Sum
## 二重ループを使わずにHashで検索をすると効率が良い。
```ruby:
def two_sum(nums, target)
  nums.each_with_index.each_with_object({}) do |(n, i), hash|
    return [hash[target - n], i] if hash[target - n]

    hash[n] = i
  end
end
```
- [ ] `each_with_index`と`each_with_object`を繋げて利用した
- [ ] 順序はどちらでも良い。
- [ ] 引数の設定に注意する

# 36 valid-sudoku
- ひとつのマスに注目するのではなく、転置な場所も同時に見ている感じ？
```ruby:
row << board[y][x]

# こうすることで、xを増やすだけで、（別のマスに注目することになるが）列についても同時に処理している
column << board[x][y]
```

- 各列・行・スペースが有効かどうか( 重複した値が存在しないか )は、他の問題同様に、Hashを使った。
```ruby:
def valid?(array)
  array.each_with_object({}) do |cell, hash|
    next if cell == '.'
    return false if hash.key?(cell)

    hash[cell] = true
  end

  true
end
```

# 49.group-anagrams.rb
- [ ] HashよりもArrayの方がメモリを使わずに済む？
  - [ ] joinした結果、容量が不要なだけかも
    - [ ] 多分、そう。`join`しなければほぼ変わらないことを確認した

## 解答
```ruby:
def group_anagrams(strs)
  anagrams = Hash.new { |h, k| h[k] = [] }

  strs.each do |str|
    hash = Hash.new(0)
    str.each_char { |c| hash[c] += 1 }
    anagrams[hash].append(str)
  end

  anagrams.values
end
```

- Hash.new([]) は同じオブジェクトを参照するために予期せぬものとなるので注意。
  - 初期値が数値の場合は、問題なく動作するようにver2.4くらいで調整が入ったらしい

## Rubyらしい別解
```ruby:
def group_anagrams(strs)
  strs.group_by { |str| str.chars.sort }.values
end
```

# 217
## Array#uniqのコードを追う
```c:array.c
rb_define_method(rb_cArray, "uniq", rb_ary_uniq, 0);
```
- Array#uniq は、`rb_ary_uniq`で定義されている

```c:array.c
static VALUE
rb_ary_uniq(VALUE ary)
{
    VALUE hash, uniq;

    if (RARRAY_LEN(ary) <= 1) {
        hash = 0;
        uniq = rb_ary_dup(ary);
    }
    else if (rb_block_given_p()) {
        hash = ary_make_hash_by(ary);
        uniq = rb_hash_values(hash);
    }
    else {
        hash = ary_make_hash(ary);
        uniq = rb_hash_values(hash);
    }

    return uniq;
}
```

```c:array.c
static VALUE
ary_make_hash(VALUE ary)
{
		// ここは適当にhash作ってるだけっぽい
    VALUE hash = ary_tmp_hash_new(ary);

		// これを見る
    return ary_add_hash(hash, ary);
}
```

- `ary_add_hash`を見る

```c:
static VALUE
ary_add_hash(VALUE hash, VALUE ary)
{
    long i;

    for (i=0; i<RARRAY_LEN(ary); i++) {
        VALUE elt = RARRAY_AREF(ary, i);

				// 難しかったので一旦やめた
        rb_hash_add_new_element(hash, elt, elt);
    }
    return hash;
}
```

## 別解1
```ruby:
def contains_duplicate?(nums)
	nums.each_with_objetc(Set.new) do |num, set|
		return true unless set.add?(num)
	end.empty?
end
```
### Setについて
- [ ] Arrayの直感的な操作とHashによる高速な処理のハイブリッドという立ち位置

```ruby:Set#add?
  # Adds the given object to the set and returns self.  If the
  # object is already in the set, returns nil.
  #
  #     Set[1, 2].add?(3)                    #=> #<Set: {1, 2, 3}>
  #     Set[1, 2].add?([3, 4])               #=> #<Set: {1, 2, [3, 4]}>
  #     Set[1, 2].add?(2)                    #=> nil
  def add?(o)
    add(o) unless include?(o)
  end
```
- `include?(o)`の場合は、何も記述指定なので`nil`を返す
- [x] `.empty?`が謎。return の時点で処理が完了しているのではないのか？
  - => 全ての要素に対して処理が終わったら、endまで来る。
  - このとき、メソッドとしては`false`を返したい。
  - なので、empty?を返している.
- [ ] `empty?`と`nil?`の違い
  - empty?は存在するが、空である。nil?はそもそも存在しない
- これだと渡させる引数 `nums`がemptyの場合にtrueを返すことになる

## 別解2
```ruby:
def contains_duplicate?(nums)
	nums.each_with_objetc(Set.new) do |num, set|
		return true unless set.add?(num)
	end and false
end
```

- `and false` 独特な表現である。
  - 左辺（つまり、each_with_object）が途中でreturnせずに全ての要素について処理が終わった場合に、and以降を評価する。
  - 必ずfalseを使うので、each処理が途中でreturn しなければ必ずfalseを返す

### まとめ
- [ ] `Array#uniq`は一度、hashを作って、そのvalueをarrayにすることで処理している
- [ ] ということは、Arrayを一旦hashにして、そのkeyを取得とかでも同じくらいの精度になる?

# 242
```ruby:
# @param {String} s
# @param {String} t
# @return {Boolean}
```

## Hashのデフォルトvalueって設定できる？
- できる
```ruby
	Hash.new(0)
```

## Hashの基礎
## シンボルに変換
`to_sym`
## split
- `.split('')`
- `chars`の方が端的かも

## 解１
```ruby:
def is_anagram(s, t)
	convert_to_hash(s) == convert_to_hash(t)
end

def convert_to_hash(str)
	str.split('').each_with_object(Hash.new(0)) do |char, hash|
		hash[char.to_sym] += 1
	end
end
```
- 最初に自分で書いた
- `each_with_object`を初めて自分で使った

## 解2
```ruby:
def is_anagram(s, t)
  s.chars.sort == t.chars.sort
end
```
- 別解。hashは使わずとも、sortしてあげれば比較できる

## 別解3
```ruby:
def is_anagram(s, t)
	s.chars.tally == t.chars.tally
end
```

- `module Enumrable#tally`
  - return hash
  - Each key is an element in self.
  - Each value is the number elements equaal to that key.
  - when with hash argument, add each value, this may be usefull for accumulating tailes across multiple enumrables;

# 347
- `bucket sort`を使った

## Bucket Sortとは？
- Hashのようにket, valueの組み合わせを持つような事象に関して、大小を比較し、上位・下位複数の〜を取得する際に便利。
  - `Enumrable#max_by`と同じかも？
- 今回はnumsに含まれる回数に着目する課題であったので、key: 数値、value:回数として、values.max+1(0 originのため)のArrayを用意した

## 解答
- O(n)
- O(n)

```ruby:
def top_k_frequent(nums, k)
  hash = nums.tally
	counter_array = Array.new(hash.values.max + 1) {[]}

	hash.each_pair { |key, value| counter_array[value].append(key) }
	counter_array.flatten.last(k)
end
```

## 別解
- `max_by`を使う

```ruby:
def top_k_frequent(nums, k)
  hash = nums.tally
	hash.max_by(k) { |_, value| value}.to_h.keys
end
```

## 誤答
```ruby:
def top_k_frequent(nums, k)
  hash = nums.tally
  max_count = hash.values.max(k)
  max_count.map { |count| hash.key(count) }
end
```
