pkg charj

struct Summary {
  	Name   : string
	FanIn  : int
	FanOut : int
}

struct Hello {
    summary : Summary
}

Summary$new(int a) {

}

Summary$analysis() -> int {

}

Summary$* {
    build(int a)  {}
    init(int a) -> int {}
}
