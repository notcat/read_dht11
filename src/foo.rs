// hey :)
// hi :)

trait Animal {
    fn speak() -> String;
}

struct Dog {
    name: String,
}

/// cat is here
struct Cat {
    name: String,
}

impl Cat {
    fn new(name: String) -> Self {
        Self { name }
    }
}

// so we can just decide what to do when you try to do it

// we are implementing the FRom trait for the Cat struct
// yes a built in trait.
// just like our Animal trait, but it's built in.
// yes and you can implement many different From<T> traits
// where T is whatever type we want to be able to convert into a Cat

// implements From<string> for Cat, which literally means we will be able to convert from a String into a cat
// we just call our Cat::new() function with the string as a name
impl From<String> for Cat {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

// so it doesnt consume?

// it's a reference, vs without it its an Owned value.
// references have a lifetime, 'static is a lifetime thatmeans it'll exist for the life of the program
// sometimes you have to put the lifetime of the reference ur referring to in types so the compiler understands
// ur intention, otherwise the borrow checker gets mad.

// hmm interesting
// i think i am understanding

// parts to a type:
// everything is optional except TypeName.

// & 'static mut TypeName<A, B, C>
// | reference
//   | lifetime
//           | mutability
//               | type
//                      // generics

// TypeName (owned value)
// mut TypeName (mutable owned value)
// &TypeName (reference)
// &mut TypeName (mutable reference)

// uhh, generics are like, functions, variables, and stuff right? yes no?

// lol idk what im saynig rly if it makes any sense

// in the world there is like, Types and then there is the actual code right.
// generics are like variables for the type system. it lets you do things like hold different types within ur type.

// so like the T here is whatever. it's a generic. we don't care what it is, but the person usin gthe List or Vec or whatever
// will probably define T somehow, or it will be inferred.

pub struct DopamineBox<T> {
    // omfg
    items: Vec<T>,
}

impl<T> DopamineBox<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }
}

// ok so we have a way to create a DopamineBox<T>, and T can literally be anything.

pub struct Hoodie;

pub struct Foodies;

pub fn ship() {
    // u see how this is bad right
    // yeah its only getting the first type and asuming the whole thing is the same type

    // yeah so we can fix this multiple ways, either with "dynamic dispatch" or "static dispatch"

    // the difference is dynamic dispatch is like how you have types in javascript/java where you have classes, and
    // classes can implement abstract/interfaces and then you can check if a variable is of that interface. common OOP
    // stuff i guess.

    // dynamic dispatch is "slower" because it has to look up the type of the variable as the code is running, and then
    // refernece stuff to figure out what it is, blah blah. it doesn't have a concrete type.

    // STATIC dispatch is done with basically match{} statements and enums. it's compiled into the program's byte-code
    // with jump statements and handlers for each possible type that can be passed in. this makes the code way faster
    // to run because each outcome is written into the program and it can jump right to the handler fo rthe type with
    // 1 compare operation, vs the very expensive lookup procedure for dynamic stuff that happens in dynamic dispatch.

    // basically javascript, everytime you cann any function on a variable, it's doing huge checks and shit on the type
    // you are, then finding a function pointer, unboxing stuff, then juming to the pointer, creatng all these allocations
    // etc, damon collection, blahb lah.. hahaha the lil pauseimsorry :) im payuing attention !!

    // but here, if you used static dispatch, the code itself is compiled to the fasest boiled form.

    // does it make sense at all lol, idk if it does

    // ok cool yeah i'll show differet ways to do it

    // yeah i can understand the idea i dont understand how static uses match to do it

    // let items = vec![Hoodie, Foodies];

    // let boxx = DopamineBox::new(items);
    // so in our case the best thing would be to like, add a list instead of just 1 item yea?
    // what if you wanted multiple types in the same list
    // like Hoodie and Foodies
    //// lol i guess so but the things in the list can only be 1 type

    // yeah im gonna need a eli5 lol
    // explain like im 5 (years old) (a retard)

    // omfg lol ok like uhhhhh well do you mean like if each item was it's own type, lets try that

    // well it depends really, do you want the list of item types to be embedded in the Type system
    // or represented at runtime..
}

// yeah i get it
// static dispatch example ( dont mind mod it just separates the scopes to make name collisions ok :)
mod staticexample {
    // does this inherit the struct from the parent
    // ah
    use super::DopamineBox;

    pub trait BoxItem {
        fn description(&self) -> String;
        fn weight(&self) -> f32;
    }

    pub struct Hoodie {
        description: &'static str,
    }

    // so do you need to impl every boxitem trait, or could you just do description
    // yea
    // hmm
    // what if something didnt have a weight.. would that just be like rusts equivalent of null or something
    // you have to Some() right 🧠🧠🧠
    // gotcha

    // the trait says it must be a float return type, so we would have to return *something* that is a float
    // else the program wouldn't compile.
    // if something didn't have a weight our BoxItem trait would be kinda badly written, not describing the trait well
    // we could use a Option<f32> return type if something iddnt have a weight i guess. yeah
    // plus, every time we get the weight of an item
    // we'd have to deal with the case of it being None... what does that mean? who knows. but the idea is basically
    // you have to handle each potential branch lol. that's not really relevant to static dispatch i guess specifcail;ly
    // but that idea is useful for static dispathc, handling every branch lol

    // just imagine the BoxItem is the thing that we can put in our dopamine box, we'll require taht every T in the
    // list of items must be a BoxItem, else the program will error.

    // all fns have to be implemented, if that's waht u mean

    impl BoxItem for Hoodie {
        fn description(&self) -> String {
            self.description.into()
        }

        // lets just say all hoodies are 0.5 weight
        fn weight(&self) -> f32 {
            0.5
        }
    }

    pub struct Foodies {
        color: &'static str,
        flavor: &'static str,
        weight: f32,
    }

    // yeah yeah eyah thats the one
    impl Foodies {
        pub fn swallow() {
            // lol
            println!("eaten.")
        }
    }

    impl BoxItem for Foodies {
        fn description(&self) -> String {
            format!("{} colored food", self.color)
        }

        fn weight(&self) -> f32 {
            self.weight
        }
    }

    pub enum ItemType {
        Hoodie(Hoodie),
        Foodies(Foodies), // god bless it autoadding the comma
    }

    impl ItemType {
        pub fn description(&self) -> String {
            // ok but now we just have to match again on self lol

            match self {
                ItemType::Hoodie(hoodie) => hoodie.description(),
                ItemType::Foodies(food) => food.description(),
            }
        }
    }

    // impl BoxItem for ItemType {

    // }

    fn create_box() {
        // two hoodies omg 🥺 id diei dont wanna imagine ple

        // ok :)
        // what lol
        // i am minding!!
        // ok we will make DopamineBox have a .add_item(item) function taht will take any item

        let items = vec![
            ItemType::Hoodie(Hoodie {
                description: "cute hoodie with a cute hood and stuff".into(),
            }),
            ItemType::Hoodie(Hoodie {
                // :(
                description: "cute hoodie (previously owned by spencer)".into(),
            }),
            ItemType::Foodies(Foodies {
                color: todo!(),
                flavor: todo!(),
                weight: todo!(),
            }),
        ];

        let boxx = DopamineBox::new(items);
        let first_item = boxx.items.get(0).unwrap();

        // ok now what is first_item? it's an Enum ItemType ya?

        //but that kinda sucks cos the enum doesn't really do anything for us ty

        // we can uhh, then match on the enum

        // say we want to print hte description out of the items

        for item in boxx.items.iter() {
            // why not

            // beacuse item is an Enum, so now we have to write code to handle each variant of ItemType
            // and get the description from each. lol

            // well they both implement BoxItem right so they have description() 100%

            // but, we don't enforce that... we kinda can't because the enum defines the item types, as long as

            // if something didn't have an i tem.description it would be up to us at this point n the match to determine
            // what to do, lol. see, we passed the blame up and up and up but eventually we have to handle that case.
            // ok so yea that makes sense, what if the foodies or something didnt have a item.description. wouldnt work.
            // okay,

            // now we can do item.description because we implemented it on the enum.
            // and al lthat does is matches on itself to determine what thing to call
            // yeah  but it's still the same amount of code written really, and not very ergonomic stull lol

            // yeah so basically this is the static dispatch i guess, just a bunch of enums and matches.
            // the benefit is it compile sdown very smoothly and the code is super fast with no "lookups".

            // we can do a little better probably, i'll show a cool

            // so we already did the match in the impl function above so we dont need to do it here
            // gttcgha
            // yeah just in a little bit nicer place lol

            // uhhhhh spencer :)
            // hi
            // can me maybe continue tommorrow? 🥺 its late..
            // i wanna see how the dynamic dispatch tomorrow

            // ok ok omg yeah it's 00:39
            //
            // yes lol dynamic dispatch
            //
            // ok if u say so :)
            //
            // spencer are u
            // are u disconnecting? 🥺
            item.description(); // hi spencerrrrrrr 👋👋👋👋👋👋 hahahahaha yeah :) uj can leave and work on pylon or smtn im just chillin ommmgggg
                                // evil. hahaha
                                // noooo i dont wanna 😭😭😭😭😭
                                // i dont want u to go away 🥺🥺

            // oh i'm still here omg haha :)

            // no i'll leave this code window open in workspace :evil:
            // you'll have to close it :)

            // haha :)

            match item {
                ItemType::Hoodie(hoodie) => hoodie.description(),
                ItemType::Foodies(item) => item.description(),
            };

            // yep but imagine how ugly this would all get as time goes on lol, al these enum variants
            // and then having to match() any time we want to do something with the generic "item".

            // if each item was completely different and didn't share any properties or anything, it would make a little
            // more sense to do this because you wouldn't have anything to share anyways.

            // but here we know each item is a item with a description, so it would be cool fi we could just do like
            // item.description() above the match

            // what if we did item.somefunctionthatexistsononebutnotanother()

            // well it wouldn't exist, bercause item is the enum type. ItemType. and that has no functions anyways
            // it's just an enum, all we can do is match on it. we can actually add a function to the enum, watch

            // so this will now return the description of the fkin item lol
            // it already looks hideous lmfao
        }

        //🤨🤨🤨🤨

        // ok

        // swallow() me to mthe foodies. nothing else.
        // yeah, show how you would write a swallow function
        // beacuse you can't swallow the hoodie.. so how do
        //

        // i was surrised when you didnt do that in the first pkace

        // yeah it's not very ergoomic, maybe we can make a struct for each type. that would make sense

        dbg!(first_item.color());

        // so it's good and ahppy with our items

        // but it's kinda hardto write code like this constantly because like what if you wanted to have the items
        // have functions on them, like uhh.. weight?
    }
}

// dynamic dispatch example

pub fn create_list() {
    let a = "spencer";
    let b = "spenther";
    let c = "spence";

    // interesting, i never knew that. i was always confused by seeing <T> in code lol

    // how do you like, handle different types?

    // yeah it just lets you create something that is indeed generic

    // :(
    // see it doesn't know what the T is for the Vec so it's upset. we hven't done anything to even give it a guess.
    // but if we like, try to insert something into the vec, it can go backwards and infer whathe type of T is from the function call
    // ahh see now we want the mutable type of Vec.
    let mut list = Vec::new();

    // now list know sit's a Vec<&str>

    // advanced typing

    // and if we try to fool it, it will know
    // see
    list.push(a);

    list.push(b);

    list.push(c);
}

// yes in this case the static string is not consumable, it will exist for the life of the progrma, so it's best
// represented as a reference, something that can never be dropped.
// where is the reference, &'static str? yeah we're saying the str is a reference &str.   & means reference

// in this case it's "consuming" a reference of the str. but we can give out as many references to the str as we want

impl From<&'static str> for Cat {
    fn from(name: &'static str) -> Self {
        // Self::new(String::from(name))
        Self::new(name.into())
    }
}

// now we can do like:
fn foobar() {
    // 🤨🤨🤨🤨
    let kitty_name = String::from("damon");

    // i wanna create a cat... 🥺

    // yeah so Cat::from() is a "static" method beacuse it doesn't take &self or self as a paramter. it creates a Cat.

    // i litearlly dont know

    // remember the fix....
    // uhhh
    // cmon.
    // u know it, we been
    // literally
    // studying it
    // the whole time
    // :)

    // ur gunna hate me
    // i hate this stupid string shit lol

    // it's a &str not a String
    // looooooool
    // specifically it's a &'static str
    // no it's so good 🥺 trust me
    // String is an owned string
    // &'static str is a reference to a string slice somewhere, and cannot be mutated. 😟😟😓 nyooo i dont wanna be static and immutable 😭😭
    // and beacuse u literally write "damon" in the program, it is static and immutable forever. :)

    // what is the solution without conversion

    // looool ok well the .into() will turn you into a fat "String"!!! but you will always exist

    let cat = Cat::new(String::from("spenther"));
    // oh lmfao that sounds bad
    // so is this like, equally as good as doing "".into()
    // ooooh ok ok ok ok ok ok ok '👍👍👍👍👍👍
    // yes! ::from("spenther")

    // ok epic so go up

    // technically "spenther" still exists somewhere in the program, immutably, but it's in the code section of the program lol
    // every time this is called, it will reference the "spenterh" memeory and create a new String somewhere in the heap and a pointer will point to it, with a length.
    // yes but that is how ever STring is made, unless it's coming from something dynamic.
    // yes it's all doing the same exact thing, just different ways to write it.
    // and really it compiles down to the asme thing too,
    // so just a matter of however uw ant to writ eit.

    // it's beacuse String::from(&str) is literally the function that gets called when you do "foo".into()

    let cat = Cat::from(kitty_name);
    //or

    // wtf illegal
    // yea, it can infer! it's nice like that.
    // uh that should error probably oh ok yeah

    // see this no work because Dog doesn't impl From<String>
    let cat: Cat = kitty_name.into();

    // what was the other example other than .into

    // damon cat
}

impl Animal for Dog {
    fn speak() -> String {
        String::from("moo")
    }
}
