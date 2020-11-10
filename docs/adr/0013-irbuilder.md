# 13. irbuilder

Date: 2020-11-10

## Status

2020-11-10 proposed

## Context

Use IRBuilder to build code:

```cpp
void createIRWithIRBuilder() {
    LLVMContext Context;
    Module *mod = new Module("sum.ll", Context);

    //1、创建IRBuilder
    IRBuilder<> builder(Context);
    //2、创建main函数
    FunctionType *ft = FunctionType::get(builder.getInt32Ty(),false);
    Function *mainfunc = Function::Create(ft, Function::ExternalLinkage, "main", mod);
    //到此为止之创建了main函数，但是函数体内的包含的Instruction没有添加，因此需要添加。

    //3、创建基本块（这个基本块是空的无内容）
    BasicBlock *entry = BasicBlock::Create(Context,"entrypoint",mainfunc);

    //4、设置插入点:插入点设置成相应BasicBlock，<#后面用builder创建的指令都会追加到这个BasicBlock里了#>
    //!!!: - 理解：上面的方式是通过直接往BasicBloock中添加Instruction方式来构造基本的basicBlock，这里借助IRBuilder方式，往basicBlock中添加命令。
    builder.SetInsertPoint(entry);

    //5、添加全局字符串（IR中字符串全部为全局变量，使用数据序列来表示，每个元素是一个char类型）
    Value *helloWorld = builder.CreateGlobalStringPtr("hello world!\n");
    //6、创建put函数
    //1)指定函数参数类型，装在一个数组中`
    std::vector<Type*> putsargs;
    putsargs.push_back(builder.getInt8Ty()->getPointerTo());
    ArrayRef<Type*>  argsRef(putsargs);
    //2）指定函数返回值类型
    FunctionType *putsType = FunctionType::get(builder.getInt32Ty(),argsRef,false);
    //3)创建“函数调用”，而不是创建函数
    FunctionCallee putsFunc = mod->getOrInsertFunction("puts", putsType);

    //7、调用函数（<#理解：通过createXXX创建出来的所有指令都在SetInsertPoint后面#>）
    builder.CreateCall(putsFunc,helloWorld); //这是创建方法的指令

    //8、创建返回ret指令
    ConstantInt *zero = ConstantInt::get(IntegerType::getInt32Ty(Context), 0);
    builder.CreateRet(zero);

    //9、验证。这一步待定！
    llvm::VerifierAnalysis::Result Res;
    Res.IRBroken = llvm::verifyModule(*mod, &dbgs(), &Res.DebugInfoBroken);

    mod->dump();

}
```


## Decision

Decision here...

## Consequences

Consequences here...
