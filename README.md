# correct
A spelling correction suggester thing. Kind of inspired to write this by [an article I read][article], but I haven't gotten around to writing the same algorithm he wrote just yet. Instead, I've been using a Rust library called [strsim][strsim] to do a fugly kind of brute force comparison against everything in the dictionary.

I'll work on implementing that other thing later on. Before I do that, I kind of want to figure out a way to return multiple possible matches. Not sure how to do that just yet other than implementing some damn collection that keeps only a given number of items.

[article]:http://www.norvig.com/spell-correct.html
[strsim]:https://crates.io/crates/strsim
