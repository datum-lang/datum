pkg examples

default$compare(int a, int b) {
    if(a > b) {
        return a;
    } else {
        return b;
    }
}

default$is_success(bool c, bool d) {
    if (c && d) {
        return "successed";
    } else {
        return "failed";
    }
}
