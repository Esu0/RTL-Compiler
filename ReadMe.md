# rustで構文解析

## what is this?

rustで構文解析をしようってプロジェクト。  
最終的にはアーキテクチャ開発に使われるRTLのコンパイラを作って、仮想マシン(といってもだいぶ高級)的な感じで動かすつもり。

## 実行例

```c
a=40+30;
b = (a + 3 * 6 <= 3);
c = b + 2;
a + b * c <= b - c * a;
```

を入力するとトークンにして...

```c
[["a", kind:Ident], ["=", kind:Reserved], [40, kind:Number], ["+", kind:Reserved], [30, kind:Number], [";", kind:Reserved], ["b", kind:Ident], ["=", kind:Reserved], ["(", kind:Reserved], ["a", kind:Ident], ["+", kind:Reserved], [3, kind:Number], ["*", kind:Reserved], [6, kind:Number], ["<=", kind:Reserved], [3, kind:Number], [")", kind:Reserved], [";", kind:Reserved], ["c", kind:Ident], ["=", kind:Reserved], ["b", kind:Ident], ["+", kind:Reserved], [2, kind:Number], [";", kind:Reserved], ["a", kind:Ident], ["+", kind:Reserved], ["b", kind:Ident], ["*", kind:Reserved], ["c", kind:Ident], ["<=", kind:Reserved], ["b", kind:Ident], ["-", kind:Reserved], ["c", kind:Ident], ["*", kind:Reserved], ["a", kind:Ident], [";", kind:Reserved], [None, kind:Eof], ]
```

構文木にしてくれる。

```c
0th statement:
        |[type:Lvar, value:"a"]
[type:Assign]
        |       |[type:Num, value:40]
        |[type:Add]
        |       |[type:Num, value:30]

1th statement:
        |[type:Lvar, value:"b"]
[type:Assign]
        |       |[type:Num, value:3]
        |[type:GreaterEq]
        |       |       |[type:Lvar, value:"a"]
        |       |[type:Add]
        |       |       |       |[type:Num, value:3]
        |       |       |[type:Mul]
        |       |       |       |[type:Num, value:6]

2th statement:
        |[type:Lvar, value:"c"]
[type:Assign]
        |       |[type:Lvar, value:"b"]
        |[type:Add]
        |       |[type:Num, value:2]

3th statement:
        |       |[type:Lvar, value:"b"]
        |[type:Sub]
        |       |       |[type:Lvar, value:"c"]
        |       |[type:Mul]
        |       |       |[type:Lvar, value:"a"]
[type:GreaterEq]
        |       |[type:Lvar, value:"a"]
        |[type:Add]
        |       |       |[type:Lvar, value:"b"]
        |       |[type:Mul]
        |       |       |[type:Lvar, value:"c"]
```
