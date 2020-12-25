pkg examples

default$say_hello() {
    println("你好，世界！");
}

default$b() {
    say_hello();
}

default$main() {
    b();
}
