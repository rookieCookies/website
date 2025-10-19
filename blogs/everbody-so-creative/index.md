# Everybody's so Creative!
After 4 years with Rust, I love the language – but I’m starting to think the ecosystem has an abstraction addiction. Or: why every Rust crate feels like a research paper on abstraction.

Hi, it’s me again. Back at it with another rant thinly disguised as a "think piece."

## The Question Nobody Wants to Answer  
Have you ever wanted to build something simple in Rust?
Maybe you reach for `bevy`, or `wgpu`, or some other library that promises everything you didn't ask for. 

And then you hit that moment – you're debugging, you hit "Go to Definition", and suddenly you're free falling through ten layers of traits, macros, and generics just to figure out how a buffer updates.

If that sounds familiar, congratulations: you've met Rust's favourite pastime – abstraction as performance art.

## Why Are We Like This?
I get it – abstractions are cool. They’re supposed to hide complexity so we can focus on cooler stuff. And Rust loves that idea. Traits, generics, lifetimes – layer upon layer of "don’t worry about it honey."

Take `nalgebra`. Fantastic crate – powerful, flexible, and deeply overqualified for 90% of use cases.

It’s not that I hate nalgebra – it’s brilliant. I just hate that this kind of maximalism has become the default.  

Want to do something slightly off-script?  
That’ll be three trait bounds, one custom derive, and a spiritual journey through `src/internal/utils/mod.rs`

You might say these libraries are built this way because we don’t know what the user might want – and fair enough, that’s been the curse of library design since the dawn of libraries. But not every problem needs a skyscraper of abstractions; most of the time, all you need is a shed.

Take `glam`, for example, which doesn’t try to solve philosophy – it just does math. You don’t need a PhD in generics to understand what `Vec3::normalize()` does, and that’s exactly the point.  

But the thing is – `nalgebra` isn't an isolated example. It’s cultural.

And that’s the real cost of abstraction – it makes the ceiling higher, but it also makes the floor invisible. (Shakespeare got nothin’ on me)

The real tragedy? Once you see it, you start writing that way too.
You start thinking, "Well, maybe I should make this generic in case someone wants to use quaternions instead of matrices..." and suddenly, congratulations – you're building for someone who doesn't exist.

## Rust, the Language
Rust is a great language. I’ll be the first to say that. It’s not perfect – no language is except [margarine](https://github.com/todaymare/margarine) – but it’s the tool I reach for first.  

That said, the Rust ecosystem feels like it's divided into two sides. (Ironic, isn’t it?)  

On one side, you’ve got artists. The folks who treat Rust like an art form – every crate is a masterpiece of generics, lifetimes, and zero-cost abstractions. They push the language to its limits, and honestly? It’s incredible to watch.  

On the other side are the people trying to ship things, the ones who’d use Zig if it weren’t allergic to syntactic sugar. They don’t care about elegance or clever abstractions – they just want their code to compile before they age into retirement.

That phrasing sounds harsh, but the thing is: neither side is wrong. 

Let's get one thing straight: over-engineering in Rust is incredibly fun. The language hands you these shiny tools and it's hard not to play with them. It's like LEGO for programmers.

And hey, when you can push the boundaries of abstractions and keep things fast? It feels like you've cracked the code to the universe.

But still; most people just want to see what the code does, not spelunk through an art installation of traits.

The core problem is that Rust being Rust turns code you can’t understand into the default.
The community shows up to tell you that if you don’t over-complicate it, you’re writing unidiomatic Rust.

If "Go to Definition" can’t take me to your implementation and I have to dig through your GitHub repo just to see how Matrix4::mul works – can I really say I know the code I’m using?

For many people, maybe that’s fine. But every dependency you bring in is still your responsibility. Obviously I don’t understand every library I use – that’d be absurd – but I’d like to live in a world where I can understand the code I bring in.

And just to be clear: abstraction isn’t the enemy. It’s what lets you write 3D engines or HTTP servers without caring about hardware or the TCP stack.
The problem is when we build for someday instead of today.

I’m not saying stop writing clever code – just make sure it earns its keep. When you feel that "maybe I should make this generic" impulse, ask: Who benefits from this, today?
If the answer is "future me," maybe wait until future you actually shows up.


Here’s my rule of thumb: **keep "Go to Definition" useful.**


# The Conclusion
I've been on both sides of the extreme. Heck, my first time trying to learn OpenGL I tried to write a compile-time zero-cost abstraction for it while I was learning it. Later, I just went with raw OpenGL in a dozen different projects.
So, yeah, I've lived both sides of the story.

From that experience, I've learned one thing: things are almost always easier when there are fewer moving parts. Simpler code doesn't mean worse code – it just means you can still understand it six months later.

So maybe try the other side of the spectrum if you haven't. Or don't.  

Anyway, this post will probably age terribly. Someone will send it to me in two years while I’m working on my eighth trait-based ECS library written entirely in macros.

But for now, I just wanted to encourage maybe one person to try to write code that's just code... not abstracted, not filled with traits or generics... just code.

Maybe one day the novelty will wear off, and Rust will chill out.  

Until then, I'll be over here using `glam`.

If you enjoyed this post, considering tipping me on- well, I haven't set up my ko-fi yet, but I'd love it if you could whisper 'nice post' at your screen for now.
