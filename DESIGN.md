# A talk about the how the language should look like

Just thinking outpaper, sorry for the grammar
I am an average programmer, meaning that the things that I want from
a language, will need most of the other programmers. Not gonna chase
some eccentric things that I have never used.

Good language means taking all the cool ideas from different languages,
and putting them into one. Same thing as Google did with their search
engine, took some from Yahoo, AskJeeves and boom. I will not reinvet
the wheels.

I like JS and Rust as well, and I feel like I would enjoy a language
that will look like them. Gleam is another one that I like. If the
syntax of the language does not apeal to me, I won't bother a lot.
Also, it is fun making a language of my own, for learning purpose.

I feel that the Pony language is a much better fit than BEAM languages.
It feels right, and I don't have the fault tolerance thing to think about.

Reference capabilities is where the things are at, it's just the matter
of make them not spooky.

## Desired things
- patter matching like gleam/elixir
- functions overload
- rusty/TSy syntax
- ability that you can do with Pony, you should do it with this one
- take inspiration for some things from Savi
- composition > inheritense
- 80/20 functional to oo things. No interfaces, abstract classes, yes objects


## The looks

actor Main {
    let mut age = 23;

    be update_age(by_how_much: (Int | String)){
       self.updating_age(by_how_much)
    }
    fn updating_age(by: Int)  {
        self.age =+ by;
    }
    fn updating_age(by: String)  {
        let newly = by
            |> String.trim_string()
            |> String.parse_to_int()

        self.age =+ newly;
    }
}
