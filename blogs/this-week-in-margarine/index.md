# This week in margarine
A quick update on the progress of my programming language margarine. It's build system, why it won't have a JIT, and other design decisions.

For those who don't know margarine, it's my programming language that's meant to be a replacement for Lua in my own projects, [check out this post](../four-years-five-failures-one-compiler/) if you want to learn more about it.

Hello! So here's the thing, this week I decided that I wanted to be done with margarine as soon as possible.
But I'm also quite tired of having way too many unfinished projects, so I had to decide on a to-do list that I needed to finish before I could call margarine done. And before you click off, a JIT was in this to-do list until yesterday.

Here, I'll show you the to-do list.
- a basic JIT
- finishing up the build system
- a GC
- some bug fixes
- stdlib 

So me being me I went with the one I have never done before, the JIT.

## The JIT
In a previous version of margarine I had used LLVM¹ to compile to native code and as far as I remember it was a horrible experience. Heck when I went back to margarine it didn't even compile even though literally nothing about my build system had changed. 

So this time I decided to use something easier, something shinier, something newer, something called [Cranelift](https://cranelift.dev/). Which claims to be faster than LLVM while admitting that it has less optimisations. That sounds like a good trade-off, right? It probably doesn't have many of the niche optimisations that LLVM has built up over the years but it has the important optimisations, right?

That sounded amazing! Not only did it mean that I didn't need to deal with LLVM's jankness but also that the JIT will compile code faster!

Now, I don't need margarine's JIT to be perfect or even optimized. I just want to add a JIT to the runtime to make it faster and I want to get it over with quickly so I could move onto the next thing in the to-do list. So I just made the JIT convert my interpreter one-to-one.

I think this'll make more sense if I showed some code, so let's take the `AddInt` instruction. (fyi, margarine's interpreter is a stack based interpreter)

```rs
AddInt => {
   let rhs = stack.pop().as_int();
   let lhs = stack.pop().as_int();
   stack.push(Value::Int(lhs + rhs))
}
```

and so the JIT would just do that exact same thing but inlined. Which made the JIT the same as the interpreter except without the instruction over-head and any optimisations Cranelift could apply along-side it. 

Now this meant that the JIT would often generate redundant loads and stores, stuff like store x then immediately override x. Which was fine, I thought. It sounds like a basic optimisation, surely Cranelift wouldn't even break a sweat, right?

Other than the fact that this isn't much of a basic optimisation, Cranelift doesn't even try to do anything about it. So I was left on my own.

Is this an unsolvable problem? No. 
Is there nothing I can do about it? Certainly not. 
Is this a problem I wanted to have when I decided to JIT? No, but it is what it is. 

So I got onto brainstorming how I could go about solving this. 
For one, I could switch to a register based VM which would immediately eliminate any interaction with the stack since I can just use cranelift variables. But this would probably require me to rewrite my entire VM and some form of register-allocation would be practically necessary to keep the register count under 256 (one byte).

Or I could convert the stack into cranelift variables at runtime, though that would require me to know the stack height at any given instruction. The idea would be to convert each stack slot to a Cranelift variable and simulate push & pops.

There's many ways I could make this JIT thing work, but after talking to people I'm not sure if it's even worth it. Sure, it'd make things faster but there's a lot more I should be focusing on right now (many things that when implemented would probably require the JIT to be reworked anyways).

So I moved onto the next item on the list.

¹: Writing this some part of me wishes I never ditched compiling to native code. I know it was the right decision but the *speed* is very alluring especially now that I don't even have a JIT. Woman in the red dress or something I dunno.

## The build system
This part of margarine has gone through a few iterations already. At the start, I just had to specify every single file I wanted to include in the CLI tool, which led to commands like

```
margarine std std/duration std/list std/rand raylib raylib/keys raylib/window flappy_bird
```

This ability to be extremely granular on what files to import was a design decision that I wanted to preserve since margarine is meant to be an embeddable language. But of course, we also need margarine to stand on its own, so I had to figure out something else. 

The first thing I tried was to just copy Rust. I had a `build.toml` file where you could specify your dependencies, and your code would be in a `src/` directory with the entry point being a function named `main` in `src/main.mar`. 

This worked for a bit; the CLI tool would fetch the github repos of the dependencies, cache them in an `artifacts/` folder and compile and then run the program. But I wanted more, because if we're creating this entire build system, we might as well make it more powerful.

That brings us to `build.mar`. When compiling, the `std` library is always included, and then the `build` function is called. The `std` library exposes the `CompilationUnit` type, an object that represents the current program's files and dependencies.

```rs
fn build() {
 var unit = CompilationUnit::new();
 unit.fetch("somelib", "https://some.other/library");
 unit.import("std/");
 unit.build();
}
```

This version isn't final because it doesn't handle libraries having dependencies. My current idea is to change the build function's signature to `fn build(): CompilationUnit`, letting libraries return their dependency info, allowing the creation of a single unit recursively.

Side note, here's a funny thing I discovered while working on this. Since margarine can run with compiler errors, as long as you don't actually execute the errors, you can actually compile the current file with new libraries.

```rs
fn build() {
 var unit = CompilationUnit::new();
 unit.fetch("rand", "https://github.com/todaymare/margarine-rand");
 unit.import("");
 unit.build()
}

fn main() {
   print(rand::rand())
}
```

which will work perfectly fine. Apart from spamming your terminal with compiler errors.

Back to the build system, another thing I recently considered was to go with Go style imports. You know, just
```
extern "https://github.com/todaymare/margarine-std" as std
```

in the middle of the program. This would allow me to not have a build system, but I'm not sure. I'll have to think more about this whole build system thing.

It's kind of annoying to constantly be working on things that aren't guaranteed to be finished, so I just decided to quickly write up something that's very important and is pretty much as complete as I'll ever make it be

## The GC
> is garbage collection just a metaphor for letting go?  
> \- William Shakespeare

We're coming to the end of the week and while I have experimented with a lot of things I wouldn't call those two complete just yet. However, the garbage collection for margarine is complete. It's definitely not the fastest or the most optimal but it does clean up garbage when needed so it's fine. 

What I did you might ask? I just copy-pasted the garbage collection I had for my old project [azurite](https://github.com/todaymare/azurite). I know I know lame! It's just a basic mark & sweep world-stopping tracing garbage collector. 

There isn't much to say about it. It just goes through the stack, marks objects as live and then kills any that are dead. Oh and I guess it's only called when you fail to allocate an object.

## End of the Week
Anyway, that's where margarine stands this week. A tiny bit more of a real boy.

Thank you for reading so far and if you enjoyed this post or have feedback on any of the design decisions I talked above consider joining [my discord server](https://discord.gg/t7gNX8Kp72), or you can e-mail me at [contact@daymare.net](mailto:contact@daymare.net).
