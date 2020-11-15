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

Summary$analysis() {

}

Summary$* {
    new(int a){}
    analysis(int a){}
}
