# Solve ten puzzle

This program is written by Rust.
Since I wrote rust for the first time, this program is so complicated.
If you wanna use this program, please type the below commands.

Rustで初めて書いたプログラムなのでとてもごちゃごちゃしていますが、もしこんなプログラムを使ってみたいという方は、下記のコマンドをお使いください。

``` 
$ cargo run <4-digit number>
```
or 

```
$ cargo run
```

If you designate a number, Expression is appeared.
If else, the number which makes 10 is shown.

4桁の数字を指定したら、その数字に対する式を、指定しなければ全ての4桁の数で10が作れる数を表示します。

At the top of main.rs, there is a statement "const ANS = 10.0". It enables you to change the eval number.
main.rsの中に、const ...があるのでそこを変えてもらったら、11パズルにしたりできます。


......English is too hard :-(
